use super::{
    apply_admitted_move_to_placement, apply_admitted_move_to_placement_with_catalog,
    apply_admitted_offset_to_placement, apply_admitted_offset_to_placement_with_catalog,
    apply_admitted_reorient_to_placement, apply_admitted_reorient_to_placement_with_catalog,
    apply_admitted_rotate_to_placement, apply_admitted_rotate_to_placement_with_catalog,
    SpatialPlacementMotionError,
};
use crate::facade::{
    admit_spatial_move, admit_spatial_offset, admit_spatial_placement, admit_spatial_reorient,
    admit_spatial_rotate, SpatialAnchorRef, SpatialCarrierPointRole,
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
    SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog, SpatialFrameRef,
    SpatialGeometricTagFailureClass, SpatialMoveSpec, SpatialOffsetSpec, SpatialPlacementSpec,
    SpatialPointWitnessRef, SpatialReorientSpec, SpatialRotateSpec, SpatialWitnessFailureClass,
};

#[test]
fn admitted_motion_can_lower_point_like_anchor_motion_into_placement() {
    let moved = apply_admitted_move_to_placement(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_move(SpatialMoveSpec::shape_origin().to([10.0, -4.0, 8.0])).expect("move"),
    )
    .expect("moved placement");
    let offset = apply_admitted_offset_to_placement(
        moved.clone(),
        &admit_spatial_offset(SpatialOffsetSpec::shape_origin().by([2.0, 0.0, -3.0]))
            .expect("offset"),
    )
    .expect("offset placement");
    let reoriented = apply_admitted_reorient_to_placement(
        offset,
        &admit_spatial_reorient(
            SpatialReorientSpec::shape_origin()
                .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0])),
        )
        .expect("reorient"),
    )
    .expect("reoriented placement");
    let rotated = apply_admitted_rotate_to_placement(
        reoriented.clone(),
        &admit_spatial_rotate(
            SpatialRotateSpec::shape_origin()
                .around([1.0, 0.0, 0.0])
                .by_radians(std::f64::consts::FRAC_PI_2),
        )
        .expect("rotate"),
    )
    .expect("rotated placement");
    let admitted_reoriented = admit_spatial_placement(reoriented.clone()).expect("admitted");
    let admitted_rotated = admit_spatial_placement(rotated.clone()).expect("admitted rotated");

    assert_eq!(moved.origin(), [10.0, -4.0, 8.0]);
    assert_eq!(reoriented.origin(), [12.0, -4.0, 5.0]);
    assert!(admitted_reoriented.facing_vector()[1] > 0.70);
    assert!(admitted_reoriented.facing_vector()[2] > 0.70);
    assert!(admitted_rotated.facing_vector()[1] < -0.70);
    assert!(admitted_rotated.facing_vector()[2] > 0.70);
}

#[test]
fn admitted_motion_can_lower_world_and_frame_origin_rotation_and_reorientation() {
    let workplane = SpatialFrameRef::workplane("wp-2", [4.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let reoriented = apply_admitted_reorient_to_placement(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_reorient(
            SpatialReorientSpec::shape_origin()
                .about(SpatialAnchorRef::frame_origin(workplane.clone()))
                .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0])),
        )
        .expect("reorient"),
    )
    .expect("reoriented placement");
    let rotated = apply_admitted_rotate_to_placement(
        SpatialPlacementSpec::world().at([5.0, 0.0, 0.0]),
        &admit_spatial_rotate(
            SpatialRotateSpec::shape_origin()
                .about(SpatialAnchorRef::world_origin())
                .around([0.0, 0.0, 1.0])
                .by_radians(std::f64::consts::FRAC_PI_2),
        )
        .expect("rotate"),
    )
    .expect("rotated placement");
    let admitted_reoriented = admit_spatial_placement(reoriented).expect("admitted reoriented");
    let admitted_rotated = admit_spatial_placement(rotated).expect("admitted rotated");

    assert!(admitted_reoriented.facing_vector()[1] > 0.70);
    assert!(admitted_reoriented.facing_vector()[2] > 0.70);
    assert!(admitted_rotated.origin()[0].abs() < 1.0e-12);
    assert!((admitted_rotated.origin()[1] - 5.0).abs() < 1.0e-12);
}

