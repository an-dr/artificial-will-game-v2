use std::cell::RefCell;

use bones_messages::extension_control::{Load, Unload};
use bones_messages::game_core::{EntityOp, EntityOpMessage};
use bones_messages::gfx::SetDisplay;
use bones_messages::input::KeyDown;
use bones_messages::persistence::{Save, ENDPOINT as PERSISTENCE};
use bones_messages::ui::{Clicked, Spec, Widget};
use bones_messages::{DecodeMessage, EncodeMessage, Message};

use crate::bones::core::host_api::{
    list_display_modes, log, native_display_mode, publish, request_exit, send, subscribe,
    DisplayMode, Level as LogLevel,
};
use crate::display_preferences::DisplayPreferences;
use crate::level::Level;
use crate::menu_state::MenuState;
use crate::resolution_options::normalize_resolutions;
use crate::screen::Screen;

const START: u32 = 1;
const START_SETTINGS: u32 = 2;
const QUIT: u32 = 3;
const LEVEL_ONE: u32 = 10;
const LEVEL_BACK: u32 = 11;
const RESUME: u32 = 20;
const PAUSE_SETTINGS: u32 = 21;
const PAUSE_LEVELS: u32 = 22;
const MAIN_MENU: u32 = 23;
const PAUSE_QUIT: u32 = 24;
const FULLSCREEN: u32 = 30;
const SETTINGS_BACK: u32 = 31;
const RESOLUTION_BASE: u32 = 100;

// `State` stays here because it is only the component adapter's thread-local
// storage and has no identity outside this file.
struct State {
    menu: MenuState,
    preferences: DisplayPreferences,
    resolutions: Vec<(u32, u32)>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            menu: MenuState::default(),
            preferences: DisplayPreferences::default(),
            resolutions: vec![(800, 600)],
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn publish_message<M: EncodeMessage>(message: M) {
    publish(M::TOPIC, &message.encode());
}

fn set_paused(paused: bool) {
    publish_message(EntityOpMessage(EntityOp::SetPaused { paused }));
}

fn reset_game_core() {
    publish_message(EntityOpMessage(EntityOp::Reset));
}

fn unload(extension: &str) {
    publish_message(Unload { extension });
}

fn load(extension: &str) {
    publish_message(Load { extension });
}

fn replace_session(previous: Option<Level>, next: Level) {
    set_paused(true);
    if previous.is_some() {
        unload("will");
    }
    if let Some(level) = previous {
        unload(level.extension_name());
    }
    reset_game_core();
    load(next.extension_name());
    load("will");
    set_paused(false);
}

fn stop_session(level: Level) {
    set_paused(true);
    unload("will");
    unload(level.extension_name());
    reset_game_core();
}

fn apply_display(preferences: DisplayPreferences) {
    publish_message(SetDisplay {
        width: preferences.width,
        height: preferences.height,
        fullscreen: preferences.fullscreen,
    });
}

fn save_display(preferences: DisplayPreferences) {
    let bytes = preferences.encode();
    publish_message(Save { bytes: &bytes });
}

fn query_resolutions() -> Vec<(u32, u32)> {
    let mut modes: Vec<_> = list_display_modes()
        .into_iter()
        .map(|DisplayMode { width, height }| (width, height))
        .collect();
    if let Some(DisplayMode { width, height }) = native_display_mode() {
        modes.push((width, height));
    }
    normalize_resolutions(modes)
}

pub fn init() {
    subscribe(Clicked::TOPIC);
    subscribe(KeyDown::TOPIC);
    subscribe("core/tick");

    let resolutions = query_resolutions();
    let loaded = send(PERSISTENCE, &[])
        .ok()
        .and_then(|bytes| DisplayPreferences::decode(&bytes))
        .filter(|saved| resolutions.contains(&(saved.width, saved.height)));
    let preferences = loaded.unwrap_or_default();
    STATE.with(|state| {
        *state.borrow_mut() = State {
            menu: MenuState::default(),
            preferences,
            resolutions,
        };
    });
    apply_display(preferences);
    log(LogLevel::Info, "menu: startup screen ready");
}

pub fn shutdown() {
    let active = STATE.with(|state| state.borrow().menu.active_level());
    if let Some(level) = active {
        stop_session(level);
    }
}

fn label(text: &str) -> Widget<'_> {
    Widget::Label { text }
}

fn button(id: u32, label: &str) -> Widget<'_> {
    Widget::Button { id, label }
}

