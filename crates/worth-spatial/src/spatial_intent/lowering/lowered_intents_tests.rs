use super::{
    lower_admitted_move_semantic_intent, lower_admitted_reorient_semantic_intent,
    LoweredSpatialIntentFamily, LoweredSpatialNumericPosture,
};
use crate::facade::{
    motion::{admit_spatial_move, admit_spatial_reorient, SpatialMoveSpec, SpatialReorientSpec},
    placement::SpatialPlacementSpec,
    refs,
};
use forge_query::facade::{ForgeQueryIntentAdmissionDecision, ForgeQueryRawIntentAdmissionRequest};

#[test]
fn lowered_move_intent_hands_off_to_real_query_entrypoint() {
    let admitted = admit_spatial_move(
        SpatialMoveSpec::shape_origin()
            .from(refs::SpatialAnchorRef::shape_origin())
            .to([3.0, 4.0, 5.0]),
    )
    .expect("admitted move");
    let lowered = lower_admitted_move_semantic_intent(SpatialPlacementSpec::world(), &admitted)
        .expect("lowered move");
    let request = lowered
        .to_query_runtime_request()
        .expect("query runtime request");
    let declaration = lowered.to_query_intent_declaration();

    assert_eq!(lowered.family(), LoweredSpatialIntentFamily::Move);
    assert_eq!(
        lowered.numeric_posture(),
        LoweredSpatialNumericPosture::Direct
    );
    assert!(matches!(
        declaration.input(),
        serde_json::Value::Object(payload)
            if payload.get("payload")
                == Some(&serde_json::json!({
                    "kind": "move",
                    "anchor_world_point": [0.0, 0.0, 0.0],
                    "target_world_point": [3.0, 4.0, 5.0],
                }))
    ));
    assert!(matches!(
        lowered
            .admit_query_runtime_intent()
            .expect("admitted query intent"),
        ForgeQueryIntentAdmissionDecision::Admitted(_)
    ));
    assert_eq!(
        request.family(),
        ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(declaration)
            .expect("rebuild")
            .family()
    );
}

#[test]
fn directional_reorient_lowering_preserves_normalized_posture_in_query_handoff() {
    let admitted = admit_spatial_reorient(
        SpatialReorientSpec::shape_origin()
            .about(refs::SpatialAnchorRef::shape_axis(
                crate::facade::refs::SpatialAxis::U,
            ))
            .toward_witness(
                crate::facade::refs::SpatialDirectionWitnessRef::frame_perpendicular_axis(
                    refs::SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
                    crate::facade::refs::SpatialAxis::W,
                ),
            ),
    )
    .expect("admitted reorient");
    let lowered = lower_admitted_reorient_semantic_intent(SpatialPlacementSpec::world(), &admitted)
        .expect("lowered reorient");
    let declaration = lowered.to_query_intent_declaration();

    assert_eq!(lowered.family(), LoweredSpatialIntentFamily::Reorient);
    assert_eq!(
        lowered.numeric_posture(),
        LoweredSpatialNumericPosture::Normalized
    );
    assert!(matches!(
        declaration.input(),
        serde_json::Value::Object(map)
            if matches!(map.get("payload"), Some(serde_json::Value::Object(kind_map))
                if matches!(kind_map.get("kind"), Some(serde_json::Value::String(kind)) if kind == "reorient_directional"))
    ));
}
