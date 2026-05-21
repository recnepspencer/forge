use worth_spatial::facade::{
    admit_spatial_anchor_match_constraint, admit_spatial_lies_on_constraint, admit_spatial_move,
    admit_spatial_offset, admit_spatial_placement, admit_spatial_reorient,
    apply_admitted_anchor_match_constraint_to_placement_with_catalog,
    apply_admitted_lies_on_constraint_to_placement_with_catalog,
    apply_admitted_move_to_placement_with_catalog, apply_admitted_offset_to_placement_with_catalog,
    apply_admitted_reorient_to_placement, apply_admitted_reorient_to_placement_with_catalog,
    SpatialAnchorMatchConstraintSpec, SpatialAnchorRef, SpatialAxis, SpatialCarrierDirectionRole,
    SpatialCarrierPointRole, SpatialCatalogResolvedDirectionWitness,
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
    SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog, SpatialFrameRef,
    SpatialGeometricTagFailureClass, SpatialLiesOnConstraintSpec, SpatialMoveSpec,
    SpatialOffsetSpec, SpatialPlacementConstraintError, SpatialPlacementMotionError,
    SpatialPlacementSpec, SpatialReorientSpec, SpatialWitnessFailureClass,
};

#[test]
fn spatial_public_facade_exports_catalog_backed_subject_anchor_translation_surfaces() {
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
    let matched = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::feature_owned("feature-anchor"),
            SpatialAnchorRef::world_origin(),
        ))
        .expect("anchor match"),
        &catalog,
    )
    .expect("matched placement");
    let placed = apply_admitted_lies_on_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
            SpatialAnchorRef::feature_owned("feature-anchor"),
            SpatialFrameRef::workplane("wp-feature", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
        ))
        .expect("lies on"),
        &catalog,
    )
    .expect("placed placement");
    let matched_to_feature = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::feature_owned("feature-anchor"),
        ))
        .expect("anchor match to feature"),
        &catalog,
    )
    .expect("matched placement to feature");
    let matched_to_shape_origin = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::feature_owned("feature-anchor"),
            SpatialAnchorRef::shape_origin(),
        ))
        .expect("anchor match to shape origin"),
        &catalog,
    )
    .expect("matched placement to shape origin");
    let admitted_placed = admit_spatial_placement(placed.clone()).expect("admitted placed");

    assert_eq!(moved.origin(), [7.0, 1.0, 6.0]);
    assert_eq!(offset.origin(), [3.0, 1.0, 3.5]);
    assert_eq!(matched.origin(), [-3.0, 1.0, 3.0]);
    assert_eq!(matched_to_feature.origin(), [4.0, 1.0, 0.0]);
    assert_eq!(matched_to_shape_origin.origin(), [-2.0, 3.0, 6.0]);
    assert_eq!(
        placed.reference_frame(),
        &SpatialFrameRef::workplane("wp-feature", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0])
    );
    assert_eq!(admitted_placed.origin(), [1.0, 2.0, 8.0]);
}

#[test]
fn spatial_public_facade_exports_external_reference_translation_surfaces() {
    let moved = apply_admitted_move_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_move(
            SpatialMoveSpec::shape_origin()
                .from(SpatialAnchorRef::world_origin())
                .to([10.0, 0.0, 3.0]),
        )
        .expect("move"),
        &SpatialFixtureWitnessCatalog::new(),
    )
    .expect("world-origin move");
    let offset = apply_admitted_offset_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_offset(
            SpatialOffsetSpec::shape_origin()
                .from(SpatialAnchorRef::frame_origin(SpatialFrameRef::workplane(
                    "wp-external",
                    [10.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0],
                )))
                .by([2.0, -1.0, 0.5]),
        )
        .expect("offset"),
        &SpatialFixtureWitnessCatalog::new(),
    )
    .expect("frame-origin offset");

    assert_eq!(moved.origin(), [11.0, 2.0, 6.0]);
    assert_eq!(offset.origin(), [3.0, 1.0, 3.5]);
}

