#![cfg_attr(test, allow(dead_code))]

wit_bindgen::generate!({
    path: "../../vendor/bones/wit",
    world: "extension",
});

mod box_state;

use std::cell::RefCell;

use bones::core::host_api::{log, publish, subscribe, Level};
use bones_messages::game_core::{
    BodyKind, EntityOp, EntityOpMessage, EntityTransform, LoadTilemap, PhysicsWorlds, Shape,
    Sprite, TilesetImage,
};
use bones_messages::gfx::LoadSprite;
use bones_messages::{DecodeMessage, EncodeMessage, Message};
use box_state::{BoxField, BoxSpawn, BOX_HALF_H, BOX_HALF_W};
use game_messages::{AttackRequested, WILL_ENTITY_ID};

const LEVEL_ONE_TMX: &[u8] = include_bytes!("../assets/level-one.tmx");
const GRASS_PNG: &[u8] = include_bytes!(
    "../../../assets/Pixel Art Top Down - Basic v1.2.3/Texture/TX Tileset Grass.png"
);
const BOX_PNG: &[u8] = include_bytes!("../../../assets/box.png");

const BOX_SPRITE_ID: u32 = 2;
const GRASS_SPRITE_ID: u32 = 3;
const FRAME_SIZE: u32 = 64;
const BOX_ID_START: u32 = 2;
const BOXES: &[BoxSpawn] = &[
    BoxSpawn {
        x: 242.0,
        y: 442.0,
        coins: 1,
    },
    BoxSpawn {
        x: 432.0,
        y: 532.0,
        coins: 2,
    },
    BoxSpawn {
        x: 532.0,
        y: 432.0,
        coins: 3,
    },
];

thread_local! {
    static BOX_FIELD: RefCell<BoxField> = RefCell::new(BoxField::new(BOX_ID_START, BOXES));
}

fn publish_entity_op(op: EntityOp) {
    publish(EntityOpMessage::TOPIC, &EntityOpMessage(op).encode());
}

fn publish_message<M: Message + EncodeMessage>(message: M) {
    publish(M::TOPIC, &message.encode());
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
        collider_half_w: BOX_HALF_W,
        collider_half_h: BOX_HALF_H,
        body_kind: BodyKind::Frictionless,
        worlds: PhysicsWorlds::RETRO,
    });
}

fn spawn_level_entities() {
    for (index, spawn) in BOXES.iter().enumerate() {
        spawn_box(BOX_ID_START + index as u32, spawn.x, spawn.y);
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
        for index in 0..BOXES.len() {
            publish_entity_op(EntityOp::Despawn {
                entity_id: BOX_ID_START + index as u32,
            });
        }
    }

    fn init() {
        BOX_FIELD.with(|field| *field.borrow_mut() = BoxField::new(BOX_ID_START, BOXES));
        subscribe(EntityTransform::TOPIC);
        subscribe(AttackRequested::TOPIC);
        load_level_assets();
        spawn_level_entities();
        configure_camera();
        log(Level::Info, "level-one: game-core setup ready");
    }

    fn on_tick(_dt: f32) {}

    fn on_message(topic: String, sender: String, payload: Vec<u8>) -> Option<Vec<u8>> {
        match topic.as_str() {
            EntityTransform::TOPIC if sender == "game-core" => {
                if let Ok(transform) = EntityTransform::decode(&payload) {
                    BOX_FIELD.with(|field| {
                        field.borrow_mut().update_transform(
                            transform.entity_id,
                            transform.x,
                            transform.y,
                        )
                    });
                }
            }
            AttackRequested::TOPIC if sender == "will" => {
                if let Ok(attack) = AttackRequested::decode(&payload) {
                    let destroyed = BOX_FIELD.with(|field| field.borrow_mut().attack(attack));
                    if let Some(destroyed) = destroyed {
                        publish_message(destroyed.hit);
                        publish_entity_op(EntityOp::Despawn {
                            entity_id: destroyed.hit.entity_id,
                        });
                        publish_message(destroyed.reward);
                    }
                }
            }
            _ => {}
        }
        None
    }
}

#[cfg(not(test))]
export!(Component);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn box_ids_are_derived_from_the_spawn_table_and_do_not_use_wills_id() {
        let ids = (0..BOXES.len())
            .map(|index| BOX_ID_START + index as u32)
            .collect::<HashSet<_>>();
        assert_eq!(ids.len(), BOXES.len());
        assert!(!ids.contains(&WILL_ENTITY_ID));
    }

    #[test]
    fn every_box_contains_a_deterministic_coin_reward() {
        assert!(BOXES.iter().all(|spawn| spawn.coins > 0));
        assert_eq!(
            BOXES.iter().map(|spawn| spawn.coins).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }
}
