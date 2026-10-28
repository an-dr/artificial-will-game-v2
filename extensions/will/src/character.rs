use crate::bones::core::host_api::publish;
use bones_messages::game_core::{BodyKind, EntityOp, PhysicsWorlds, Shape, SpritePresentation};
use bones_messages::gfx::LoadSprite;
use bones_messages::{EncodeMessage, Message};
use game_messages::{WILL_ENTITY_ID, WILL_SPAWN};

use sprite_sheet::SpriteSheet;

use crate::player_state::PlayerState;
use crate::publish_entity_op;

const SHEET_JSON: &str = include_str!("../../../assets/william/tv_robot_sheet.json");
const SHEET_PNG: &[u8] = include_bytes!("../../../assets/william/tv_robot_sheet.png");

/// Every animation Will plays, in sprite-id order from `FIRST_SPRITE_ID`.
const ANIMATIONS: [&str; 16] = [
    "idle_down",
    "idle_up",
    "idle_left",
    "idle_right",
    "walk_down",
    "walk_up",
    "walk_left",
    "walk_right",
    "attack_down",
    "attack_up",
    "attack_left",
    "attack_right",
    "hurt_down",
    "hurt_up",
    "hurt_left",
    "hurt_right",
];
const FIRST_SPRITE_ID: u32 = 50;
/// The sheet's own 96 px frame, so pixels stay crisp; the robot is 64 px tall.
const DRAW_SIZE: u32 = 96;
const COLLIDER_HALF_W: f32 = 20.0;
const COLLIDER_HALF_H: f32 = 28.0;
/// Bones centres a sprite on its entity, so the sheet's feet are moved to the
/// frame point that lands on the collider's bottom edge. Workaround until
/// bones supports sprite origins - see TECHDEBT.md.
const FEET_TARGET: (i32, i32) = (48, 48 + COLLIDER_HALF_H as i32 * 96 / DRAW_SIZE as i32);

thread_local! {
    static SHEET: SpriteSheet =
        SpriteSheet::parse(SHEET_JSON).expect("William's sprite sheet parses");
}

fn sprite_id(animation: &str) -> u32 {
    let index = ANIMATIONS
        .iter()
        .position(|name| *name == animation)
        .unwrap_or_else(|| panic!("Will has no `{animation}` animation"));
    FIRST_SPRITE_ID + index as u32
}

pub fn load_sprites() {
    SHEET.with(|sheet| {
        let shift = sheet.shift_feet_to(FEET_TARGET);
        for animation in ANIMATIONS {
            let strip = sheet
                .cut_strip(SHEET_PNG, animation, shift)
                .unwrap_or_else(|error| panic!("William `{animation}`: {error}"));
            publish(
                LoadSprite::TOPIC,
                &LoadSprite {
                    id: sprite_id(animation),
                    png_bytes: &strip,
                }
                .encode(),
            );
        }
    });
}

fn presentation(state: &PlayerState) -> SpritePresentation {
    let animation = state.animation();
    SHEET.with(|sheet| {
        sheet
            .presentation(animation, sprite_id(animation), [DRAW_SIZE, DRAW_SIZE])
            .unwrap_or_else(|error| panic!("William `{animation}`: {error}"))
    })
}

fn spawn_op(state: &PlayerState) -> EntityOp {
    let presentation = presentation(state);
    EntityOp::Spawn {
        entity_id: WILL_ENTITY_ID,
        x: WILL_SPAWN.0,
        y: WILL_SPAWN.1,
        sprite: Some(presentation.sprite),
        square_color: (0, 0, 0, 0),
        shape: Shape::Rect,
        collider_half_w: COLLIDER_HALF_W,
        collider_half_h: COLLIDER_HALF_H,
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
        presentation: presentation(state),
    });
}

#[cfg(test)]
mod tests;
