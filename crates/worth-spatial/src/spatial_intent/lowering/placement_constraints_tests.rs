use super::{
    apply_admitted_anchor_match_constraint_to_placement,
    apply_admitted_anchor_match_constraint_to_placement_with_catalog,
    apply_admitted_lies_on_constraint_to_placement,
    apply_admitted_lies_on_constraint_to_placement_with_catalog,
    apply_admitted_points_toward_constraint_to_placement,
    apply_admitted_points_toward_constraint_to_placement_with_catalog,
    SpatialPlacementConstraintError,
};
use crate::facade::{
    admit_spatial_anchor_match_constraint, admit_spatial_frame, admit_spatial_lies_on_constraint,
    admit_spatial_placement, admit_spatial_points_toward_constraint,
    SpatialAnchorMatchConstraintSpec, SpatialAnchorRef, SpatialCarrierPointRole,
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
    SpatialFixtureWitnessCatalog, SpatialFrameRef, SpatialGeometricTagFailureClass,
    SpatialLiesOnConstraintSpec, SpatialPlacementSpec, SpatialPointsTowardConstraintSpec,
    SpatialWitnessFailureClass,
};

#[test]
fn admitted_constraints_can_lower_shape_origin_constraint_intent_into_placement() {
    let workplane = SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let placed = apply_admitted_lies_on_constraint_to_placement(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            workplane.clone(),
        ))
        .expect("lies-on"),
    )
    .expect("placed on frame");
    let pointed = apply_admitted_points_toward_constraint_to_placement(
        placed.clone(),
        &admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            [0.0, 1.0, 7.0],
        ))
        .expect("points-toward"),
    )
    .expect("pointed");
    let matched = apply_admitted_anchor_match_constraint_to_placement(
        placed,
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::world_origin(),
        ))
        .expect("match"),
    )
    .expect("matched");
    let admitted_pointed = admit_spatial_placement(pointed.clone()).expect("admitted pointed");

    assert_eq!(pointed.reference_frame(), &workplane);
    assert_eq!(pointed.origin(), [0.0, 0.0, 0.0]);
    assert!(admitted_pointed.facing_vector()[2] > 0.0);
    assert!(
        admitted_pointed.facing_vector()[0].abs() > 0.0
            || admitted_pointed.facing_vector()[1].abs() > 0.0
    );
    let matched_frame = admit_spatial_frame(matched.reference_frame().clone()).expect("frame");
    assert_eq!(matched.reference_frame(), &workplane);
    assert_eq!(
        matched_frame.basis().embed_point(matched.origin()),
        [0.0, 0.0, 0.0]
    );
}

#[test]
fn admitted_constraints_can_lower_feature_owned_lies_on_with_catalog() {
    let workplane = SpatialFrameRef::workplane("wp-feature", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-anchor",
        SpatialCarrierPointRole::Anchor,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 1.0, 8.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let placed = apply_admitted_lies_on_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
            SpatialAnchorRef::feature_owned("feature-anchor"),
            workplane.clone(),
        ))
        .expect("lies-on"),
        &catalog,
    )
    .expect("feature-owned lies-on should lower");
    let admitted = admit_spatial_placement(placed.clone()).expect("admitted placed");

    assert_eq!(placed.reference_frame(), &workplane);
    assert_eq!(admitted.origin(), [1.0, 2.0, 0.0]);
}

