#[cfg(not(test))]
wit_bindgen::generate!({
    path: "../../vendor/bones/wit",
    world: "extension",
});

mod display_preferences;
mod level;
mod menu_state;
mod screen;
mod session_request;

#[cfg(not(test))]
struct Component;

#[cfg(not(test))]
impl Guest for Component {
    fn shutdown() {}

    fn init() {}

    fn on_tick(_dt: f32) {}

    fn on_message(_topic: String, _sender: String, _payload: Vec<u8>) -> Option<Vec<u8>> {
        None
    }
}

#[cfg(not(test))]
export!(Component);
