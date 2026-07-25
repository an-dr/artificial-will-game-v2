use crate::bones::core::host_api::publish;
use bones_messages::game_core::{BodyKind, EntityOp, PhysicsWorlds, Shape};
use bones_messages::gfx::LoadSprite;
use bones_messages::{EncodeMessage, Message};
use game_messages::{WILL_ENTITY_ID, WILL_SPAWN};

use crate::player_state::{
    PlayerState, ATTACK_DOWN_SPRITE_ID, ATTACK_SIDE_SPRITE_ID, ATTACK_UP_SPRITE_ID,
    IDLE_DOWN_SPRITE_ID, IDLE_SIDE_SPRITE_ID, IDLE_UP_SPRITE_ID, WALK_DOWN_SPRITE_ID,
    WALK_SIDE_SPRITE_ID, WALK_UP_SPRITE_ID,
};
use crate::publish_entity_op;

const IDLE_DOWN_PNG: &[u8] = include_bytes!("../../../assets/RPGMCharacter_v1.0/_down idle.png");
const IDLE_UP_PNG: &[u8] = include_bytes!("../../../assets/RPGMCharacter_v1.0/_up idle.png");
const IDLE_SIDE_PNG: &[u8] = include_bytes!("../../../assets/RPGMCharacter_v1.0/_side idle.png");
const WALK_DOWN_PNG: &[u8] = include_bytes!("../../../assets/RPGMCharacter_v1.0/_down walk.png");
const WALK_UP_PNG: &[u8] = include_bytes!("../../../assets/RPGMCharacter_v1.0/_up walk.png");
const WALK_SIDE_PNG: &[u8] = include_bytes!("../../../assets/RPGMCharacter_v1.0/_side walk.png");
const ATTACK_DOWN_PNG: &[u8] =
    include_bytes!("../../../assets/RPGMCharacter_v1.0/_down attack.png");
const ATTACK_UP_PNG: &[u8] = include_bytes!("../../../assets/RPGMCharacter_v1.0/_up attack.png");
const ATTACK_SIDE_PNG: &[u8] =
    include_bytes!("../../../assets/RPGMCharacter_v1.0/_side attack.png");

pub fn load_sprites() {
    let sprites: [(u32, &[u8]); 9] = [
        (IDLE_DOWN_SPRITE_ID, IDLE_DOWN_PNG),
        (IDLE_UP_SPRITE_ID, IDLE_UP_PNG),
        (IDLE_SIDE_SPRITE_ID, IDLE_SIDE_PNG),
        (WALK_DOWN_SPRITE_ID, WALK_DOWN_PNG),
        (WALK_UP_SPRITE_ID, WALK_UP_PNG),
        (WALK_SIDE_SPRITE_ID, WALK_SIDE_PNG),
        (ATTACK_DOWN_SPRITE_ID, ATTACK_DOWN_PNG),
        (ATTACK_UP_SPRITE_ID, ATTACK_UP_PNG),
        (ATTACK_SIDE_SPRITE_ID, ATTACK_SIDE_PNG),
    ];
    for (id, png_bytes) in sprites {
        publish(LoadSprite::TOPIC, &LoadSprite { id, png_bytes }.encode());
    }
}

fn spawn_op(state: &PlayerState) -> EntityOp {
    let presentation = state.presentation();
    EntityOp::Spawn {
        entity_id: WILL_ENTITY_ID,
        x: WILL_SPAWN.0,
        y: WILL_SPAWN.1,
        sprite: Some(presentation.sprite),
        square_color: (0, 0, 0, 0),
        shape: Shape::Rect,
        collider_half_w: 10.0,
        collider_half_h: 27.0,
        body_kind: BodyKind::Frictionless,
        worlds: PhysicsWorlds::RETRO,
    }
}

pub fn spawn(state: &PlayerState) {
    publish_entity_op(spawn_op(state));
    publish_presentation(state);
}

pub fn publish_presentation(state: &PlayerState) {
    publish_entity_op(EntityOp::SetSprite {
        entity_id: WILL_ENTITY_ID,
        presentation: state.presentation(),
    });
}

#[cfg(test)]
mod tests;
