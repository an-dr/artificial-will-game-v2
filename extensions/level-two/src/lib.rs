#![cfg_attr(test, allow(dead_code))]

wit_bindgen::generate!({
    path: "../../vendor/bones/wit",
    world: "extension",
});

mod slime_state;

use std::cell::RefCell;

use bones::core::host_api::{log, publish, subscribe, Level};
use bones_messages::game_core::{
    BodyKind, Collision, EntityOp, EntityOpMessage, EntityTransform, LoadTilemap, PhysicsWorlds,
    Shape, Sprite, SpritePresentation, TilesetImage,
};
use bones_messages::gfx::LoadSprite;
use bones_messages::{DecodeMessage, EncodeMessage, Message};
use game_messages::{AttackRequested, PauseChanged, PlayerDamaged, WILL_ENTITY_ID, WILL_SPAWN};
use slime_state::{
    SlimeAnimation, SlimeField, SlimeSpawn, SlimeVisual, SLIME_COLLIDER_HALF_H,
    SLIME_COLLIDER_HALF_W,
};

const LEVEL_TWO_TMX: &[u8] = include_bytes!("../assets/level-two.tmx");
const GRASS_PNG: &[u8] = include_bytes!(
    "../../../assets/Pixel Art Top Down - Basic v1.2.3/Texture/TX Tileset Grass.png"
);
const ROCK_CLUSTER_PNG: &[u8] = include_bytes!("../assets/rock-cluster.png");
const BOULDER_PNG: &[u8] = include_bytes!("../assets/boulder.png");
const SLIME_ONE_IDLE_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime1/With_shadow/Slime1_Idle_with_shadow.png"
);
const SLIME_TWO_IDLE_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime2/With_shadow/Slime2_Idle_with_shadow.png"
);
const SLIME_THREE_IDLE_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime3/With_shadow/Slime3_Idle_with_shadow.png"
);
const SLIME_ONE_WALK_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime1/With_shadow/Slime1_Walk_with_shadow.png"
);
const SLIME_TWO_WALK_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime2/With_shadow/Slime2_Walk_with_shadow.png"
);
const SLIME_THREE_WALK_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime3/With_shadow/Slime3_Walk_with_shadow.png"
);
const SLIME_ONE_ATTACK_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime1/With_shadow/Slime1_Attack_with_shadow.png"
);
const SLIME_TWO_ATTACK_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime2/With_shadow/Slime2_Attack_with_shadow.png"
);
const SLIME_THREE_ATTACK_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime3/With_shadow/Slime3_Attack_with_shadow.png"
);
const SLIME_ONE_HURT_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime1/With_shadow/Slime1_Hurt_with_shadow.png"
);
const SLIME_TWO_HURT_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime2/With_shadow/Slime2_Hurt_with_shadow.png"
);
const SLIME_THREE_HURT_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime3/With_shadow/Slime3_Hurt_with_shadow.png"
);
const SLIME_ONE_DEATH_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime1/With_shadow/Slime1_Death_with_shadow.png"
);
const SLIME_TWO_DEATH_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime2/With_shadow/Slime2_Death_with_shadow.png"
);
const SLIME_THREE_DEATH_PNG: &[u8] = include_bytes!(
    "../../../assets/craftpix-net-788364-free-slime-mobs-pixel-art-top-down-sprite-pack/PNG/Slime3/With_shadow/Slime3_Death_with_shadow.png"
);

const GRASS_SPRITE_ID: u32 = 20;
const ROCK_CLUSTER_SPRITE_ID: u32 = 21;
const BOULDER_SPRITE_ID: u32 = 22;
const SLIME_IDLE_SPRITE_IDS: [u32; 3] = [30, 31, 32];
const SLIME_WALK_SPRITE_IDS: [u32; 3] = [33, 34, 35];
const SLIME_ATTACK_SPRITE_IDS: [u32; 3] = [36, 37, 38];
const SLIME_HURT_SPRITE_IDS: [u32; 3] = [39, 40, 41];
const SLIME_DEATH_SPRITE_IDS: [u32; 3] = [42, 43, 44];
const ROCK_FRAME_SIZE: u32 = 128;
const SLIME_FRAME_SIZE: u32 = 64;
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
const SLIMES: &[SlimeSpawn] = &[
    SlimeSpawn::new(0, 480.0, 288.0),
    SlimeSpawn::new(1, 480.0, 576.0),
    SlimeSpawn::new(2, 240.0, 528.0),
    SlimeSpawn::new(0, 800.0, 512.0),
    SlimeSpawn::new(1, 640.0, 752.0),
    SlimeSpawn::new(2, 416.0, 720.0),
];

