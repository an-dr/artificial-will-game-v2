use super::*;
use game_messages::WILL_SPAWN;

const PROPS_JSON: &str = include_str!(
    "../../../../assets/robot_game_assets/biomes/white_harbour/white_harbour_props.json"
);
const TMX: &str = include_str!("../../assets/level-three.tmx");

fn overlaps(a: Wall, b: Wall) -> bool {
    (a.x - b.x).abs() < a.half_w + b.half_w && (a.y - b.y).abs() < a.half_h + b.half_h
}

fn solids() -> Vec<Wall> {
    let props = PROPS.iter().filter_map(|prop| prop.collider());
    let robots = CHARACTERS.iter().filter(|c| c.cast.solid()).map(|c| Wall {
        x: c.x,
        y: c.y,
        half_w: CHARACTER_HALF_W,
        half_h: CHARACTER_HALF_H,
    });
    props.chain(robots).collect()
}

#[test]
fn prop_art_matches_the_pack_atlas() {
    let atlas: serde_json::Value = serde_json::from_str(PROPS_JSON).unwrap();
    for kind in PROP_KINDS {
        let art = kind.art();
        let entry = &atlas[art.name];
        let n = |v: &serde_json::Value, i: usize| v[i].as_u64().unwrap() as u32;
        let numbers = [
            0,
            0,
            n(&entry["rect"], 2),
            n(&entry["rect"], 3),
            n(&entry["anchor"], 0),
            n(&entry["anchor"], 1),
        ];
        assert_eq!((numbers[2], numbers[3]), art.size, "{} size", art.name);
        assert_eq!((numbers[4], numbers[5]), art.anchor, "{} anchor", art.name);
    }
}

#[test]
fn sea_runs_match_the_all_sea_tiles_of_the_map() {
    let csv = TMX
        .split("<data encoding=\"csv\">")
        .nth(1)
        .unwrap()
        .split("</data>")
        .next()
        .unwrap();
    let tiles: Vec<u32> = csv
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    let (w, h) = MAP_TILES;
    assert_eq!(tiles.len(), (w * h) as usize);
    // GIDs 17..=20 are the all-sea base variants.
    for ty in 0..h {
        for tx in 0..w {
            let sea = (17..=20).contains(&tiles[(ty * w + tx) as usize]);
            let walled = SEA_RUNS
                .iter()
                .any(|&(x, y, len)| y == ty && (x..x + len).contains(&tx));
            assert_eq!(sea, walled, "tile ({tx}, {ty})");
        }
    }
}

#[test]
fn will_spawns_clear_of_every_wall_and_solid() {
    let will = Wall {
        x: WILL_SPAWN.0,
        y: WILL_SPAWN.1,
        half_w: 20.0 + 16.0,
        half_h: 28.0 + 16.0,
    };
    for solid in walls().into_iter().chain(solids()) {
        assert!(!overlaps(will, solid), "{solid:?} blocks Will's spawn");
    }
}

#[test]
fn solids_stand_on_land_and_apart() {
    let solids = solids();
    for (i, a) in solids.iter().enumerate() {
        for wall in walls() {
            assert!(
                !overlaps(*a, wall),
                "{a:?} stands in the sea or off the map"
            );
        }
        for b in &solids[i + 1..] {
            assert!(!overlaps(*a, *b), "{a:?} overlaps {b:?}");
        }
    }
}

#[test]
fn the_cast_has_robots_and_spirits() {
    assert!(CHARACTERS.iter().filter(|c| c.cast.solid()).count() >= 4);
    assert!(CHARACTERS.iter().filter(|c| !c.cast.solid()).count() >= 4);
}
