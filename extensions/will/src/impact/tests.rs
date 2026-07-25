use super::*;

fn hit(sequence: u32, x: f32, y: f32) -> HitConfirmed {
    HitConfirmed {
        sequence,
        entity_id: 40,
        x,
        y,
    }
}

#[test]
fn confirmed_hit_draws_a_brief_world_space_marker() {
    let mut feedback = ImpactFeedback::default();
    feedback.confirm(hit(1, 120.4, 80.6));
    let [outline, center] = feedback.rectangles().expect("visible marker");

    assert_eq!(
        (outline.x, outline.y, outline.w, outline.h),
        (105, 66, 30, 30)
    );
    assert!(!outline.screen_space);
    assert!(center.filled);

    feedback.tick(IMPACT_SECONDS);
    assert_eq!(feedback.rectangles(), None);
}

#[test]
fn duplicate_or_invalid_confirmations_are_ignored() {
    let mut feedback = ImpactFeedback::default();
    feedback.confirm(hit(3, 10.0, 20.0));
    let original = feedback.rectangles();
    feedback.confirm(hit(3, 50.0, 60.0));
    feedback.confirm(hit(4, f32::NAN, 60.0));
    assert_eq!(feedback.rectangles(), original);
}
