wit_bindgen::generate!({
    path: "../../vendor/bones/wit",
    world: "extension",
});

use bones::core::host_api::{log, publish, Level};
use bones_messages::game_core::{
    BodyKind, EntityOp, EntityOpMessage, LoadTilemap, PhysicsWorlds, Shape, Sprite, TilesetImage,
};
use bones_messages::gfx::LoadSprite;
use bones_messages::{EncodeMessage, Message};

const LEVEL_ONE_TMX: &[u8] = include_bytes!("../assets/level-one.tmx");
const GRASS_PNG: &[u8] = include_bytes!(
    "../../../assets/Pixel Art Top Down - Basic v1.2.3/Texture/TX Tileset Grass.png"
);
const BOX_PNG: &[u8] = include_bytes!("../../../assets/box.png");

const WILL_ENTITY_ID: u32 = 1;
const BOX_SPRITE_ID: u32 = 2;
const GRASS_SPRITE_ID: u32 = 3;
const FRAME_SIZE: u32 = 64;

fn publish_entity_op(op: EntityOp) {
    publish(EntityOpMessage::TOPIC, &EntityOpMessage(op).encode());
}

fn load_level_assets() {
    publish(
        LoadSprite::TOPIC,
        &LoadSprite {
            id: BOX_SPRITE_ID,
            png_bytes: BOX_PNG,
        }
        .encode(),
    );
    publish(
        LoadTilemap::TOPIC,
        &LoadTilemap {
            tmx_bytes: LEVEL_ONE_TMX,
            tileset_images: vec![TilesetImage {
                name: "grass",
                sprite_id: GRASS_SPRITE_ID,
                png_bytes: GRASS_PNG,
            }],
        }
        .encode(),
    );
}

fn spawn_box(entity_id: u32, x: f32, y: f32) {
    publish_entity_op(EntityOp::Spawn {
        entity_id,
        x,
        y,
        sprite: Some(Sprite {
            sprite_id: BOX_SPRITE_ID,
            frame_w: FRAME_SIZE,
            frame_h: FRAME_SIZE,
            frame_count: 1,
            frame_duration: 0.0,
        }),
        square_color: (0, 0, 0, 0),
        shape: Shape::Rect,
        collider_half_w: 20.0,
        collider_half_h: 28.0,
        body_kind: BodyKind::Frictionless,
        worlds: PhysicsWorlds::RETRO,
    });
}

fn spawn_level_entities() {
    spawn_box(2, 242.0, 442.0);
    spawn_box(3, 432.0, 532.0);
    spawn_box(4, 532.0, 432.0);
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
    fn shutdown() {}

    fn init() {
        load_level_assets();
        spawn_level_entities();
        configure_camera();
        log(Level::Info, "level-one: game-core setup ready");
    }

    fn on_tick(_dt: f32) {}

    fn on_message(_topic: String, _sender: String, _payload: Vec<u8>) -> Option<Vec<u8>> {
        None
    }
}

export!(Component);