#[test]
fn admitted_constraints_can_lower_world_and_frame_origin_points_toward_intent() {
    let target_frame = SpatialFrameRef::workplane("target", [10.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let pointed = apply_admitted_points_toward_constraint_to_placement(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
            SpatialAnchorRef::world_origin(),
            [0.0, 1.0, 7.0],
        ))
        .expect("points-toward"),
    )
    .expect("pointed");
    let pointed_from_frame = apply_admitted_points_toward_constraint_to_placement(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
            SpatialAnchorRef::frame_origin(target_frame.clone()),
            [10.0, 3.0, 9.0],
        ))
        .expect("points-toward from frame"),
    )
    .expect("pointed from frame");

    let admitted_pointed = admit_spatial_placement(pointed).expect("admitted pointed");
    let expected_frame_direction = {
        let delta = [0.0_f64, 3.0, 4.0];
        let length = (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt();
        [delta[0] / length, delta[1] / length, delta[2] / length]
    };

    assert!(admitted_pointed.facing_vector()[1] > 0.14);
    assert!(admitted_pointed.facing_vector()[2] > 0.98);
    let admitted_frame_pointed =
        admit_spatial_placement(pointed_from_frame).expect("admitted frame pointed");
    assert_eq!(
        admitted_frame_pointed.facing_vector(),
        expected_frame_direction
    );
}

#[test]
fn admitted_constraints_can_lower_feature_owned_anchor_points_toward_with_catalog() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-anchor",
        SpatialCarrierPointRole::Anchor,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [10.0, 0.0, 5.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let pointed = apply_admitted_points_toward_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
            SpatialAnchorRef::feature_owned("feature-anchor"),
            [10.0, 3.0, 9.0],
        ))
        .expect("points-toward from feature"),
        &catalog,
    )
    .expect("pointed from feature");
    let admitted_pointed = admit_spatial_placement(pointed).expect("admitted pointed");
    let expected_direction = {
        let delta = [0.0_f64, 3.0, 4.0];
        let length = (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt();
        [delta[0] / length, delta[1] / length, delta[2] / length]
    };

    assert_eq!(admitted_pointed.facing_vector(), expected_direction);
}

#[test]
fn admitted_constraints_can_lower_geometric_tag_anchor_constraints_with_catalog() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_geometric_tag_point(
        "tag-anchor",
        Ok(SpatialCatalogResolvedPointWitness::new(
            [10.0, 0.0, 5.0],
            SpatialCatalogWitnessResolutionClass::CarrierDerived,
        )),
    );
    let pointed = apply_admitted_points_toward_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
            SpatialAnchorRef::geometric_tag("tag-anchor"),
            [10.0, 3.0, 9.0],
        ))
        .expect("points-toward from tag"),
        &catalog,
    )
    .expect("pointed from tag");
    let matched = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::geometric_tag("tag-anchor"),
        ))
        .expect("anchor match to tag"),
        &catalog,
    )
    .expect("matched to tag");
    let admitted_pointed = admit_spatial_placement(pointed).expect("admitted pointed");
    let expected_direction = {
        let delta = [0.0_f64, 3.0, 4.0];
        let length = (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt();
        [delta[0] / length, delta[1] / length, delta[2] / length]
    };

    assert_eq!(admitted_pointed.facing_vector(), expected_direction);
    assert_eq!(matched.origin(), [10.0, 0.0, 5.0]);
}

#[test]
fn admitted_constraints_can_lower_point_like_target_anchor_match_with_catalog() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "target-anchor",
        SpatialCarrierPointRole::Anchor,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [10.0, 3.0, 9.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let matched_to_feature = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::feature_owned("target-anchor"),
        ))
        .expect("anchor match to feature"),
        &catalog,
    )
    .expect("shape-origin to feature target should lower");
    let matched_to_shape_origin = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::feature_owned("target-anchor"),
            SpatialAnchorRef::shape_origin(),
        ))
        .expect("anchor match to shape origin"),
        &catalog,
    )
    .expect("feature anchor to shape-origin target should lower");

    assert_eq!(matched_to_feature.origin(), [10.0, 3.0, 9.0]);
    assert_eq!(matched_to_shape_origin.origin(), [-6.0, 3.0, -1.0]);
}

#[test]
fn lies_on_lowering_preserves_feature_owned_anchor_witness_failure_truth() {
    let error = apply_admitted_lies_on_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
            SpatialAnchorRef::feature_owned("feature-anchor"),
            SpatialFrameRef::world(),
        ))
        .expect("lies-on from feature"),
        &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
            "feature-anchor",
            SpatialCarrierPointRole::Anchor,
            Err(SpatialWitnessFailureClass::Undefined),
        ),
    )
    .expect_err("feature-owned lies-on witness failure should stay typed");

    assert_eq!(
        error,
        SpatialPlacementConstraintError::AnchorWitnessFailure(
            SpatialWitnessFailureClass::Undefined
        )
    );
}

