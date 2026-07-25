use bones_messages::game_core::{ObjectFacing, Sprite, SpritePresentation};

use crate::player_mode::PlayerMode;

pub const IDLE_DOWN_SPRITE_ID: u32 = 1;
pub const IDLE_UP_SPRITE_ID: u32 = 4;
pub const IDLE_SIDE_SPRITE_ID: u32 = 5;
pub const WALK_DOWN_SPRITE_ID: u32 = 6;
pub const WALK_UP_SPRITE_ID: u32 = 7;
pub const WALK_SIDE_SPRITE_ID: u32 = 8;
pub const ATTACK_DOWN_SPRITE_ID: u32 = 9;
pub const ATTACK_UP_SPRITE_ID: u32 = 10;
pub const ATTACK_SIDE_SPRITE_ID: u32 = 11;

const FRAME_SIZE: u32 = 64;
const DRAW_SIZE: u32 = 128;
const FRAMES_PER_ROW: u32 = 4;
const FRAME_DURATION_SECONDS: f32 = 0.125;
const LOOPING_FRAME_COUNT: u32 = 5;
const ATTACK_FRAME_COUNT: u32 = 2;
const ATTACK_DURATION_SECONDS: f32 = FRAME_DURATION_SECONDS * ATTACK_FRAME_COUNT as f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerState {
    mode: PlayerMode,
    facing: ObjectFacing,
    attack_button_down: bool,
    attack_remaining: f32,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            mode: PlayerMode::Idle,
            facing: ObjectFacing::Down,
            attack_button_down: false,
            attack_remaining: 0.0,
        }
    }
}

impl PlayerState {
    pub fn press_attack(&mut self) -> bool {
        if self.attack_button_down {
            return false;
        }
        self.attack_button_down = true;
        if self.mode == PlayerMode::Attacking {
            return false;
        }
        self.mode = PlayerMode::Attacking;
        self.attack_remaining = ATTACK_DURATION_SECONDS;
        true
    }

    pub fn release_attack(&mut self) {
        self.attack_button_down = false;
    }

    pub fn tick(&mut self, dt: f32, vx: f32, vy: f32) -> bool {
        if self.mode == PlayerMode::Attacking {
            self.attack_remaining -= dt.max(0.0);
            if self.attack_remaining > f32::EPSILON {
                return false;
            }
        }

        let next_mode = if vx == 0.0 && vy == 0.0 {
            PlayerMode::Idle
        } else {
            PlayerMode::Moving
        };
        let next_facing = ObjectFacing::cardinal_from_velocity(vx, vy).unwrap_or(self.facing);
        let changed = self.mode != next_mode || self.facing != next_facing;
        self.mode = next_mode;
        self.facing = next_facing;
        self.attack_remaining = 0.0;
        changed
    }

    pub fn presentation(&self) -> SpritePresentation {
        let attacking = self.mode == PlayerMode::Attacking;
        let sprite_id = match (self.mode, self.facing) {
            (PlayerMode::Idle, ObjectFacing::Down) => IDLE_DOWN_SPRITE_ID,
            (PlayerMode::Idle, ObjectFacing::Up) => IDLE_UP_SPRITE_ID,
            (
                PlayerMode::Idle,
                ObjectFacing::Left
                | ObjectFacing::Right
                | ObjectFacing::UpLeft
                | ObjectFacing::UpRight
                | ObjectFacing::DownLeft
                | ObjectFacing::DownRight,
            ) => IDLE_SIDE_SPRITE_ID,
            (PlayerMode::Moving, ObjectFacing::Down) => WALK_DOWN_SPRITE_ID,
            (PlayerMode::Moving, ObjectFacing::Up) => WALK_UP_SPRITE_ID,
            (
                PlayerMode::Moving,
                ObjectFacing::Left
                | ObjectFacing::Right
                | ObjectFacing::UpLeft
                | ObjectFacing::UpRight
                | ObjectFacing::DownLeft
                | ObjectFacing::DownRight,
            ) => WALK_SIDE_SPRITE_ID,
            (PlayerMode::Attacking, ObjectFacing::Down) => ATTACK_DOWN_SPRITE_ID,
            (PlayerMode::Attacking, ObjectFacing::Up) => ATTACK_UP_SPRITE_ID,
            (
                PlayerMode::Attacking,
                ObjectFacing::Left
                | ObjectFacing::Right
                | ObjectFacing::UpLeft
                | ObjectFacing::UpRight
                | ObjectFacing::DownLeft
                | ObjectFacing::DownRight,
            ) => ATTACK_SIDE_SPRITE_ID,
        };
        SpritePresentation {
            sprite: Sprite {
                sprite_id,
                frame_w: FRAME_SIZE,
                frame_h: FRAME_SIZE,
                frame_count: if attacking {
                    ATTACK_FRAME_COUNT
                } else {
                    LOOPING_FRAME_COUNT
                },
                frame_duration: FRAME_DURATION_SECONDS,
            },
            frames_per_row: FRAMES_PER_ROW,
            draw_w: DRAW_SIZE,
            draw_h: DRAW_SIZE,
            looping: !attacking,
            advance_while_stopped: true,
            flip_h: matches!(
                self.facing,
                ObjectFacing::Right | ObjectFacing::UpRight | ObjectFacing::DownRight
            ),
            flip_v: false,
        }
    }
}

#[cfg(test)]
mod tests;
