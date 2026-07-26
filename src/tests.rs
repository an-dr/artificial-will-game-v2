use std::sync::{Arc, Mutex};

use bones_messages::game_core::{Collision, EntityOp, EntityOpMessage};
use bones_messages::gfx::{DrawRect, DrawSprite, DrawText};
use bones_messages::input::KeyDown;
use bones_messages::{DecodeMessage, EncodeMessage, Message};
use bus::Envelope;
use game_messages::{
    AttackDirection, AttackRequested, HitConfirmed, PauseChanged, PlayerDamaged, PlayerDefeated,
    PlayerStats, RewardGranted,
};

struct SavesDir(std::path::PathBuf);

impl SavesDir {
    fn create(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "artificial-will-{label}-test-{}",
            std::process::id()
        ));
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

fn publish_message<M: Message + EncodeMessage>(
    built: &mut runner::BuiltEngine,
    sender: &str,
    message: M,
) {
    built.runner.bus().publish(Envelope {
        topic: M::TOPIC.to_owned(),
        sender: sender.to_owned(),
        correlation: None,
        payload: message.encode(),
    });
}

fn pump(built: &mut runner::BuiltEngine, frames: usize) {
    for _ in 0..frames {
        built.runner.step(1.0 / 60.0);
        built.supervisor.check();
    }
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
    let saves = SavesDir::create("menu");
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

#[test]
fn combat_rewards_hud_and_game_over_flow_across_extensions() {
    let saves = SavesDir::create("combat");
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

    let captured = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&captured);
    let spy = built
        .runner
        .bus()
        .register("combat-flow-test", move |event: &Envelope| {
            sink.lock().unwrap().push(event.clone());
        });
    for topic in ["game/*", "game-core/*", "gfx/*"] {
        spy.subscribe(topic);
    }

    pump(&mut built, 2);
    press(&mut built, "Return");
    press(&mut built, "Return");
    pump(&mut built, 10);
    assert!(captured.lock().unwrap().iter().any(|event| {
        event.sender == "menu"
            && event.topic == DrawRect::TOPIC
            && DrawRect::decode(&event.payload).is_ok_and(|rectangle| {
                rectangle.screen_space
                    && (rectangle.x, rectangle.y, rectangle.w, rectangle.h) == (0, 0, 1, 1)
                    && rectangle.color.3 == 0
            })
    }));
    captured.lock().unwrap().clear();

    press(&mut built, "Space");
    pump(&mut built, 2);
    assert!(captured.lock().unwrap().iter().any(|event| {
        event.sender == "will"
            && event.topic == AttackRequested::TOPIC
            && AttackRequested::decode(&event.payload).is_ok()
    }));

    publish_message(
        &mut built,
        "will",
        AttackRequested {
            sequence: 100,
            origin_x: 180.0,
            origin_y: 442.0,
            direction: AttackDirection::Right,
            reach: 80.0,
            half_width: 24.0,
        },
    );
    pump(&mut built, 10);
    {
        let events = captured.lock().unwrap();
        assert!(events.iter().any(|event| {
            event.sender == "level_one"
                && event.topic == HitConfirmed::TOPIC
                && HitConfirmed::decode(&event.payload)
                    .is_ok_and(|hit| hit.entity_id == 2 && hit.sequence == 100)
        }));
        assert!(events.iter().any(|event| {
            event.sender == "level_one"
                && event.topic == RewardGranted::TOPIC
                && RewardGranted::decode(&event.payload)
                    .is_ok_and(|reward| reward.experience == 0 && reward.coins == 1)
        }));
        assert!(events.iter().any(|event| {
            event.sender == "level_one"
                && event.topic == EntityOpMessage::TOPIC
                && EntityOpMessage::decode(&event.payload)
                    .is_ok_and(|message| matches!(message.0, EntityOp::Despawn { entity_id: 2 }))
        }));
        assert!(events.iter().any(|event| {
            event.sender == "will"
                && event.topic == PlayerStats::TOPIC
                && PlayerStats::decode(&event.payload).is_ok_and(|stats| stats.coins == 3)
        }));
        assert!(events.iter().any(|event| {
            event.topic == DrawText::TOPIC
                && DrawText::decode(&event.payload)
                    .is_ok_and(|text| text.screen_space && text.text == "COINS 3")
        }));
    }

    press(&mut built, "Escape");
    press(&mut built, "Down");
    press(&mut built, "Down");
    press(&mut built, "Return");
    press(&mut built, "Down");
    press(&mut built, "Return");
    pump(&mut built, 5);
    assert!(built
        .supervisor
        .registry
        .call("test", "level_two", &[])
        .is_ok());
    captured.lock().unwrap().clear();

    let attack_slime = |built: &mut runner::BuiltEngine, sequence| {
        publish_message(
            built,
            "will",
            AttackRequested {
                sequence,
                origin_x: 420.0,
                origin_y: 288.0,
                direction: AttackDirection::Right,
                reach: 80.0,
                half_width: 24.0,
            },
        );
    };
    attack_slime(&mut built, 200);
    pump(&mut built, 2);
    assert!(captured.lock().unwrap().iter().any(|event| {
        event.sender == "level_two"
            && event.topic == HitConfirmed::TOPIC
            && HitConfirmed::decode(&event.payload)
                .is_ok_and(|hit| hit.entity_id == 200 && hit.sequence == 200)
    }));
    assert!(captured.lock().unwrap().iter().any(|event| {
        event.sender == "level_two"
            && event.topic == EntityOpMessage::TOPIC
            && EntityOpMessage::decode(&event.payload).is_ok_and(|message| {
                matches!(
                    message.0,
                    EntityOp::SetSprite {
                        entity_id: 200,
                        presentation
                    } if presentation.sprite.sprite_id == 39 && presentation.sprite.frame_count == 5
                )
            })
    }));
    assert!(!captured.lock().unwrap().iter().any(|event| {
        event.sender == "will"
            && event.topic == DrawRect::TOPIC
            && DrawRect::decode(&event.payload).is_ok_and(|rectangle| !rectangle.screen_space)
    }));
    captured.lock().unwrap().clear();

    attack_slime(&mut built, 201);
    pump(&mut built, 6);
    {
        let events = captured.lock().unwrap();
        assert!(events.iter().any(|event| {
            event.sender == "level_two"
                && event.topic == HitConfirmed::TOPIC
                && HitConfirmed::decode(&event.payload)
                    .is_ok_and(|hit| hit.entity_id == 200 && hit.sequence == 201)
        }));
        assert!(events.iter().any(|event| {
            event.sender == "level_two"
                && event.topic == RewardGranted::TOPIC
                && RewardGranted::decode(&event.payload)
                    .is_ok_and(|reward| reward.experience == 1 && reward.coins == 0)
        }));
        assert!(events.iter().any(|event| {
            event.sender == "will"
                && event.topic == PlayerStats::TOPIC
                && PlayerStats::decode(&event.payload)
                    .is_ok_and(|stats| stats.experience == 1 && stats.level == 1)
        }));
        assert!(events.iter().any(|event| {
            event.sender == "level_two"
                && event.topic == EntityOpMessage::TOPIC
                && EntityOpMessage::decode(&event.payload).is_ok_and(|message| {
                    matches!(
                        message.0,
                        EntityOp::Spawn {
                            entity_id: 200,
                            sprite: Some(sprite),
                            collider_half_w: 0.0,
                            collider_half_h: 0.0,
                            ..
                        } if sprite.sprite_id == 42 && sprite.frame_count == 10
                    )
                })
        }));
    }
    captured.lock().unwrap().clear();
    publish_message(
        &mut built,
        "level_two",
        PlayerDamaged {
            amount: 1,
            source_x: 464.0,
            source_y: 600.0,
        },
    );
    pump(&mut built, 2);
    {
        let events = captured.lock().unwrap();
        assert!(events.iter().any(|event| {
            event.sender == "will"
                && event.topic == EntityOpMessage::TOPIC
                && EntityOpMessage::decode(&event.payload).is_ok_and(|message| {
                    matches!(
                        message.0,
                        EntityOp::SetVelocity {
                            entity_id: 1,
                            vx,
                            vy
                        } if vx.abs() < 0.001 && vy < -200.0
                    )
                })
        }));
        assert!(!events.iter().any(|event| {
            event.sender == "will"
                && event.topic == DrawRect::TOPIC
                && DrawRect::decode(&event.payload).is_ok_and(|rectangle| !rectangle.screen_space)
        }));
        assert!(events.iter().any(|event| {
            event.sender == "will"
                && event.topic == EntityOpMessage::TOPIC
                && EntityOpMessage::decode(&event.payload).is_ok_and(|message| {
                    matches!(
                        message.0,
                        EntityOp::SetSpriteTint {
                            entity_id: 1,
                            tint: (255, 48, 58, 255)
                        }
                    )
                })
        }));
        assert!(events.iter().any(|event| {
            event.sender == "will"
                && event.topic == PlayerStats::TOPIC
                && PlayerStats::decode(&event.payload).is_ok_and(|stats| stats.lives == 2)
        }));
    }
    pump(&mut built, 50);
    {
        let events = captured.lock().unwrap();
        assert!(events.iter().any(|event| {
            event.sender == "will"
                && event.topic == EntityOpMessage::TOPIC
                && EntityOpMessage::decode(&event.payload).is_ok_and(|message| {
                    matches!(
                        message.0,
                        EntityOp::SetSprite {
                            entity_id: 1,
                            presentation
                        } if presentation.sprite.sprite_id == 1
                            && presentation.sprite.frame_count == 5
                            && presentation.looping
                    )
                })
        }));
        assert!(events.iter().any(|event| {
            event.sender == "level_two"
                && event.topic == EntityOpMessage::TOPIC
                && EntityOpMessage::decode(&event.payload)
                    .is_ok_and(|message| matches!(message.0, EntityOp::Despawn { entity_id: 200 }))
        }));
    }
    captured.lock().unwrap().clear();
    publish_message(
        &mut built,
        "level_two",
        PlayerDamaged {
            amount: 3,
            source_x: 480.0,
            source_y: 576.0,
        },
    );
    pump(&mut built, 20);
    {
        let events = captured.lock().unwrap();
        assert!(events.iter().any(|event| {
            event.sender == "will"
                && event.topic == PlayerDefeated::TOPIC
                && PlayerDefeated::decode(&event.payload).is_ok()
        }));
        let stats = events
            .iter()
            .filter(|event| event.sender == "will" && event.topic == PlayerStats::TOPIC)
            .filter_map(|event| PlayerStats::decode(&event.payload).ok())
            .collect::<Vec<_>>();
        assert!(stats.iter().any(|stats| stats.lives == 0));
        assert_eq!(
            stats.last(),
            Some(&PlayerStats {
                lives: 0,
                experience: 1,
                level: 1,
                coins: 0,
            })
        );
        assert!(events.iter().any(|event| {
            event.sender == "menu"
                && event.topic == DrawText::TOPIC
                && DrawText::decode(&event.payload)
                    .is_ok_and(|text| text.screen_space && text.text == "GAME OVER")
        }));
        assert!(events.iter().any(|event| {
            event.sender == "menu"
                && event.topic == PauseChanged::TOPIC
                && PauseChanged::decode(&event.payload).is_ok_and(|pause| pause.paused)
        }));
        assert!(events.iter().any(|event| {
            event.sender == "menu"
                && event.topic == DrawText::TOPIC
                && DrawText::decode(&event.payload)
                    .is_ok_and(|text| text.screen_space && text.text == "Press Enter to Main Menu")
        }));
    }
    assert!(built.supervisor.registry.call("test", "will", &[]).is_ok());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_two", &[])
        .is_ok());

    captured.lock().unwrap().clear();
    press(&mut built, "Return");
    pump(&mut built, 5);
    assert!(built.supervisor.registry.call("test", "will", &[]).is_err());
    assert!(built
        .supervisor
        .registry
        .call("test", "level_two", &[])
        .is_err());
    assert!(captured.lock().unwrap().iter().any(|event| {
        event.sender == "menu"
            && event.topic == DrawText::TOPIC
            && DrawText::decode(&event.payload)
                .is_ok_and(|text| text.screen_space && text.text == "ARTIFICIAL WILL")
    }));
    assert!(captured.lock().unwrap().iter().any(|event| {
        event.sender == "menu"
            && event.topic == DrawRect::TOPIC
            && DrawRect::decode(&event.payload).is_ok_and(|rectangle| {
                rectangle.screen_space
                    && (rectangle.x, rectangle.y, rectangle.w, rectangle.h) == (0, 0, 800, 600)
                    && rectangle.color.3 == 255
            })
    }));
}

