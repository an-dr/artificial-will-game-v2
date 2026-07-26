use super::*;
use game_messages::AttackDirection;

const SPAWNS: &[SlimeSpawn] = &[
    SlimeSpawn::new(0, 100.0, 100.0),
    SlimeSpawn::new(1, 500.0, 500.0),
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
    field.update_transform(WILL_ENTITY_ID, 200.0, 100.0);
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
    assert!(field.tick(10.0).damages.is_empty());
}

#[test]
fn hurt_and_contact_reactions_stop_motion_then_recover() {
    let mut field = SlimeField::new(200, &SPAWNS[..1], (172.0, 100.0));
    let hit = field.attack(attack(1)).unwrap();
    assert_eq!(hit.visual.animation, SlimeAnimation::Hurt);
    assert_eq!(
        (field.velocities()[0].vx, field.velocities()[0].vy),
        (0.0, 0.0)
    );

    let during = field.tick(SLIME_HURT_DURATION * 0.5);
    assert!(during.visuals.is_empty());
    let recovered = field.tick(SLIME_HURT_DURATION);
    assert_eq!(recovered.visuals[0].animation, SlimeAnimation::Walk);
    assert_eq!(recovered.velocities[0].vx, SLIME_SPEED);
}

#[test]
fn death_visual_is_removed_only_after_its_animation_finishes() {
    let mut field = SlimeField::new(200, &SPAWNS[..1], (172.0, 100.0));
    field.attack(attack(1));
    let death = field.attack(attack(2)).unwrap();
    assert_eq!(death.visual.animation, SlimeAnimation::Death);

    assert!(field.tick(SLIME_DEATH_DURATION * 0.5).despawns.is_empty());
    assert_eq!(
        field.tick(SLIME_DEATH_DURATION).despawns,
        vec![death.hit.entity_id]
    );
    assert!(field.tick(1.0).despawns.is_empty());
}

#[test]
fn pause_freezes_reaction_timers() {
    let mut field = SlimeField::new(200, &SPAWNS[..1], (172.0, 100.0));
    field.attack(attack(1));
    field.set_paused(true);
    field.tick(10.0);
    field.set_paused(false);

    assert!(field.tick(SLIME_HURT_DURATION * 0.5).visuals.is_empty());
    assert_eq!(
        field.tick(SLIME_HURT_DURATION).visuals[0].animation,
        SlimeAnimation::Walk
    );
}

#[test]
fn close_range_starts_a_telegraphed_attack_without_contact_damage() {
    let mut field = SlimeField::new(200, &SPAWNS[..1], (100.0, 100.0));
    let started = field.tick(0.016);
    assert!(started.damages.is_empty());
    assert_eq!(started.visuals[0].animation, SlimeAnimation::Attack);
    assert_eq!(
        (started.velocities[0].vx, started.velocities[0].vy),
        (0.0, 0.0)
    );

    assert!(field.tick(0.40).damages.is_empty());
    let impact = field.tick(0.05);
    assert_eq!(
        impact.damages,
        vec![SlimeDamage {
            amount: 1,
            source_x: 100.0,
            source_y: 100.0,
        }]
    );
    assert!(field.tick(0.10).damages.is_empty());
}

#[test]
fn leaving_attack_range_during_windup_avoids_the_strike() {
    let mut field = SlimeField::new(200, &SPAWNS[..1], (150.0, 100.0));
    field.tick(0.0);
    field.update_transform(WILL_ENTITY_ID, 300.0, 100.0);
    assert!(field.tick(0.50).damages.is_empty());
}

#[test]
fn completed_attack_has_a_cooldown_before_another_windup() {
    let mut field = SlimeField::new(200, &SPAWNS[..1], (150.0, 100.0));
    field.tick(0.0);
    field.tick(SLIME_ATTACK_DURATIONS[0]);

    let cooling_down = field.tick(SLIME_ATTACK_COOLDOWN * 0.9);
    assert!(cooling_down.damages.is_empty());
    assert!(cooling_down.visuals.is_empty());
    assert_eq!(
        (cooling_down.velocities[0].vx, cooling_down.velocities[0].vy),
        (0.0, 0.0)
    );

    let ready = field.tick(SLIME_ATTACK_COOLDOWN);
    assert_eq!(ready.visuals[0].animation, SlimeAnimation::Attack);
    assert!(ready.damages.is_empty());
}

#[test]
fn being_hurt_interrupts_a_pending_enemy_strike() {
    let mut field = SlimeField::new(200, &SPAWNS[..1], (150.0, 100.0));
    field.tick(0.0);
    assert_eq!(
        field.attack(attack(1)).unwrap().visual.animation,
        SlimeAnimation::Hurt
    );
    assert!(field.tick(SLIME_HURT_DURATION).damages.is_empty());
}