#[test]
fn admitted_motion_can_lower_feature_owned_anchor_rotation_and_reorientation_with_catalog() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-pivot",
        SpatialCarrierPointRole::Anchor,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 0.0, 0.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let reoriented = apply_admitted_reorient_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_reorient(
            SpatialReorientSpec::shape_origin()
                .about(SpatialAnchorRef::feature_owned("feature-pivot"))
                .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0])),
        )
        .expect("reorient"),
        &catalog,
    )
    .expect("reoriented placement");
    let rotated = apply_admitted_rotate_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([5.0, 0.0, 0.0]),
        &admit_spatial_rotate(
            SpatialRotateSpec::shape_origin()
                .about(SpatialAnchorRef::feature_owned("feature-pivot"))
                .around([0.0, 0.0, 1.0])
                .by_radians(std::f64::consts::FRAC_PI_2),
        )
        .expect("rotate"),
        &catalog,
    )
    .expect("rotated placement");
    let admitted_reoriented = admit_spatial_placement(reoriented).expect("admitted reoriented");
    let admitted_rotated = admit_spatial_placement(rotated).expect("admitted rotated");

    assert!(admitted_reoriented.facing_vector()[1] > 0.70);
    assert!(admitted_reoriented.facing_vector()[2] > 0.70);
    assert!((admitted_rotated.origin()[0] - 4.0).abs() < 1.0e-12);
    assert!((admitted_rotated.origin()[1] - 1.0).abs() < 1.0e-12);
}

#[test]
fn admitted_motion_can_lower_feature_owned_anchor_move_and_offset_with_catalog() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-anchor",
        SpatialCarrierPointRole::Anchor,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 1.0, 0.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let moved = apply_admitted_move_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_move(
            SpatialMoveSpec::shape_origin()
                .from(SpatialAnchorRef::feature_owned("feature-anchor"))
                .to([10.0, 0.0, 3.0]),
        )
        .expect("move"),
        &catalog,
    )
    .expect("moved placement");
    let offset = apply_admitted_offset_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_offset(
            SpatialOffsetSpec::shape_origin()
                .from(SpatialAnchorRef::feature_owned("feature-anchor"))
                .by([2.0, -1.0, 0.5]),
        )
        .expect("offset"),
        &catalog,
    )
    .expect("offset placement");

    assert_eq!(moved.origin(), [7.0, 1.0, 6.0]);
    assert_eq!(offset.origin(), [3.0, 1.0, 3.5]);
}

#[test]
fn admitted_motion_can_lower_geometric_tag_anchor_move_and_offset_with_catalog() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_geometric_tag_point(
        "tag-anchor",
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 1.0, 0.0],
            SpatialCatalogWitnessResolutionClass::CarrierDerived,
        )),
    );
    let moved = apply_admitted_move_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_move(
            SpatialMoveSpec::shape_origin()
                .from(SpatialAnchorRef::geometric_tag("tag-anchor"))
                .to([10.0, 0.0, 3.0]),
        )
        .expect("move"),
        &catalog,
    )
    .expect("moved placement");
    let offset = apply_admitted_offset_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_offset(
            SpatialOffsetSpec::shape_origin()
                .from(SpatialAnchorRef::geometric_tag("tag-anchor"))
                .by([2.0, -1.0, 0.5]),
        )
        .expect("offset"),
        &catalog,
    )
    .expect("offset placement");

    assert_eq!(moved.origin(), [7.0, 1.0, 6.0]);
    assert_eq!(offset.origin(), [3.0, 1.0, 3.5]);
}

