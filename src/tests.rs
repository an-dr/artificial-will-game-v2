use std::sync::{Arc, Mutex};

use bones_messages::gfx::{DrawRect, DrawSprite, DrawText};
use bones_messages::input::KeyDown;
use bones_messages::{DecodeMessage, EncodeMessage, Message};
use bus::Envelope;

struct SavesDir(std::path::PathBuf);

impl SavesDir {
    fn create() -> Self {
        let path =
            std::env::temp_dir().join(format!("artificial-will-menu-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        Self(path)
    }
}

impl Drop for SavesDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

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

fn assert_screen_title(
    built: &mut runner::BuiltEngine,
    captured_graphics: &Arc<Mutex<Vec<Envelope>>>,
    expected: &str,
) {
    captured_graphics.lock().unwrap().clear();
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();
    let graphics = captured_graphics.lock().unwrap();
    assert!(
        graphics.iter().any(|event| {
            event.topic == DrawText::TOPIC
                && DrawText::decode(&event.payload)
                    .is_ok_and(|text| text.screen_space && text.text == expected)
        }),
        "expected menu title {expected:?}"
    );
}

#[test]
fn menu_controls_level_load_unload_and_switch_lifecycle() {
    let saves = SavesDir::create();
    let extensions = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("extensions/target/wasm32-wasip2/release");
    let mut built = runner::Engine::new()
        .extensions_dir(extensions)
        .startup_extension("menu")
        .extension_controller("menu")
        .saves_dir(saves.0.clone())
        .module(game_core::GameCore::new())
        .build()
        .unwrap();

    let captured_graphics = Arc::new(Mutex::new(Vec::new()));
    let graphics_sink = Arc::clone(&captured_graphics);
    let graphics_spy = built
        .runner
        .bus()
        .register("game-ui-test", move |event: &Envelope| {
            graphics_sink.lock().unwrap().push(event.clone());
        });
    graphics_spy.subscribe("gfx/*");
    built.runner.step(1.0 / 60.0);
    built.runner.step(1.0 / 60.0);
    let graphics = captured_graphics.lock().unwrap();
    assert!(graphics.iter().any(|event| {
        event.topic == DrawRect::TOPIC
            && DrawRect::decode(&event.payload).is_ok_and(|rectangle| rectangle.screen_space)
    }));
    assert!(graphics.iter().any(|event| {
        event.topic == DrawText::TOPIC
            && DrawText::decode(&event.payload).is_ok_and(|text| text.screen_space)
    }));
    drop(graphics);
    assert_screen_title(&mut built, &captured_graphics, "ARTIFICIAL WILL");

    assert!(built.supervisor.registry.call("test", "menu", &[]).is_ok());
    assert!(built.supervisor.registry.call("test", "will", &[]).is_err());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_err());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_two", &[])
        .is_err());

    press(&mut built, "Return");
    assert_screen_title(&mut built, &captured_graphics, "SELECT LEVEL");
    press(&mut built, "Return");
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();

    assert!(built.supervisor.registry.call("test", "will", &[]).is_ok());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_ok());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_two", &[])
        .is_err());

    press(&mut built, "Escape");
    assert_screen_title(&mut built, &captured_graphics, "SYSTEM PAUSED");
    press(&mut built, "Down");
    press(&mut built, "Down");
    press(&mut built, "Down");
    press(&mut built, "Return");
    assert_screen_title(&mut built, &captured_graphics, "ARTIFICIAL WILL");
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();

    assert!(built.supervisor.registry.call("test", "will", &[]).is_err());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_err());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_two", &[])
        .is_err());

    press(&mut built, "Return");
    assert_screen_title(&mut built, &captured_graphics, "SELECT LEVEL");
    press(&mut built, "Down");
    press(&mut built, "Return");
    for _ in 0..4 {
        built.runner.step(1.0 / 60.0);
        built.supervisor.check();
    }

    assert!(built.supervisor.registry.call("test", "will", &[]).is_ok());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_err());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_two", &[])
        .is_ok());
    let graphics = captured_graphics.lock().unwrap();
    let drawn_sprite_ids = graphics
        .iter()
        .filter(|event| event.topic == DrawSprite::TOPIC)
        .filter_map(|event| DrawSprite::decode(&event.payload).ok())
        .map(|sprite| sprite.id)
        .collect::<Vec<_>>();
    assert!(drawn_sprite_ids.contains(&20));
    assert!(drawn_sprite_ids.contains(&21));
    assert!(drawn_sprite_ids.contains(&22));
    assert!(drawn_sprite_ids.iter().any(|id| (30..=32).contains(id)));
    drop(graphics);

    press(&mut built, "Escape");
    assert_screen_title(&mut built, &captured_graphics, "SYSTEM PAUSED");
    press(&mut built, "Down");
    press(&mut built, "Down");
    press(&mut built, "Return");
    assert_screen_title(&mut built, &captured_graphics, "SELECT LEVEL");
    press(&mut built, "Return");
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();

    assert!(built.supervisor.registry.call("test", "will", &[]).is_ok());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_ok());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_two", &[])
        .is_err());
}
