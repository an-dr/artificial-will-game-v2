use std::cell::RefCell;

use bones_messages::extension_control::{Load, Unload};
use bones_messages::game_core::{EntityOp, EntityOpMessage};
use bones_messages::gfx::{DrawRect, DrawText, SetDisplay};
use bones_messages::input::{KeyDown, MouseDown, MouseMove};
use bones_messages::persistence::{Save, ENDPOINT as PERSISTENCE};
use bones_messages::renderer::DisplayChanged;
use bones_messages::{DecodeMessage, EncodeMessage, Message};

use crate::bones::core::host_api::{
    list_display_modes, log, native_display_mode, publish, request_exit, send, subscribe,
    DisplayMode, Level as LogLevel,
};
use crate::display_preferences::DisplayPreferences;
use crate::game_ui::{
    build_buttons, find_button_at, move_selection, panel_height, FULLSCREEN, LEVEL_BACK, LEVEL_ONE,
    MAIN_MENU, PAUSE_LEVELS, PAUSE_QUIT, PAUSE_SETTINGS, QUIT, RESOLUTION_BASE, RESUME,
    SCREEN_HEIGHT, SCREEN_WIDTH, SETTINGS_BACK, START, START_SETTINGS,
};
use crate::level::Level;
use crate::menu_state::MenuState;
use crate::resolution_options::normalize_resolutions;
use crate::screen::Screen;

const MENU_LAYER: u8 = 250;
const PANEL_WIDTH: u32 = 420;
const PANEL_COLOR: (u8, u8, u8, u8) = (14, 18, 30, 255);
const BORDER_COLOR: (u8, u8, u8, u8) = (85, 190, 220, 255);
const TITLE_COLOR: (u8, u8, u8, u8) = (235, 248, 255, 255);
const SUBTITLE_COLOR: (u8, u8, u8, u8) = (145, 178, 195, 255);
const BUTTON_COLOR: (u8, u8, u8, u8) = (34, 52, 78, 255);
const SELECTED_COLOR: (u8, u8, u8, u8) = (45, 132, 160, 255);

// `State` stays here because it is only the component adapter's thread-local
// storage and has no identity outside this file.
struct State {
    menu: MenuState,
    preferences: DisplayPreferences,
    resolutions: Vec<(u32, u32)>,
    window_size: (u32, u32),
    selected: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            menu: MenuState::default(),
            preferences: DisplayPreferences::default(),
            resolutions: vec![(800, 600)],
            window_size: (800, 600),
            selected: 0,
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
    subscribe(KeyDown::TOPIC);
    subscribe(MouseDown::TOPIC);
    subscribe(MouseMove::TOPIC);
    subscribe(DisplayChanged::TOPIC);
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
            window_size: (800, 600),
            selected: 0,
        };
    });
    apply_display(preferences);
    log(LogLevel::Info, "menu: game-native startup screen ready");
}

pub fn shutdown() {
    let active = STATE.with(|state| state.borrow().menu.active_level());
    if let Some(level) = active {
        stop_session(level);
    }
}

fn screen_copy() -> (Screen, DisplayPreferences, Vec<(u32, u32)>, usize) {
    STATE.with(|state| {
        let state = state.borrow();
        (
            state.menu.screen(),
            state.preferences,
            state.resolutions.clone(),
            state.selected,
        )
    })
}

fn title_for(screen: Screen) -> (&'static str, &'static str) {
    match screen {
        Screen::Start => ("ARTIFICIAL WILL", "A machine with a will of its own"),
        Screen::LevelSelection => ("SELECT LEVEL", "Choose where Will wakes"),
        Screen::Pause => ("SYSTEM PAUSED", "Escape resumes the simulation"),
        Screen::Settings => ("DISPLAY SETTINGS", "Changes apply immediately"),
        Screen::Gameplay => ("", ""),
    }
}

fn draw_rect(x: i32, y: i32, width: u32, height: u32, filled: bool, color: (u8, u8, u8, u8)) {
    publish_message(DrawRect {
        x,
        y,
        w: width,
        h: height,
        filled,
        color,
        layer: MENU_LAYER,
        screen_space: true,
    });
}

fn draw_text(text: &str, x: i32, y: i32, size: u16, color: (u8, u8, u8, u8)) {
    publish_message(DrawText {
        text,
        x,
        y,
        size,
        color,
        layer: MENU_LAYER,
        screen_space: true,
    });
}

