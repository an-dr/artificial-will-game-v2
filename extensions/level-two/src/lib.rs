#![cfg_attr(test, allow(dead_code))]

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
use game_messages::WILL_ENTITY_ID;

const LEVEL_TWO_TMX: &[u8] = include_bytes!("../assets/level-two.tmx");
const GRASS_PNG: &[u8] = include_bytes!(
    "../../../assets/Pixel Art Top Down - Basic v1.2.3/Texture/TX Tileset Grass.png"
);
const ROCK_CLUSTER_PNG: &[u8] = include_bytes!("../assets/rock-cluster.png");
const BOULDER_PNG: &[u8] = include_bytes!("../assets/boulder.png");
const SLIME_ONE_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime1/With_shadow/Slime1_Idle_with_shadow.png"
);
const SLIME_TWO_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime2/With_shadow/Slime2_Idle_with_shadow.png"
);
const SLIME_THREE_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime3/With_shadow/Slime3_Idle_with_shadow.png"
);

const GRASS_SPRITE_ID: u32 = 20;
const ROCK_CLUSTER_SPRITE_ID: u32 = 21;
const BOULDER_SPRITE_ID: u32 = 22;
const SLIME_SPRITE_IDS: [u32; 3] = [30, 31, 32];
const ROCK_FRAME_SIZE: u32 = 128;
const SLIME_FRAME_SIZE: u32 = 64;
const SLIME_FRAME_COUNT: u32 = 6;
const SLIME_FRAME_DURATION: f32 = 0.16;
const SLIME_COLLIDER_HALF_W: f32 = 18.0;
const SLIME_COLLIDER_HALF_H: f32 = 14.0;
const ROCK_ID_START: u32 = 100;
const SLIME_ID_START: u32 = 200;

#[derive(Clone, Copy)]
struct Rock {
    sprite_id: u32,
    x: f32,
    y: f32,
    half_w: f32,
    half_h: f32,
    draw_w: u32,
    draw_h: u32,
}

const ROCKS: &[Rock] = &[
    Rock::cluster(128.0, 128.0),
    Rock::boulder(320.0, 144.0),
    Rock::cluster(576.0, 128.0),
    Rock::boulder(832.0, 160.0),
    Rock::boulder(160.0, 320.0),
    Rock::cluster(368.0, 288.0),
    Rock::cluster(672.0, 304.0),
    Rock::boulder(880.0, 336.0),
    Rock::cluster(144.0, 544.0),
    Rock::boulder(368.0, 528.0),
    Rock::cluster(672.0, 512.0),
    Rock::boulder(880.0, 576.0),
    Rock::boulder(192.0, 768.0),
    Rock::cluster(480.0, 800.0),
    Rock::boulder(800.0, 800.0),
];
const SLIMES: &[(u32, f32, f32)] = &[
    (SLIME_SPRITE_IDS[0], 480.0, 288.0),
    (SLIME_SPRITE_IDS[1], 480.0, 576.0),
    (SLIME_SPRITE_IDS[2], 240.0, 528.0),
    (SLIME_SPRITE_IDS[0], 800.0, 512.0),
    (SLIME_SPRITE_IDS[1], 640.0, 752.0),
    (SLIME_SPRITE_IDS[2], 416.0, 720.0),
];

impl Rock {
    const fn cluster(x: f32, y: f32) -> Self {
        Self {
            sprite_id: ROCK_CLUSTER_SPRITE_ID,
            x,
            y,
            half_w: 34.0,
            half_h: 28.0,
            draw_w: 96,
            draw_h: 96,
        }
    }

    const fn boulder(x: f32, y: f32) -> Self {
        Self {
            sprite_id: BOULDER_SPRITE_ID,
            x,
            y,
            half_w: 28.0,
            half_h: 20.0,
            draw_w: 76,
            draw_h: 68,
        }
    }
}

fn publish_entity_op(op: EntityOp) {
    publish(EntityOpMessage::TOPIC, &EntityOpMessage(op).encode());
}

fn load_ruins_map() {
    publish(
        LoadTilemap::TOPIC,
        &LoadTilemap {
            tmx_bytes: LEVEL_TWO_TMX,
            tileset_images: vec![TilesetImage {
                name: "grass",
                sprite_id: GRASS_SPRITE_ID,
                png_bytes: GRASS_PNG,
            }],
        }
        .encode(),
    );
}

fn rock_presentation(rock: Rock) -> SpritePresentation {
    SpritePresentation {
        sprite: Sprite {
            sprite_id: rock.sprite_id,
            frame_w: ROCK_FRAME_SIZE,
            frame_h: ROCK_FRAME_SIZE,
            frame_count: 1,
            frame_duration: 0.0,
        },
        frames_per_row: 1,
        draw_w: rock.draw_w,
        draw_h: rock.draw_h,
        looping: false,
        advance_while_stopped: false,
        flip_h: false,
        flip_v: false,
    }
}

fn load_rock_sprites() {
    for (id, png_bytes) in [
        (ROCK_CLUSTER_SPRITE_ID, ROCK_CLUSTER_PNG),
        (BOULDER_SPRITE_ID, BOULDER_PNG),
    ] {
        publish(LoadSprite::TOPIC, &LoadSprite { id, png_bytes }.encode());
    }
}

fn spawn_rocks() {
    for (index, &rock) in ROCKS.iter().enumerate() {
        let entity_id = ROCK_ID_START + index as u32;
        let presentation = rock_presentation(rock);
        publish_entity_op(EntityOp::Spawn {
            entity_id,
            x: rock.x,
            y: rock.y,
            sprite: Some(presentation.sprite),
            square_color: (0, 0, 0, 0),
            shape: Shape::Rect,
            collider_half_w: rock.half_w,
            collider_half_h: rock.half_h,
            body_kind: BodyKind::Fixed,
            worlds: PhysicsWorlds::RETRO,
        });
        publish_entity_op(EntityOp::SetSprite {
            entity_id,
            presentation,
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
        load_ruins_map();
        load_rock_sprites();
        spawn_rocks();
        load_slime_sprites();
        spawn_slimes();
        configure_camera();
        log(
            Level::Info,
            "level-two: overgrown ruins and idle slimes ready",
        );
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
