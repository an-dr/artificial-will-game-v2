use std::sync::{Arc, Mutex};

use bones_messages::gfx::{DrawRect, DrawText};
use bones_messages::input::KeyDown;
use bones_messages::{DecodeMessage, EncodeMessage, Message};
use bus::Envelope;

fn press(built: &mut runner::BuiltEngine, key: &'static str) {
    built.runner.bus().publish(Envelope {
        topic: KeyDown::TOPIC.to_owned(),
        sender: "platform".to_owned(),
        correlation: None,
        payload: KeyDown { key }.encode(),
    });
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();
}

#[test]
fn menu_controls_the_complete_level_one_load_unload_reload_lifecycle() {
    let saves =
        std::env::temp_dir().join(format!("artificial-will-menu-test-{}", std::process::id()));
    let extensions = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("extensions/target/wasm32-wasip2/release");
    let mut built = runner::Engine::new()
        .extensions_dir(extensions)
        .startup_extension("menu")
        .extension_controller("menu")
        .saves_dir(saves)
        .module(game_core::GameCore::new())
        .build()
        .unwrap();

    let graphics = Arc::new(Mutex::new(Vec::new()));
    let graphics_sink = Arc::clone(&graphics);
    let graphics_spy = built
        .runner
        .bus()
        .register("game-ui-test", move |event: &Envelope| {
            graphics_sink.lock().unwrap().push(event.clone());
        });
    graphics_spy.subscribe("gfx/*");
    built.runner.step(1.0 / 60.0);
    built.runner.step(1.0 / 60.0);
    let graphics = graphics.lock().unwrap();
    assert!(graphics.iter().any(|event| {
        event.topic == DrawRect::TOPIC
            && DrawRect::decode(&event.payload).is_ok_and(|rectangle| rectangle.screen_space)
    }));
    assert!(graphics.iter().any(|event| {
        event.topic == DrawText::TOPIC
            && DrawText::decode(&event.payload).is_ok_and(|text| text.screen_space)
    }));
    drop(graphics);

    assert!(built.supervisor.registry.call("test", "menu", &[]).is_ok());
    assert!(built.supervisor.registry.call("test", "will", &[]).is_err());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_err());

    press(&mut built, "Return");
    press(&mut built, "Return");
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();

    assert!(built.supervisor.registry.call("test", "will", &[]).is_ok());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_ok());

    press(&mut built, "Escape");
    press(&mut built, "Down");
    press(&mut built, "Down");
    press(&mut built, "Down");
    press(&mut built, "Return");
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();

    assert!(built.supervisor.registry.call("test", "will", &[]).is_err());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_err());

    press(&mut built, "Return");
    press(&mut built, "Return");
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();

    assert!(built.supervisor.registry.call("test", "will", &[]).is_ok());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_ok());
}