pub fn publish_ui() {
    let (screen, preferences, resolutions, selected) = screen_copy();
    let buttons = build_buttons(screen, preferences, &resolutions);
    if screen == Screen::Gameplay {
        draw_rect(-2, -2, 1, 1, true, (0, 0, 0, 0));
        return;
    }

    let height = panel_height(buttons.len());
    let x = (SCREEN_WIDTH - PANEL_WIDTH as i32) / 2;
    let y = (SCREEN_HEIGHT - height as i32) / 2;
    draw_rect(x, y, PANEL_WIDTH, height, true, PANEL_COLOR);
    draw_rect(x, y, PANEL_WIDTH, height, false, BORDER_COLOR);

    let (title, subtitle) = title_for(screen);
    draw_text(title, x + 28, y + 22, 24, TITLE_COLOR);
    draw_text(subtitle, x + 28, y + 58, 14, SUBTITLE_COLOR);

    for (index, button) in buttons.iter().enumerate() {
        draw_rect(
            button.x,
            button.y,
            button.width,
            button.height,
            true,
            if index == selected {
                SELECTED_COLOR
            } else {
                BUTTON_COLOR
            },
        );
        if index == selected {
            draw_text(">", button.x + 14, button.y + 12, 18, TITLE_COLOR);
        }
        let text_x = button.x + (button.width as i32 - button.label.chars().count() as i32 * 8) / 2;
        draw_text(&button.label, text_x, button.y + 12, 16, TITLE_COLOR);
    }
}

fn reset_selection(state: &mut State) {
    state.selected = 0;
}

fn activate_button(id: u32) {
    if matches!(id, QUIT | PAUSE_QUIT) {
        request_exit();
        return;
    }
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match id {
            START => {
                state.menu.open_level_selection();
                reset_selection(&mut state);
            }
            START_SETTINGS | PAUSE_SETTINGS => {
                state.menu.open_settings();
                reset_selection(&mut state);
            }
            LEVEL_ONE => {
                let previous = state.menu.active_level();
                state.menu.select_level(Level::One);
                reset_selection(&mut state);
                replace_session(previous, Level::One);
            }
            LEVEL_BACK => {
                state.menu.cancel_level_selection();
                reset_selection(&mut state);
            }
            RESUME => {
                if state.menu.resume() {
                    reset_selection(&mut state);
                    set_paused(false);
                }
            }
            PAUSE_LEVELS => {
                state.menu.open_level_selection();
                reset_selection(&mut state);
            }
            MAIN_MENU => {
                let active = state.menu.active_level();
                state.menu.return_to_start();
                reset_selection(&mut state);
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
                reset_selection(&mut state);
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

fn handle_escape() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match state.menu.screen() {
            Screen::Gameplay if state.menu.pause() => {
                reset_selection(&mut state);
                set_paused(true);
            }
            Screen::Pause if state.menu.resume() => {
                reset_selection(&mut state);
                set_paused(false);
            }
            Screen::Settings => {
                state.menu.close_settings();
                reset_selection(&mut state);
            }
            Screen::LevelSelection => {
                state.menu.cancel_level_selection();
                reset_selection(&mut state);
            }
            _ => {}
        }
    });
}

fn handle_key(key: &str) {
    if key == "Escape" {
        handle_escape();
        return;
    }
    let action = STATE.with(|state| {
        let mut state = state.borrow_mut();
        let buttons = build_buttons(state.menu.screen(), state.preferences, &state.resolutions);
        match key {
            "Up" | "W" | "Left" | "A" => {
                state.selected = move_selection(state.selected, buttons.len(), -1);
                None
            }
            "Down" | "S" | "Right" | "D" => {
                state.selected = move_selection(state.selected, buttons.len(), 1);
                None
            }
            "Return" | "Enter" | "Space" => buttons.get(state.selected).map(|button| button.id),
            _ => None,
        }
    });
    if let Some(id) = action {
        activate_button(id);
    }
}

fn button_at_pointer(x: f32, y: f32) -> Option<(usize, u32)> {
    STATE.with(|state| {
        let state = state.borrow();
        let buttons = build_buttons(state.menu.screen(), state.preferences, &state.resolutions);
        let index = find_button_at(&buttons, x, y, state.window_size)?;
        Some((index, buttons[index].id))
    })
}

fn handle_pointer_move(x: f32, y: f32) {
    if let Some((index, _)) = button_at_pointer(x, y) {
        STATE.with(|state| state.borrow_mut().selected = index);
    }
}

fn handle_pointer_down(button: u8, x: f32, y: f32) {
    if button != 1 {
        return;
    }
    if let Some((index, id)) = button_at_pointer(x, y) {
        STATE.with(|state| state.borrow_mut().selected = index);
        activate_button(id);
    }
}

pub fn handle_message(topic: &str, payload: &[u8]) {
    match topic {
        KeyDown::TOPIC => {
            if let Ok(key) = KeyDown::decode(payload) {
                handle_key(key.key);
            }
        }
        MouseMove::TOPIC => {
            if let Ok(pointer) = MouseMove::decode(payload) {
                handle_pointer_move(pointer.x, pointer.y);
            }
        }
        MouseDown::TOPIC => {
            if let Ok(pointer) = MouseDown::decode(payload) {
                handle_pointer_down(pointer.button, pointer.x, pointer.y);
            }
        }
        DisplayChanged::TOPIC => {
            if let Ok(display) = DisplayChanged::decode(payload) {
                STATE
                    .with(|state| state.borrow_mut().window_size = (display.width, display.height));
            }
        }
        _ => {}
    }
}
