use super::{
    admit_spatial_move, admit_spatial_offset, admit_spatial_reorient, admit_spatial_rotate,
    SpatialMotionError, SpatialMoveSpec, SpatialOffsetSpec, SpatialReorientSpec, SpatialRotateSpec,
};
use crate::facade::{
    refs::{
        SpatialAnchorRef, SpatialAxis, SpatialDirectionWitnessRef, SpatialFrameRef,
        SpatialPointWitnessRef,
    },
    witness_resolution::{SpatialWitnessFailureClass, SpatialWitnessResolutionClass},
};

#[test]
fn admitted_spatial_motion_preserves_anchor_and_resolved_witness_truth() {
    let move_plan = admit_spatial_move(
        SpatialMoveSpec::shape_origin()
            .from(SpatialAnchorRef::shape_axis(SpatialAxis::W))
            .to([10.0, 0.0, 3.0]),
    )
    .expect("move plan");
    let rotate_plan = admit_spatial_rotate(
        SpatialRotateSpec::shape_origin()
            .about(SpatialAnchorRef::shape_origin())
            .around_witness(SpatialDirectionWitnessRef::frame_axis(
                SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 1.0, 1.0]),
                SpatialAxis::W,
            ))
            .by_radians(0.5),
    )
    .expect("rotate plan");
    let reorient_plan = admit_spatial_reorient(
        SpatialReorientSpec::shape_origin()
            .about(SpatialAnchorRef::shape_origin())
            .toward_witness(SpatialDirectionWitnessRef::world_direction([1.0, 0.0, 1.0])),
    )
    .expect("reorient plan");
    let offset_plan = admit_spatial_offset(
        SpatialOffsetSpec::shape_origin()
            .from(SpatialAnchorRef::shape_axis(SpatialAxis::U))
            .by([0.0, 0.0, 2.0]),
    )
    .expect("offset plan");

    assert_eq!(
        move_plan.spec().anchor(),
        &SpatialAnchorRef::shape_axis(SpatialAxis::W)
    );
    assert_eq!(move_plan.destination_point(), [10.0, 0.0, 3.0]);
    assert_eq!(
        move_plan.resolved_destination_witness().resolution_class(),
        SpatialWitnessResolutionClass::DirectWorld
    );
    assert_eq!(
        rotate_plan.resolved_axis_witness().resolution_class(),
        SpatialWitnessResolutionClass::FrameDerived
    );
    assert!(rotate_plan.normalized_axis()[1] > 0.70);
    assert!(rotate_plan.normalized_axis()[2] > 0.70);
    assert_eq!(
        reorient_plan
            .resolved_direction_witness()
            .resolution_class(),
        SpatialWitnessResolutionClass::DirectWorld
    );
    assert!(reorient_plan.normalized_facing()[0] > 0.70);
    assert!(reorient_plan.normalized_facing()[2] > 0.70);
    assert_eq!(
        offset_plan.spec().anchor(),
        &SpatialAnchorRef::shape_axis(SpatialAxis::U)
    );
}

#[test]
fn prepositional_motion_aliases_preserve_frame_and_fallback_witness_meaning() {
    let workplane = SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let parallel =
        admit_spatial_reorient(SpatialReorientSpec::shape_origin().parallel_to(workplane.clone()))
            .expect("parallel");
    let perpendicular =
        admit_spatial_reorient(SpatialReorientSpec::shape_origin().perpendicular_to(workplane))
            .expect("perpendicular");

    assert_eq!(
        parallel.resolved_direction_witness().resolution_class(),
        SpatialWitnessResolutionClass::FrameDerived
    );
    assert_eq!(
        perpendicular
            .resolved_direction_witness()
            .resolution_class(),
        SpatialWitnessResolutionClass::FallbackDerived
    );
    assert_eq!(parallel.normalized_facing(), [0.0, 0.0, 1.0]);
    assert!(perpendicular.normalized_facing()[2].abs() < 1.0e-12);
}

#[test]
fn admitted_spatial_motion_distinguishes_undefined_and_unsupported_witness_failures() {
    let ambiguous_move = admit_spatial_move(
        SpatialMoveSpec::shape_origin()
            .to_witness(SpatialPointWitnessRef::ambiguous_curve_point("curve-1")),
    )
    .expect_err("ambiguous move target should fail");
    let undefined = admit_spatial_reorient(
        SpatialReorientSpec::shape_origin()
            .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 0.0])),
    )
    .expect_err("undefined witness should fail");
    let unsupported = admit_spatial_reorient(SpatialReorientSpec::shape_origin().toward_witness(
        SpatialDirectionWitnessRef::surface_normal("surface-1", 0.5, 0.5),
    ))
    .expect_err("unsupported witness should fail");

    assert_eq!(
        ambiguous_move,
        SpatialMotionError::DestinationWitnessFailure(SpatialWitnessFailureClass::Ambiguous)
    );
    assert_eq!(
        undefined,
        SpatialMotionError::DirectionWitnessFailure(SpatialWitnessFailureClass::Undefined)
    );
    assert_eq!(
        unsupported,
        SpatialMotionError::DirectionWitnessFailure(SpatialWitnessFailureClass::Unsupported)
    );
}
