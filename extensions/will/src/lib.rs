wit_bindgen::generate!({
    path: "../../vendor/bones/wit",
    world: "extension",
});

mod character;
mod held_keys;
mod player_mode;
mod player_state;

use std::cell::{Cell, RefCell};

use bones::core::host_api::{log, publish, subscribe, Level};
use bones_messages::game_core::{EntityOp, EntityOpMessage};
use bones_messages::input::{KeyDown, KeyUp};
use bones_messages::{DecodeMessage, EncodeMessage, Message};
use held_keys::HeldKeys;
use player_state::PlayerState;

const WILL_ENTITY_ID: u32 = 1;

thread_local! {
    static HELD_KEYS: RefCell<HeldKeys> = RefCell::new(HeldKeys::default());
    static PLAYER_STATE: RefCell<PlayerState> = RefCell::new(PlayerState::default());
    static PAUSED: Cell<bool> = const { Cell::new(false) };
}

fn publish_entity_op(op: EntityOp) {
    publish(EntityOpMessage::TOPIC, &EntityOpMessage(op).encode());
}

fn set_key_held(key: &str, is_down: bool) {
    HELD_KEYS.with(|held| held.borrow_mut().set(key, is_down));
}

fn publish_player_presentation() {
    PLAYER_STATE.with(|state| character::publish_presentation(&state.borrow()));
}

fn reset_state() {
    HELD_KEYS.with(|held| *held.borrow_mut() = HeldKeys::default());
    PLAYER_STATE.with(|state| *state.borrow_mut() = PlayerState::default());
    PAUSED.with(|paused| paused.set(false));
}

fn set_paused(paused: bool) {
    PAUSED.with(|value| value.set(paused));
    if paused {
        HELD_KEYS.with(|held| *held.borrow_mut() = HeldKeys::default());
    }
}

fn handle_key_down(key: &str) {
    if key == "Space" {
        let changed = PLAYER_STATE.with(|state| state.borrow_mut().press_attack());
        if changed {
            publish_player_presentation();
        }
    } else {
        set_key_held(key, true);
    }
}

fn handle_key_up(key: &str) {
    if key == "Space" {
        PLAYER_STATE.with(|state| state.borrow_mut().release_attack());
    } else {
        set_key_held(key, false);
    }
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
        subscribe(EntityOpMessage::TOPIC);
        subscribe("core/tick");

        character::load_sprites();
        PLAYER_STATE.with(|state| character::spawn(&state.borrow()));

        log(
            Level::Info,
            "will: character ready; move with WASD or arrow keys",
        );
    }

    fn on_tick(dt: f32) {
        if PAUSED.with(Cell::get) {
            return;
        }
        let (vx, vy) = HELD_KEYS.with(|held| held.borrow().velocity());
        publish_entity_op(EntityOp::SetVelocity {
            entity_id: WILL_ENTITY_ID,
            vx,
            vy,
        });
        let presentation_changed = PLAYER_STATE.with(|state| state.borrow_mut().tick(dt, vx, vy));
        if presentation_changed {
            publish_player_presentation();
        }
    }

    fn on_message(topic: String, _sender: String, payload: Vec<u8>) -> Option<Vec<u8>> {
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
            EntityOpMessage::TOPIC => {
                if let Ok(EntityOpMessage(op)) = EntityOpMessage::decode(&payload) {
                    match op {
                        EntityOp::SetPaused { paused } => set_paused(paused),
                        EntityOp::Reset => reset_state(),
                        _ => {}
                    }
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
