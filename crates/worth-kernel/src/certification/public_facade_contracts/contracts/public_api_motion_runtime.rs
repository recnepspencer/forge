use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    authoring::{construction::*, intents::*},
    diagnostics::motion::*,
};
use worth_spatial::facade::{
    SpatialAnchorRef, SpatialAxis, SpatialCarrierPointRole, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass, SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog,
    SpatialFrameRef, SpatialPointWitnessRef, SpatialWitnessFailureClass,
};

#[test]
fn kernel_public_facade_exports_motion_replay_parity_reports() {
    let move_report = prepare_primitive_construction_move_replay_parity_report(
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .to([10.0, 0.0, 3.0]),
    );
    let rotate_report = prepare_primitive_construction_rotate_replay_parity_report(
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .around([0.0, 0.0, 1.0])
        .by_radians(0.5),
    );
    let reorient_report = prepare_primitive_construction_reorient_replay_parity_report(
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 4,
                radius: 1.0,
                height: 2.0,
            },
        ))
        .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0])),
    );
    let points_report = prepare_primitive_construction_points_toward_replay_parity_report(
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 1.0,
            },
        ))
        .so(SpatialAnchorRef::shape_origin())
        .points_toward([1.0, 2.0, 3.0]),
    );

    for report in [
        &move_report,
        &rotate_report,
        &reorient_report,
        &points_report,
    ] {
        assert!(report.parity_verified());
        assert_eq!(report.direct_report(), report.replay_report());
        assert_ne!(
            report.report_digest(),
            report.direct_report().report_digest()
        );
        assert_ne!(
            report.report_digest(),
            report.replay_report().report_digest()
        );
    }
}

#[test]
fn kernel_public_facade_exports_motion_branch_preview_runtime_reports() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.motion-runtime".to_string(),
    )
    .expect("workspace");
    let available = prepare_primitive_construction_move_branch_preview_runtime_report(
        &mut workspace,
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .to([10.0, 0.0, 3.0]),
    )
    .expect("available runtime report");
    let blocked = prepare_primitive_construction_move_branch_preview_runtime_report(
        &mut workspace,
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .from(SpatialAnchorRef::shape_axis(SpatialAxis::W))
        .to([10.0, 0.0, 3.0]),
    )
    .expect("blocked runtime report");
    let rotate = prepare_primitive_construction_rotate_branch_preview_runtime_report(
        &mut workspace,
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .about(SpatialAnchorRef::frame_origin(SpatialFrameRef::workplane(
            "runtime-pivot",
            [4.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
        )))
        .around([0.0, 0.0, 1.0])
        .by_radians(0.5),
    )
    .expect("rotate runtime report");
    let reorient = prepare_primitive_construction_reorient_branch_preview_runtime_report(
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
    .expect("reorient runtime report");
    let points = prepare_primitive_construction_points_toward_branch_preview_runtime_report(
        &mut workspace,
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 1.0,
            },
        ))
        .so(SpatialAnchorRef::world_origin())
        .points_toward([1.0, 2.0, 3.0]),
    )
    .expect("points runtime report");
    let external_move = prepare_primitive_construction_move_branch_preview_runtime_report(
        &mut workspace,
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .from(SpatialAnchorRef::world_origin())
        .to([10.0, 0.0, 3.0]),
    )
    .expect("external move runtime report");

    assert_eq!(
        available.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(available.runtime_report().is_some());
    assert_eq!(
        blocked.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::PlacementLoweringBlocked(
            worth_spatial::facade::SpatialPlacementMotionError::UnsupportedMoveAnchor
        )
    );
    assert!(blocked.runtime_report().is_none());
    assert_eq!(
        rotate.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(rotate.runtime_report().is_some());
    assert_eq!(
        reorient.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(reorient.runtime_report().is_some());
    assert_eq!(
        points.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(points.runtime_report().is_some());
    assert_eq!(
        external_move.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(external_move.runtime_report().is_some());
}

#[test]
fn kernel_public_facade_exports_catalog_backed_motion_runtime_reports() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.motion-runtime.catalog".to_string(),
    )
    .expect("workspace");
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-public-runtime",
        SpatialCarrierPointRole::Origin,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [2.0, 3.0, 4.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let report = prepare_primitive_construction_move_branch_preview_runtime_report_with_catalog(
        &mut workspace,
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .to_witness(SpatialPointWitnessRef::feature_origin(
            "feature-public-runtime",
        )),
        &catalog,
    )
    .expect("catalog runtime report");

    assert_eq!(
        report.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(report.runtime_report().is_some());
}

#[test]
fn kernel_public_facade_preserves_feature_owned_anchor_witness_failure_in_motion_runtime() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.motion-runtime.anchor-failure".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_rotate_branch_preview_runtime_report(
        &mut workspace,
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .about(SpatialAnchorRef::feature_owned("feature-public-runtime"))
        .around([0.0, 0.0, 1.0])
        .by_radians(0.5),
    )
    .expect("runtime report");
    let blocked = prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog(
        &mut workspace,
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .about(SpatialAnchorRef::feature_owned("feature-public-runtime"))
        .around([0.0, 0.0, 1.0])
        .by_radians(0.5),
        &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
            "feature-public-runtime",
            SpatialCarrierPointRole::Anchor,
            Err(SpatialWitnessFailureClass::Undefined),
        ),
    )
    .expect("catalog runtime report");

    assert_eq!(
        report.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::PlacementLoweringBlocked(
            worth_spatial::facade::SpatialPlacementMotionError::AnchorWitnessFailure(
                SpatialWitnessFailureClass::Unsupported
            )
        )
    );
    assert!(report.runtime_report().is_none());
    assert_eq!(
        blocked.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::PlacementLoweringBlocked(
            worth_spatial::facade::SpatialPlacementMotionError::AnchorWitnessFailure(
                SpatialWitnessFailureClass::Undefined
            )
        )
    );
    assert!(blocked.runtime_report().is_none());
}
