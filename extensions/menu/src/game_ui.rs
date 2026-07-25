use crate::display_preferences::DisplayPreferences;
use crate::screen::Screen;

pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;
pub const START: u32 = 1;
pub const START_SETTINGS: u32 = 2;
pub const QUIT: u32 = 3;
pub const LEVEL_ONE: u32 = 10;
pub const LEVEL_BACK: u32 = 11;
pub const RESUME: u32 = 20;
pub const PAUSE_SETTINGS: u32 = 21;
pub const PAUSE_LEVELS: u32 = 22;
pub const MAIN_MENU: u32 = 23;
pub const PAUSE_QUIT: u32 = 24;
pub const FULLSCREEN: u32 = 30;
pub const SETTINGS_BACK: u32 = 31;
pub const RESOLUTION_BASE: u32 = 100;

const PANEL_WIDTH: u32 = 420;
const TITLE_HEIGHT: i32 = 104;
const PANEL_PADDING: i32 = 28;
const BUTTON_HEIGHT: u32 = 44;
const BUTTON_GAP: i32 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonLayout {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl ButtonLayout {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x as f32
            && x < (self.x + self.width as i32) as f32
            && y >= self.y as f32
            && y < (self.y + self.height as i32) as f32
    }
}

pub fn build_buttons(
    screen: Screen,
    preferences: DisplayPreferences,
    resolutions: &[(u32, u32)],
) -> Vec<ButtonLayout> {
    let entries: Vec<(u32, String)> = match screen {
        Screen::Start => vec![
            (START, "Start".to_owned()),
            (START_SETTINGS, "Settings".to_owned()),
            (QUIT, "Quit".to_owned()),
        ],
        Screen::LevelSelection => vec![
            (LEVEL_ONE, "Level One".to_owned()),
            (LEVEL_BACK, "Back".to_owned()),
        ],
        Screen::Pause => vec![
            (RESUME, "Resume".to_owned()),
            (PAUSE_SETTINGS, "Settings".to_owned()),
            (PAUSE_LEVELS, "Level Selection".to_owned()),
            (MAIN_MENU, "Main Menu".to_owned()),
            (PAUSE_QUIT, "Quit".to_owned()),
        ],
        Screen::Settings => {
            let mut entries = vec![(
                FULLSCREEN,
                format!(
                    "Fullscreen: {}",
                    if preferences.fullscreen { "On" } else { "Off" }
                ),
            )];
            entries.extend(
                resolutions
                    .iter()
                    .enumerate()
                    .map(|(index, &(width, height))| {
                        (
                            RESOLUTION_BASE + index as u32,
                            format!(
                                "{width} x {height}{}",
                                if (width, height) == (preferences.width, preferences.height) {
                                    "  <"
                                } else {
                                    ""
                                }
                            ),
                        )
                    }),
            );
            entries.push((SETTINGS_BACK, "Back".to_owned()));
            entries
        }
        Screen::Gameplay => Vec::new(),
    };
    let panel_height = panel_height(entries.len());
    let panel_x = (SCREEN_WIDTH - PANEL_WIDTH as i32) / 2;
    let panel_y = (SCREEN_HEIGHT - panel_height as i32) / 2;
    let button_x = panel_x + PANEL_PADDING;
    let button_width = PANEL_WIDTH - (PANEL_PADDING * 2) as u32;
    entries
        .into_iter()
        .enumerate()
        .map(|(index, (id, label))| ButtonLayout {
            id,
            x: button_x,
            y: panel_y + TITLE_HEIGHT + index as i32 * (BUTTON_HEIGHT as i32 + BUTTON_GAP),
            width: button_width,
            height: BUTTON_HEIGHT,
            label,
        })
        .collect()
}

pub fn panel_height(button_count: usize) -> u32 {
    let buttons_height = button_count as i32 * BUTTON_HEIGHT as i32
        + button_count.saturating_sub(1) as i32 * BUTTON_GAP;
    (TITLE_HEIGHT + buttons_height + PANEL_PADDING) as u32
}

pub fn find_button_at(
    buttons: &[ButtonLayout],
    physical_x: f32,
    physical_y: f32,
    window_size: (u32, u32),
) -> Option<usize> {
    let (window_width, window_height) = window_size;
    if window_width == 0 || window_height == 0 {
        return None;
    }
    let logical_x = physical_x * SCREEN_WIDTH as f32 / window_width as f32;
    let logical_y = physical_y * SCREEN_HEIGHT as f32 / window_height as f32;
    buttons
        .iter()
        .position(|button| button.contains(logical_x, logical_y))
}

pub fn move_selection(current: usize, button_count: usize, direction: i32) -> usize {
    if button_count == 0 {
        return 0;
    }
    (current as i32 + direction).rem_euclid(button_count as i32) as usize
}

#[cfg(test)]
mod tests;
