use game_messages::AttackDirection;

use super::*;

fn attack(sequence: u32) -> AttackRequested {
    AttackRequested {
        sequence,
        origin_x: 100.0,
        origin_y: 100.0,
        direction: AttackDirection::Right,
        reach: 80.0,
        half_width: 20.0,
    }
}

#[test]
fn one_swing_destroys_only_the_nearest_box_and_yields_its_coins() {
    let mut field = BoxField::new(
        2,
        &[
            BoxSpawn {
                x: 150.0,
                y: 100.0,
                coins: 2,
            },
            BoxSpawn {
                x: 130.0,
                y: 100.0,
                coins: 4,
            },
        ],
    );

    let destroyed = field.attack(attack(1)).expect("nearest box hit");
    assert_eq!(destroyed.hit.entity_id, 3);
    assert_eq!(destroyed.reward.experience, 0);
    assert_eq!(destroyed.reward.coins, 4);

    let next = field.attack(attack(2)).expect("remaining box hit");
    assert_eq!(next.hit.entity_id, 2);
}

#[test]
fn repeated_attack_sequence_cannot_destroy_a_second_box() {
    let mut field = BoxField::new(
        2,
        &[
            BoxSpawn {
                x: 120.0,
                y: 100.0,
                coins: 1,
            },
            BoxSpawn {
                x: 140.0,
                y: 100.0,
                coins: 1,
            },
        ],
    );
    assert!(field.attack(attack(7)).is_some());
    assert_eq!(field.attack(attack(7)), None);
    assert_eq!(
        field.attack(attack(8)).expect("second swing").hit.entity_id,
        3
    );
}

#[test]
fn authoritative_transform_updates_drive_target_selection() {
    let mut field = BoxField::new(
        2,
        &[BoxSpawn {
            x: 300.0,
            y: 300.0,
            coins: 1,
        }],
    );
    field.update_transform(2, 130.0, 100.0);
    assert_eq!(field.attack(attack(1)).expect("moved box").hit.entity_id, 2);
}

#[test]
fn malformed_or_unknown_transform_updates_are_ignored() {
    let mut field = BoxField::new(
        2,
        &[BoxSpawn {
            x: 130.0,
            y: 100.0,
            coins: 1,
        }],
    );
    field.update_transform(2, f32::NAN, 0.0);
    field.update_transform(99, 110.0, 100.0);
    assert_eq!((field.boxes[0].x, field.boxes[0].y), (130.0, 100.0));
}
