use super::*;

#[test]
fn starts_idle_facing_down_with_the_original_grid_and_scale() {
    let state = PlayerState::default();
    let presentation = state.presentation();
    assert_eq!(presentation.sprite.sprite_id, IDLE_DOWN_SPRITE_ID);
    assert_eq!(presentation.sprite.frame_count, 5);
    assert_eq!(presentation.sprite.frame_duration, 0.125);
    assert_eq!(presentation.frames_per_row, 4);
    assert_eq!((presentation.draw_w, presentation.draw_h), (128, 128));
    assert!(presentation.looping);
    assert!(presentation.advance_while_stopped);
    assert!(!presentation.flip_h);
}

#[test]
fn movement_selects_the_dominant_facing_and_mirrors_only_right() {
    let mut state = PlayerState::default();
    assert!(state.tick(0.016, -160.0, 160.0));
    let left = state.presentation();
    assert_eq!(left.sprite.sprite_id, WALK_SIDE_SPRITE_ID);
    assert!(!left.flip_h);

    assert!(state.tick(0.016, 160.0, 0.0));
    let right = state.presentation();
    assert_eq!(right.sprite.sprite_id, WALK_SIDE_SPRITE_ID);
    assert!(right.flip_h);

    assert!(state.tick(0.016, 0.0, -160.0));
    assert_eq!(state.presentation().sprite.sprite_id, WALK_UP_SPRITE_ID);
}

#[test]
fn attack_is_edge_triggered_one_shot_and_freezes_facing() {
    let mut state = PlayerState::default();
    state.tick(0.016, -160.0, 0.0);
    assert!(state.press_attack());
    assert!(!state.press_attack());

    let attack = state.presentation();
    assert_eq!(attack.sprite.sprite_id, ATTACK_SIDE_SPRITE_ID);
    assert_eq!(attack.sprite.frame_count, 2);
    assert!(!attack.looping);
    assert!(!attack.flip_h);

    assert!(!state.tick(0.249, 160.0, 0.0));
    assert_eq!(state.presentation().sprite.sprite_id, ATTACK_SIDE_SPRITE_ID);
    assert!(state.tick(0.001, 160.0, 0.0));
    let resumed = state.presentation();
    assert_eq!(resumed.sprite.sprite_id, WALK_SIDE_SPRITE_ID);
    assert!(resumed.flip_h);
}

#[test]
fn an_attack_press_during_attack_is_not_buffered() {
    let mut state = PlayerState::default();
    assert!(state.press_attack());
    state.release_attack();
    assert!(!state.press_attack());
    state.release_attack();

    assert!(state.tick(0.25, 0.0, 0.0));
    assert_eq!(state.presentation().sprite.sprite_id, IDLE_DOWN_SPRITE_ID);
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
fn damage_state_locks_actions_and_uses_a_frozen_idle_frame_until_recovery() {
    let mut state = PlayerState::default();
    state.tick(0.016, 160.0, 0.0);
    assert!(state.start_damage());
    assert!(!state.start_damage());
    assert!(!state.press_attack());
    assert!(!state.tick(10.0, -160.0, 0.0));

    let damaged = state.presentation();
    assert_eq!(damaged.sprite.sprite_id, IDLE_SIDE_SPRITE_ID);
    assert_eq!(damaged.sprite.frame_count, 1);
    assert!(!damaged.looping);
    assert!(damaged.flip_h);

    assert!(state.recover_from_damage());
    assert_eq!(state.presentation().sprite.sprite_id, IDLE_SIDE_SPRITE_ID);
    assert!(!state.recover_from_damage());
}
