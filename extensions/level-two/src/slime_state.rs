use bones_messages::game_core::Collision;
use game_messages::WILL_ENTITY_ID;

pub const SLIME_SPEED: f32 = 72.0;
pub const SLIME_AWARENESS_RADIUS: f32 = 240.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlimeSpawn {
    pub sprite_id: u32,
    pub x: f32,
    pub y: f32,
}

impl SlimeSpawn {
    pub const fn new(sprite_id: u32, x: f32, y: f32) -> Self {
        Self { sprite_id, x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SlimeState {
    entity_id: u32,
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlimeVelocity {
    pub entity_id: u32,
    pub vx: f32,
    pub vy: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlimeField {
    slimes: Vec<SlimeState>,
    will_x: f32,
    will_y: f32,
    paused: bool,
}

impl SlimeField {
    pub fn new(id_start: u32, spawns: &[SlimeSpawn], will_spawn: (f32, f32)) -> Self {
        Self {
            slimes: spawns
                .iter()
                .enumerate()
                .map(|(index, spawn)| SlimeState {
                    entity_id: id_start + index as u32,
                    x: spawn.x,
                    y: spawn.y,
                })
                .collect(),
            will_x: will_spawn.0,
            will_y: will_spawn.1,
            paused: false,
        }
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn update_transform(&mut self, entity_id: u32, x: f32, y: f32) {
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        if entity_id == WILL_ENTITY_ID {
            self.will_x = x;
            self.will_y = y;
        } else if let Some(slime) = self
            .slimes
            .iter_mut()
            .find(|slime| slime.entity_id == entity_id)
        {
            slime.x = x;
            slime.y = y;
        }
    }

    pub fn velocities(&self) -> Vec<SlimeVelocity> {
        self.slimes
            .iter()
            .map(|slime| {
                let dx = self.will_x - slime.x;
                let dy = self.will_y - slime.y;
                let distance_squared = dx * dx + dy * dy;
                let (vx, vy) = if self.paused
                    || distance_squared <= f32::EPSILON
                    || distance_squared > SLIME_AWARENESS_RADIUS * SLIME_AWARENESS_RADIUS
                {
                    (0.0, 0.0)
                } else {
                    let scale = SLIME_SPEED / distance_squared.sqrt();
                    (dx * scale, dy * scale)
                };
                SlimeVelocity {
                    entity_id: slime.entity_id,
                    vx,
                    vy,
                }
            })
            .collect()
    }

    pub fn is_will_contact(&self, collision: Collision) -> bool {
        let other = if collision.entity_id_a == WILL_ENTITY_ID {
            collision.entity_id_b
        } else if collision.entity_id_b == WILL_ENTITY_ID {
            collision.entity_id_a
        } else {
            return false;
        };
        self.slimes.iter().any(|slime| slime.entity_id == other)
    }
}

#[cfg(test)]
mod tests;
