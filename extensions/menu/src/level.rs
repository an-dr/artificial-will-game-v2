/// A selectable game level and its runtime extension identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// The introductory grassland level.
    One,
    /// The rock-filled ruins level.
    Two,
}

impl Level {
    /// Returns the extension endpoint name that implements this level.
    pub const fn extension_name(self) -> &'static str {
        match self {
            Self::One => "level_one",
            Self::Two => "level_two",
        }
    }
}