#[test]
fn slime_contact_is_harmless_and_a_timed_attack_deals_damage() {
    let saves = SavesDir::create("slime-attack");
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

    let captured = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&captured);
    let spy = built
        .runner
        .bus()
        .register("slime-attack-test", move |event: &Envelope| {
            sink.lock().unwrap().push(event.clone());
        });
    for topic in ["game/*", "game-core/*"] {
        spy.subscribe(topic);
    }

    pump(&mut built, 2);
    press(&mut built, "Return");
    press(&mut built, "Down");
    press(&mut built, "Return");
    pump(&mut built, 5);
    assert!(built
        .supervisor
        .registry
        .call("test", "level_two", &[])
        .is_ok());

    captured.lock().unwrap().clear();
    publish_message(
        &mut built,
        "game-core",
        Collision {
            entity_id_a: 1,
            entity_id_b: 201,
        },
    );
    pump(&mut built, 2);
    assert!(!captured
        .lock()
        .unwrap()
        .iter()
        .any(|event| { event.sender == "level_two" && event.topic == PlayerDamaged::TOPIC }));

    captured.lock().unwrap().clear();
    pump(&mut built, 100);
    let events = captured.lock().unwrap();
    assert!(events.iter().any(|event| {
        event.sender == "level_two"
            && event.topic == EntityOpMessage::TOPIC
            && EntityOpMessage::decode(&event.payload).is_ok_and(|message| {
                matches!(
                    message.0,
                    EntityOp::SetSprite { presentation, .. }
                        if (36..=38).contains(&presentation.sprite.sprite_id)
                )
            })
    }));
    assert_eq!(
        events
            .iter()
            .filter(|event| { event.sender == "level_two" && event.topic == PlayerDamaged::TOPIC })
            .filter_map(|event| PlayerDamaged::decode(&event.payload).ok())
            .count(),
        1
    );
    assert!(events.iter().any(|event| {
        event.sender == "will"
            && event.topic == PlayerStats::TOPIC
            && PlayerStats::decode(&event.payload).is_ok_and(|stats| stats.lives == 2)
    }));
}
