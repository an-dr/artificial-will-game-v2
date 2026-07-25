use super::*;
use game_messages::WILL_SPAWN;
use std::collections::HashSet;

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
    assert!(tile_ids.iter().any(|id| matches!(id, 1..=4)));
    assert!(tile_ids.iter().any(|id| matches!(id, 11 | 12 | 15)));
}

#[test]
fn rock_field_is_dense_and_keeps_wills_spawn_clear() {
    assert!(ROCKS.len() >= 14);
    let spawn = WILL_SPAWN;
    assert!(ROCKS.iter().all(|rock| {
        (spawn.0 - rock.x).abs() > rock.half_w + 32.0
            || (spawn.1 - rock.y).abs() > rock.half_h + 32.0
    }));
}

#[test]
fn rock_entity_ids_leave_room_for_will_and_slimes() {
    let rock_ids = (0..ROCKS.len())
        .map(|index| ROCK_ID_START + index as u32)
        .collect::<HashSet<_>>();
    let slime_ids = (0..SLIMES.len())
        .map(|index| SLIME_ID_START + index as u32)
        .collect::<HashSet<_>>();
    assert!(!rock_ids.contains(&WILL_ENTITY_ID));
    assert!(rock_ids.is_disjoint(&slime_ids));
}

#[test]
fn hostile_slimes_start_clear_of_will_and_rocks() {
    assert_eq!(SLIMES.len(), 6);
    let will_spawn = WILL_SPAWN;
    for slime in SLIMES {
        assert!(
            (will_spawn.0 - slime.x).abs() > SLIME_COLLIDER_HALF_W + 10.0
                || (will_spawn.1 - slime.y).abs() > SLIME_COLLIDER_HALF_H + 27.0
        );
        assert!(ROCKS.iter().all(|rock| {
            (rock.x - slime.x).abs() > rock.half_w + SLIME_COLLIDER_HALF_W
                || (rock.y - slime.y).abs() > rock.half_h + SLIME_COLLIDER_HALF_H
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
    let slime_ids = (0..SLIMES.len())
        .map(|index| SLIME_ID_START + index as u32)
        .collect::<HashSet<_>>();
    assert_eq!(slime_ids.len(), SLIMES.len());
    assert!(!slime_ids.contains(&WILL_ENTITY_ID));
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