#[test]
fn spatial_public_facade_exports_directional_reorient_anchor_surfaces() {
    let shape_u = apply_admitted_reorient_to_placement(
        SpatialPlacementSpec::world(),
        &admit_spatial_reorient(
            SpatialReorientSpec::shape_origin()
                .about(SpatialAnchorRef::shape_axis(SpatialAxis::U))
                .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0])),
        )
        .expect("reorient"),
    )
    .expect("shape-u reorient");
    let shape_v = apply_admitted_reorient_to_placement(
        SpatialPlacementSpec::world(),
        &admit_spatial_reorient(
            SpatialReorientSpec::shape_origin()
                .about(SpatialAnchorRef::shape_axis(SpatialAxis::V))
                .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0])),
        )
        .expect("reorient"),
    )
    .expect("shape-v reorient");
    let shape_w = apply_admitted_reorient_to_placement(
        SpatialPlacementSpec::world(),
        &admit_spatial_reorient(
            SpatialReorientSpec::shape_origin()
                .about(SpatialAnchorRef::shape_axis(SpatialAxis::W))
                .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0])),
        )
        .expect("reorient"),
    )
    .expect("shape-w reorient");
    let frame_axis = apply_admitted_reorient_to_placement(
        SpatialPlacementSpec::world(),
        &admit_spatial_reorient(
            SpatialReorientSpec::shape_origin()
                .about(SpatialAnchorRef::frame_axis(
                    SpatialFrameRef::world(),
                    SpatialAxis::U,
                ))
                .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0])),
        )
        .expect("reorient"),
    )
    .expect("frame-axis reorient");

    let admitted_shape_u = admit_spatial_placement(shape_u).expect("shape-u");
    let admitted_shape_v = admit_spatial_placement(shape_v).expect("shape-v");
    assert!(admitted_shape_u.facing_vector()[0].abs() < 1.0e-12);
    assert!(admitted_shape_u.facing_vector()[1] < -0.99);
    assert!(admitted_shape_u.facing_vector()[2].abs() < 1.0e-12);
    assert!(admitted_shape_v.facing_vector()[0] > 0.99);
    assert!(admitted_shape_v.facing_vector()[1].abs() < 1.0e-12);
    assert!(admitted_shape_v.facing_vector()[2].abs() < 1.0e-12);
    assert!(
        admit_spatial_placement(shape_w)
            .expect("shape-w")
            .facing_vector()[1]
            > 0.70
    );
    assert!(
        admit_spatial_placement(frame_axis)
            .expect("frame-axis")
            .facing_vector()[0]
            < -0.99
    );
}

#[test]
fn spatial_public_facade_preserves_feature_owned_lies_on_witness_failure_truth() {
    let error = apply_admitted_lies_on_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
            SpatialAnchorRef::feature_owned("feature-anchor"),
            SpatialFrameRef::workplane("wp-feature", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
        ))
        .expect("lies on"),
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
fn spatial_public_facade_preserves_target_anchor_match_witness_failure_truth() {
    let error = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::feature_owned("target-anchor"),
        ))
        .expect("anchor match"),
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
fn spatial_public_facade_exports_catalog_backed_geometric_tag_anchor_lowering() {
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
    let matched = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::geometric_tag("tag-anchor"),
        ))
        .expect("anchor match to tag"),
        &catalog,
    )
    .expect("matched placement to tag");

    assert_eq!(moved.origin(), [7.0, 1.0, 6.0]);
    assert_eq!(matched.origin(), [4.0, 1.0, 0.0]);
}

#[test]
fn spatial_public_facade_preserves_geometric_tag_anchor_failure_truth() {
    let error = apply_admitted_anchor_match_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([1.0, 2.0, 3.0]),
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::geometric_tag("tag-anchor"),
        ))
        .expect("anchor match to tag"),
        &SpatialFixtureWitnessCatalog::new()
            .with_geometric_tag_point("tag-anchor", Err(SpatialWitnessFailureClass::Ambiguous)),
    )
    .expect_err("geometric-tag anchor failure should stay typed");

    assert_eq!(
        error,
        SpatialPlacementConstraintError::AnchorTagFailure(
            SpatialGeometricTagFailureClass::Resolution(SpatialWitnessFailureClass::Ambiguous)
        )
    );
}

#[test]
fn spatial_public_facade_preserves_directional_feature_anchor_ambiguity_truth() {
    let error = apply_admitted_reorient_to_placement_with_catalog(
        SpatialPlacementSpec::world(),
        &admit_spatial_reorient(
            SpatialReorientSpec::shape_origin()
                .about(SpatialAnchorRef::feature_owned("feature-axis"))
                .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0])),
        )
        .expect("reorient"),
        &SpatialFixtureWitnessCatalog::new()
            .with_feature_owned_point(
                "feature-axis",
                SpatialCarrierPointRole::Anchor,
                Ok(SpatialCatalogResolvedPointWitness::new(
                    [0.0, 0.0, 0.0],
                    SpatialCatalogWitnessResolutionClass::FallbackDerived,
                )),
            )
            .with_feature_owned_direction(
                "feature-axis",
                SpatialCarrierDirectionRole::Axis,
                Ok(SpatialCatalogResolvedDirectionWitness::new(
                    [1.0, 0.0, 0.0],
                    SpatialCatalogWitnessResolutionClass::CarrierDerived,
                )),
            ),
    )
    .expect_err("feature-owned point+direction meaning should stay ambiguous");

    assert_eq!(
        error,
        SpatialPlacementMotionError::AmbiguousReorientAnchorMeaning
    );
}
