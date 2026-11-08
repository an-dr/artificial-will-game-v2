#![cfg_attr(test, allow(dead_code))]

mod layout;

use bones_messages::game_core::{
    BodyKind, EntityOp, EntityOpMessage, LoadTilemap, PhysicsWorlds, Shape, Sprite,
    SpritePresentation, TilesetImage,
};
use bones_messages::gfx::LoadSprite;
use bones_messages::{EncodeMessage, Message};
use bones_wasm_sdk::bindings::bones::extension::host_api::{log, publish, Level};
use bones_wasm_sdk::bus;
use game_messages::WILL_ENTITY_ID;
use layout::{
    walls, Cast, Prop, PropKind, CHARACTERS, CHARACTER_HALF_H, CHARACTER_HALF_W, PROPS, PROP_KINDS,
    PROP_SCALE, SPAWN_PAD,
};
use sprite_sheet::SpriteSheet;

macro_rules! pack {
    ($path:literal) => {
        concat!("../../../assets/robot_game_assets/", $path)
    };
}

const LEVEL_THREE_TMX: &[u8] = include_bytes!("../assets/level-three.tmx");
const TILES_PNG: &[u8] = include_bytes!(pack!("biomes/white_harbour/white_harbour_tiles.png"));
const PAD_PNG: &[u8] = include_bytes!(pack!("biomes/shared/spawn_pad_anim.png"));
/// A 1x1 transparent pixel. Collider-only entities wear it because game-core
/// draws a sprite-less entity as a square and the renderer ignores the
/// square's alpha, so an "invisible" (0, 0, 0, 0) square shows black.
const INVISIBLE_PNG: &[u8] = include_bytes!("../assets/invisible.png");
const BEAM_PNG: &[u8] = include_bytes!(pack!("biomes/shared/spawn_beam_anim.png"));

const TILES_SPRITE_ID: u32 = 300;
const FIRST_PROP_SPRITE_ID: u32 = 310;
const PAD_SPRITE_ID: u32 = 330;
const BEAM_SPRITE_ID: u32 = 331;
const INVISIBLE_SPRITE_ID: u32 = 332;
const FIRST_CHARACTER_SPRITE_ID: u32 = 340;

const FIRST_WALL_ID: u32 = 1000;
const FIRST_PROP_ID: u32 = 1100;
const FIRST_PROP_COLLIDER_ID: u32 = 1200;
const FIRST_CHARACTER_ID: u32 = 1300;
const PAD_ID: u32 = 1400;
const BEAM_ID: u32 = 1401;

/// Spawn pad and beam strips: four frames left to right at 6 fps.
const PAD_FRAME: (u32, u32) = (96, 56);
const BEAM_FRAME: (u32, u32) = (64, 248);
const PAD_ANCHOR_Y: f32 = 32.0;
const BEAM_ANCHOR_Y: f32 = 232.0;
const STRIP_FRAMES: u32 = 4;
const STRIP_FPS: f32 = 6.0;

/// Character frames are 96 px with feet at (48, 78); the strips are cut so
/// the feet land on the collider's bottom edge, as Will's are.
const CHARACTER_FRAME: u32 = 96;
const FEET_TARGET: (i32, i32) = (48, 48 + CHARACTER_HALF_H as i32);

fn prop_png(kind: PropKind) -> &'static [u8] {
    match kind {
        PropKind::House1 => include_bytes!(pack!("biomes/white_harbour/props/house_1.png")),
        PropKind::House2 => include_bytes!(pack!("biomes/white_harbour/props/house_2.png")),
        PropKind::House3 => include_bytes!(pack!("biomes/white_harbour/props/house_3.png")),
        PropKind::Tree1 => include_bytes!(pack!("biomes/white_harbour/props/tree_1.png")),
        PropKind::Tree2 => include_bytes!(pack!("biomes/white_harbour/props/tree_2.png")),
        PropKind::Lighthouse => {
            include_bytes!(pack!("biomes/white_harbour/props/lighthouse_1.png"))
        }
        PropKind::Pier => include_bytes!(pack!("biomes/white_harbour/props/pier_1.png")),
        PropKind::Boat => include_bytes!(pack!("biomes/white_harbour/props/boat_1.png")),
        PropKind::FlowerPot => include_bytes!(pack!("biomes/white_harbour/props/decor_1.png")),
        PropKind::Rubble => include_bytes!(pack!("biomes/white_harbour/props/rubble_1.png")),
    }
}

