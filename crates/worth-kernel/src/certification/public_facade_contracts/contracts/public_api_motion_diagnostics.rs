use super::spatial_fixture_witness_catalog::SpatialFixtureWitnessCatalog;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    authoring::{construction::*, intents::*},
    diagnostics::motion::*,
};
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialCarrierPointRole, SpatialPointWitnessRef,
};
use worth_spatial::facade::witness_catalog::{
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
};

#[test]
fn kernel_public_facade_exports_motion_diagnostics_without_certification_bundle_lane() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.motion-diagnostics".to_string(),
    )
    .expect("workspace");
    let intent = MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 6,
    }))
    .to([10.0, 0.0, 3.0]);
    let witness = prepare_primitive_construction_move_witness_resolution_report(intent.clone());
    let dx = prepare_primitive_construction_motion_dx_surface_report(&mut workspace).expect("dx");

    assert_eq!(
        witness.status(),
        PrimitiveConstructionMotionWitnessResolutionStatus::Admitted
    );
    assert!(!dx.rows().is_empty());
}

#[test]
fn kernel_public_facade_exports_catalog_backed_motion_diagnostics_without_certification_bundle_lane(
) {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.motion-diagnostics.catalog".to_string(),
    )
    .expect("workspace");
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-public-motion",
        SpatialCarrierPointRole::Origin,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 5.0, 6.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let intent =
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 4,
        }))
        .so(SpatialAnchorRef::shape_origin())
        .points_toward_witness(SpatialPointWitnessRef::feature_origin(
            "feature-public-motion",
        ));
    let witness =
        prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog(
            intent.clone(),
            &catalog,
        );
    let dx = prepare_primitive_construction_motion_dx_surface_report(&mut workspace).expect("dx");

    assert_eq!(
        witness.kind(),
        PrimitiveConstructionMotionWitnessResolutionKind::PointsToward
    );
    assert!(!dx.rows().is_empty());
}
