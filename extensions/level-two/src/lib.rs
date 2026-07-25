wit_bindgen::generate!({
    path: "../../vendor/bones/wit",
    world: "extension",
});

use bones::core::host_api::{log, publish, Level};
use bones_messages::game_core::{
    BodyKind, EntityOp, EntityOpMessage, LoadTilemap, PhysicsWorlds, Shape, Sprite,
    SpritePresentation, TilesetImage,
};
use bones_messages::gfx::LoadSprite;
use bones_messages::{EncodeMessage, Message};

const LEVEL_TWO_TMX: &[u8] = include_bytes!("../assets/level-two.tmx");
const STONE_PNG: &[u8] = include_bytes!(
    "../../../assets/Pixel Art Top Down - Basic v1.2.3/Texture/TX Tileset Stone Ground.png"
);
const SLIME_ONE_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime1/With_shadow/Slime1_Idle_with_shadow.png"
);
const SLIME_TWO_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime2/With_shadow/Slime2_Idle_with_shadow.png"
);
const SLIME_THREE_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime3/With_shadow/Slime3_Idle_with_shadow.png"
);

const WILL_ENTITY_ID: u32 = 1;
const STONE_SPRITE_ID: u32 = 20;
const SLIME_SPRITE_IDS: [u32; 3] = [30, 31, 32];
const SLIME_FRAME_SIZE: u32 = 64;
const SLIME_FRAME_COUNT: u32 = 6;
const SLIME_FRAME_DURATION: f32 = 0.16;
const SLIME_COLLIDER_HALF_W: f32 = 18.0;
const SLIME_COLLIDER_HALF_H: f32 = 14.0;
const ROCK_ID_START: u32 = 100;
const SLIME_ID_START: u32 = 200;
const ROCK_COLOR: (u8, u8, u8, u8) = (92, 94, 88, 255);
const ROCKS: &[(f32, f32, f32, f32)] = &[
    (144.0, 144.0, 30.0, 26.0),
    (272.0, 160.0, 24.0, 32.0),
    (416.0, 144.0, 34.0, 24.0),
    (560.0, 176.0, 28.0, 30.0),
    (704.0, 144.0, 32.0, 26.0),
    (864.0, 176.0, 26.0, 34.0),
    (160.0, 304.0, 34.0, 28.0),
    (336.0, 288.0, 24.0, 36.0),
    (624.0, 304.0, 36.0, 24.0),
    (832.0, 304.0, 30.0, 30.0),
    (144.0, 448.0, 28.0, 34.0),
    (304.0, 432.0, 34.0, 24.0),
    (688.0, 448.0, 32.0, 30.0),
    (864.0, 432.0, 24.0, 36.0),
    (160.0, 608.0, 36.0, 26.0),
    (352.0, 576.0, 28.0, 34.0),
    (544.0, 640.0, 34.0, 28.0),
    (736.0, 576.0, 30.0, 36.0),
    (880.0, 672.0, 36.0, 24.0),
    (288.0, 768.0, 30.0, 30.0),
    (512.0, 800.0, 36.0, 28.0),
    (752.0, 784.0, 28.0, 36.0),
];
const SLIMES: &[(u32, f32, f32)] = &[
    (SLIME_SPRITE_IDS[0], 480.0, 288.0),
    (SLIME_SPRITE_IDS[1], 480.0, 576.0),
    (SLIME_SPRITE_IDS[2], 240.0, 528.0),
    (SLIME_SPRITE_IDS[0], 800.0, 512.0),
    (SLIME_SPRITE_IDS[1], 640.0, 752.0),
    (SLIME_SPRITE_IDS[2], 416.0, 720.0),
];

fn publish_entity_op(op: EntityOp) {
    publish(EntityOpMessage::TOPIC, &EntityOpMessage(op).encode());
}

fn load_stone_map() {
    publish(
        LoadTilemap::TOPIC,
        &LoadTilemap {
            tmx_bytes: LEVEL_TWO_TMX,
            tileset_images: vec![TilesetImage {
                name: "stone",
                sprite_id: STONE_SPRITE_ID,
                png_bytes: STONE_PNG,
            }],
        }
        .encode(),
    );
}

fn spawn_rocks() {
    for (index, &(x, y, half_w, half_h)) in ROCKS.iter().enumerate() {
        publish_entity_op(EntityOp::Spawn {
            entity_id: ROCK_ID_START + index as u32,
            x,
            y,
            sprite: None,
            square_color: ROCK_COLOR,
            shape: Shape::Rect,
            collider_half_w: half_w,
            collider_half_h: half_h,
            body_kind: BodyKind::Fixed,
            worlds: PhysicsWorlds::RETRO,
        });
    }
}

fn slime_presentation(sprite_id: u32) -> SpritePresentation {
    SpritePresentation {
        sprite: Sprite {
            sprite_id,
            frame_w: SLIME_FRAME_SIZE,
            frame_h: SLIME_FRAME_SIZE,
            frame_count: SLIME_FRAME_COUNT,
            frame_duration: SLIME_FRAME_DURATION,
        },
        frames_per_row: SLIME_FRAME_COUNT,
        draw_w: SLIME_FRAME_SIZE,
        draw_h: SLIME_FRAME_SIZE,
        looping: true,
        advance_while_stopped: true,
        flip_h: false,
        flip_v: false,
    }
}

fn load_slime_sprites() {
    for (id, png_bytes) in [
        (SLIME_SPRITE_IDS[0], SLIME_ONE_PNG),
        (SLIME_SPRITE_IDS[1], SLIME_TWO_PNG),
        (SLIME_SPRITE_IDS[2], SLIME_THREE_PNG),
    ] {
        publish(LoadSprite::TOPIC, &LoadSprite { id, png_bytes }.encode());
    }
}

fn spawn_slimes() {
    for (index, &(sprite_id, x, y)) in SLIMES.iter().enumerate() {
        let entity_id = SLIME_ID_START + index as u32;
        let presentation = slime_presentation(sprite_id);
        publish_entity_op(EntityOp::Spawn {
            entity_id,
            x,
            y,
            sprite: Some(presentation.sprite),
            square_color: (0, 0, 0, 0),
            shape: Shape::Rect,
            collider_half_w: SLIME_COLLIDER_HALF_W,
            collider_half_h: SLIME_COLLIDER_HALF_H,
            body_kind: BodyKind::Fixed,
            worlds: PhysicsWorlds::RETRO,
        });
        publish_entity_op(EntityOp::SetSprite {
            entity_id,
            presentation,
        });
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

struct Component;

impl Guest for Component {
    fn shutdown() {
        for index in 0..ROCKS.len() {
            publish_entity_op(EntityOp::Despawn {
                entity_id: ROCK_ID_START + index as u32,
            });
        }
        for index in 0..SLIMES.len() {
            publish_entity_op(EntityOp::Despawn {
                entity_id: SLIME_ID_START + index as u32,
            });
        }
    }

    fn init() {
        load_stone_map();
        spawn_rocks();
        load_slime_sprites();
        spawn_slimes();
        configure_camera();
        log(Level::Info, "level-two: stone field and idle slimes ready");
    }

    fn on_tick(_dt: f32) {}

    fn on_message(_topic: String, _sender: String, _payload: Vec<u8>) -> Option<Vec<u8>> {
        None
    }
}

#[cfg(not(test))]
export!(Component);

#[cfg(test)]
mod tests;