fn character_sheet(cast: Cast) -> (&'static str, &'static [u8]) {
    macro_rules! sheet {
        ($stem:literal) => {
            (
                include_str!(concat!(
                    "../../../assets/robot_game_assets/characters/",
                    $stem,
                    ".json"
                )),
                include_bytes!(concat!(
                    "../../../assets/robot_game_assets/characters/",
                    $stem,
                    ".png"
                )) as &[u8],
            )
        };
    }
    match cast {
        Cast::Scrap => sheet!("robot_npcs/npc_scrap"),
        Cast::Radio => sheet!("robot_npcs/npc_radio"),
        Cast::Merchant => sheet!("robot_npcs/npc_merchant"),
        Cast::Gardener => sheet!("robot_npcs/npc_gardener"),
        Cast::SpiritElder => sheet!("spirits/spirit_elder"),
        Cast::SpiritChild => sheet!("spirits/spirit_child"),
        Cast::SpiritFox => sheet!("spirits/spirit_fox"),
        Cast::SpiritDeer => sheet!("spirits/spirit_deer"),
        Cast::SpiritPartridge => sheet!("spirits/spirit_partridge"),
        Cast::SpiritHare => sheet!("spirits/spirit_hare"),
    }
}

fn publish_entity_op(op: EntityOp) {
    bus::publish(&EntityOpMessage(op));
}

fn load_sprite(id: u32, png_bytes: &[u8]) {
    publish(LoadSprite::TOPIC, &LoadSprite { id, png_bytes }.encode());
}

fn still(sprite_id: u32, frame: (u32, u32), draw: (u32, u32)) -> SpritePresentation {
    looping(sprite_id, frame, 1, 0.0, draw)
}

fn looping(
    sprite_id: u32,
    frame: (u32, u32),
    frame_count: u32,
    frame_duration: f32,
    draw: (u32, u32),
) -> SpritePresentation {
    SpritePresentation {
        sprite: Sprite {
            sprite_id,
            frame_w: frame.0,
            frame_h: frame.1,
            frame_count,
            frame_duration,
        },
        frames_per_row: frame_count,
        draw_w: draw.0,
        draw_h: draw.1,
        looping: frame_count > 1,
        advance_while_stopped: true,
        flip_h: false,
        flip_v: false,
    }
}

/// Spawns an entity with a sprite (`None`: the invisible pixel) and, when
/// `collider` is non-zero, a fixed body.
fn spawn(
    entity_id: u32,
    (x, y): (f32, f32),
    presentation: Option<SpritePresentation>,
    collider: (f32, f32),
) {
    let presentation = presentation.unwrap_or_else(|| still(INVISIBLE_SPRITE_ID, (1, 1), (1, 1)));
    publish_entity_op(EntityOp::Spawn {
        entity_id,
        x,
        y,
        sprite: Some(presentation.sprite),
        square_color: (0, 0, 0, 0),
        shape: Shape::Rect,
        collider_half_w: collider.0,
        collider_half_h: collider.1,
        body_kind: BodyKind::Fixed,
        worlds: PhysicsWorlds::RETRO,
    });
    publish_entity_op(EntityOp::SetSprite {
        entity_id,
        presentation,
    });
}

fn load_harbour_map() {
    // Padded to match the `.tmx` tileset's margin 2 / spacing 4.
    let tiles = sprite_sheet::extrude_tiles(TILES_PNG, 64, 2).expect("harbour tileset decodes");
    publish(
        LoadTilemap::TOPIC,
        &LoadTilemap {
            tmx_bytes: LEVEL_THREE_TMX,
            tileset_images: vec![TilesetImage {
                name: "white_harbour",
                sprite_id: TILES_SPRITE_ID,
                png_bytes: &tiles,
            }],
        }
        .encode(),
    );
}

fn spawn_walls() {
    for (index, wall) in walls().into_iter().enumerate() {
        spawn(
            FIRST_WALL_ID + index as u32,
            (wall.x, wall.y),
            None,
            (wall.half_w, wall.half_h),
        );
    }
}

fn prop_sprite_id(kind: PropKind) -> u32 {
    let index = PROP_KINDS
        .iter()
        .position(|k| *k == kind)
        .expect("listed prop kind");
    FIRST_PROP_SPRITE_ID + index as u32
}

fn spawn_props() {
    for kind in PROP_KINDS {
        load_sprite(prop_sprite_id(kind), prop_png(kind));
    }
    for (index, &prop) in PROPS.iter().enumerate() {
        let index = index as u32;
        spawn_prop(FIRST_PROP_ID + index, FIRST_PROP_COLLIDER_ID + index, prop);
    }
}

