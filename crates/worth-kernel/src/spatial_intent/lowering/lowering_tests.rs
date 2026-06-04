use super::PrimitiveConstructionSpatialIntentError;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::specs::{RegularPyramidSpec, WireBodySpec};
use crate::facade::authoring::intents::{
    MoveSpatialIntent, OffsetSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent,
};
use crate::test_support::SpatialFixtureWitnessCatalog;
use worth_geom::ParameterSpacePoint;
use worth_spatial::facade::placement::{admit_spatial_placement, SpatialPlacementConstraintError};
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialAxis, SpatialCarrierPointRole, SpatialFrameRef,
};
use worth_spatial::facade::witness_catalog::{
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
    SpatialGeometricTagFailureClass,
};
use worth_spatial::facade::witness_resolution::SpatialWitnessFailureClass;

#[test]
fn primitive_construction_motion_finish_updates_embedded_placement() {
    let moved = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .to([10.0, 0.0, 3.0])
    .finish()
    .expect("moved wire");
    let offset = OffsetSpatialIntent::shape(moved)
        .by([0.0, -2.0, 1.0])
        .finish()
        .expect("offset wire");
    let reoriented = ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .toward([0.0, 1.0, 1.0])
    .finish()
    .expect("reoriented pyramid");
    let rotated = RotateSpatialIntent::shape(reoriented.clone())
        .around([1.0, 0.0, 0.0])
        .by_radians(std::f64::consts::FRAC_PI_2)
        .finish()
        .expect("rotated pyramid");
    let constrained =
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .so(SpatialAnchorRef::shape_origin())
        .lies_on(SpatialFrameRef::workplane(
            "wp-1",
            [0.0, 0.0, 5.0],
            [0.0, 0.0, 1.0],
        ))
        .finish()
        .expect("wire placed on workplane");
    let admitted_reoriented =
        admit_spatial_placement(reoriented.placement_spec()).expect("reoriented placement");
    let admitted_rotated =
        admit_spatial_placement(rotated.placement_spec()).expect("rotated placement");

    assert_eq!(offset.placement_spec().origin(), [10.0, -2.0, 4.0]);
    assert!(admitted_reoriented.facing_vector()[1] > 0.70);
    assert!(admitted_reoriented.facing_vector()[2] > 0.70);
    assert!(admitted_rotated.facing_vector()[1] < -0.70);
    assert!(admitted_rotated.facing_vector()[2] > 0.70);
    assert_eq!(constrained.placement_spec().origin(), [0.0, 0.0, 0.0]);
}

