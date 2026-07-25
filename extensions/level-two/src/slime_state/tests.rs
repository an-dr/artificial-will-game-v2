use super::*;
use game_messages::AttackDirection;

const SPAWNS: &[SlimeSpawn] = &[
    SlimeSpawn::new(30, 100.0, 100.0),
    SlimeSpawn::new(31, 500.0, 500.0),
];

#[test]
fn nearby_slimes_chase_at_constant_speed_and_distant_slimes_stop() {
    let field = SlimeField::new(200, SPAWNS, (172.0, 196.0));
    let velocities = field.velocities();
    assert!((velocities[0].vx - 43.2).abs() < 0.001);
    assert!((velocities[0].vy - 57.6).abs() < 0.001);
    assert_eq!((velocities[1].vx, velocities[1].vy), (0.0, 0.0));
}

#[test]
fn authoritative_updates_change_both_pursuer_and_target_positions() {
    let mut field = SlimeField::new(200, SPAWNS, (500.0, 500.0));
    field.update_transform(WILL_ENTITY_ID, 160.0, 100.0);
    field.update_transform(200, 120.0, 100.0);
    let velocity = field.velocities()[0];
    assert_eq!((velocity.vx, velocity.vy), (SLIME_SPEED, 0.0));
}

#[test]
fn malformed_updates_are_ignored_and_pause_stops_every_slime() {
    let mut field = SlimeField::new(200, SPAWNS, (172.0, 196.0));
    field.update_transform(WILL_ENTITY_ID, f32::NAN, 0.0);
    assert_ne!(field.velocities()[0].vx, 0.0);

    field.set_paused(true);
    assert!(field
        .velocities()
        .iter()
        .all(|velocity| (velocity.vx, velocity.vy) == (0.0, 0.0)));
}

#[test]
fn will_contact_accepts_either_collision_order_and_only_slimes() {
    let field = SlimeField::new(200, SPAWNS, (0.0, 0.0));
    assert!(field.is_will_contact(Collision {
        entity_id_a: WILL_ENTITY_ID,
        entity_id_b: 200,
    }));
    assert!(field.is_will_contact(Collision {
        entity_id_a: 201,
        entity_id_b: WILL_ENTITY_ID,
    }));
    assert!(!field.is_will_contact(Collision {
        entity_id_a: WILL_ENTITY_ID,
        entity_id_b: 100,
    }));
}

fn attack(sequence: u32) -> AttackRequested {
    AttackRequested {
        sequence,
        origin_x: 50.0,
        origin_y: 100.0,
        direction: AttackDirection::Right,
        reach: 80.0,
        half_width: 20.0,
    }
}

#[test]
fn slime_requires_two_unique_hits_and_rewards_only_on_defeat() {
    let mut field = SlimeField::new(200, &SPAWNS[..1], (0.0, 0.0));
    let first = field.attack(attack(1)).expect("first hit");
    assert_eq!(first.hit.entity_id, 200);
    assert!(!first.defeated);
    assert_eq!(first.reward, None);

    assert_eq!(field.attack(attack(1)), None);
    let second = field.attack(attack(2)).expect("second hit");
    assert!(second.defeated);
    assert_eq!(
        second.reward,
        Some(RewardGranted {
            experience: 1,
            coins: 0,
        })
    );
    assert_eq!(field.attack(attack(3)), None);
}

#[test]
fn defeated_slime_stops_moving_and_cannot_damage_will() {
    let mut field = SlimeField::new(200, &SPAWNS[..1], (172.0, 100.0));
    field.attack(attack(1));
    field.attack(attack(2));

    assert!(field.velocities().is_empty());
    assert!(!field.is_will_contact(Collision {
        entity_id_a: WILL_ENTITY_ID,
        entity_id_b: 200,
    }));
}
