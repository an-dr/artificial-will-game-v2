//! Game-owned message contracts and entity constants shared by extensions.

use bones_messages::{DecodeError, DecodeMessage, EncodeMessage, Message, Reader, Writer};

/// Stable entity id used by the Will extension.
pub const WILL_ENTITY_ID: u32 = 1;
/// Initial world-space position used when Will spawns.
pub const WILL_SPAWN: (f32, f32) = (464.0, 464.0);

/// Announces whether gameplay listeners should pause their behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PauseChanged {
    pub paused: bool,
}

impl Message for PauseChanged {
    const TOPIC: &'static str = "game/pause-changed";
}

impl EncodeMessage for PauseChanged {
    fn encode(&self) -> Vec<u8> {
        Writer::new().u8(u8::from(self.paused)).finish()
    }
}

impl<'a> DecodeMessage<'a> for PauseChanged {
    fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        let mut reader = Reader::new(payload);
        let paused = match reader.read_u8()? {
            0 => false,
            1 => true,
            tag => {
                return Err(DecodeError::InvalidTag {
                    message: "pause state",
                    tag,
                })
            }
        };
        reader.finish()?;
        Ok(Self { paused })
    }
}

/// Announces that the current gameplay session is being discarded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionReset;

impl Message for SessionReset {
    const TOPIC: &'static str = "game/session-reset";
}

impl EncodeMessage for SessionReset {
    fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl<'a> DecodeMessage<'a> for SessionReset {
    fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        Reader::new(payload).finish()?;
        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pause_changed_round_trips_and_rejects_unknown_states() {
        let message = PauseChanged { paused: true };
        assert_eq!(PauseChanged::decode(&message.encode()), Ok(message));
        assert!(PauseChanged::decode(&[2]).is_err());
    }

    #[test]
    fn session_reset_accepts_only_an_empty_payload() {
        assert_eq!(
            SessionReset::decode(&SessionReset.encode()),
            Ok(SessionReset)
        );
        assert!(SessionReset::decode(&[0]).is_err());
    }
}
