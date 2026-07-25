use crate::level::Level;
use crate::screen::Screen;
use crate::session_request::SessionRequest;

/// Pure navigation state for the game menu and active level session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuState {
    screen: Screen,
    settings_return: Screen,
    active_level: Option<Level>,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            screen: Screen::Start,
            settings_return: Screen::Start,
            active_level: None,
        }
    }
}

impl MenuState {
    /// Returns the currently visible menu screen.
    pub fn screen(self) -> Screen {
        self.screen
    }

    /// Returns the level backing the current gameplay session, if any.
    pub fn active_level(self) -> Option<Level> {
        self.active_level
    }

    /// Opens level selection from the current menu context.
    pub fn open_level_selection(&mut self) {
        self.screen = Screen::LevelSelection;
    }

    /// Selects `level`, enters gameplay, and returns the required lifecycle effect.
    pub fn select_level(&mut self, level: Level) -> SessionRequest {
        let previous = self.active_level.replace(level);
        self.screen = Screen::Gameplay;
        SessionRequest::Replace {
            previous,
            next: level,
        }
    }

    /// Restarts the active level after defeat and returns to live gameplay.
    pub fn restart_active_level(&mut self) -> Option<SessionRequest> {
        let level = self.active_level?;
        self.screen = Screen::Gameplay;
        Some(SessionRequest::Replace {
            previous: Some(level),
            next: level,
        })
    }

    /// Returns from level selection; `false` means the transition was unavailable.
    pub fn cancel_level_selection(&mut self) -> bool {
        if self.screen != Screen::LevelSelection {
            return false;
        }
        self.screen = if self.active_level.is_some() {
            Screen::Pause
        } else {
            Screen::Start
        };
        true
    }

    /// Pauses live gameplay; `false` means no live gameplay screen was active.
    pub fn pause(&mut self) -> bool {
        if self.screen != Screen::Gameplay || self.active_level.is_none() {
            return false;
        }
        self.screen = Screen::Pause;
        true
    }

    /// Resumes paused gameplay; `false` means the pause screen was not active.
    pub fn resume(&mut self) -> bool {
        if self.screen != Screen::Pause {
            return false;
        }
        self.screen = Screen::Gameplay;
        true
    }

    /// Opens settings; `false` means the current screen cannot open them.
    pub fn open_settings(&mut self) -> bool {
        if !matches!(self.screen, Screen::Start | Screen::Pause) {
            return false;
        }
        self.settings_return = self.screen;
        self.screen = Screen::Settings;
        true
    }

    /// Closes settings to their opener; `false` means settings were not open.
    pub fn close_settings(&mut self) -> bool {
        if self.screen != Screen::Settings {
            return false;
        }
        self.screen = self.settings_return;
        true
    }

    /// Returns home and requests that any active session be stopped.
    pub fn return_to_start(&mut self) -> Option<SessionRequest> {
        let request = self.active_level.take().map(SessionRequest::Stop);
        self.screen = Screen::Start;
        request
    }
}

#[cfg(test)]
mod tests;
