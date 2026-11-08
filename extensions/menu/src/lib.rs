#![cfg_attr(test, allow(dead_code))]

mod display_preferences;
mod game_ui;
mod level;
mod menu_state;
mod resolution_options;
mod screen;
mod session_request;

mod runtime;

#[cfg_attr(test, allow(dead_code))]
use bones_wasm_sdk::Guest;

struct Component;

impl Guest for Component {
    fn shutdown() {
        runtime::shutdown();
    }

    fn init() {
        runtime::init();
    }

    fn on_tick(_dt: f32) {
        runtime::publish_ui();
    }

    fn on_message(topic: String, sender: String, payload: Vec<u8>) -> Option<Vec<u8>> {
        runtime::handle_message(&topic, &sender, &payload);
        None
    }
}

#[cfg(not(test))]
bones_wasm_sdk::export!(Component);
