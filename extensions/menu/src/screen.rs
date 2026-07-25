/// A visible state in the game menu flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    /// Initial title screen.
    Start,
    /// Level chooser shown before or during a session.
    LevelSelection,
    /// Menu-hidden live gameplay.
    Gameplay,
    /// In-session pause menu.
    Pause,
    /// Display preferences opened from start or pause.
    Settings,
}