pub fn publish_ui() {
    STATE.with(|state| {
        let state = state.borrow();
        let mut owned_labels = Vec::new();
        let mut widgets = match state.menu.screen() {
            Screen::Start => vec![
                label("A robot named Will. A world to overcome."),
                button(START, "Start"),
                button(START_SETTINGS, "Settings"),
                button(QUIT, "Quit"),
            ],
            Screen::LevelSelection => vec![
                label("Choose a level"),
                button(LEVEL_ONE, "Level One"),
                button(LEVEL_BACK, "Back"),
            ],
            Screen::Pause => vec![
                label("Game paused"),
                button(RESUME, "Resume"),
                button(PAUSE_SETTINGS, "Settings"),
                button(PAUSE_LEVELS, "Level Selection"),
                button(MAIN_MENU, "Main Menu"),
                button(PAUSE_QUIT, "Quit"),
            ],
            Screen::Settings => {
                owned_labels.extend(state.resolutions.iter().map(|&(width, height)| {
                    format!(
                        "{width} x {height}{}",
                        if (width, height) == (state.preferences.width, state.preferences.height) {
                            " (selected)"
                        } else {
                            ""
                        }
                    )
                }));
                let mut widgets = vec![
                    label("Screen preferences"),
                    button(
                        FULLSCREEN,
                        if state.preferences.fullscreen {
                            "Fullscreen: On"
                        } else {
                            "Fullscreen: Off"
                        },
                    ),
                ];
                widgets.extend(
                    owned_labels
                        .iter()
                        .enumerate()
                        .map(|(index, text)| button(RESOLUTION_BASE + index as u32, text)),
                );
                widgets.push(button(SETTINGS_BACK, "Back"));
                widgets
            }
            Screen::Gameplay => return,
        };
        let title = match state.menu.screen() {
            Screen::Start => "Artificial Will",
            Screen::LevelSelection => "Level Selection",
            Screen::Pause => "Paused",
            Screen::Settings => "Settings",
            Screen::Gameplay => return,
        };
        publish_message(Spec {
            title,
            widgets: std::mem::take(&mut widgets),
        });
    });
}

fn handle_clicked(id: u32) {
    if matches!(id, QUIT | PAUSE_QUIT) {
        request_exit();
        return;
    }
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match id {
            START => state.menu.open_level_selection(),
            START_SETTINGS | PAUSE_SETTINGS => {
                state.menu.open_settings();
            }
            LEVEL_ONE => {
                let previous = state.menu.active_level();
                state.menu.select_level(Level::One);
                replace_session(previous, Level::One);
            }
            LEVEL_BACK => {
                state.menu.cancel_level_selection();
            }
            RESUME => {
                if state.menu.resume() {
                    set_paused(false);
                }
            }
            PAUSE_LEVELS => state.menu.open_level_selection(),
            MAIN_MENU => {
                let active = state.menu.active_level();
                state.menu.return_to_start();
                if let Some(level) = active {
                    stop_session(level);
                }
            }
            FULLSCREEN => {
                state.preferences.fullscreen = !state.preferences.fullscreen;
                apply_display(state.preferences);
                save_display(state.preferences);
            }
            SETTINGS_BACK => {
                state.menu.close_settings();
            }
            id if id >= RESOLUTION_BASE => {
                let index = (id - RESOLUTION_BASE) as usize;
                if let Some(&(width, height)) = state.resolutions.get(index) {
                    state.preferences.width = width;
                    state.preferences.height = height;
                    apply_display(state.preferences);
                    save_display(state.preferences);
                }
            }
            _ => {}
        }
    });
}

fn handle_key(key: &str) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match key {
            "Escape" => match state.menu.screen() {
                Screen::Gameplay if state.menu.pause() => set_paused(true),
                Screen::Pause if state.menu.resume() => set_paused(false),
                Screen::Settings => {
                    state.menu.close_settings();
                }
                Screen::LevelSelection => {
                    state.menu.cancel_level_selection();
                }
                _ => {}
            },
            "Return" | "Enter" if state.menu.screen() == Screen::Start => {
                state.menu.open_level_selection();
            }
            _ => {}
        }
    });
}

pub fn handle_message(topic: &str, payload: &[u8]) {
    match topic {
        Clicked::TOPIC => {
            if let Ok(clicked) = Clicked::decode(payload) {
                handle_clicked(clicked.id);
            }
        }
        KeyDown::TOPIC => {
            if let Ok(key) = KeyDown::decode(payload) {
                handle_key(key.key);
            }
        }
        _ => {}
    }
}
