use worth_kernel::facade::authoring::{construction::*, intents::*};
use worth_spatial::facade::{
    admit_spatial_placement, SpatialAnchorRef, SpatialAxis, SpatialCarrierDirectionRole,
    SpatialCarrierPointRole, SpatialCatalogResolvedDirectionWitness,
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
    SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog, SpatialFrameRef,
    SpatialGeometricTagFailureClass, SpatialPlacementConstraintError, SpatialPlacementMotionError,
    SpatialWitnessFailureClass,
};

#[test]
fn kernel_public_facade_exports_catalog_backed_subject_anchor_translation_finishers() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-anchor",
        SpatialCarrierPointRole::Anchor,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 1.0, 0.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let moved = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .from(SpatialAnchorRef::feature_owned("feature-anchor"))
    .to([10.0, 0.0, 3.0])
    .finish_with_catalog(&catalog)
    .expect("feature-owned move should lower");
    let offset = OffsetSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .from(SpatialAnchorRef::feature_owned("feature-anchor"))
    .by([2.0, -1.0, 0.5])
    .finish_with_catalog(&catalog)
    .expect("feature-owned offset should lower");
    let matched = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .so(SpatialAnchorRef::feature_owned("feature-anchor"))
    .matches(SpatialAnchorRef::world_origin())
    .finish_with_catalog(&catalog)
    .expect("feature-owned anchor match should lower");
    let placed = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .so(SpatialAnchorRef::feature_owned("feature-anchor"))
    .lies_on(SpatialFrameRef::workplane(
        "wp-feature",
        [0.0, 0.0, 5.0],
        [0.0, 0.0, 1.0],
    ))
    .finish_with_catalog(&catalog)
    .expect("feature-owned lies-on should lower");
    let matched_to_feature =
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .so(SpatialAnchorRef::shape_origin())
        .matches(SpatialAnchorRef::feature_owned("feature-anchor"))
        .finish_with_catalog(&catalog)
        .expect("shape-origin to feature target should lower");
    let matched_to_shape_origin =
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .so(SpatialAnchorRef::feature_owned("feature-anchor"))
        .matches(SpatialAnchorRef::shape_origin())
        .finish_with_catalog(&catalog)
        .expect("feature-owned anchor to shape-origin target should lower");

    assert_eq!(moved.placement_spec().origin(), [6.0, -1.0, 3.0]);
    assert_eq!(offset.placement_spec().origin(), [2.0, -1.0, 0.5]);
    assert_eq!(matched.placement_spec().origin(), [-4.0, -1.0, 0.0]);
    assert_eq!(placed.placement_spec().origin(), [0.0, 0.0, 0.0]);
    assert_eq!(
        matched_to_feature.placement_spec().origin(),
        [4.0, 1.0, 0.0]
    );
    assert_eq!(
        matched_to_shape_origin.placement_spec().origin(),
        [-4.0, -1.0, 0.0]
    );
    assert_eq!(
        admit_spatial_placement(placed.placement_spec())
            .expect("admitted placed")
            .origin(),
        [0.0, 0.0, 5.0]
    );
}

#[test]
fn kernel_public_facade_exports_external_reference_translation_finishers() {
    let moved = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .from(SpatialAnchorRef::world_origin())
    .to([10.0, 0.0, 3.0])
    .finish()
    .expect("world-origin move should lower");
    let offset = OffsetSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .from(SpatialAnchorRef::frame_origin(SpatialFrameRef::workplane(
        "wp-external",
        [10.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
    )))
    .by([2.0, -1.0, 0.5])
    .finish()
    .expect("frame-origin offset should lower");

    assert_eq!(moved.placement_spec().origin(), [10.0, 0.0, 3.0]);
    assert_eq!(offset.placement_spec().origin(), [2.0, -1.0, 0.5]);
}

#[test]
fn kernel_public_facade_exports_directional_reorient_anchor_finishers() {
    let shape_u = ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::shape_axis(SpatialAxis::U))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
    .finish()
    .expect("shape-u reorient should lower");
    let shape_v = ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::shape_axis(SpatialAxis::V))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
    .finish()
    .expect("shape-v reorient should lower");
    let shape_w = ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::shape_axis(SpatialAxis::W))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0]))
    .finish()
    .expect("shape-w reorient should lower");
    let frame_axis = ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::frame_axis(
        SpatialFrameRef::world(),
        SpatialAxis::U,
    ))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
    .finish()
    .expect("frame-axis reorient should lower");

    let admitted_shape_u = admit_spatial_placement(shape_u.placement_spec()).expect("shape-u");
    let admitted_shape_v = admit_spatial_placement(shape_v.placement_spec()).expect("shape-v");
    assert!(admitted_shape_u.facing_vector()[0].abs() < 1.0e-12);
    assert!(admitted_shape_u.facing_vector()[1] < -0.99);
    assert!(admitted_shape_u.facing_vector()[2].abs() < 1.0e-12);
    assert!(admitted_shape_v.facing_vector()[0] > 0.99);
    assert!(admitted_shape_v.facing_vector()[1].abs() < 1.0e-12);
    assert!(admitted_shape_v.facing_vector()[2].abs() < 1.0e-12);
    assert!(
        admit_spatial_placement(shape_w.placement_spec())
            .expect("shape-w")
            .facing_vector()[1]
            > 0.70
    );
    assert!(
        admit_spatial_placement(frame_axis.placement_spec())
            .expect("frame-axis")
            .facing_vector()[0]
            < -0.99
    );
}

