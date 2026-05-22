use super::PrimitiveConstructionSpatialIntentError;
use crate::construction::{PrimitiveConstructionIntent, RegularPyramidSpec};
use crate::facade::ReorientSpatialIntent;
use worth_spatial::facade::{
    admit_spatial_placement, SpatialAnchorRef, SpatialAxis, SpatialCarrierDirectionRole,
    SpatialCarrierPointRole, SpatialCatalogResolvedDirectionWitness,
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
    SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog, SpatialPlacementMotionError,
};

#[test]
fn primitive_construction_reorient_finish_supports_full_shape_axis_and_frame_axis_anchors() {
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
    .expect("shape-u-axis reorient should lower");
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
    .expect("shape-v-axis reorient should lower");
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
    .expect("shape-w-axis reorient should lower");
    let frame_axis = ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::frame_axis(
        worth_spatial::facade::SpatialFrameRef::world(),
        SpatialAxis::U,
    ))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
    .finish()
    .expect("frame-axis reorient should lower");

    let admitted_shape_u = admit_spatial_placement(shape_u.placement_spec()).expect("shape-u");
    let admitted_shape_v = admit_spatial_placement(shape_v.placement_spec()).expect("shape-v");
    let admitted_shape_w = admit_spatial_placement(shape_w.placement_spec()).expect("shape-w");
    let admitted_frame_axis =
        admit_spatial_placement(frame_axis.placement_spec()).expect("frame-axis");

    assert!(admitted_shape_u.facing_vector()[0].abs() < 1.0e-12);
    assert!(admitted_shape_u.facing_vector()[1] < -0.99);
    assert!(admitted_shape_u.facing_vector()[2].abs() < 1.0e-12);
    assert!(admitted_shape_v.facing_vector()[0] > 0.99);
    assert!(admitted_shape_v.facing_vector()[1].abs() < 1.0e-12);
    assert!(admitted_shape_v.facing_vector()[2].abs() < 1.0e-12);
    assert!(admitted_shape_w.facing_vector()[1] > 0.70);
    assert!(admitted_shape_w.facing_vector()[2] > 0.70);
    assert!(admitted_frame_axis.facing_vector()[0] < -0.99);
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
    let feature_axis = ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 2.0,
            height: 5.0,
        },
    ))
    .about(SpatialAnchorRef::feature_owned("feature-axis"))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
    .finish_with_catalog(&catalog)
    .expect("feature-owned directional reorient should lower");
    let ambiguity_error = ReorientSpatialIntent::shape(
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

    let admitted_feature_axis =
        admit_spatial_placement(feature_axis.placement_spec()).expect("feature-axis");

    assert!(admitted_feature_axis.facing_vector()[0] < -0.99);
    assert!(admitted_feature_axis.facing_vector()[1].abs() < 1.0e-12);
    assert!(admitted_feature_axis.facing_vector()[2].abs() < 1.0e-12);
    assert_eq!(
        ambiguity_error,
        PrimitiveConstructionSpatialIntentError::PlacementLowering(
            SpatialPlacementMotionError::AmbiguousReorientAnchorMeaning
        )
    );
}