#[test]
fn primitive_construction_motion_finish_supports_point_like_world_and_frame_anchors() {
    let workplane = SpatialFrameRef::workplane("wp-2", [4.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let rotated = RotateSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::frame_origin(workplane))
    .around([0.0, 0.0, 1.0])
    .by_radians(std::f64::consts::FRAC_PI_2)
    .finish()
    .expect("frame-origin rotate should lower");
    let pointed = ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .so(SpatialAnchorRef::world_origin())
    .points_toward([0.0, 3.0, 9.0])
    .finish()
    .expect("world-origin points-toward should lower");
    let admitted_rotated =
        admit_spatial_placement(rotated.placement_spec()).expect("rotated placement");
    let admitted_pointed =
        admit_spatial_placement(pointed.placement_spec()).expect("pointed placement");

    assert!((admitted_rotated.origin()[0] - 4.0).abs() < 1.0e-12);
    assert!((admitted_rotated.origin()[1] + 4.0).abs() < 1.0e-12);
    assert!(admitted_pointed.facing_vector()[1] > 0.30);
    assert!(admitted_pointed.facing_vector()[2] > 0.90);
}

#[test]
fn primitive_construction_motion_finish_supports_external_reference_translation_anchors() {
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
fn primitive_construction_motion_finish_rejects_unsupported_non_point_like_anchor() {
    let error = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .from(SpatialAnchorRef::shape_axis(SpatialAxis::W))
    .to([10.0, 0.0, 3.0])
    .finish()
    .expect_err("unsupported anchor should fail");

    assert!(matches!(
        error,
        PrimitiveConstructionSpatialIntentError::PlacementLowering(_)
    ));
}

#[test]
fn primitive_construction_motion_finish_with_catalog_supports_feature_owned_anchor_paths() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-anchor",
        SpatialCarrierPointRole::Anchor,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 0.0, 0.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let rotated = RotateSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::feature_owned("feature-anchor"))
    .around([0.0, 0.0, 1.0])
    .by_radians(std::f64::consts::FRAC_PI_2)
    .finish_with_catalog(&catalog)
    .expect("feature-owned rotate should lower");
    let pointed = ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .so(SpatialAnchorRef::feature_owned("feature-anchor"))
    .points_toward([4.0, 3.0, 4.0])
    .finish_with_catalog(&catalog)
    .expect("feature-owned points-toward should lower");
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
    let admitted_rotated =
        admit_spatial_placement(rotated.placement_spec()).expect("rotated placement");
    let admitted_pointed =
        admit_spatial_placement(pointed.placement_spec()).expect("pointed placement");

    assert!((admitted_rotated.origin()[0] - 4.0).abs() < 1.0e-12);
    assert!((admitted_rotated.origin()[1] + 4.0).abs() < 1.0e-12);
    assert!(admitted_pointed.facing_vector()[1] > 0.59);
    assert!(admitted_pointed.facing_vector()[2] > 0.79);
    assert_eq!(moved.placement_spec().origin(), [6.0, 0.0, 3.0]);
    assert_eq!(offset.placement_spec().origin(), [2.0, -1.0, 0.5]);
    assert_eq!(matched.placement_spec().origin(), [-4.0, 0.0, 0.0]);
    assert_eq!(placed.placement_spec().origin(), [0.0, 0.0, 0.0]);
    assert_eq!(
        matched_to_feature.placement_spec().origin(),
        [4.0, 0.0, 0.0]
    );
    assert_eq!(
        matched_to_shape_origin.placement_spec().origin(),
        [-4.0, 0.0, 0.0]
    );
    assert_eq!(
        admit_spatial_placement(placed.placement_spec())
            .expect("placed placement")
            .origin(),
        [0.0, 0.0, 5.0]
    );
}

#[test]
fn primitive_construction_motion_finish_with_catalog_supports_geometric_tag_anchor_paths() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_geometric_tag_point(
        "tag-anchor",
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 0.0, 0.0],
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

    assert_eq!(moved.placement_spec().origin(), [6.0, 0.0, 3.0]);
    assert_eq!(matched.placement_spec().origin(), [4.0, 0.0, 0.0]);
}

#[test]
fn primitive_construction_motion_finish_with_catalog_rejects_parameter_space_anchor_paths() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_parameter_space_point(
        worth_spatial::facade::refs::SpatialCarrierKind::Surface,
        "surface-anchor",
        ParameterSpacePoint::try_new([0.25, 0.75]).unwrap(),
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 0.0, 0.0],
            SpatialCatalogWitnessResolutionClass::CarrierDerived,
        )),
    );
    let error = RotateSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::parameter_space(
        "surface-anchor",
        "0.25,0.75",
    ))
    .around([0.0, 0.0, 1.0])
    .by_radians(std::f64::consts::FRAC_PI_2)
    .finish_with_catalog(&catalog)
    .expect_err("parameter-space anchor should stay unsupported");

    assert!(matches!(
        error,
        PrimitiveConstructionSpatialIntentError::PlacementLowering(_)
    ));
}

#[test]
fn primitive_construction_motion_finish_with_catalog_preserves_geometric_tag_failure_truth() {
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
fn primitive_construction_motion_finish_with_catalog_preserves_feature_owned_lies_on_witness_failure_truth(
) {
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
fn primitive_construction_motion_finish_with_catalog_preserves_target_anchor_match_witness_failure_truth(
) {
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
    .expect_err("target-side feature-owned anchor witness failure should stay typed");

    assert_eq!(
        error,
        PrimitiveConstructionSpatialIntentError::ConstraintLowering(
            SpatialPlacementConstraintError::AnchorWitnessFailure(
                SpatialWitnessFailureClass::Exhausted,
            ),
        )
    );
}
