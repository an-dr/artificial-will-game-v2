const VERSION: u8 = 1;
const ENCODED_LEN: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayPreferences {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

impl Default for DisplayPreferences {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            fullscreen: false,
        }
    }
}

impl DisplayPreferences {
    pub fn encode(self) -> [u8; ENCODED_LEN] {
        let mut bytes = [0; ENCODED_LEN];
        bytes[0] = VERSION;
        bytes[1..5].copy_from_slice(&self.width.to_le_bytes());
        bytes[5..9].copy_from_slice(&self.height.to_le_bytes());
        bytes[9] = u8::from(self.fullscreen);
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != ENCODED_LEN || bytes[0] != VERSION || bytes[9] > 1 {
            return None;
        }
        let width = u32::from_le_bytes(bytes[1..5].try_into().ok()?);
        let height = u32::from_le_bytes(bytes[5..9].try_into().ok()?);
        if width == 0 || height == 0 {
            return None;
        }
        Some(Self {
            width,
            height,
            fullscreen: bytes[9] == 1,
        })
    }
}

#[cfg(test)]
mod tests;
