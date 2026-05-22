use super::{
    admit_requested_spatial_direction_witness, admit_requested_spatial_point_witness,
    request_spatial_direction_witness, request_spatial_point_witness,
    resolve_admitted_spatial_direction_witness, resolve_admitted_spatial_point_witness,
};
use crate::spatial_intent::refs::{
    EmptySpatialWitnessCatalog, SpatialAxis, SpatialDirectionWitnessRef, SpatialFrameRef,
    SpatialPointWitnessRef,
};
use crate::spatial_intent::resolution::SpatialWitnessFailureClass;

#[test]
fn point_progression_preserves_direct_world_success() {
    let requested =
        request_spatial_point_witness(SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]));
    let admitted = match admit_requested_spatial_point_witness(requested) {
        forge_proof::TransitionOutcome::Success(value) => value,
        other => panic!("unexpected admission outcome: {other:?}"),
    };
    let resolved =
        match resolve_admitted_spatial_point_witness(admitted, &EmptySpatialWitnessCatalog) {
            forge_proof::TransitionOutcome::Success(value) => value,
            other => panic!("unexpected resolution outcome: {other:?}"),
        };

    assert_eq!(resolved.payload().resolved_world_point(), [1.0, 2.0, 3.0]);
}

#[test]
fn point_progression_allows_carrier_request_but_denies_at_resolution() {
    let requested =
        request_spatial_point_witness(SpatialPointWitnessRef::ambiguous_curve_point("curve-1"));
    let admitted = match admit_requested_spatial_point_witness(requested) {
        forge_proof::TransitionOutcome::Success(value) => value,
        other => panic!("unexpected admission outcome: {other:?}"),
    };
    let denied = resolve_admitted_spatial_point_witness(admitted, &EmptySpatialWitnessCatalog);

    assert!(matches!(
        denied,
        forge_proof::TransitionOutcome::Denied(SpatialWitnessFailureClass::Ambiguous)
    ));
}

#[test]
fn direction_progression_preserves_frame_fallback_path() {
    let requested =
        request_spatial_direction_witness(SpatialDirectionWitnessRef::frame_perpendicular_axis(
            SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            SpatialAxis::W,
        ));
    let admitted = match admit_requested_spatial_direction_witness(requested) {
        forge_proof::TransitionOutcome::Success(value) => value,
        other => panic!("unexpected admission outcome: {other:?}"),
    };
    let resolved =
        match resolve_admitted_spatial_direction_witness(admitted, &EmptySpatialWitnessCatalog) {
            forge_proof::TransitionOutcome::Success(value) => value,
            other => panic!("unexpected resolution outcome: {other:?}"),
        };

    let direction = resolved.payload().resolved_world_direction();
    assert!(direction[0].abs() > 0.99);
    assert!(direction[2].abs() < 1.0e-12);
}