#[test]
fn anchor_match_lowering_preserves_target_anchor_witness_failure_truth() {
    let error = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::feature_owned("target-anchor"),
        ))
        .expect("anchor match to feature"),
        &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
            "target-anchor",
            SpatialCarrierPointRole::Anchor,
            Err(SpatialWitnessFailureClass::Exhausted),
        ),
    )
    .expect_err("target-side anchor witness failure should stay typed");

    assert_eq!(
        error,
        SpatialPlacementConstraintError::AnchorWitnessFailure(
            SpatialWitnessFailureClass::Exhausted
        )
    );
}

#[test]
fn points_toward_lowering_preserves_feature_owned_anchor_witness_failure_truth() {
    let error = apply_admitted_points_toward_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
            SpatialAnchorRef::feature_owned("feature-anchor"),
            [10.0, 3.0, 9.0],
        ))
        .expect("points-toward from feature"),
        &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
            "feature-anchor",
            SpatialCarrierPointRole::Anchor,
            Err(SpatialWitnessFailureClass::Undefined),
        ),
    )
    .expect_err("feature-owned anchor witness failure should stay typed");

    assert_eq!(
        error,
        SpatialPlacementConstraintError::AnchorWitnessFailure(
            SpatialWitnessFailureClass::Undefined
        )
    );
}

#[test]
fn constraint_lowering_preserves_geometric_tag_anchor_failure_truth() {
    let error = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::geometric_tag("tag-anchor"),
        ))
        .expect("anchor match to tag"),
        &SpatialFixtureWitnessCatalog::new()
            .with_geometric_tag_point("tag-anchor", Err(SpatialWitnessFailureClass::Exhausted)),
    )
    .expect_err("geometric-tag anchor failure should stay typed");

    assert_eq!(
        error,
        SpatialPlacementConstraintError::AnchorTagFailure(
            SpatialGeometricTagFailureClass::Resolution(SpatialWitnessFailureClass::Exhausted)
        )
    );
}

#[test]
fn constraint_lowering_preserves_direction_like_and_unsupported_tag_classes() {
    let direction_like = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::geometric_tag("tag-direction"),
        ))
        .expect("anchor match to tag"),
        &SpatialFixtureWitnessCatalog::new().with_geometric_tag_direction(
            "tag-direction",
            Ok(crate::facade::SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 1.0, 0.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        ),
    )
    .expect_err("direction-like tags should stay typed");
    let unsupported_class = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([2.0, 3.0, 4.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::geometric_tag("tag-unsupported"),
        ))
        .expect("anchor match to tag"),
        &SpatialFixtureWitnessCatalog::new()
            .with_geometric_tag_unsupported_class("tag-unsupported"),
    )
    .expect_err("unsupported tag class should stay typed");

    assert_eq!(
        direction_like,
        SpatialPlacementConstraintError::AnchorTagFailure(
            SpatialGeometricTagFailureClass::ResolvedDirectionLike
        )
    );
    assert_eq!(
        unsupported_class,
        SpatialPlacementConstraintError::AnchorTagFailure(
            SpatialGeometricTagFailureClass::ResolvedUnsupportedClass
        )
    );
}

#[test]
fn constraint_lowering_rejects_non_point_like_constraint_anchors() {
    let error = apply_admitted_lies_on_constraint_to_placement(
        SpatialPlacementSpec::world(),
        &admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
            SpatialAnchorRef::shape_axis(crate::facade::SpatialAxis::W),
            SpatialFrameRef::world(),
        ))
        .expect("lies-on"),
    )
    .expect_err("unsupported shape-axis constraint should fail");

    assert_eq!(
        error,
        SpatialPlacementConstraintError::UnsupportedLiesOnAnchor
    );
}
