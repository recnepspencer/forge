use worth_spatial::facade::{
    admit_spatial_placement_with_catalog, admit_spatial_points_toward_constraint_with_catalog,
    admit_spatial_reorient_with_catalog,
    apply_admitted_points_toward_constraint_to_placement_with_catalog, SpatialAnchorRef,
    SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
    SpatialCatalogResolvedDirectionWitness, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass, SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog,
    SpatialPlacementSpec, SpatialPointWitnessRef, SpatialPointsTowardConstraintSpec,
    SpatialReorientSpec, SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

#[test]
fn spatial_public_facade_exports_catalog_backed_carrier_witness_admission() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_parameter_space_direction(
            SpatialCarrierKind::Surface,
            "surface-2",
            [0.5, 0.25],
            SpatialCarrierDirectionRole::Normal,
            Ok(SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 0.0, 2.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        )
        .with_feature_owned_point(
            "feature-1",
            SpatialCarrierPointRole::Origin,
            Ok(SpatialCatalogResolvedPointWitness::new(
                [2.0, 3.0, 4.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        )
        .with_feature_owned_point(
            "feature-1",
            SpatialCarrierPointRole::Anchor,
            Ok(SpatialCatalogResolvedPointWitness::new(
                [2.0, 3.0, 4.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        );
    let placement =
        admit_spatial_placement_with_catalog(
            SpatialPlacementSpec::world().facing_witness(
                SpatialDirectionWitnessRef::surface_normal("surface-2", 0.5, 0.25),
            ),
            &catalog,
        )
        .expect("placement");
    let toward = admit_spatial_points_toward_constraint_with_catalog(
        SpatialPointsTowardConstraintSpec::with_witness(
            SpatialAnchorRef::shape_origin(),
            SpatialPointWitnessRef::feature_origin("feature-1"),
        ),
        &catalog,
    )
    .expect("points toward");

    assert_eq!(
        placement.resolved_direction_witness().resolution_class(),
        SpatialWitnessResolutionClass::CarrierDerived
    );
    assert_eq!(placement.facing_vector(), [0.0, 0.0, 1.0]);
    assert_eq!(
        toward.resolved_target_witness().resolution_class(),
        SpatialWitnessResolutionClass::FallbackDerived
    );
    assert_eq!(toward.target_point(), [2.0, 3.0, 4.0]);

    let feature_anchor_pointed = apply_admitted_points_toward_constraint_to_placement_with_catalog(
        SpatialPlacementSpec::world().at([0.0, 0.0, 0.0]),
        &admit_spatial_points_toward_constraint_with_catalog(
            SpatialPointsTowardConstraintSpec::new(
                SpatialAnchorRef::feature_owned("feature-1"),
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
            SpatialCarrierKind::Curve,
            "curve-3",
            [0.75, 0.0],
            SpatialCarrierDirectionRole::Tangent,
            Err(SpatialWitnessFailureClass::Exhausted),
        )
        .with_feature_owned_direction(
            "feature-2",
            SpatialCarrierDirectionRole::Axis,
            Err(SpatialWitnessFailureClass::Undefined),
        );
    let exhausted = admit_spatial_reorient_with_catalog(
        SpatialReorientSpec::shape_origin()
            .toward_witness(SpatialDirectionWitnessRef::curve_tangent("curve-3", 0.75)),
        &catalog,
    )
    .expect_err("exhausted");
    let undefined = admit_spatial_reorient_with_catalog(
        SpatialReorientSpec::shape_origin()
            .toward_witness(SpatialDirectionWitnessRef::feature_axis("feature-2")),
        &catalog,
    )
    .expect_err("undefined");

    assert_eq!(
        exhausted,
        worth_spatial::facade::SpatialMotionError::DirectionWitnessFailure(
            SpatialWitnessFailureClass::Exhausted
        )
    );
    assert_eq!(
        undefined,
        worth_spatial::facade::SpatialMotionError::DirectionWitnessFailure(
            SpatialWitnessFailureClass::Undefined
        )
    );
}
