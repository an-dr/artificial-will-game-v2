use super::*;
use std::collections::HashSet;

#[test]
fn entity_ids_are_unique_and_clear_of_will() {
    let ids = entity_ids();
    let unique: HashSet<_> = ids.iter().collect();
    assert_eq!(unique.len(), ids.len());
    assert!(!ids.contains(&WILL_ENTITY_ID));
    assert!(ids
        .iter()
        .all(|&id| id < FIRST_PROP_ID || id >= FIRST_WALL_ID + walls().len() as u32));
}

#[test]
fn every_character_animation_cuts_from_its_sheet() {
    for character in CHARACTERS {
        let (json, png) = character_sheet(character.cast);
        let sheet = SpriteSheet::parse(json).unwrap();
        sheet
            .cut_strip(png, character.animation, sheet.shift_feet_to(FEET_TARGET))
            .unwrap_or_else(|error| panic!("{:?}: {error}", character.cast));
    }
}
