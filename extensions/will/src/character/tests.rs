use super::*;

#[test]
fn will_is_pushable_out_of_fixed_level_obstacles() {
    let op = spawn_op(&PlayerState::default());
    assert!(matches!(
        op,
        EntityOp::Spawn {
            entity_id: WILL_ENTITY_ID,
            x,
            y,
            body_kind: BodyKind::Frictionless,
            ..
        } if (x, y) == WILL_SPAWN
    ));
}

#[test]
fn every_state_animation_exists_in_the_sheet_and_has_a_sprite() {
    use crate::player_state::ATTACK_DURATION_SECONDS;
    let sheet = SpriteSheet::parse(SHEET_JSON).unwrap();
    for name in ANIMATIONS {
        sheet
            .cut_strip(SHEET_PNG, name, sheet.shift_feet_to(FEET_TARGET))
            .unwrap();
    }
    let attack = sheet.animation("attack_down").unwrap();
    let frames = sheet.rects("attack_down").unwrap().len() as u32;
    assert!((attack.duration(frames) - ATTACK_DURATION_SECONDS).abs() < 1e-6);
    let ids: std::collections::HashSet<u32> =
        ANIMATIONS.iter().map(|name| sprite_id(name)).collect();
    assert_eq!(ids.len(), ANIMATIONS.len());
}

#[test]
fn will_is_drawn_at_the_sheet_frame_size() {
    let presentation = presentation(&PlayerState::default());
    assert_eq!(presentation.sprite.sprite_id, FIRST_SPRITE_ID);
    assert_eq!((presentation.draw_w, presentation.draw_h), (96, 96));
    assert!(presentation.looping);
}
