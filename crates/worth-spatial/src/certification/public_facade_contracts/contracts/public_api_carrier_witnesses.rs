use worth_geom::ParameterSpacePoint;
use worth_spatial::facade::{
    constraints::{
        admit_spatial_points_toward_constraint_with_catalog,
        apply_admitted_points_toward_constraint_to_placement_with_catalog,
        SpatialPointsTowardConstraintSpec,
    },
    motion::{admit_spatial_reorient_with_catalog, SpatialReorientSpec},
    placement::{admit_spatial_placement_with_catalog, SpatialPlacementSpec},
    refs, witness_catalog, witness_resolution,
};

use crate::spatial_fixture_witness_catalog::SpatialFixtureWitnessCatalog;

#[test]
fn spatial_public_facade_exports_catalog_backed_carrier_witness_admission() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_parameter_space_direction(
            refs::SpatialCarrierKind::Surface,
            "surface-2",
            ParameterSpacePoint::try_new([0.5, 0.25]).unwrap(),
            refs::SpatialCarrierDirectionRole::Normal,
            Ok(
                witness_catalog::SpatialCatalogResolvedDirectionWitness::new(
                    [0.0, 0.0, 2.0],
                    witness_catalog::SpatialCatalogWitnessResolutionClass::CarrierDerived,
                ),
            ),
        )
        .with_feature_owned_point(
            "feature-1",
            refs::SpatialCarrierPointRole::Origin,
            Ok(witness_catalog::SpatialCatalogResolvedPointWitness::new(
                [2.0, 3.0, 4.0],
                witness_catalog::SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        )
        .with_feature_owned_point(
            "feature-1",
            refs::SpatialCarrierPointRole::Anchor,
            Ok(witness_catalog::SpatialCatalogResolvedPointWitness::new(
                [2.0, 3.0, 4.0],
                witness_catalog::SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        );
    let placement = admit_spatial_placement_with_catalog(
        SpatialPlacementSpec::world().facing_witness(
            refs::SpatialDirectionWitnessRef::surface_normal("surface-2", 0.5, 0.25),
        ),
        &catalog,
    )
    .expect("placement");
    let toward = admit_spatial_points_toward_constraint_with_catalog(
        SpatialPointsTowardConstraintSpec::with_witness(
            refs::SpatialAnchorRef::shape_origin(),
            refs::SpatialPointWitnessRef::feature_origin("feature-1"),
        ),
        &catalog,
    )
    .expect("points toward");

    assert_eq!(
        placement.resolved_direction_witness().resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::CarrierDerived
    );
    assert_eq!(placement.facing_vector(), [0.0, 0.0, 1.0]);
    assert_eq!(
        toward.resolved_target_witness().resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::FallbackDerived
    );
    assert_eq!(toward.target_point(), [2.0, 3.0, 4.0]);

    let feature_anchor_pointed = apply_admitted_points_toward_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([0.0, 0.0, 0.0]),
        &admit_spatial_points_toward_constraint_with_catalog(
            SpatialPointsTowardConstraintSpec::new(
                refs::SpatialAnchorRef::feature_owned("feature-1"),
                [2.0, 5.0, 4.0],
            ),
            &catalog,
        )
        .expect("feature-anchor points-toward"),
        &catalog,
    )
    .expect("feature-anchor lowered");
    let admitted_feature_anchor_pointed =
        admit_spatial_placement_with_catalog(feature_anchor_pointed, &catalog)
            .expect("feature-anchor admitted placement");

    assert!(admitted_feature_anchor_pointed.facing_vector()[1] > 0.83);
}

#[test]
fn spatial_public_facade_preserves_catalog_backed_undefined_and_exhausted_truth() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_parameter_space_direction(
            refs::SpatialCarrierKind::Curve,
            "curve-3",
            ParameterSpacePoint::try_new([0.75, 0.0]).unwrap(),
            refs::SpatialCarrierDirectionRole::Tangent,
            Err(witness_resolution::SpatialWitnessFailureClass::Exhausted),
        )
        .with_feature_owned_direction(
            "feature-2",
            refs::SpatialCarrierDirectionRole::Axis,
            Err(witness_resolution::SpatialWitnessFailureClass::Undefined),
        );
    let exhausted = admit_spatial_reorient_with_catalog(
        SpatialReorientSpec::shape_origin().toward_witness(
            refs::SpatialDirectionWitnessRef::curve_tangent("curve-3", 0.75),
        ),
        &catalog,
    )
    .expect_err("exhausted");
    let undefined = admit_spatial_reorient_with_catalog(
        SpatialReorientSpec::shape_origin()
            .toward_witness(refs::SpatialDirectionWitnessRef::feature_axis("feature-2")),
        &catalog,
    )
    .expect_err("undefined");

    assert_eq!(
        exhausted,
        worth_spatial::facade::motion::SpatialMotionError::DirectionWitnessFailure(
            witness_resolution::SpatialWitnessFailureClass::Exhausted
        )
    );
    assert_eq!(
        undefined,
        worth_spatial::facade::motion::SpatialMotionError::DirectionWitnessFailure(
            witness_resolution::SpatialWitnessFailureClass::Undefined
        )
    );
}
