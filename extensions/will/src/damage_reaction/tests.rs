use super::*;

#[test]
fn knockback_points_away_from_the_damage_source_then_stops() {
    let mut reaction = DamageReaction::default();
    reaction.start((100.0, 100.0), Some((80.0, 100.0)), AttackDirection::Up);
    assert_eq!(reaction.velocity_for_tick(0.016), (KNOCKBACK_SPEED, 0.0));

    assert!(!reaction.tick(KNOCKBACK_SECONDS));
    assert_eq!(reaction.velocity_for_tick(0.016), (0.0, 0.0));
    assert!(reaction.active());
}

#[test]
fn coincident_or_invalid_sources_fall_back_opposite_the_facing() {
    let mut reaction = DamageReaction::default();
    reaction.start(
        (100.0, 100.0),
        Some((f32::NAN, 100.0)),
        AttackDirection::Down,
    );
    assert_eq!(reaction.velocity_for_tick(0.016), (0.0, -KNOCKBACK_SPEED));
}

#[test]
fn sprite_tint_blinks_red_and_reaction_completes_once() {
    let mut reaction = DamageReaction::default();
    reaction.start((120.0, 80.0), None, AttackDirection::Right);
    assert_eq!(reaction.tint(), DAMAGE_TINT);
    reaction.tick(FLASH_HALF_PERIOD * 1.1);
    assert_eq!(reaction.tint(), NORMAL_TINT);
    reaction.tick(FLASH_HALF_PERIOD);
    assert_eq!(reaction.tint(), DAMAGE_TINT);

    assert!(reaction.tick(DAMAGE_REACTION_SECONDS));
    assert!(!reaction.active());
    assert_eq!(reaction.tint(), NORMAL_TINT);
    assert!(!reaction.tick(1.0));
}

#[test]
fn starting_again_replaces_the_previous_direction_and_duration() {
    let mut reaction = DamageReaction::default();
    reaction.start((100.0, 100.0), Some((80.0, 100.0)), AttackDirection::Up);
    reaction.tick(0.1);
    reaction.start((100.0, 100.0), Some((100.0, 120.0)), AttackDirection::Left);
    assert_eq!(reaction.velocity_for_tick(0.016), (0.0, -KNOCKBACK_SPEED));
    assert!(!reaction.tick(DAMAGE_REACTION_SECONDS * 0.5));
}

#[test]
fn long_frames_do_not_apply_more_than_the_knockback_distance() {
    let mut reaction = DamageReaction::default();
    reaction.start((100.0, 100.0), Some((80.0, 100.0)), AttackDirection::Up);
    let (vx, vy) = reaction.velocity_for_tick(0.5);
    assert_eq!(vy, 0.0);
    assert!((vx * 0.5 - KNOCKBACK_SPEED * KNOCKBACK_SECONDS).abs() < 0.001);
}
