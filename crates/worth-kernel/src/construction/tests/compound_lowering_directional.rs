use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::specs::RegularPyramidSpec;
use crate::construction::tests::support::compound_lowering::{
    ConstructionReorientPlan, PrimitiveConstructionMotionLoweringError,
    SpatialFixtureWitnessCatalog,
};
use worth_spatial::facade::placement::SpatialPlacementMotionError;
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialAxis, SpatialCarrierDirectionRole, SpatialCarrierPointRole,
    SpatialDirectionWitnessRef,
};
use worth_spatial::facade::refs::{
    SpatialCatalogResolvedDirectionWitness, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass,
};

#[test]
fn primitive_construction_reorient_finish_supports_full_shape_axis_and_frame_axis_anchors() {
    let shape_u = ConstructionReorientPlan::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::shape_axis(SpatialAxis::U))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
    .finish()
    .expect("shape-u-axis reorient should lower");
    let shape_v = ConstructionReorientPlan::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::shape_axis(SpatialAxis::V))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
    .finish()
    .expect("shape-v-axis reorient should lower");
    let shape_w = ConstructionReorientPlan::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::shape_axis(SpatialAxis::W))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0]))
    .finish()
    .expect("shape-w-axis reorient should lower");
    let frame_axis = ConstructionReorientPlan::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::frame_axis(
        worth_spatial::facade::refs::SpatialFrameRef::world(),
        SpatialAxis::U,
    ))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
    .finish()
    .expect("frame-axis reorient should lower");

    assert!(matches!(
        shape_u.placement_spec().direction_witness(),
        SpatialDirectionWitnessRef::WorldDirection(direction)
            if direction[0].abs() < 1.0e-12
                && direction[1] < -0.99
                && direction[2].abs() < 1.0e-12
    ));
    assert!(matches!(
        shape_v.placement_spec().direction_witness(),
        SpatialDirectionWitnessRef::WorldDirection(direction)
            if direction[0] > 0.99
                && direction[1].abs() < 1.0e-12
                && direction[2].abs() < 1.0e-12
    ));
    assert!(matches!(
        shape_w.placement_spec().direction_witness(),
        SpatialDirectionWitnessRef::WorldDirection(direction)
            if direction[1] > 0.70 && direction[2] > 0.70
    ));
    assert!(matches!(
        frame_axis.placement_spec().direction_witness(),
        SpatialDirectionWitnessRef::WorldDirection(direction) if direction[0] < -0.99
    ));
}

#[test]
fn primitive_construction_reorient_finish_preserves_directional_feature_ambiguity_truth() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_direction(
        "feature-axis",
        SpatialCarrierDirectionRole::Axis,
        Ok(SpatialCatalogResolvedDirectionWitness::new(
            [1.0, 0.0, 0.0],
            SpatialCatalogWitnessResolutionClass::CarrierDerived,
        )),
    );
    let feature_axis = ConstructionReorientPlan::shape(
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        }),
    )
    .about(SpatialAnchorRef::feature_owned("feature-axis"))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
    .finish_with_catalog(&catalog)
    .expect("feature-owned directional reorient should lower");
    let ambiguity_error = ConstructionReorientPlan::shape(
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        }),
    )
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

    assert!(matches!(
        feature_axis.placement_spec().direction_witness(),
        SpatialDirectionWitnessRef::WorldDirection(direction)
            if direction[0] < -0.99
                && direction[1].abs() < 1.0e-12
                && direction[2].abs() < 1.0e-12
    ));
    assert_eq!(
        ambiguity_error,
        PrimitiveConstructionMotionLoweringError::PlacementLowering(
            SpatialPlacementMotionError::AmbiguousReorientAnchorMeaning
        )
    );
}
