use crate::display_preferences::DisplayPreferences;
use crate::screen::Screen;
use game_ui::{Button, Canvas, MenuLayout, VerticalMenu};

pub const START: u32 = 1;
pub const START_SETTINGS: u32 = 2;
pub const QUIT: u32 = 3;
pub const LEVEL_ONE: u32 = 10;
pub const LEVEL_TWO: u32 = 11;
pub const LEVEL_BACK: u32 = 12;
pub const RESUME: u32 = 20;
pub const PAUSE_SETTINGS: u32 = 21;
pub const PAUSE_LEVELS: u32 = 22;
pub const MAIN_MENU: u32 = 23;
pub const PAUSE_QUIT: u32 = 24;
pub const GAME_OVER_MAIN_MENU: u32 = 25;
pub const FULLSCREEN: u32 = 30;
pub const SETTINGS_BACK: u32 = 31;
pub const RESOLUTION_BASE: u32 = 100;

const PANEL_WIDTH: u32 = 420;
const TITLE_HEIGHT: i32 = 104;
const PANEL_PADDING: i32 = 28;
const BUTTON_HEIGHT: u32 = 44;
const BUTTON_GAP: u32 = 10;

fn vertical_menu(canvas: Canvas) -> VerticalMenu {
    VerticalMenu {
        canvas,
        panel_width: PANEL_WIDTH.min(canvas.width),
        header_height: TITLE_HEIGHT as u32,
        padding: PANEL_PADDING as u32,
        button_height: BUTTON_HEIGHT,
        gap: BUTTON_GAP,
    }
}

pub fn build_layout(
    canvas: Canvas,
    screen: Screen,
    preferences: DisplayPreferences,
    resolutions: &[(u32, u32)],
) -> MenuLayout {
    let buttons = match screen {
        Screen::Start => vec![
            Button::new(START, "Start"),
            Button::new(START_SETTINGS, "Settings"),
            Button::new(QUIT, "Quit"),
        ],
        Screen::LevelSelection => vec![
            Button::new(LEVEL_ONE, "Level One"),
            Button::new(LEVEL_TWO, "Level Two"),
            Button::new(LEVEL_BACK, "Back"),
        ],
        Screen::Pause => vec![
            Button::new(RESUME, "Resume"),
            Button::new(PAUSE_SETTINGS, "Settings"),
            Button::new(PAUSE_LEVELS, "Level Selection"),
            Button::new(MAIN_MENU, "Main Menu"),
            Button::new(PAUSE_QUIT, "Quit"),
        ],
        Screen::GameOver => vec![Button::new(GAME_OVER_MAIN_MENU, "Press Enter to Main Menu")],
        Screen::Settings => {
            let mut buttons = vec![Button::new(
                FULLSCREEN,
                format!(
                    "Fullscreen: {}",
                    if preferences.fullscreen { "On" } else { "Off" }
                ),
            )];
            buttons.extend(
                resolutions
                    .iter()
                    .enumerate()
                    .map(|(index, &(width, height))| {
                        Button::new(
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
            buttons.push(Button::new(SETTINGS_BACK, "Back"));
            buttons
        }
        Screen::Gameplay => Vec::new(),
    };
    vertical_menu(canvas).layout(buttons)
}

#[cfg(test)]
mod tests;