#[test]
fn kernel_public_facade_preserves_feature_owned_lies_on_witness_failure_truth() {
    let error = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .so(SpatialAnchorRef::feature_owned("feature-anchor"))
    .lies_on(SpatialFrameRef::workplane(
        "wp-feature",
        [0.0, 0.0, 5.0],
        [0.0, 0.0, 1.0],
    ))
    .finish_with_catalog(
        &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
            "feature-anchor",
            SpatialCarrierPointRole::Anchor,
            Err(SpatialWitnessFailureClass::Undefined),
        ),
    )
    .expect_err("feature-owned lies-on witness failure should stay typed");

    assert_eq!(
        error,
        PrimitiveConstructionSpatialIntentError::ConstraintLowering(
            SpatialPlacementConstraintError::AnchorWitnessFailure(
                SpatialWitnessFailureClass::Undefined,
            ),
        )
    );
}

#[test]
fn kernel_public_facade_preserves_target_anchor_match_witness_failure_truth() {
    let error = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .so(SpatialAnchorRef::shape_origin())
    .matches(SpatialAnchorRef::feature_owned("target-anchor"))
    .finish_with_catalog(
        &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
            "target-anchor",
            SpatialCarrierPointRole::Anchor,
            Err(SpatialWitnessFailureClass::Exhausted),
        ),
    )
    .expect_err("target-side anchor witness failure should stay typed");

    assert_eq!(
        error,
        PrimitiveConstructionSpatialIntentError::ConstraintLowering(
            SpatialPlacementConstraintError::AnchorWitnessFailure(
                SpatialWitnessFailureClass::Exhausted,
            ),
        )
    );
}

#[test]
fn kernel_public_facade_exports_catalog_backed_geometric_tag_anchor_finishers() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_geometric_tag_point(
        "tag-anchor",
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 1.0, 0.0],
            SpatialCatalogWitnessResolutionClass::CarrierDerived,
        )),
    );
    let moved = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .from(SpatialAnchorRef::geometric_tag("tag-anchor"))
    .to([10.0, 0.0, 3.0])
    .finish_with_catalog(&catalog)
    .expect("geometric-tag move should lower");
    let matched = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .so(SpatialAnchorRef::shape_origin())
    .matches(SpatialAnchorRef::geometric_tag("tag-anchor"))
    .finish_with_catalog(&catalog)
    .expect("shape-origin to tag target should lower");

    assert_eq!(moved.placement_spec().origin(), [6.0, -1.0, 3.0]);
    assert_eq!(matched.placement_spec().origin(), [4.0, 1.0, 0.0]);
}

#[test]
fn kernel_public_facade_preserves_geometric_tag_anchor_failure_truth() {
    let error = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .so(SpatialAnchorRef::shape_origin())
    .matches(SpatialAnchorRef::geometric_tag("tag-anchor"))
    .finish_with_catalog(
        &SpatialFixtureWitnessCatalog::new()
            .with_geometric_tag_point("tag-anchor", Err(SpatialWitnessFailureClass::Ambiguous)),
    )
    .expect_err("geometric-tag target failure should stay typed");

    assert_eq!(
        error,
        PrimitiveConstructionSpatialIntentError::ConstraintLowering(
            SpatialPlacementConstraintError::AnchorTagFailure(
                SpatialGeometricTagFailureClass::Resolution(SpatialWitnessFailureClass::Ambiguous,),
            ),
        )
    );
}

#[test]
fn kernel_public_facade_preserves_directional_feature_anchor_ambiguity_truth() {
    let error = ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::feature_owned("feature-axis"))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
    .finish_with_catalog(
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
        PrimitiveConstructionSpatialIntentError::PlacementLowering(
            SpatialPlacementMotionError::AmbiguousReorientAnchorMeaning
        )
    );
}
