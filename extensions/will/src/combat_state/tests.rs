use super::*;

#[test]
fn starts_with_three_lives_and_session_scoped_progress() {
    assert_eq!(
        CombatState::default().stats(),
        PlayerStats {
            lives: 3,
            experience: 0,
            level: 1,
            coins: 0,
        }
    );
}

#[test]
fn attacks_use_the_latest_authoritative_position_and_monotonic_sequence() {
    let mut state = CombatState::default();
    state.update_position(12.0, 34.0);
    let first = state.attack(AttackDirection::Left);
    let second = state.attack(AttackDirection::Up);

    assert_eq!(
        (first.sequence, first.origin_x, first.origin_y),
        (1, 12.0, 34.0)
    );
    assert_eq!(first.direction, AttackDirection::Left);
    assert_eq!(first.reach, ATTACK_REACH);
    assert_eq!(first.half_width, ATTACK_HALF_WIDTH);
    assert_eq!(second.sequence, 2);
    assert_eq!(second.direction, AttackDirection::Up);
}

#[test]
fn invalid_transform_snapshots_do_not_poison_attack_origins() {
    let mut state = CombatState::default();
    state.update_position(12.0, 34.0);
    state.update_position(f32::NAN, f32::INFINITY);
    let attack = state.attack(AttackDirection::Down);
    assert_eq!((attack.origin_x, attack.origin_y), (12.0, 34.0));
}

#[test]
fn exposes_the_latest_valid_position_for_world_space_feedback() {
    let mut state = CombatState::default();
    state.update_position(12.0, 34.0);
    assert_eq!(state.position(), (12.0, 34.0));
}

#[test]
fn damage_has_invulnerability_and_zero_lives_defeats_once() {
    let mut state = CombatState::default();
    assert_eq!(state.damage(1), DamageOutcome::Applied);
    assert_eq!(state.stats().lives, 2);
    assert_eq!(state.damage(1), DamageOutcome::Ignored);

    state.tick(DAMAGE_INVULNERABILITY_SECONDS);
    assert_eq!(state.damage(1), DamageOutcome::Applied);
    state.tick(DAMAGE_INVULNERABILITY_SECONDS);
    assert_eq!(state.damage(1), DamageOutcome::Defeated);
    assert_eq!(state.stats().lives, 0);
    assert_eq!(state.damage(1), DamageOutcome::Ignored);
}

#[test]
fn rewards_grow_coins_and_derive_levels_from_experience() {
    let mut state = CombatState::default();
    state.grant(RewardGranted {
        experience: 2,
        coins: 4,
    });
    assert_eq!(state.stats().level, 1);
    state.grant(RewardGranted {
        experience: 1,
        coins: 2,
    });
    assert_eq!(
        state.stats(),
        PlayerStats {
            lives: 3,
            experience: 3,
            level: 2,
            coins: 6,
        }
    );
}

#[test]
fn rewards_saturate_and_are_ignored_after_defeat() {
    let mut state = CombatState::default();
    state.grant(RewardGranted {
        experience: u32::MAX,
        coins: u32::MAX,
    });
    state.grant(RewardGranted {
        experience: 1,
        coins: 1,
    });
    assert_eq!(state.stats().experience, u32::MAX);
    assert_eq!(state.stats().coins, u32::MAX);

    assert_eq!(state.damage(3), DamageOutcome::Defeated);
    state.grant(RewardGranted {
        experience: 1,
        coins: 1,
    });
    assert_eq!(state.stats().experience, u32::MAX);
    assert_eq!(state.stats().coins, u32::MAX);
}
