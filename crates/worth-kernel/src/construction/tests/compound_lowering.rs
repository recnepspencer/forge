use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::specs::{RegularPyramidSpec, WireBodySpec};
use crate::construction::tests::support::compound_lowering::SpatialFixtureWitnessCatalog;
use crate::construction::tests::support::compound_lowering::{
    ConstructionMovePlan, ConstructionOffsetPlan, ConstructionReorientPlan, ConstructionRotatePlan,
    PrimitiveConstructionMotionLoweringError,
};
use worth_geom::ParameterSpacePoint;
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialAxis, SpatialCarrierPointRole, SpatialDirectionWitnessRef,
    SpatialFrameRef,
};
use worth_spatial::facade::refs::{
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
};

#[test]
fn primitive_construction_motion_finish_updates_embedded_placement() {
    let moved = ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .to([10.0, 0.0, 3.0])
    .finish()
    .expect("moved wire");
    let offset = ConstructionOffsetPlan::shape(moved)
        .by([0.0, -2.0, 1.0])
        .finish()
        .expect("offset wire");
    let reoriented = ConstructionReorientPlan::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .toward([0.0, 1.0, 1.0])
    .finish()
    .expect("reoriented pyramid");
    let rotated = ConstructionRotatePlan::shape(reoriented.clone())
        .around([1.0, 0.0, 0.0])
        .by_radians(std::f64::consts::FRAC_PI_2)
        .finish()
        .expect("rotated pyramid");
    let constrained =
        ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
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
    assert_eq!(offset.placement_spec().origin(), [10.0, -2.0, 4.0]);
    assert!(matches!(
        reoriented.placement_spec().direction_witness(),
        SpatialDirectionWitnessRef::WorldDirection(direction)
            if direction[1] > 0.70 && direction[2] > 0.70
    ));
    assert!(matches!(
        rotated.placement_spec().direction_witness(),
        SpatialDirectionWitnessRef::WorldDirection(direction)
            if direction[1] < -0.70 && direction[2] > 0.70
    ));
    assert_eq!(constrained.placement_spec().origin(), [0.0, 0.0, 0.0]);
}

#[test]
fn primitive_construction_motion_finish_supports_point_like_world_and_frame_anchors() {
    let workplane = SpatialFrameRef::workplane("wp-2", [4.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let rotated = ConstructionRotatePlan::shape(PrimitiveConstructionIntent::regular_pyramid(
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
    let pointed = ConstructionReorientPlan::shape(PrimitiveConstructionIntent::regular_pyramid(
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
    assert!((rotated.placement_spec().origin()[0] - 4.0).abs() < 1.0e-12);
    assert!((rotated.placement_spec().origin()[1] + 4.0).abs() < 1.0e-12);
    assert!(matches!(
        pointed.placement_spec().direction_witness(),
        SpatialDirectionWitnessRef::WorldDirection(direction)
            if direction[1] > 0.30 && direction[2] > 0.90
    ));
}

#[test]
fn primitive_construction_motion_finish_supports_external_reference_translation_anchors() {
    let moved = ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .from(SpatialAnchorRef::world_origin())
    .to([10.0, 0.0, 3.0])
    .finish()
    .expect("world-origin move should lower");
    let offset =
        ConstructionOffsetPlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
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
    let error = ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .from(SpatialAnchorRef::shape_axis(SpatialAxis::W))
    .to([10.0, 0.0, 3.0])
    .finish()
    .expect_err("unsupported anchor should fail");

    assert!(matches!(
        error,
        PrimitiveConstructionMotionLoweringError::PlacementLowering(_)
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
    let rotated = ConstructionRotatePlan::shape(PrimitiveConstructionIntent::regular_pyramid(
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
    let pointed = ConstructionReorientPlan::shape(PrimitiveConstructionIntent::regular_pyramid(
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
    let moved = ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .from(SpatialAnchorRef::feature_owned("feature-anchor"))
    .to([10.0, 0.0, 3.0])
    .finish_with_catalog(&catalog)
    .expect("feature-owned move should lower");
    let offset =
        ConstructionOffsetPlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .from(SpatialAnchorRef::feature_owned("feature-anchor"))
        .by([2.0, -1.0, 0.5])
        .finish_with_catalog(&catalog)
        .expect("feature-owned offset should lower");
    let matched =
        ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .so(SpatialAnchorRef::feature_owned("feature-anchor"))
        .matches(SpatialAnchorRef::world_origin())
        .finish_with_catalog(&catalog)
        .expect("feature-owned anchor match should lower");
    let placed =
        ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
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
        ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .so(SpatialAnchorRef::shape_origin())
        .matches(SpatialAnchorRef::feature_owned("feature-anchor"))
        .finish_with_catalog(&catalog)
        .expect("shape-origin to feature target should lower");
    let matched_to_shape_origin =
        ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .so(SpatialAnchorRef::feature_owned("feature-anchor"))
        .matches(SpatialAnchorRef::shape_origin())
        .finish_with_catalog(&catalog)
        .expect("feature-owned anchor to shape-origin target should lower");
    assert!((rotated.placement_spec().origin()[0] - 4.0).abs() < 1.0e-12);
    assert!((rotated.placement_spec().origin()[1] + 4.0).abs() < 1.0e-12);
    assert!(matches!(
        pointed.placement_spec().direction_witness(),
        SpatialDirectionWitnessRef::WorldDirection(direction)
            if direction[1] > 0.59 && direction[2] > 0.79
    ));
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
        placed.placement_spec().reference_frame(),
        &SpatialFrameRef::workplane("wp-feature", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0])
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
    let moved = ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .from(SpatialAnchorRef::geometric_tag("tag-anchor"))
    .to([10.0, 0.0, 3.0])
    .finish_with_catalog(&catalog)
    .expect("geometric-tag move should lower");
    let matched =
        ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
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
    let error = ConstructionRotatePlan::shape(PrimitiveConstructionIntent::regular_pyramid(
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
        PrimitiveConstructionMotionLoweringError::PlacementLowering(_)
    ));
}
