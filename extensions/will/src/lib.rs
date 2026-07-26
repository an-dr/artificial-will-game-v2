#![cfg_attr(test, allow(dead_code))]

wit_bindgen::generate!({
    path: "../../vendor/bones/wit",
    world: "extension",
});

mod character;
mod combat_state;
mod damage_reaction;
mod held_keys;
mod hud;
mod impact;
mod player_mode;
mod player_state;

use std::cell::{Cell, RefCell};

use bones::core::host_api::{log, publish, subscribe, Level};
use bones_messages::game_core::{EntityOp, EntityOpMessage, EntityTransform};
use bones_messages::input::{KeyDown, KeyUp};
use bones_messages::{DecodeMessage, EncodeMessage, Message};
use combat_state::{CombatState, DamageOutcome};
use damage_reaction::DamageReaction;
use game_messages::{
    HitConfirmed, PauseChanged, PlayerDamaged, PlayerDefeated, RewardGranted, SessionReset,
    WILL_ENTITY_ID,
};
use held_keys::HeldKeys;
use impact::ImpactFeedback;
use player_state::PlayerState;

thread_local! {
    static HELD_KEYS: RefCell<HeldKeys> = RefCell::new(HeldKeys::default());
    static PLAYER_STATE: RefCell<PlayerState> = RefCell::new(PlayerState::default());
    static COMBAT_STATE: RefCell<CombatState> = RefCell::new(CombatState::default());
    static DAMAGE_REACTION: RefCell<DamageReaction> = RefCell::new(DamageReaction::default());
    static IMPACT_FEEDBACK: RefCell<ImpactFeedback> = RefCell::new(ImpactFeedback::default());
    static PAUSED: Cell<bool> = const { Cell::new(false) };
}

fn publish_entity_op(op: EntityOp) {
    publish(EntityOpMessage::TOPIC, &EntityOpMessage(op).encode());
}

fn publish_message<M: Message + EncodeMessage>(message: M) {
    publish(M::TOPIC, &message.encode());
}

fn set_key_held(key: &str, is_down: bool) {
    HELD_KEYS.with(|held| held.borrow_mut().set(key, is_down));
}

fn publish_player_presentation() {
    PLAYER_STATE.with(|state| character::publish_presentation(&state.borrow()));
}

fn publish_player_stats() {
    COMBAT_STATE.with(|state| publish_message(state.borrow().stats()));
}

fn publish_game_ui() {
    let (stats, position) = COMBAT_STATE.with(|state| {
        let state = state.borrow();
        (state.stats(), state.position())
    });
    for command in hud::commands(stats) {
        command.publish_with(publish);
    }
    DAMAGE_REACTION.with(|reaction| {
        if let Some(rectangles) = reaction.borrow().rectangles(position) {
            for rectangle in rectangles {
                publish_message(rectangle);
            }
        }
    });
    IMPACT_FEEDBACK.with(|feedback| {
        if let Some(rectangles) = feedback.borrow().rectangles() {
            for rectangle in rectangles {
                publish_message(rectangle);
            }
        }
    });
}

fn reset_state() {
    HELD_KEYS.with(|held| *held.borrow_mut() = HeldKeys::default());
    PLAYER_STATE.with(|state| *state.borrow_mut() = PlayerState::default());
    COMBAT_STATE.with(|state| *state.borrow_mut() = CombatState::default());
    DAMAGE_REACTION.with(|reaction| *reaction.borrow_mut() = DamageReaction::default());
    IMPACT_FEEDBACK.with(|feedback| *feedback.borrow_mut() = ImpactFeedback::default());
    PAUSED.with(|paused| paused.set(false));
}

fn set_paused(paused: bool) {
    PAUSED.with(|value| value.set(paused));
    if paused {
        HELD_KEYS.with(|held| *held.borrow_mut() = HeldKeys::default());
    }
}

fn handle_key_down(key: &str) {
    if DAMAGE_REACTION.with(|reaction| reaction.borrow().active()) {
        return;
    }
    if key == "Space" {
        let direction = PLAYER_STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.press_attack().then(|| state.attack_direction())
        });
        if let Some(direction) = direction {
            publish_player_presentation();
            let attack = COMBAT_STATE.with(|state| state.borrow_mut().attack(direction));
            publish_message(attack);
        }
    } else {
        set_key_held(key, true);
    }
}

fn handle_key_up(key: &str) {
    if DAMAGE_REACTION.with(|reaction| reaction.borrow().active()) {
        return;
    }
    if key == "Space" {
        PLAYER_STATE.with(|state| state.borrow_mut().release_attack());
    } else {
        set_key_held(key, false);
    }
}

fn is_active_level(sender: &str) -> bool {
    matches!(sender, "level_one" | "level_two")
}

