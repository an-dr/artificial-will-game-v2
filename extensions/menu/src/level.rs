#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    One,
    Two,
}

impl Level {
    pub const fn extension_name(self) -> &'static str {
        match self {
            Self::One => "level_one",
            Self::Two => "level_two",
        }
    }
}