#[test]
fn admitted_motion_move_and_offset_preserve_frame_relative_translation_truth() {
    let workplane = SpatialFrameRef::workplane("wp-frame", [10.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let moved = apply_admitted_move_to_placement(
        SpatialPlacementSpec::world()
            .relative_to(workplane.clone())
            .at([1.0, 2.0, 3.0]),
        &admit_spatial_move(SpatialMoveSpec::shape_origin().to([20.0, 5.0, 3.0])).expect("move"),
    )
    .expect("moved placement");
    let offset = apply_admitted_offset_to_placement(
        SpatialPlacementSpec::world()
            .relative_to(workplane.clone())
            .at([1.0, 2.0, 3.0]),
        &admit_spatial_offset(SpatialOffsetSpec::shape_origin().by([2.0, -1.0, 0.5]))
            .expect("offset"),
    )
    .expect("offset placement");
    let admitted_frame = crate::facade::admit_spatial_frame(workplane).expect("frame");

    assert_eq!(moved.origin(), [5.0, -10.0, 3.0]);
    assert_eq!(offset.origin(), [0.0, 0.0, 3.5]);
    assert_eq!(
        admitted_frame.basis().embed_point(moved.origin()),
        [20.0, 5.0, 3.0]
    );
    assert_eq!(
        admitted_frame.basis().embed_point(offset.origin()),
        [10.0, 0.0, 3.5]
    );
}

#[test]
fn admitted_motion_can_lower_external_reference_move_and_offset_anchors() {
    let workplane = SpatialFrameRef::workplane("wp-external", [10.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let moved_from_world = apply_admitted_move_to_placement(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_move(
            SpatialMoveSpec::shape_origin()
                .from(SpatialAnchorRef::world_origin())
                .to([10.0, 0.0, 3.0]),
        )
        .expect("move from world origin"),
    )
    .expect("world-origin move should lower");
    let offset_from_frame = apply_admitted_offset_to_placement(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_offset(
            SpatialOffsetSpec::shape_origin()
                .from(SpatialAnchorRef::frame_origin(workplane))
                .by([2.0, -1.0, 0.5]),
        )
        .expect("offset from frame origin"),
    )
    .expect("frame-origin offset should lower");

    assert_eq!(moved_from_world.origin(), [11.0, 2.0, 6.0]);
    assert_eq!(offset_from_frame.origin(), [3.0, 1.0, 3.5]);
}

#[test]
fn lowering_preserves_feature_owned_anchor_witness_failure_truth() {
    let error = apply_admitted_rotate_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([5.0, 0.0, 0.0]),
        &admit_spatial_rotate(
            SpatialRotateSpec::shape_origin()
                .about(SpatialAnchorRef::feature_owned("feature-pivot"))
                .around([0.0, 0.0, 1.0])
                .by_radians(std::f64::consts::FRAC_PI_2),
        )
        .expect("rotate"),
        &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
            "feature-pivot",
            SpatialCarrierPointRole::Anchor,
            Err(SpatialWitnessFailureClass::Exhausted),
        ),
    )
    .expect_err("feature-owned anchor witness failure should stay typed");

    assert_eq!(
        error,
        SpatialPlacementMotionError::AnchorWitnessFailure(SpatialWitnessFailureClass::Exhausted)
    );
}

#[test]
fn lowering_preserves_geometric_tag_anchor_failure_truth() {
    let error = apply_admitted_move_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_move(
            SpatialMoveSpec::shape_origin()
                .from(SpatialAnchorRef::geometric_tag("tag-anchor"))
                .to([10.0, 0.0, 3.0]),
        )
        .expect("move"),
        &SpatialFixtureWitnessCatalog::new()
            .with_geometric_tag_point("tag-anchor", Err(SpatialWitnessFailureClass::Ambiguous)),
    )
    .expect_err("geometric-tag anchor failure should stay typed");

    assert_eq!(
        error,
        SpatialPlacementMotionError::AnchorTagFailure(SpatialGeometricTagFailureClass::Resolution(
            SpatialWitnessFailureClass::Ambiguous
        ))
    );
}

#[test]
fn lowering_preserves_direction_like_and_unsupported_tag_classes() {
    let direction_like = apply_admitted_move_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_move(
            SpatialMoveSpec::shape_origin()
                .from(SpatialAnchorRef::geometric_tag("tag-direction"))
                .to([10.0, 0.0, 3.0]),
        )
        .expect("move"),
        &SpatialFixtureWitnessCatalog::new().with_geometric_tag_direction(
            "tag-direction",
            Ok(crate::facade::SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 1.0, 0.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        ),
    )
    .expect_err("direction-like tags should stay typed, not lower as points");
    let unsupported_class = apply_admitted_move_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_move(
            SpatialMoveSpec::shape_origin()
                .from(SpatialAnchorRef::geometric_tag("tag-unsupported"))
                .to([10.0, 0.0, 3.0]),
        )
        .expect("move"),
        &SpatialFixtureWitnessCatalog::new()
            .with_geometric_tag_unsupported_class("tag-unsupported"),
    )
    .expect_err("unsupported tag class should stay typed");

    assert_eq!(
        direction_like,
        SpatialPlacementMotionError::AnchorTagFailure(
            SpatialGeometricTagFailureClass::ResolvedDirectionLike
        )
    );
    assert_eq!(
        unsupported_class,
        SpatialPlacementMotionError::AnchorTagFailure(
            SpatialGeometricTagFailureClass::ResolvedUnsupportedClass
        )
    );
}

#[test]
fn lowering_rejects_non_point_like_anchors_for_current_placement_model() {
    let error = apply_admitted_move_to_placement(
        SpatialPlacementSpec::world(),
        &admit_spatial_move(
            SpatialMoveSpec::shape_origin()
                .from(SpatialAnchorRef::shape_axis(crate::facade::SpatialAxis::W))
                .to_witness(SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0])),
        )
        .expect("move"),
    )
    .expect_err("unsupported shape-axis move anchor should fail");

    assert_eq!(error, SpatialPlacementMotionError::UnsupportedMoveAnchor);
}
