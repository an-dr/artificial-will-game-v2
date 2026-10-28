use super::*;

#[test]
fn starts_idle_facing_down() {
    assert_eq!(PlayerState::default().animation(), "idle_down");
}

#[test]
fn movement_selects_the_dominant_facing_with_a_sheet_row_per_side() {
    let mut state = PlayerState::default();
    assert!(state.tick(0.016, -160.0, 160.0));
    assert_eq!(state.animation(), "walk_left");

    assert!(state.tick(0.016, 160.0, 0.0));
    assert_eq!(state.animation(), "walk_right");

    assert!(state.tick(0.016, 0.0, -160.0));
    assert_eq!(state.animation(), "walk_up");

    assert!(state.tick(0.016, 0.0, 0.0));
    assert_eq!(state.animation(), "idle_up");
}

#[test]
fn attack_is_edge_triggered_one_shot_and_freezes_facing() {
    let mut state = PlayerState::default();
    state.tick(0.016, -160.0, 0.0);
    assert!(state.press_attack());
    assert!(!state.press_attack());
    assert_eq!(state.animation(), "attack_left");

    assert!(!state.tick(ATTACK_DURATION_SECONDS - 0.001, 160.0, 0.0));
    assert_eq!(state.animation(), "attack_left");
    assert!(state.tick(0.001, 160.0, 0.0));
    assert_eq!(state.animation(), "walk_right");
}

#[test]
fn an_attack_press_during_attack_is_not_buffered() {
    let mut state = PlayerState::default();
    assert!(state.press_attack());
    state.release_attack();
    assert!(!state.press_attack());
    state.release_attack();

    assert!(state.tick(ATTACK_DURATION_SECONDS, 0.0, 0.0));
    assert_eq!(state.animation(), "idle_down");
}

#[test]
fn attack_direction_uses_the_frozen_cardinal_facing() {
    let mut state = PlayerState::default();
    state.tick(0.016, -160.0, 0.0);
    assert_eq!(state.attack_direction(), AttackDirection::Left);
    state.press_attack();
    state.tick(0.1, 160.0, 0.0);
    assert_eq!(state.attack_direction(), AttackDirection::Left);
}

#[test]
fn damage_state_locks_actions_and_plays_hurt_until_recovery() {
    let mut state = PlayerState::default();
    state.tick(0.016, 160.0, 0.0);
    assert!(state.start_damage());
    assert!(!state.start_damage());
    assert!(!state.press_attack());
    assert!(!state.tick(10.0, -160.0, 0.0));
    assert_eq!(state.animation(), "hurt_right");

    assert!(state.recover_from_damage());
    assert_eq!(state.animation(), "idle_right");
    assert!(!state.recover_from_damage());
}
