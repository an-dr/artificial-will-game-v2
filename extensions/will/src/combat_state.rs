use game_messages::{AttackDirection, AttackRequested, PlayerStats, RewardGranted, WILL_SPAWN};

pub const STARTING_LIVES: u8 = 3;
pub const EXPERIENCE_PER_LEVEL: u32 = 3;
pub const DAMAGE_INVULNERABILITY_SECONDS: f32 = 0.8;
pub const ATTACK_REACH: f32 = 56.0;
pub const ATTACK_HALF_WIDTH: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageOutcome {
    Ignored,
    Applied,
    Defeated,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CombatState {
    position_x: f32,
    position_y: f32,
    lives: u8,
    experience: u32,
    coins: u32,
    invulnerability_remaining: f32,
    next_attack_sequence: u32,
    defeated: bool,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            position_x: WILL_SPAWN.0,
            position_y: WILL_SPAWN.1,
            lives: STARTING_LIVES,
            experience: 0,
            coins: 0,
            invulnerability_remaining: 0.0,
            next_attack_sequence: 1,
            defeated: false,
        }
    }
}

impl CombatState {
    pub fn tick(&mut self, dt: f32) {
        self.invulnerability_remaining = (self.invulnerability_remaining - dt.max(0.0)).max(0.0);
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        if x.is_finite() && y.is_finite() {
            self.position_x = x;
            self.position_y = y;
        }
    }

    pub fn attack(&mut self, direction: AttackDirection) -> AttackRequested {
        let request = AttackRequested {
            sequence: self.next_attack_sequence,
            origin_x: self.position_x,
            origin_y: self.position_y,
            direction,
            reach: ATTACK_REACH,
            half_width: ATTACK_HALF_WIDTH,
        };
        self.next_attack_sequence = self.next_attack_sequence.wrapping_add(1);
        request
    }

    pub fn damage(&mut self, amount: u8) -> DamageOutcome {
        if amount == 0 || self.defeated || self.invulnerability_remaining > 0.0 {
            return DamageOutcome::Ignored;
        }

        self.lives = self.lives.saturating_sub(amount);
        if self.lives == 0 {
            self.defeated = true;
            DamageOutcome::Defeated
        } else {
            self.invulnerability_remaining = DAMAGE_INVULNERABILITY_SECONDS;
            DamageOutcome::Applied
        }
    }

    pub fn grant(&mut self, reward: RewardGranted) {
        if self.defeated {
            return;
        }
        self.experience = self.experience.saturating_add(reward.experience);
        self.coins = self.coins.saturating_add(reward.coins);
    }

    pub fn stats(&self) -> PlayerStats {
        PlayerStats {
            lives: self.lives,
            experience: self.experience,
            level: 1 + self.experience / EXPERIENCE_PER_LEVEL,
            coins: self.coins,
        }
    }
}

#[cfg(test)]
mod tests;
