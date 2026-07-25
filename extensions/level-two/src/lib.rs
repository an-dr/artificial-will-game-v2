wit_bindgen::generate!({
    path: "../../vendor/bones/wit",
    world: "extension",
});

use bones::core::host_api::{log, publish, Level};
use bones_messages::game_core::{
    BodyKind, EntityOp, EntityOpMessage, LoadTilemap, PhysicsWorlds, Shape, TilesetImage,
};
use bones_messages::{EncodeMessage, Message};

const LEVEL_TWO_TMX: &[u8] = include_bytes!("../assets/level-two.tmx");
const STONE_PNG: &[u8] = include_bytes!(
    "../../../assets/Pixel Art Top Down - Basic v1.2.3/Texture/TX Tileset Stone Ground.png"
);

const WILL_ENTITY_ID: u32 = 1;
const STONE_SPRITE_ID: u32 = 20;
const ROCK_ID_START: u32 = 100;
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
    }

    fn init() {
        load_stone_map();
        spawn_rocks();
        configure_camera();
        log(Level::Info, "level-two: stone field ready");
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
