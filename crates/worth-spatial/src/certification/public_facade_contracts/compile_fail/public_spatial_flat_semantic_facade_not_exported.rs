use worth_spatial::facade::{
    admit_spatial_move, admit_spatial_placement, analyze_spatial_intent_conflict,
    evaluate_primitive_construction_birth_consequence, lower_admitted_move_intent,
    plan_primitive_construction_birth, SpatialAuthoredActKind, SpatialMoveSpec,
    SpatialPlacementSpec,
};

fn main() {
    let _ = admit_spatial_move;
    let _ = admit_spatial_placement;
    let _ = analyze_spatial_intent_conflict;
    let _ = evaluate_primitive_construction_birth_consequence;
    let _ = lower_admitted_move_intent;
    let _ = plan_primitive_construction_birth;
    let _ = SpatialAuthoredActKind::Move;
    let _ = SpatialMoveSpec::shape_origin();
    let _ = SpatialPlacementSpec::world();
}
