use game_messages::{
    select_attack_target, AttackRequested, AttackTarget, HitConfirmed, RewardGranted,
    WILL_ENTITY_ID,
};

pub const SLIME_SPEED: f32 = 72.0;
pub const SLIME_AWARENESS_RADIUS: f32 = 360.0;
pub const SLIME_ATTACK_RANGE: f32 = 62.0;
pub const SLIME_ATTACK_HIT_RANGE: f32 = 72.0;
pub const SLIME_ATTACK_COOLDOWN: f32 = 0.70;
pub const SLIME_ATTACK_IMPACT_FRACTION: f32 = 0.55;
pub const SLIME_COLLIDER_HALF_W: f32 = 18.0;
pub const SLIME_COLLIDER_HALF_H: f32 = 14.0;
pub const SLIME_HEALTH: u8 = 2;
pub const SLIME_EXPERIENCE: u32 = 1;
pub const SLIME_ATTACK_DURATIONS: [f32; 3] = [0.80, 0.88, 0.72];
pub const SLIME_HURT_DURATION: f32 = 0.40;
pub const SLIME_DEATH_DURATION: f32 = 0.80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlimeAnimation {
    Idle,
    Walk,
    Attack,
    Hurt,
    Death,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlimeSpawn {
    pub sprite_set: usize,
    pub x: f32,
    pub y: f32,
}

impl SlimeSpawn {
    pub const fn new(sprite_set: usize, x: f32, y: f32) -> Self {
        Self { sprite_set, x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SlimeState {
    entity_id: u32,
    x: f32,
    y: f32,
    health: u8,
    sprite_set: usize,
    animation: SlimeAnimation,
    reaction_remaining: f32,
    attack_impact_pending: bool,
    attack_cooldown_remaining: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlimeVelocity {
    pub entity_id: u32,
    pub vx: f32,
    pub vy: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlimeHit {
    pub hit: HitConfirmed,
    pub defeated: bool,
    pub reward: Option<RewardGranted>,
    pub visual: SlimeVisual,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlimeVisual {
    pub entity_id: u32,
    pub sprite_set: usize,
    pub animation: SlimeAnimation,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlimeDamage {
    pub amount: u8,
    pub source_x: f32,
    pub source_y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlimeTick {
    pub velocities: Vec<SlimeVelocity>,
    pub visuals: Vec<SlimeVisual>,
    pub damages: Vec<SlimeDamage>,
    pub despawns: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlimeField {
    slimes: Vec<SlimeState>,
    will_x: f32,
    will_y: f32,
    paused: bool,
    last_attack_sequence: Option<u32>,
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
                    health: SLIME_HEALTH,
                    sprite_set: spawn.sprite_set,
                    animation: SlimeAnimation::Idle,
                    reaction_remaining: 0.0,
                    attack_impact_pending: false,
                    attack_cooldown_remaining: 0.0,
                })
                .collect(),
            will_x: will_spawn.0,
            will_y: will_spawn.1,
            paused: false,
            last_attack_sequence: None,
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
            if slime.health > 0 {
                slime.x = x;
                slime.y = y;
            }
        }
    }

    fn pursuit_velocity(will_x: f32, will_y: f32, slime: &SlimeState) -> (f32, f32) {
        let dx = will_x - slime.x;
        let dy = will_y - slime.y;
        let distance_squared = dx * dx + dy * dy;
        if distance_squared <= f32::EPSILON
            || distance_squared > SLIME_AWARENESS_RADIUS * SLIME_AWARENESS_RADIUS
        {
            (0.0, 0.0)
        } else {
            let scale = SLIME_SPEED / distance_squared.sqrt();
            (dx * scale, dy * scale)
        }
    }

    fn distance_squared(will_x: f32, will_y: f32, slime: &SlimeState) -> f32 {
        let dx = will_x - slime.x;
        let dy = will_y - slime.y;
        dx * dx + dy * dy
    }

    pub fn tick(&mut self, dt: f32) -> SlimeTick {
        let mut tick = SlimeTick {
            velocities: Vec::new(),
            visuals: Vec::new(),
            damages: Vec::new(),
            despawns: Vec::new(),
        };
        let dt = dt.max(0.0);
        for slime in &mut self.slimes {
            if slime.animation == SlimeAnimation::Death && slime.reaction_remaining <= 0.0 {
                continue;
            }
            if self.paused {
                if slime.health > 0 {
                    tick.velocities.push(SlimeVelocity {
                        entity_id: slime.entity_id,
                        vx: 0.0,
                        vy: 0.0,
                    });
                }
                continue;
            }
            slime.attack_cooldown_remaining = (slime.attack_cooldown_remaining - dt).max(0.0);

            if matches!(
                slime.animation,
                SlimeAnimation::Attack | SlimeAnimation::Hurt | SlimeAnimation::Death
            ) {
                slime.reaction_remaining = (slime.reaction_remaining - dt).max(0.0);
                if slime.animation == SlimeAnimation::Attack && slime.attack_impact_pending {
                    let impact_remaining = SLIME_ATTACK_DURATIONS[slime.sprite_set]
                        * (1.0 - SLIME_ATTACK_IMPACT_FRACTION);
                    if slime.reaction_remaining <= impact_remaining {
                        slime.attack_impact_pending = false;
                        if Self::distance_squared(self.will_x, self.will_y, slime)
                            <= SLIME_ATTACK_HIT_RANGE * SLIME_ATTACK_HIT_RANGE
                        {
                            tick.damages.push(SlimeDamage {
                                amount: 1,
                                source_x: slime.x,
                                source_y: slime.y,
                            });
                        }
                    }
                }
                if slime.reaction_remaining > 0.0 {
                    tick.velocities.push(SlimeVelocity {
                        entity_id: slime.entity_id,
                        vx: 0.0,
                        vy: 0.0,
                    });
                    continue;
                }
                if slime.animation == SlimeAnimation::Death {
                    tick.despawns.push(slime.entity_id);
                    continue;
                }
                if slime.animation == SlimeAnimation::Attack {
                    slime.attack_cooldown_remaining = SLIME_ATTACK_COOLDOWN;
                }
            }

            let distance_squared = Self::distance_squared(self.will_x, self.will_y, slime);
            if distance_squared <= SLIME_ATTACK_RANGE * SLIME_ATTACK_RANGE
                && slime.attack_cooldown_remaining <= 0.0
            {
                slime.animation = SlimeAnimation::Attack;
                slime.reaction_remaining = SLIME_ATTACK_DURATIONS[slime.sprite_set];
                slime.attack_impact_pending = true;
                tick.visuals.push(Self::visual(slime));
                tick.velocities.push(SlimeVelocity {
                    entity_id: slime.entity_id,
                    vx: 0.0,
                    vy: 0.0,
                });
                continue;
            }

            let (vx, vy) = if distance_squared <= SLIME_ATTACK_RANGE * SLIME_ATTACK_RANGE {
                (0.0, 0.0)
            } else {
                Self::pursuit_velocity(self.will_x, self.will_y, slime)
            };
            let desired = if vx == 0.0 && vy == 0.0 {
                SlimeAnimation::Idle
            } else {
                SlimeAnimation::Walk
            };
            if slime.animation != desired {
                slime.animation = desired;
                tick.visuals.push(Self::visual(slime));
            }
            tick.velocities.push(SlimeVelocity {
                entity_id: slime.entity_id,
                vx,
                vy,
            });
        }
        tick
    }

    fn visual(slime: &SlimeState) -> SlimeVisual {
        SlimeVisual {
            entity_id: slime.entity_id,
            sprite_set: slime.sprite_set,
            animation: slime.animation,
            x: slime.x,
            y: slime.y,
        }
    }

    #[cfg(test)]
    pub fn velocities(&self) -> Vec<SlimeVelocity> {
        self.slimes
            .iter()
            .filter(|slime| slime.health > 0)
            .map(|slime| {
                let (vx, vy) = if self.paused
                    || matches!(
                        slime.animation,
                        SlimeAnimation::Attack | SlimeAnimation::Hurt
                    )
                    || Self::distance_squared(self.will_x, self.will_y, slime)
                        <= SLIME_ATTACK_RANGE * SLIME_ATTACK_RANGE
                {
                    (0.0, 0.0)
                } else {
                    Self::pursuit_velocity(self.will_x, self.will_y, slime)
                };
                SlimeVelocity {
                    entity_id: slime.entity_id,
                    vx,
                    vy,
                }
            })
            .collect()
    }

    pub fn attack(&mut self, attack: AttackRequested) -> Option<SlimeHit> {
        if self.last_attack_sequence == Some(attack.sequence) {
            return None;
        }
        self.last_attack_sequence = Some(attack.sequence);

        let target = select_attack_target(
            attack,
            self.slimes
                .iter()
                .filter(|slime| slime.health > 0)
                .map(|slime| AttackTarget {
                    entity_id: slime.entity_id,
                    x: slime.x,
                    y: slime.y,
                    half_w: SLIME_COLLIDER_HALF_W,
                    half_h: SLIME_COLLIDER_HALF_H,
                }),
        )?;
        let slime = self
            .slimes
            .iter_mut()
            .find(|slime| slime.entity_id == target.entity_id)?;
        slime.health = slime.health.saturating_sub(1);
        slime.attack_impact_pending = false;
        slime.attack_cooldown_remaining = SLIME_ATTACK_COOLDOWN;
        let defeated = slime.health == 0;
        slime.animation = if defeated {
            SlimeAnimation::Death
        } else {
            SlimeAnimation::Hurt
        };
        slime.reaction_remaining = if defeated {
            SLIME_DEATH_DURATION
        } else {
            SLIME_HURT_DURATION
        };
        Some(SlimeHit {
            hit: HitConfirmed {
                sequence: attack.sequence,
                entity_id: slime.entity_id,
                x: slime.x,
                y: slime.y,
            },
            defeated,
            reward: defeated.then_some(RewardGranted {
                experience: SLIME_EXPERIENCE,
                coins: 0,
            }),
            visual: Self::visual(slime),
        })
    }
}

#[cfg(test)]
mod tests;
