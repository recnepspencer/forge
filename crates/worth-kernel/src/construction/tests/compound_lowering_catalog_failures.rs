use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::specs::WireBodySpec;
use crate::construction::tests::support::compound_lowering::{
    ConstructionMovePlan, PrimitiveConstructionMotionLoweringError, SpatialFixtureWitnessCatalog,
};
use worth_spatial::facade::anchor_selection::SpatialWitnessFailureClass;
use worth_spatial::facade::placement::SpatialPlacementConstraintError;
use worth_spatial::facade::refs::SpatialGeometricTagFailureClass;
use worth_spatial::facade::refs::{SpatialAnchorRef, SpatialCarrierPointRole, SpatialFrameRef};

#[test]
fn primitive_construction_motion_finish_with_catalog_preserves_geometric_tag_failure_truth() {
    let error = ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
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
        PrimitiveConstructionMotionLoweringError::ConstraintLowering(
            SpatialPlacementConstraintError::AnchorTagFailure(
                SpatialGeometricTagFailureClass::Resolution(SpatialWitnessFailureClass::Ambiguous,),
            ),
        )
    );
}

#[test]
fn primitive_construction_motion_finish_with_catalog_preserves_feature_owned_lies_on_witness_failure_truth(
) {
    let error = ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
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
        PrimitiveConstructionMotionLoweringError::ConstraintLowering(
            SpatialPlacementConstraintError::AnchorWitnessFailure(
                SpatialWitnessFailureClass::Undefined,
            ),
        )
    );
}

#[test]
fn primitive_construction_motion_finish_with_catalog_preserves_target_anchor_match_witness_failure_truth(
) {
    let error = ConstructionMovePlan::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
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
        PrimitiveConstructionMotionLoweringError::ConstraintLowering(
            SpatialPlacementConstraintError::AnchorWitnessFailure(
                SpatialWitnessFailureClass::Exhausted,
            ),
        )
    );
}
