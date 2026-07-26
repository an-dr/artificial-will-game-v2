use super::*;

fn attack(direction: AttackDirection) -> AttackRequested {
    AttackRequested {
        sequence: 9,
        origin_x: 100.0,
        origin_y: 100.0,
        direction,
        reach: 64.0,
        half_width: 18.0,
    }
}

fn target(entity_id: u32, x: f32, y: f32) -> AttackTarget {
    AttackTarget {
        entity_id,
        x,
        y,
        half_w: 10.0,
        half_h: 10.0,
    }
}

#[test]
fn every_combat_message_round_trips() {
    let attack = attack(AttackDirection::Left);
    assert_eq!(AttackRequested::decode(&attack.encode()), Ok(attack));

    let damage = PlayerDamaged {
        amount: 1,
        source_x: 12.0,
        source_y: 13.0,
    };
    assert_eq!(PlayerDamaged::decode(&damage.encode()), Ok(damage));
    assert_eq!(damage.source(), Some((12.0, 13.0)));

    let reward = RewardGranted {
        experience: 2,
        coins: 3,
    };
    assert_eq!(RewardGranted::decode(&reward.encode()), Ok(reward));

    let hit = HitConfirmed {
        sequence: 9,
        entity_id: 42,
        x: 12.0,
        y: 13.0,
    };
    assert_eq!(HitConfirmed::decode(&hit.encode()), Ok(hit));

    assert_eq!(PlayerDefeated::decode(&[]), Ok(PlayerDefeated));

    let stats = PlayerStats {
        lives: 3,
        experience: 4,
        level: 2,
        coins: 5,
    };
    assert_eq!(PlayerStats::decode(&stats.encode()), Ok(stats));
}

#[test]
fn damage_rejects_non_finite_source_coordinates() {
    let damage = PlayerDamaged {
        amount: 1,
        source_x: f32::NAN,
        source_y: 13.0,
    };
    assert_eq!(damage.source(), None);
}

#[test]
fn attack_decode_rejects_unknown_directions_and_trailing_bytes() {
    let mut invalid_direction = attack(AttackDirection::Up).encode();
    invalid_direction[12] = 99;
    assert_eq!(
        AttackRequested::decode(&invalid_direction),
        Err(DecodeError::InvalidTag {
            message: "attack direction",
            tag: 99
        })
    );

    let mut trailing = attack(AttackDirection::Up).encode();
    trailing.push(0);
    assert_eq!(
        AttackRequested::decode(&trailing),
        Err(DecodeError::TrailingBytes)
    );
}

#[test]
fn targeting_selects_the_nearest_target_in_the_forward_lane() {
    let selected = select_attack_target(
        attack(AttackDirection::Right),
        [
            target(1, 80.0, 100.0),
            target(2, 150.0, 140.0),
            target(3, 155.0, 105.0),
            target(4, 125.0, 100.0),
        ],
    );
    assert_eq!(selected, Some(target(4, 125.0, 100.0)));
}

#[test]
fn targeting_accounts_for_target_extents_and_breaks_ties_by_id() {
    let edge = AttackTarget {
        entity_id: 8,
        x: 174.0,
        y: 128.0,
        half_w: 12.0,
        half_h: 10.0,
    };
    assert_eq!(
        select_attack_target(attack(AttackDirection::Right), [edge]),
        Some(edge)
    );

    let a = target(5, 130.0, 90.0);
    let b = target(3, 130.0, 110.0);
    assert_eq!(
        select_attack_target(attack(AttackDirection::Right), [a, b]),
        Some(b)
    );
}

#[test]
fn targeting_rejects_invalid_attack_dimensions() {
    let mut invalid = attack(AttackDirection::Down);
    invalid.reach = -1.0;
    assert_eq!(
        select_attack_target(invalid, [target(1, 100.0, 110.0)]),
        None
    );

    invalid.reach = f32::NAN;
    assert_eq!(
        select_attack_target(invalid, [target(1, 100.0, 110.0)]),
        None
    );

    invalid = attack(AttackDirection::Down);
    invalid.origin_x = f32::INFINITY;
    assert_eq!(
        select_attack_target(invalid, [target(1, 100.0, 110.0)]),
        None
    );
}

#[test]
fn targeting_ignores_targets_with_invalid_geometry() {
    let invalid = [
        AttackTarget {
            x: f32::NAN,
            ..target(1, 100.0, 110.0)
        },
        AttackTarget {
            half_w: -1.0,
            ..target(2, 100.0, 110.0)
        },
        AttackTarget {
            half_h: f32::INFINITY,
            ..target(3, 100.0, 110.0)
        },
    ];
    assert_eq!(
        select_attack_target(attack(AttackDirection::Down), invalid),
        None
    );
}
