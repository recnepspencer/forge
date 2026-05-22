use super::{
    admit_lowered_spatial_runtime_intent, lower_admitted_move_intent,
    lower_admitted_reorient_intent, LoweredSpatialIntentFamily, LoweredSpatialNumericPosture,
    RuntimeAnchorSemantic,
};
use crate::facade::{
    admit_spatial_move, admit_spatial_reorient, SpatialAnchorRef, SpatialFrameRef, SpatialMoveSpec,
    SpatialReorientSpec,
};
use crate::spatial_intent::lowering::lowered_intents::LoweredSpatialRuntimePayload;
use forge_query::facade::{ForgeQueryIntentAdmissionDecision, ForgeQueryRawIntentAdmissionRequest};

#[test]
fn lowered_move_intent_hands_off_to_real_query_runtime_entrypoint() {
    let admitted = admit_spatial_move(
        SpatialMoveSpec::shape_origin()
            .from(SpatialAnchorRef::shape_origin())
            .to([3.0, 4.0, 5.0]),
    )
    .expect("admitted move");
    let lowered =
        lower_admitted_move_intent(crate::facade::SpatialPlacementSpec::world(), &admitted)
            .expect("lowered move");
    let declaration = lowered.payload().runtime_declaration();
    let request = declaration
        .to_query_runtime_request()
        .expect("query runtime request");

    assert_eq!(
        declaration.subject_anchor(),
        Some(RuntimeAnchorSemantic::ShapeOriginPoint)
    );
    assert_eq!(
        declaration.numeric_posture(),
        LoweredSpatialNumericPosture::Direct
    );
    assert!(matches!(
        declaration.payload(),
        LoweredSpatialRuntimePayload::Move {
            anchor_world_point,
            target_world_point
        } if *anchor_world_point == [0.0, 0.0, 0.0] && *target_world_point == [3.0, 4.0, 5.0]
    ));
    assert!(matches!(
        admit_lowered_spatial_runtime_intent(declaration).expect("admitted query intent"),
        ForgeQueryIntentAdmissionDecision::Admitted(_)
    ));
    assert_eq!(
        request.family(),
        ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
            declaration.to_query_intent_declaration()
        )
        .expect("rebuild")
        .family()
    );
}

#[test]
fn directional_reorient_lowering_preserves_fallback_posture_in_runtime_declaration() {
    let admitted = admit_spatial_reorient(
        SpatialReorientSpec::shape_origin()
            .about(SpatialAnchorRef::shape_axis(crate::facade::SpatialAxis::U))
            .toward_witness(
                crate::facade::SpatialDirectionWitnessRef::frame_perpendicular_axis(
                    SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
                    crate::facade::SpatialAxis::W,
                ),
            ),
    )
    .expect("admitted reorient");
    let lowered =
        lower_admitted_reorient_intent(crate::facade::SpatialPlacementSpec::world(), &admitted)
            .expect("lowered reorient");
    let declaration = lowered.payload().runtime_declaration();

    assert_eq!(declaration.family(), LoweredSpatialIntentFamily::Reorient);
    assert_eq!(
        declaration.numeric_posture(),
        LoweredSpatialNumericPosture::Normalized
    );
    assert!(matches!(
        declaration.payload(),
        LoweredSpatialRuntimePayload::ReorientDirectional { .. }
    ));
}