fn spawn_prop(sprite_entity: u32, collider_entity: u32, prop: Prop) {
    let art = prop.kind.art();
    let draw = (art.size.0 * PROP_SCALE, art.size.1 * PROP_SCALE);
    let presentation = still(prop_sprite_id(prop.kind), art.size, draw);
    // The collider is its own entity: a sprite's entity is its centre, but a
    // prop blocks only at its base.
    if let Some(collider) = prop.collider() {
        spawn(
            collider_entity,
            (collider.x, collider.y),
            None,
            (collider.half_w, collider.half_h),
        );
    }
    spawn(
        sprite_entity,
        prop.sprite_centre(),
        Some(presentation),
        (0.0, 0.0),
    );
}

fn spawn_pad() {
    load_sprite(PAD_SPRITE_ID, PAD_PNG);
    load_sprite(BEAM_SPRITE_ID, BEAM_PNG);
    let duration = 1.0 / STRIP_FPS;
    let (x, y) = SPAWN_PAD;
    // Both anchors land on the pad's centre; the beam draws after the pad.
    let pad_y = y + (PAD_FRAME.1 as f32 / 2.0 - PAD_ANCHOR_Y);
    let beam_y = y + (BEAM_FRAME.1 as f32 / 2.0 - BEAM_ANCHOR_Y);
    let pad = looping(PAD_SPRITE_ID, PAD_FRAME, STRIP_FRAMES, duration, PAD_FRAME);
    let beam = looping(
        BEAM_SPRITE_ID,
        BEAM_FRAME,
        STRIP_FRAMES,
        duration,
        BEAM_FRAME,
    );
    spawn(PAD_ID, (x, pad_y), Some(pad), (0.0, 0.0));
    spawn(BEAM_ID, (x, beam_y), Some(beam), (0.0, 0.0));
}

fn spawn_characters() {
    for (index, character) in CHARACTERS.iter().enumerate() {
        let index = index as u32;
        let sprite_id = FIRST_CHARACTER_SPRITE_ID + index;
        let (json, png) = character_sheet(character.cast);
        let presentation = SpriteSheet::parse(json)
            .and_then(|sheet| {
                let strip =
                    sheet.cut_strip(png, character.animation, sheet.shift_feet_to(FEET_TARGET))?;
                load_sprite(sprite_id, &strip);
                sheet.presentation(
                    character.animation,
                    sprite_id,
                    [CHARACTER_FRAME, CHARACTER_FRAME],
                )
            })
            .unwrap_or_else(|error| {
                panic!("{:?} `{}`: {error}", character.cast, character.animation)
            });
        let collider = if character.cast.solid() {
            (CHARACTER_HALF_W, CHARACTER_HALF_H)
        } else {
            (0.0, 0.0)
        };
        spawn(
            FIRST_CHARACTER_ID + index,
            (character.x, character.y),
            Some(presentation),
            collider,
        );
    }
}

fn configure_camera() {
    publish_entity_op(EntityOp::SetCameraFollow {
        entity_id: WILL_ENTITY_ID,
        viewport_w: 800.0,
        viewport_h: 600.0,
        zoom: 1.0,
    });
    publish_entity_op(EntityOp::SetCameraSmoothing {
        responsiveness: 5.0,
    });
}

/// Every entity id this level spawns, so shutdown despawns exactly that set.
fn entity_ids() -> Vec<u32> {
    let walls = (0..walls().len() as u32).map(|i| FIRST_WALL_ID + i);
    let props =
        (0..PROPS.len() as u32).flat_map(|i| [FIRST_PROP_ID + i, FIRST_PROP_COLLIDER_ID + i]);
    let characters = (0..CHARACTERS.len() as u32).map(|i| FIRST_CHARACTER_ID + i);
    walls
        .chain(props)
        .chain(characters)
        .chain([PAD_ID, BEAM_ID])
        .collect()
}

use bones_wasm_sdk::Guest;

struct Component;

impl Guest for Component {
    fn init() {
        load_sprite(INVISIBLE_SPRITE_ID, INVISIBLE_PNG);
        load_harbour_map();
        spawn_walls();
        spawn_pad();
        spawn_props();
        spawn_characters();
        configure_camera();
        log(Level::Info, "level-three: White Harbour ready, no enemies");
    }

    fn shutdown() {
        // Despawning an id that was never spawned (a prop with no collider)
        // is a no-op in game-core.
        for entity_id in entity_ids() {
            publish_entity_op(EntityOp::Despawn { entity_id });
        }
    }

    fn on_tick(_dt: f32) {}

    fn on_message(_topic: String, _sender: String, _payload: Vec<u8>) -> Option<Vec<u8>> {
        None
    }
}

#[cfg(not(test))]
bones_wasm_sdk::export!(Component);

#[cfg(test)]
mod tests;