fn start_damage_reaction(message: PlayerDamaged) {
    HELD_KEYS.with(|held| *held.borrow_mut() = HeldKeys::default());
    let facing = PLAYER_STATE.with(|state| {
        let mut state = state.borrow_mut();
        let facing = state.attack_direction();
        state.start_damage();
        facing
    });
    let position = COMBAT_STATE.with(|state| state.borrow().position());
    DAMAGE_REACTION.with(|reaction| {
        reaction
            .borrow_mut()
            .start(position, message.source(), facing)
    });
    publish_player_presentation();
}

struct Component;

impl Guest for Component {
    fn shutdown() {
        publish_entity_op(EntityOp::Despawn {
            entity_id: WILL_ENTITY_ID,
        });
        reset_state();
    }

    fn init() {
        reset_state();
        subscribe(KeyDown::TOPIC);
        subscribe(KeyUp::TOPIC);
        subscribe(PauseChanged::TOPIC);
        subscribe(SessionReset::TOPIC);
        subscribe(EntityTransform::TOPIC);
        subscribe(PlayerDamaged::TOPIC);
        subscribe(RewardGranted::TOPIC);
        subscribe(HitConfirmed::TOPIC);
        subscribe("core/tick");

        character::load_sprites();
        PLAYER_STATE.with(|state| character::spawn(&state.borrow()));
        publish_player_stats();

        log(
            Level::Info,
            "will: character ready; move with WASD or arrow keys",
        );
    }

    fn on_tick(dt: f32) {
        if PAUSED.with(Cell::get) {
            publish_game_ui();
            return;
        }
        let reacting = DAMAGE_REACTION.with(|reaction| reaction.borrow().active());
        let (vx, vy) = if reacting {
            DAMAGE_REACTION.with(|reaction| reaction.borrow().velocity_for_tick(dt))
        } else {
            HELD_KEYS.with(|held| held.borrow().velocity())
        };
        publish_entity_op(EntityOp::SetVelocity {
            entity_id: WILL_ENTITY_ID,
            vx,
            vy,
        });
        let presentation_changed = if reacting {
            let recovered = DAMAGE_REACTION.with(|reaction| reaction.borrow_mut().tick(dt));
            recovered && PLAYER_STATE.with(|state| state.borrow_mut().recover_from_damage())
        } else {
            PLAYER_STATE.with(|state| state.borrow_mut().tick(dt, vx, vy))
        };
        if presentation_changed {
            publish_player_presentation();
        }
        COMBAT_STATE.with(|state| state.borrow_mut().tick(dt));
        IMPACT_FEEDBACK.with(|feedback| feedback.borrow_mut().tick(dt));
        publish_game_ui();
    }

    fn on_message(topic: String, sender: String, payload: Vec<u8>) -> Option<Vec<u8>> {
        match topic.as_str() {
            KeyDown::TOPIC => {
                if !PAUSED.with(Cell::get) {
                    if let Ok(message) = KeyDown::decode(&payload) {
                        handle_key_down(message.key);
                    }
                }
            }
            KeyUp::TOPIC => {
                if !PAUSED.with(Cell::get) {
                    if let Ok(message) = KeyUp::decode(&payload) {
                        handle_key_up(message.key);
                    }
                }
            }
            PauseChanged::TOPIC => {
                if let Ok(message) = PauseChanged::decode(&payload) {
                    set_paused(message.paused);
                }
            }
            SessionReset::TOPIC if SessionReset::decode(&payload).is_ok() => {
                reset_state();
                publish_player_stats();
            }
            EntityTransform::TOPIC if sender == "game-core" => {
                if let Ok(message) = EntityTransform::decode(&payload) {
                    if message.entity_id == WILL_ENTITY_ID {
                        COMBAT_STATE
                            .with(|state| state.borrow_mut().update_position(message.x, message.y));
                    }
                }
            }
            PlayerDamaged::TOPIC if is_active_level(&sender) => {
                if let Ok(message) = PlayerDamaged::decode(&payload) {
                    let outcome =
                        COMBAT_STATE.with(|state| state.borrow_mut().damage(message.amount));
                    if outcome != DamageOutcome::Ignored {
                        start_damage_reaction(message);
                        publish_player_stats();
                    }
                    if outcome == DamageOutcome::Defeated {
                        publish_message(PlayerDefeated);
                    }
                }
            }
            RewardGranted::TOPIC if is_active_level(&sender) => {
                if let Ok(message) = RewardGranted::decode(&payload) {
                    COMBAT_STATE.with(|state| state.borrow_mut().grant(message));
                    publish_player_stats();
                }
            }
            HitConfirmed::TOPIC if is_active_level(&sender) => {
                if let Ok(message) = HitConfirmed::decode(&payload) {
                    IMPACT_FEEDBACK.with(|feedback| feedback.borrow_mut().confirm(message));
                }
            }
            _ => {}
        }
        None
    }
}

#[cfg(not(test))]
export!(Component);

#[cfg(test)]
mod tests;
