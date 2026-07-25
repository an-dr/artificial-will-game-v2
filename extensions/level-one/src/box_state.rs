use game_messages::{
    select_attack_target, AttackRequested, AttackTarget, HitConfirmed, RewardGranted,
};

pub const BOX_HALF_W: f32 = 20.0;
pub const BOX_HALF_H: f32 = 28.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxSpawn {
    pub x: f32,
    pub y: f32,
    pub coins: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BoxState {
    entity_id: u32,
    x: f32,
    y: f32,
    coins: u32,
    health: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DestroyedBox {
    pub hit: HitConfirmed,
    pub reward: RewardGranted,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoxField {
    boxes: Vec<BoxState>,
    last_attack_sequence: Option<u32>,
}

impl BoxField {
    pub fn new(id_start: u32, spawns: &[BoxSpawn]) -> Self {
        Self {
            boxes: spawns
                .iter()
                .enumerate()
                .map(|(index, spawn)| BoxState {
                    entity_id: id_start + index as u32,
                    x: spawn.x,
                    y: spawn.y,
                    coins: spawn.coins,
                    health: 1,
                })
                .collect(),
            last_attack_sequence: None,
        }
    }

    pub fn update_transform(&mut self, entity_id: u32, x: f32, y: f32) {
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        if let Some(box_state) = self
            .boxes
            .iter_mut()
            .find(|box_state| box_state.entity_id == entity_id && box_state.health > 0)
        {
            box_state.x = x;
            box_state.y = y;
        }
    }

    pub fn attack(&mut self, attack: AttackRequested) -> Option<DestroyedBox> {
        if self.last_attack_sequence == Some(attack.sequence) {
            return None;
        }
        self.last_attack_sequence = Some(attack.sequence);

        let target = select_attack_target(
            attack,
            self.boxes
                .iter()
                .filter(|box_state| box_state.health > 0)
                .map(|box_state| AttackTarget {
                    entity_id: box_state.entity_id,
                    x: box_state.x,
                    y: box_state.y,
                    half_w: BOX_HALF_W,
                    half_h: BOX_HALF_H,
                }),
        )?;
        let box_state = self
            .boxes
            .iter_mut()
            .find(|box_state| box_state.entity_id == target.entity_id)?;
        box_state.health = box_state.health.saturating_sub(1);
        (box_state.health == 0).then_some(DestroyedBox {
            hit: HitConfirmed {
                sequence: attack.sequence,
                entity_id: box_state.entity_id,
                x: box_state.x,
                y: box_state.y,
            },
            reward: RewardGranted {
                experience: 0,
                coins: box_state.coins,
            },
        })
    }
}

#[cfg(test)]
mod tests;
