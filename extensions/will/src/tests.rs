use super::*;

#[test]
fn pausing_clears_movement_and_reset_restores_active_defaults() {
    reset_state();
    HELD_KEYS.with(|held| held.borrow_mut().set("W", true));
    assert_ne!(HELD_KEYS.with(|held| held.borrow().velocity()), (0.0, 0.0));

    set_paused(true);

    assert!(PAUSED.with(Cell::get));
    assert_eq!(HELD_KEYS.with(|held| held.borrow().velocity()), (0.0, 0.0));

    reset_state();
    assert!(!PAUSED.with(Cell::get));
}