thread_local! {
    static SLIME_FIELD: RefCell<SlimeField> =
        RefCell::new(SlimeField::new(SLIME_ID_START, SLIMES, WILL_SPAWN));
}

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

fn publish_message<M: Message + EncodeMessage>(message: M) {
    publish(M::TOPIC, &message.encode());
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

fn slime_sheet(sprite_set: usize, animation: SlimeAnimation) -> (u32, u32, f32, bool) {
    let (ids, frame_counts, frame_duration, looping) = match animation {
        SlimeAnimation::Idle => (&SLIME_IDLE_SPRITE_IDS, [6, 6, 6], 0.16, true),
        SlimeAnimation::Walk => (&SLIME_WALK_SPRITE_IDS, [8, 8, 8], 0.10, true),
        SlimeAnimation::Attack => (&SLIME_ATTACK_SPRITE_IDS, [10, 11, 9], 0.08, false),
        SlimeAnimation::Hurt => (&SLIME_HURT_SPRITE_IDS, [5, 5, 5], 0.08, false),
        SlimeAnimation::Death => (&SLIME_DEATH_SPRITE_IDS, [10, 10, 10], 0.08, false),
    };
    (
        ids[sprite_set],
        frame_counts[sprite_set],
        frame_duration,
        looping,
    )
}

fn slime_presentation(sprite_set: usize, animation: SlimeAnimation) -> SpritePresentation {
    let (sprite_id, frame_count, frame_duration, looping) = slime_sheet(sprite_set, animation);
    SpritePresentation {
        sprite: Sprite {
            sprite_id,
            frame_w: SLIME_FRAME_SIZE,
            frame_h: SLIME_FRAME_SIZE,
            frame_count,
            frame_duration,
        },
        frames_per_row: frame_count,
        draw_w: SLIME_FRAME_SIZE,
        draw_h: SLIME_FRAME_SIZE,
        looping,
        advance_while_stopped: true,
        flip_h: false,
        flip_v: false,
    }
}

fn load_slime_sprites() {
    for (id, png_bytes) in [
        (SLIME_IDLE_SPRITE_IDS[0], SLIME_ONE_IDLE_PNG),
        (SLIME_IDLE_SPRITE_IDS[1], SLIME_TWO_IDLE_PNG),
        (SLIME_IDLE_SPRITE_IDS[2], SLIME_THREE_IDLE_PNG),
        (SLIME_WALK_SPRITE_IDS[0], SLIME_ONE_WALK_PNG),
        (SLIME_WALK_SPRITE_IDS[1], SLIME_TWO_WALK_PNG),
        (SLIME_WALK_SPRITE_IDS[2], SLIME_THREE_WALK_PNG),
        (SLIME_ATTACK_SPRITE_IDS[0], SLIME_ONE_ATTACK_PNG),
        (SLIME_ATTACK_SPRITE_IDS[1], SLIME_TWO_ATTACK_PNG),
        (SLIME_ATTACK_SPRITE_IDS[2], SLIME_THREE_ATTACK_PNG),
        (SLIME_HURT_SPRITE_IDS[0], SLIME_ONE_HURT_PNG),
        (SLIME_HURT_SPRITE_IDS[1], SLIME_TWO_HURT_PNG),
        (SLIME_HURT_SPRITE_IDS[2], SLIME_THREE_HURT_PNG),
        (SLIME_DEATH_SPRITE_IDS[0], SLIME_ONE_DEATH_PNG),
        (SLIME_DEATH_SPRITE_IDS[1], SLIME_TWO_DEATH_PNG),
        (SLIME_DEATH_SPRITE_IDS[2], SLIME_THREE_DEATH_PNG),
    ] {
        publish(LoadSprite::TOPIC, &LoadSprite { id, png_bytes }.encode());
    }
}

fn spawn_slimes() {
    for (index, slime) in SLIMES.iter().enumerate() {
        let entity_id = SLIME_ID_START + index as u32;
        let presentation = slime_presentation(slime.sprite_set, SlimeAnimation::Idle);
        publish_entity_op(EntityOp::Spawn {
            entity_id,
            x: slime.x,
            y: slime.y,
            sprite: Some(presentation.sprite),
            square_color: (0, 0, 0, 0),
            shape: Shape::Rect,
            collider_half_w: SLIME_COLLIDER_HALF_W,
            collider_half_h: SLIME_COLLIDER_HALF_H,
            body_kind: BodyKind::Frictionless,
            worlds: PhysicsWorlds::RETRO,
        });
        publish_entity_op(EntityOp::SetSprite {
            entity_id,
            presentation,
        });
    }
}

fn publish_slime_visual(visual: SlimeVisual) {
    let presentation = slime_presentation(visual.sprite_set, visual.animation);
    if visual.animation == SlimeAnimation::Death {
        publish_entity_op(EntityOp::Despawn {
            entity_id: visual.entity_id,
        });
        publish_entity_op(EntityOp::Spawn {
            entity_id: visual.entity_id,
            x: visual.x,
            y: visual.y,
            sprite: Some(presentation.sprite),
            square_color: (0, 0, 0, 0),
            shape: Shape::Rect,
            collider_half_w: 0.0,
            collider_half_h: 0.0,
            body_kind: BodyKind::Fixed,
            worlds: PhysicsWorlds::RETRO,
        });
    }
    publish_entity_op(EntityOp::SetSprite {
        entity_id: visual.entity_id,
        presentation,
    });
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
        SLIME_FIELD.with(|field| {
            *field.borrow_mut() = SlimeField::new(SLIME_ID_START, SLIMES, WILL_SPAWN)
        });
        subscribe(EntityTransform::TOPIC);
        subscribe(Collision::TOPIC);
        subscribe(PauseChanged::TOPIC);
        subscribe(AttackRequested::TOPIC);
        subscribe("core/tick");
        load_ruins_map();
        load_rock_sprites();
        spawn_rocks();
        load_slime_sprites();
        spawn_slimes();
        configure_camera();
        log(
            Level::Info,
            "level-two: overgrown ruins and hostile slimes ready",
        );
    }

    fn on_tick(dt: f32) {
        SLIME_FIELD.with(|field| {
            let tick = field.borrow_mut().tick(dt);
            for velocity in tick.velocities {
                publish_entity_op(EntityOp::SetVelocity {
                    entity_id: velocity.entity_id,
                    vx: velocity.vx,
                    vy: velocity.vy,
                });
            }
            for visual in tick.visuals {
                publish_slime_visual(visual);
            }
            for entity_id in tick.despawns {
                publish_entity_op(EntityOp::Despawn { entity_id });
            }
        });
    }

    fn on_message(topic: String, sender: String, payload: Vec<u8>) -> Option<Vec<u8>> {
        match topic.as_str() {
            EntityTransform::TOPIC if sender == "game-core" => {
                if let Ok(transform) = EntityTransform::decode(&payload) {
                    SLIME_FIELD.with(|field| {
                        field.borrow_mut().update_transform(
                            transform.entity_id,
                            transform.x,
                            transform.y,
                        )
                    });
                }
            }
            Collision::TOPIC if sender == "game-core" => {
                if let Ok(collision) = Collision::decode(&payload) {
                    let contact =
                        SLIME_FIELD.with(|field| field.borrow_mut().will_contact(collision));
                    if let Some(contact) = contact {
                        publish_message(PlayerDamaged {
                            amount: 1,
                            source_x: contact.source_x,
                            source_y: contact.source_y,
                        });
                        if let Some(visual) = contact.visual {
                            publish_slime_visual(visual);
                        }
                    }
                }
            }
            PauseChanged::TOPIC if sender == "menu" => {
                if let Ok(pause) = PauseChanged::decode(&payload) {
                    SLIME_FIELD.with(|field| field.borrow_mut().set_paused(pause.paused));
                }
            }
            AttackRequested::TOPIC if sender == "will" => {
                if let Ok(attack) = AttackRequested::decode(&payload) {
                    let hit = SLIME_FIELD.with(|field| field.borrow_mut().attack(attack));
                    if let Some(hit) = hit {
                        publish_message(hit.hit);
                        publish_slime_visual(hit.visual);
                        if let Some(reward) = hit.reward {
                            publish_message(reward);
                        }
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
mod tests;
