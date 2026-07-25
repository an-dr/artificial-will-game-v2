use bones_messages::{DecodeError, DecodeMessage, EncodeMessage, Message, Reader, Writer};

/// Cardinal direction of one melee attack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackDirection {
    Up,
    Down,
    Left,
    Right,
}

impl AttackDirection {
    fn encode(self, writer: Writer) -> Writer {
        writer.u8(match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 3,
        })
    }

    fn decode(reader: &mut Reader<'_>) -> Result<Self, DecodeError> {
        match reader.read_u8()? {
            0 => Ok(Self::Up),
            1 => Ok(Self::Down),
            2 => Ok(Self::Left),
            3 => Ok(Self::Right),
            tag => Err(DecodeError::InvalidTag {
                message: "attack direction",
                tag,
            }),
        }
    }
}

/// One accepted melee swing at an authoritative world-space origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttackRequested {
    pub sequence: u32,
    pub origin_x: f32,
    pub origin_y: f32,
    pub direction: AttackDirection,
    pub reach: f32,
    pub half_width: f32,
}

impl Message for AttackRequested {
    const TOPIC: &'static str = "game/attack-requested";
}

impl EncodeMessage for AttackRequested {
    fn encode(&self) -> Vec<u8> {
        self.direction
            .encode(
                Writer::new()
                    .u32(self.sequence)
                    .f32(self.origin_x)
                    .f32(self.origin_y),
            )
            .f32(self.reach)
            .f32(self.half_width)
            .finish()
    }
}

impl<'a> DecodeMessage<'a> for AttackRequested {
    fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        let mut reader = Reader::new(payload);
        let sequence = reader.read_u32()?;
        let origin_x = reader.read_f32()?;
        let origin_y = reader.read_f32()?;
        let direction = AttackDirection::decode(&mut reader)?;
        let message = Self {
            sequence,
            origin_x,
            origin_y,
            direction,
            reach: reader.read_f32()?,
            half_width: reader.read_f32()?,
        };
        reader.finish()?;
        Ok(message)
    }
}

/// Axis-aligned target considered by the shared melee targeting rule.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttackTarget {
    pub entity_id: u32,
    pub x: f32,
    pub y: f32,
    pub half_w: f32,
    pub half_h: f32,
}

/// Selects the nearest target intersecting the attack's forward lane.
pub fn select_attack_target(
    attack: AttackRequested,
    targets: impl IntoIterator<Item = AttackTarget>,
) -> Option<AttackTarget> {
    if attack.reach <= 0.0
        || attack.half_width < 0.0
        || !attack.origin_x.is_finite()
        || !attack.origin_y.is_finite()
        || !attack.reach.is_finite()
        || !attack.half_width.is_finite()
    {
        return None;
    }

    let mut best: Option<(f32, AttackTarget)> = None;
    for target in targets {
        if !target.x.is_finite()
            || !target.y.is_finite()
            || !target.half_w.is_finite()
            || !target.half_h.is_finite()
            || target.half_w < 0.0
            || target.half_h < 0.0
        {
            continue;
        }
        let dx = target.x - attack.origin_x;
        let dy = target.y - attack.origin_y;
        let (forward, lateral, forward_extent, lateral_extent) = match attack.direction {
            AttackDirection::Up => (-dy, dx, target.half_h, target.half_w),
            AttackDirection::Down => (dy, dx, target.half_h, target.half_w),
            AttackDirection::Left => (-dx, dy, target.half_w, target.half_h),
            AttackDirection::Right => (dx, dy, target.half_w, target.half_h),
        };
        let valid = forward + forward_extent >= 0.0
            && forward - forward_extent <= attack.reach
            && lateral.abs() <= attack.half_width + lateral_extent;
        if !valid {
            continue;
        }
        let distance_squared = dx * dx + dy * dy;
        let replace = best.is_none_or(|(best_distance, best_target)| {
            distance_squared < best_distance
                || (distance_squared == best_distance && target.entity_id < best_target.entity_id)
        });
        if replace {
            best = Some((distance_squared, target));
        }
    }
    best.map(|(_, target)| target)
}

/// Damage requested by the active level after hostile contact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerDamaged {
    pub amount: u8,
}

impl Message for PlayerDamaged {
    const TOPIC: &'static str = "game/player-damaged";
}

impl EncodeMessage for PlayerDamaged {
    fn encode(&self) -> Vec<u8> {
        Writer::new().u8(self.amount).finish()
    }
}

impl<'a> DecodeMessage<'a> for PlayerDamaged {
    fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        let mut reader = Reader::new(payload);
        let message = Self {
            amount: reader.read_u8()?,
        };
        reader.finish()?;
        Ok(message)
    }
}

/// Session-scoped progression granted by a destroyed level-owned target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RewardGranted {
    pub experience: u32,
    pub coins: u32,
}

impl Message for RewardGranted {
    const TOPIC: &'static str = "game/reward-granted";
}

impl EncodeMessage for RewardGranted {
    fn encode(&self) -> Vec<u8> {
        Writer::new().u32(self.experience).u32(self.coins).finish()
    }
}

impl<'a> DecodeMessage<'a> for RewardGranted {
    fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        let mut reader = Reader::new(payload);
        let message = Self {
            experience: reader.read_u32()?,
            coins: reader.read_u32()?,
        };
        reader.finish()?;
        Ok(message)
    }
}

/// Confirms the target and world position affected by a melee swing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitConfirmed {
    pub sequence: u32,
    pub entity_id: u32,
    pub x: f32,
    pub y: f32,
}

impl Message for HitConfirmed {
    const TOPIC: &'static str = "game/hit-confirmed";
}

impl EncodeMessage for HitConfirmed {
    fn encode(&self) -> Vec<u8> {
        Writer::new()
            .u32(self.sequence)
            .u32(self.entity_id)
            .f32(self.x)
            .f32(self.y)
            .finish()
    }
}

impl<'a> DecodeMessage<'a> for HitConfirmed {
    fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        let mut reader = Reader::new(payload);
        let message = Self {
            sequence: reader.read_u32()?,
            entity_id: reader.read_u32()?,
            x: reader.read_f32()?,
            y: reader.read_f32()?,
        };
        reader.finish()?;
        Ok(message)
    }
}

/// Announces that Will exhausted all lives and the active level must restart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerDefeated;

impl Message for PlayerDefeated {
    const TOPIC: &'static str = "game/player-defeated";
}

impl EncodeMessage for PlayerDefeated {
    fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl<'a> DecodeMessage<'a> for PlayerDefeated {
    fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        Reader::new(payload).finish()?;
        Ok(Self)
    }
}

/// Current session-scoped values rendered by the gameplay HUD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerStats {
    pub lives: u8,
    pub experience: u32,
    pub level: u32,
    pub coins: u32,
}

impl Message for PlayerStats {
    const TOPIC: &'static str = "game/player-stats";
}

impl EncodeMessage for PlayerStats {
    fn encode(&self) -> Vec<u8> {
        Writer::new()
            .u8(self.lives)
            .u32(self.experience)
            .u32(self.level)
            .u32(self.coins)
            .finish()
    }
}

impl<'a> DecodeMessage<'a> for PlayerStats {
    fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        let mut reader = Reader::new(payload);
        let message = Self {
            lives: reader.read_u8()?,
            experience: reader.read_u32()?,
            level: reader.read_u32()?,
            coins: reader.read_u32()?,
        };
        reader.finish()?;
        Ok(message)
    }
}

#[cfg(test)]
mod tests;
