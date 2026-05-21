use super::{
    EmptySpatialWitnessCatalog, SpatialCatalogResolvedDirectionWitness,
    SpatialCatalogResolvedGeometricTag, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass, SpatialWitnessCatalog,
};
use crate::facade::{
    SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
    SpatialWitnessFailureClass,
};
use crate::test_support::SpatialFixtureWitnessCatalog;
use worth_geom::ParameterSpacePoint;

#[test]
fn empty_catalog_rejects_advanced_carrier_witnesses_as_unsupported() {
    let catalog = EmptySpatialWitnessCatalog;

    assert_eq!(
        catalog.resolve_parameter_space_direction(
            SpatialCarrierKind::Curve,
            "curve-1",
            ParameterSpacePoint::try_new([0.25, 0.0]).unwrap(),
            SpatialCarrierDirectionRole::Tangent,
        ),
        Err(SpatialWitnessFailureClass::Unsupported)
    );
    assert_eq!(
        catalog.resolve_feature_owned_point("feature-1", SpatialCarrierPointRole::Origin),
        Err(SpatialWitnessFailureClass::Unsupported)
    );
    assert_eq!(
        catalog.resolve_geometric_tag("tag-1"),
        Err(SpatialWitnessFailureClass::Unsupported)
    );
}

#[test]
fn fixture_catalog_preserves_resolved_and_failure_outcomes() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_parameter_space_direction(
            SpatialCarrierKind::Curve,
            "curve-1",
            ParameterSpacePoint::try_new([0.25, 0.0]).unwrap(),
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
        .with_geometric_tag_point(
            "tag-1",
            Ok(SpatialCatalogResolvedPointWitness::new(
                [9.0, 8.0, 7.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        )
        .with_parameter_space_point(
            SpatialCarrierKind::Surface,
            "surface-1",
            ParameterSpacePoint::try_new([0.5, 0.25]).unwrap(),
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
                ParameterSpacePoint::try_new([0.25, 0.0]).unwrap(),
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
        catalog.resolve_geometric_tag("tag-1").expect("tag point"),
        SpatialCatalogResolvedGeometricTag::PointLike(SpatialCatalogResolvedPointWitness::new(
            [9.0, 8.0, 7.0],
            SpatialCatalogWitnessResolutionClass::CarrierDerived,
        ))
    );
    assert_eq!(
        catalog
            .resolve_parameter_space_point(
                SpatialCarrierKind::Surface,
                "surface-1",
                ParameterSpacePoint::try_new([0.5, 0.25]).unwrap(),
            )
            .expect("point")
            .resolution_class(),
        SpatialCatalogWitnessResolutionClass::FallbackDerived
    );
}

#[test]
fn fixture_catalog_distinguishes_point_direction_and_unsupported_tag_meaning() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_geometric_tag_direction(
            "tag-direction",
            Ok(SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 1.0, 0.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        )
        .with_geometric_tag_unsupported_class("tag-unsupported");

    assert_eq!(
        catalog
            .resolve_geometric_tag("tag-direction")
            .expect("tag direction"),
        SpatialCatalogResolvedGeometricTag::DirectionLike(
            SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 1.0, 0.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            ),
        )
    );
    assert_eq!(
        catalog
            .resolve_geometric_tag("tag-unsupported")
            .expect("unsupported class"),
        SpatialCatalogResolvedGeometricTag::UnsupportedClass
    );
}
