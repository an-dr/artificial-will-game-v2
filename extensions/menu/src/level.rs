#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    One,
}

impl Level {
    pub const fn extension_name(self) -> &'static str {
        match self {
            Self::One => "level_one",
        }
    }
}
