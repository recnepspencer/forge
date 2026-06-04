use super::spatial_fixture_witness_catalog::SpatialFixtureWitnessCatalog;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::ParameterSpacePoint;
use worth_kernel::facade::{
    authoring::{construction::*, create::CreateSpatialIntent, intents::*},
    diagnostics::motion::*,
};
use worth_spatial::facade::placement::admit_spatial_placement_with_catalog;
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
    SpatialDirectionWitnessRef,
};
use worth_spatial::facade::witness_catalog::{
    SpatialCatalogResolvedDirectionWitness, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass,
};
use worth_spatial::facade::witness_resolution::SpatialWitnessResolutionClass;

#[test]
fn kernel_public_facade_exports_catalog_backed_carrier_motion_and_placement_surfaces() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_parameter_space_direction(
            SpatialCarrierKind::Curve,
            "curve-4",
            ParameterSpacePoint::try_new([0.5, 0.0]).unwrap(),
            SpatialCarrierDirectionRole::Tangent,
            Ok(SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 2.0, 0.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        )
        .with_parameter_space_direction(
            SpatialCarrierKind::Surface,
            "surface-4",
            ParameterSpacePoint::try_new([0.25, 0.75]).unwrap(),
            SpatialCarrierDirectionRole::Normal,
            Ok(SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 0.0, 3.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        )
        .with_feature_owned_point(
            "feature-anchor",
            SpatialCarrierPointRole::Anchor,
            Ok(SpatialCatalogResolvedPointWitness::new(
                [4.0, 0.0, 0.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        );
    let report = prepare_primitive_construction_reorient_witness_resolution_report_with_catalog(
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 4,
                radius: 1.0,
                height: 2.0,
            },
        ))
        .toward_witness(SpatialDirectionWitnessRef::curve_tangent("curve-4", 0.5)),
        &catalog,
    );
    let placed = CreateSpatialIntent::new(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 4,
    }))
    .finish();
    let admitted =
        admit_spatial_placement_with_catalog(
            placed.placement_spec().clone().facing_witness(
                SpatialDirectionWitnessRef::surface_normal("surface-4", 0.25, 0.75),
            ),
            &catalog,
        )
        .expect("placement");
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api-carrier-motion".to_string(),
    )
    .expect("workspace");
    let mut session = primitive_construction_authoring(&mut workspace).expect("authoring session");
    let rotated = session
        .author_with_catalog(
            RotateSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                RegularPyramidSpec {
                    sides: 4,
                    radius: 1.0,
                    height: 2.0,
                },
            ))
            .about(SpatialAnchorRef::feature_owned("feature-anchor"))
            .around([0.0, 0.0, 1.0])
            .by_radians(std::f64::consts::FRAC_PI_2),
            &catalog,
        )
        .and_then(|entry| entry.prepare_result());

    assert_eq!(
        report.resolution_class(),
        Some(SpatialWitnessResolutionClass::CarrierDerived)
    );
    assert_eq!(report.resolved_world_direction(), Some([0.0, 1.0, 0.0]));
    assert_eq!(
        admitted.resolved_direction_witness().resolution_class(),
        SpatialWitnessResolutionClass::FallbackDerived
    );
    assert_eq!(admitted.facing_vector(), [0.0, 0.0, 1.0]);
    assert!(rotated.is_ok());
}
