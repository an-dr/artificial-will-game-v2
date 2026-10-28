use bones_messages::game_core::ObjectFacing;
use game_messages::AttackDirection;

use crate::player_mode::PlayerMode;

/// William's attack: four frames at 12 fps (`attack_*` in the sheet).
pub const ATTACK_DURATION_SECONDS: f32 = 4.0 / 12.0;

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
    pub fn attack_direction(&self) -> AttackDirection {
        match self.facing {
            ObjectFacing::Up | ObjectFacing::UpLeft | ObjectFacing::UpRight => AttackDirection::Up,
            ObjectFacing::Down | ObjectFacing::DownLeft | ObjectFacing::DownRight => {
                AttackDirection::Down
            }
            ObjectFacing::Left => AttackDirection::Left,
            ObjectFacing::Right => AttackDirection::Right,
        }
    }

    pub fn press_attack(&mut self) -> bool {
        if self.attack_button_down {
            return false;
        }
        self.attack_button_down = true;
        if matches!(self.mode, PlayerMode::Attacking | PlayerMode::Damaged) {
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
        if self.mode == PlayerMode::Damaged {
            return false;
        }
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

    pub fn start_damage(&mut self) -> bool {
        let changed = self.mode != PlayerMode::Damaged;
        self.mode = PlayerMode::Damaged;
        self.attack_button_down = false;
        self.attack_remaining = 0.0;
        changed
    }

    pub fn recover_from_damage(&mut self) -> bool {
        if self.mode != PlayerMode::Damaged {
            return false;
        }
        self.mode = PlayerMode::Idle;
        true
    }

    /// The sheet animation for the current mode and facing; diagonals use
    /// the side the character is turned to.
    pub fn animation(&self) -> &'static str {
        use ObjectFacing::{Down, DownLeft, DownRight, Left, Right, Up, UpLeft, UpRight};
        match (self.mode, self.facing) {
            (PlayerMode::Idle, Down) => "idle_down",
            (PlayerMode::Idle, Up) => "idle_up",
            (PlayerMode::Idle, Left | UpLeft | DownLeft) => "idle_left",
            (PlayerMode::Idle, Right | UpRight | DownRight) => "idle_right",
            (PlayerMode::Moving, Down) => "walk_down",
            (PlayerMode::Moving, Up) => "walk_up",
            (PlayerMode::Moving, Left | UpLeft | DownLeft) => "walk_left",
            (PlayerMode::Moving, Right | UpRight | DownRight) => "walk_right",
            (PlayerMode::Attacking, Down) => "attack_down",
            (PlayerMode::Attacking, Up) => "attack_up",
            (PlayerMode::Attacking, Left | UpLeft | DownLeft) => "attack_left",
            (PlayerMode::Attacking, Right | UpRight | DownRight) => "attack_right",
            (PlayerMode::Damaged, Down) => "hurt_down",
            (PlayerMode::Damaged, Up) => "hurt_up",
            (PlayerMode::Damaged, Left | UpLeft | DownLeft) => "hurt_left",
            (PlayerMode::Damaged, Right | UpRight | DownRight) => "hurt_right",
        }
    }
}

#[cfg(test)]
mod tests;
