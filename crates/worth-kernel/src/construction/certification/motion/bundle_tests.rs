use super::{
    prepare_primitive_construction_move_motion_report_bundle,
    prepare_primitive_construction_points_toward_motion_report_bundle_with_catalog,
    prepare_primitive_construction_reorient_motion_report_bundle,
};
use crate::construction::{
    PrimitiveConstructionIntent, PrimitiveConstructionMotionRuntimeSurfaceStatus,
    PrimitiveConstructionMotionWitnessResolutionStatus, RegularPyramidSpec, WireBodySpec,
};
use crate::facade::{MoveSpatialIntent, ReorientSpatialIntent};
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_spatial::facade::{
    SpatialAnchorRef, SpatialCarrierPointRole, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass, SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog,
    SpatialPointWitnessRef, SpatialWitnessResolutionClass,
};

fn workspace(name: &str) -> forge_query::facade::ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        name.to_string(),
    )
    .expect("workspace")
}

#[test]
fn motion_report_bundle_binds_direct_replay_query_and_runtime_truth() {
    let mut workspace = workspace("worth-kernel.motion-bundle.direct");
    let bundle = prepare_primitive_construction_reorient_motion_report_bundle(
        &mut workspace,
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 4,
                radius: 1.0,
                height: 2.0,
            },
        ))
        .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0])),
    )
    .expect("bundle");

    assert!(bundle.bundle_verified());
    assert_eq!(
        bundle.witness_report().status(),
        PrimitiveConstructionMotionWitnessResolutionStatus::Admitted
    );
    assert!(bundle.replay_parity_report().parity_verified());
    assert!(bundle.query_inspection_parity_report().parity_verified());
    assert!(bundle.query_projection_receipt_report().parity_verified());
    assert_eq!(
        bundle
            .branch_preview_runtime_report()
            .runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
}

#[test]
fn motion_report_bundle_preserves_rejected_and_lowering_blocked_truth() {
    let mut rejected_workspace = workspace("worth-kernel.motion-bundle.rejected");
    let rejected = prepare_primitive_construction_reorient_motion_report_bundle(
        &mut rejected_workspace,
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 4,
                radius: 1.0,
                height: 2.0,
            },
        ))
        .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 0.0])),
    )
    .expect("rejected bundle");
    let mut blocked_workspace = workspace("worth-kernel.motion-bundle.blocked");
    let blocked = prepare_primitive_construction_move_motion_report_bundle(
        &mut blocked_workspace,
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .from(SpatialAnchorRef::world_origin())
        .to([10.0, 0.0, 3.0]),
    )
    .expect("blocked bundle");

    assert!(rejected.bundle_verified());
    assert_eq!(
        rejected.witness_report().status(),
        PrimitiveConstructionMotionWitnessResolutionStatus::Rejected
    );
    assert_eq!(
        rejected
            .branch_preview_runtime_report()
            .runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::MotionRejected
    );
    assert!(blocked.bundle_verified());
    assert_eq!(
        blocked
            .branch_preview_runtime_report()
            .runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::PlacementLoweringBlocked(
            worth_spatial::facade::SpatialPlacementMotionError::UnsupportedMoveAnchor
        )
    );
}

#[test]
fn motion_report_bundle_supports_catalog_backed_point_witnesses() {
    let mut workspace = workspace("worth-kernel.motion-bundle.catalog");
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-bundle",
        SpatialCarrierPointRole::Origin,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 5.0, 6.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let bundle = prepare_primitive_construction_points_toward_motion_report_bundle_with_catalog(
        &mut workspace,
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 1.0,
            },
        ))
        .so(SpatialAnchorRef::shape_origin())
        .points_toward_witness(SpatialPointWitnessRef::feature_origin("feature-bundle")),
        &catalog,
    )
    .expect("catalog bundle");

    assert!(bundle.bundle_verified());
    assert_eq!(
        bundle.witness_report().resolution_class(),
        Some(SpatialWitnessResolutionClass::FallbackDerived)
    );
    assert_eq!(
        bundle
            .branch_preview_runtime_report()
            .runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
}
