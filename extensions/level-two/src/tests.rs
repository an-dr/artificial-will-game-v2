use super::*;

#[test]
fn ground_mix_uses_only_opaque_grass_and_broken_stone_cells() {
    let tmx = std::str::from_utf8(LEVEL_TWO_TMX).unwrap();
    let csv = tmx
        .split("<data encoding=\"csv\">")
        .nth(1)
        .unwrap()
        .split("</data>")
        .next()
        .unwrap();
    let tile_ids = csv
        .split(|character: char| character == ',' || character.is_whitespace())
        .filter(|value| !value.is_empty())
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(tile_ids.len(), 16 * 16);
    assert!(tile_ids
        .iter()
        .all(|id| { matches!(id, 1 | 2 | 3 | 4 | 11 | 12 | 15) }));
    assert!(tile_ids.iter().any(|id| matches!(id, 1 | 2 | 3 | 4)));
    assert!(tile_ids.iter().any(|id| matches!(id, 11 | 12 | 15)));
}

#[test]
fn rock_field_is_dense_and_keeps_wills_spawn_clear() {
    assert!(ROCKS.len() >= 14);
    let spawn = (464.0, 464.0);
    assert!(ROCKS.iter().all(|rock| {
        (spawn.0 - rock.x).abs() > rock.half_w + 32.0
            || (spawn.1 - rock.y).abs() > rock.half_h + 32.0
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
        assert!(ROCKS.iter().all(|rock| {
            (rock.x - x).abs() > rock.half_w + SLIME_COLLIDER_HALF_W
                || (rock.y - y).abs() > rock.half_h + SLIME_COLLIDER_HALF_H
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

#[test]
fn every_rock_uses_art_with_an_aligned_fixed_collider() {
    assert!(ROCKS.iter().all(|rock| {
        matches!(rock.sprite_id, ROCK_CLUSTER_SPRITE_ID | BOULDER_SPRITE_ID)
            && rock.half_w > 0.0
            && rock.half_h > 0.0
            && rock.draw_w > (rock.half_w * 2.0) as u32
            && rock.draw_h > (rock.half_h * 2.0) as u32
    }));
}
