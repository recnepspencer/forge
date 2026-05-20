use super::{
    EmptySpatialWitnessCatalog, SpatialCatalogResolvedDirectionWitness,
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
    SpatialFixtureWitnessCatalog, SpatialWitnessCatalog,
};
use crate::facade::{
    SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
    SpatialWitnessFailureClass,
};

#[test]
fn empty_catalog_rejects_advanced_carrier_witnesses_as_unsupported() {
    let catalog = EmptySpatialWitnessCatalog;

    assert_eq!(
        catalog.resolve_parameter_space_direction(
            SpatialCarrierKind::Curve,
            "curve-1",
            [0.25, 0.0],
            SpatialCarrierDirectionRole::Tangent,
        ),
        Err(SpatialWitnessFailureClass::Unsupported)
    );
    assert_eq!(
        catalog.resolve_feature_owned_point("feature-1", SpatialCarrierPointRole::Origin),
        Err(SpatialWitnessFailureClass::Unsupported)
    );
}

#[test]
fn fixture_catalog_preserves_resolved_and_failure_outcomes() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_parameter_space_direction(
            SpatialCarrierKind::Curve,
            "curve-1",
            [0.25, 0.0],
            SpatialCarrierDirectionRole::Tangent,
            Ok(SpatialCatalogResolvedDirectionWitness::new(
                [1.0, 0.0, 0.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        )
        .with_feature_owned_point(
            "feature-1",
            SpatialCarrierPointRole::Origin,
            Err(SpatialWitnessFailureClass::Undefined),
        )
        .with_parameter_space_point(
            SpatialCarrierKind::Surface,
            "surface-1",
            [0.5, 0.25],
            Ok(SpatialCatalogResolvedPointWitness::new(
                [2.0, 3.0, 4.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        );

    assert_eq!(
        catalog
            .resolve_parameter_space_direction(
                SpatialCarrierKind::Curve,
                "curve-1",
                [0.25, 0.0],
                SpatialCarrierDirectionRole::Tangent,
            )
            .expect("direction")
            .world_direction(),
        [1.0, 0.0, 0.0]
    );
    assert_eq!(
        catalog.resolve_feature_owned_point("feature-1", SpatialCarrierPointRole::Origin),
        Err(SpatialWitnessFailureClass::Undefined)
    );
    assert_eq!(
        catalog
            .resolve_parameter_space_point(SpatialCarrierKind::Surface, "surface-1", [0.5, 0.25],)
            .expect("point")
            .resolution_class(),
        SpatialCatalogWitnessResolutionClass::FallbackDerived
    );
}
