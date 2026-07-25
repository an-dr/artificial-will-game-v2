use bones_messages::input::KeyDown;
use bones_messages::ui::Clicked;
use bones_messages::{EncodeMessage, Message};
use bus::Envelope;

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

    assert!(built.supervisor.registry.call("test", "menu", &[]).is_ok());
    assert!(built.supervisor.registry.call("test", "will", &[]).is_err());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_err());

    for id in [1, 10] {
        built.runner.bus().publish(Envelope {
            topic: Clicked::TOPIC.to_owned(),
            sender: "ui".to_owned(),
            correlation: None,
            payload: Clicked { id }.encode(),
        });
        built.runner.step(1.0 / 60.0);
        built.supervisor.check();
    }
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();

    assert!(built.supervisor.registry.call("test", "will", &[]).is_ok());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_ok());

    built.runner.bus().publish(Envelope {
        topic: KeyDown::TOPIC.to_owned(),
        sender: "platform".to_owned(),
        correlation: None,
        payload: KeyDown { key: "Escape" }.encode(),
    });
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();
    built.runner.bus().publish(Envelope {
        topic: Clicked::TOPIC.to_owned(),
        sender: "ui".to_owned(),
        correlation: None,
        payload: Clicked { id: 23 }.encode(),
    });
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();

    assert!(built.supervisor.registry.call("test", "will", &[]).is_err());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_err());

    for id in [1, 10] {
        built.runner.bus().publish(Envelope {
            topic: Clicked::TOPIC.to_owned(),
            sender: "ui".to_owned(),
            correlation: None,
            payload: Clicked { id }.encode(),
        });
        built.runner.step(1.0 / 60.0);
        built.supervisor.check();
    }
    built.runner.step(1.0 / 60.0);
    built.supervisor.check();

    assert!(built.supervisor.registry.call("test", "will", &[]).is_ok());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_one", &[])
        .is_ok());
}
