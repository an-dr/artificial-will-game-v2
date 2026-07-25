use super::*;

#[test]
fn wasd_and_arrow_aliases_drive_at_the_original_speed() {
    for (key, expected) in [
        ("W", (0.0, -160.0)),
        ("Up", (0.0, -160.0)),
        ("S", (0.0, 160.0)),
        ("Down", (0.0, 160.0)),
        ("A", (-160.0, 0.0)),
        ("Left", (-160.0, 0.0)),
        ("D", (160.0, 0.0)),
        ("Right", (160.0, 0.0)),
    ] {
        let mut held = HeldKeys::default();
        held.set(key, true);
        assert_eq!(held.velocity(), expected);
    }
}

#[test]
fn opposing_keys_cancel_and_releasing_one_restores_the_other() {
    let mut held = HeldKeys::default();
    held.set("A", true);
    held.set("Right", true);
    assert_eq!(held.velocity(), (0.0, 0.0));

    held.set("Right", false);
    assert_eq!(held.velocity(), (-160.0, 0.0));
}

#[test]
fn diagonal_movement_is_not_normalized() {
    let mut held = HeldKeys::default();
    held.set("W", true);
    held.set("D", true);
    assert_eq!(held.velocity(), (160.0, -160.0));
}

#[test]
fn unrelated_keys_do_not_change_movement() {
    let mut held = HeldKeys::default();
    held.set("Space", true);
    assert_eq!(held.velocity(), (0.0, 0.0));
}
