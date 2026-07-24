wit_bindgen::generate!({
    path: "../../vendor/bones/wit",
    world: "extension",
});

use bones::core::host_api::{log, publish, subscribe, Level};

struct Component;

impl Guest for Component {
    fn init() {
        subscribe("core/tick");
        log(Level::Info, "will: init");
    }

    fn on_tick(dt: f32) {
        log(Level::Debug, &format!("will: tick dt={dt}"));
    }

    fn on_message(topic: String, sender: String, _payload: Vec<u8>) -> Option<Vec<u8>> {
        log(Level::Debug, &format!("will: message on {topic} from {sender}"));
        publish("will/received", topic.as_bytes());
        None
    }
}

export!(Component);
