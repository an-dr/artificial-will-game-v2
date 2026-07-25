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
