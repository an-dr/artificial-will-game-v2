use super::*;

#[test]
fn rock_field_is_dense_and_keeps_wills_spawn_clear() {
    assert!(ROCKS.len() >= 20);
    let spawn = (464.0, 464.0);
    assert!(ROCKS.iter().all(|&(x, y, half_w, half_h)| {
        (spawn.0 - x).abs() > half_w + 32.0 || (spawn.1 - y).abs() > half_h + 32.0
    }));
}

#[test]
fn rock_entity_ids_leave_room_for_will_and_slimes() {
    assert!(ROCK_ID_START > WILL_ENTITY_ID);
    assert!(ROCK_ID_START + (ROCKS.len() as u32) < SLIME_ID_START);
}

#[test]
fn passive_slimes_are_clear_of_will_and_rocks() {
    assert_eq!(SLIMES.len(), 6);
    let will_spawn = (464.0, 464.0);
    for &(_, x, y) in SLIMES {
        assert!(
            (will_spawn.0 - x).abs() > SLIME_COLLIDER_HALF_W + 10.0
                || (will_spawn.1 - y).abs() > SLIME_COLLIDER_HALF_H + 27.0
        );
        assert!(ROCKS.iter().all(|&(rock_x, rock_y, half_w, half_h)| {
            (rock_x - x).abs() > half_w + SLIME_COLLIDER_HALF_W
                || (rock_y - y).abs() > half_h + SLIME_COLLIDER_HALF_H
        }));
    }
}

#[test]
fn slime_idle_presentation_loops_while_stationary() {
    for sprite_id in SLIME_SPRITE_IDS {
        let presentation = slime_presentation(sprite_id);
        assert_eq!(presentation.sprite.sprite_id, sprite_id);
        assert_eq!(presentation.sprite.frame_count, SLIME_FRAME_COUNT);
        assert!(presentation.looping);
        assert!(presentation.advance_while_stopped);
    }
}

#[test]
fn slime_ids_are_unique_and_separate_from_level_geometry() {
    let last_rock_id = ROCK_ID_START + ROCKS.len() as u32 - 1;
    let last_slime_id = SLIME_ID_START + SLIMES.len() as u32 - 1;
    assert!(SLIME_ID_START > last_rock_id);
    assert_eq!(last_slime_id - SLIME_ID_START + 1, SLIMES.len() as u32);
}
