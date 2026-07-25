#[cfg(not(test))]
wit_bindgen::generate!({
    path: "../../vendor/bones/wit",
    world: "extension",
});

mod display_preferences;
mod level;
mod menu_state;
mod resolution_options;
mod screen;
mod session_request;

#[cfg(not(test))]
mod runtime;

#[cfg(not(test))]
struct Component;

#[cfg(not(test))]
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

    fn on_message(topic: String, _sender: String, payload: Vec<u8>) -> Option<Vec<u8>> {
        runtime::handle_message(&topic, &payload);
        None
    }
}

#[cfg(not(test))]
export!(Component);
