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
    assert!(ROCK_ID_START + (ROCKS.len() as u32) < 200);
}
