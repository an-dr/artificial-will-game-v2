use crate::level::Level;
use crate::screen::Screen;
use crate::session_request::SessionRequest;

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
    pub fn screen(self) -> Screen {
        self.screen
    }

    pub fn active_level(self) -> Option<Level> {
        self.active_level
    }

    pub fn open_level_selection(&mut self) {
        self.screen = Screen::LevelSelection;
    }

    pub fn select_level(&mut self, level: Level) -> SessionRequest {
        self.active_level = Some(level);
        self.screen = Screen::Gameplay;
        SessionRequest::Replace(level)
    }

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

    pub fn pause(&mut self) -> bool {
        if self.screen != Screen::Gameplay || self.active_level.is_none() {
            return false;
        }
        self.screen = Screen::Pause;
        true
    }

    pub fn resume(&mut self) -> bool {
        if self.screen != Screen::Pause {
            return false;
        }
        self.screen = Screen::Gameplay;
        true
    }

    pub fn open_settings(&mut self) -> bool {
        if !matches!(self.screen, Screen::Start | Screen::Pause) {
            return false;
        }
        self.settings_return = self.screen;
        self.screen = Screen::Settings;
        true
    }

    pub fn close_settings(&mut self) -> bool {
        if self.screen != Screen::Settings {
            return false;
        }
        self.screen = self.settings_return;
        true
    }

    pub fn return_to_start(&mut self) -> Option<SessionRequest> {
        let request = self.active_level.take().map(|_| SessionRequest::Stop);
        self.screen = Screen::Start;
        request
    }
}

#[cfg(test)]
mod tests;
