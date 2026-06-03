use super::spatial_fixture_witness_catalog::SpatialFixtureWitnessCatalog;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    authoring::{construction::*, intents::*},
    certification::motion::*,
    diagnostics::motion::*,
};
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialCarrierPointRole, SpatialPointWitnessRef,
};
use worth_spatial::facade::witness_catalog::{
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
};

#[test]
fn kernel_public_facade_exports_motion_report_bundle() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.motion-bundle".to_string(),
    )
    .expect("workspace");
    let bundle = prepare_primitive_construction_move_motion_report_bundle(
        &mut workspace,
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .to([10.0, 0.0, 3.0]),
    )
    .expect("bundle");

    assert_eq!(
        bundle.truth(),
        &PrimitiveConstructionMotionCanonicalTruth::from_witness_report(bundle.witness_report())
    );
    assert!(bundle.replay_parity_report().parity_verified());
    assert!(bundle.query_inspection_parity_report().parity_verified());
    assert_eq!(
        bundle
            .branch_preview_runtime_report()
            .runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
}

#[test]
fn kernel_public_facade_exports_catalog_backed_motion_report_bundle() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.motion-bundle.catalog".to_string(),
    )
    .expect("workspace");
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-public-bundle",
        SpatialCarrierPointRole::Origin,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 5.0, 6.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let bundle = prepare_primitive_construction_points_toward_motion_report_bundle_with_catalog(
        &mut workspace,
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 4,
        }))
        .so(SpatialAnchorRef::shape_origin())
        .points_toward_witness(SpatialPointWitnessRef::feature_origin(
            "feature-public-bundle",
        )),
        &catalog,
    )
    .expect("catalog bundle");

    assert_eq!(
        bundle.truth(),
        &PrimitiveConstructionMotionCanonicalTruth::from_witness_report(bundle.witness_report())
    );
    assert_eq!(
        bundle
            .branch_preview_runtime_report()
            .runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
}
