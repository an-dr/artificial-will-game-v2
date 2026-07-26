use game_messages::AttackDirection;

pub const DAMAGE_REACTION_SECONDS: f32 = 0.45;
pub const KNOCKBACK_SECONDS: f32 = 0.18;
pub const KNOCKBACK_SPEED: f32 = 260.0;
const FLASH_HALF_PERIOD: f32 = 0.055;
pub const NORMAL_TINT: (u8, u8, u8, u8) = (255, 255, 255, 255);
pub const DAMAGE_TINT: (u8, u8, u8, u8) = (255, 48, 58, 255);

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DamageReaction {
    remaining: f32,
    knockback_remaining: f32,
    vx: f32,
    vy: f32,
}

impl DamageReaction {
    pub fn start(
        &mut self,
        player: (f32, f32),
        source: Option<(f32, f32)>,
        facing: AttackDirection,
    ) {
        let fallback = match facing {
            AttackDirection::Up => (0.0, 1.0),
            AttackDirection::Down => (0.0, -1.0),
            AttackDirection::Left => (1.0, 0.0),
            AttackDirection::Right => (-1.0, 0.0),
        };
        let (dx, dy) = source
            .map(|(source_x, source_y)| (player.0 - source_x, player.1 - source_y))
            .filter(|(dx, dy)| dx.is_finite() && dy.is_finite() && dx * dx + dy * dy > 0.001)
            .unwrap_or(fallback);
        let scale = KNOCKBACK_SPEED / (dx * dx + dy * dy).sqrt();
        self.remaining = DAMAGE_REACTION_SECONDS;
        self.knockback_remaining = KNOCKBACK_SECONDS;
        self.vx = dx * scale;
        self.vy = dy * scale;
    }

    pub fn active(&self) -> bool {
        self.remaining > 0.0
    }

    pub fn velocity_for_tick(&self, dt: f32) -> (f32, f32) {
        if self.knockback_remaining > 0.0 {
            let fraction = if dt > self.knockback_remaining {
                self.knockback_remaining / dt
            } else {
                1.0
            };
            (self.vx * fraction, self.vy * fraction)
        } else {
            (0.0, 0.0)
        }
    }

    /// Advances the reaction and returns `true` exactly when recovery completes.
    pub fn tick(&mut self, dt: f32) -> bool {
        if !self.active() {
            return false;
        }
        let dt = dt.max(0.0);
        self.remaining = (self.remaining - dt).max(0.0);
        self.knockback_remaining = (self.knockback_remaining - dt).max(0.0);
        self.remaining == 0.0
    }

    pub fn tint(&self) -> (u8, u8, u8, u8) {
        if !self.active() {
            return NORMAL_TINT;
        }
        let elapsed = DAMAGE_REACTION_SECONDS - self.remaining;
        if ((elapsed / FLASH_HALF_PERIOD) as u32).is_multiple_of(2) {
            DAMAGE_TINT
        } else {
            NORMAL_TINT
        }
    }
}

#[cfg(test)]
mod tests;
