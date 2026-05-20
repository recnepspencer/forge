use super::{
    prepare_primitive_construction_move_branch_preview_runtime_report,
    prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_reorient_branch_preview_runtime_report,
    prepare_primitive_construction_rotate_branch_preview_runtime_report,
    PrimitiveConstructionMotionRuntimeSurfaceStatus,
};
use crate::construction::{
    PrimitiveConstructionIntent, PrimitiveConstructionMotionWitnessResolutionStatus,
    RegularPyramidSpec, WireBodySpec,
};
use crate::facade::{MoveSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent};
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_spatial::facade::{
    SpatialAnchorRef, SpatialCarrierPointRole, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass, SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog,
    SpatialPlacementMotionError, SpatialPointWitnessRef,
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
fn motion_branch_preview_runtime_reports_preserve_available_runtime_truth() {
    let mut workspace = workspace("worth-kernel.motion-runtime.available");
    let report = prepare_primitive_construction_reorient_branch_preview_runtime_report(
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
    .expect("branch preview runtime report");

    assert_eq!(
        report.motion_status(),
        PrimitiveConstructionMotionWitnessResolutionStatus::Admitted
    );
    assert_eq!(
        report.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(report.runtime_report().is_some());
}

#[test]
fn motion_branch_preview_runtime_reports_preserve_rejected_motion_truth() {
    let mut workspace = workspace("worth-kernel.motion-runtime.rejected");
    let report = prepare_primitive_construction_reorient_branch_preview_runtime_report(
        &mut workspace,
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 4,
                radius: 1.0,
                height: 2.0,
            },
        ))
        .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 0.0])),
    )
    .expect("branch preview runtime report");

    assert_eq!(
        report.motion_status(),
        PrimitiveConstructionMotionWitnessResolutionStatus::Rejected
    );
    assert_eq!(
        report.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::MotionRejected
    );
    assert!(report.runtime_report().is_none());
}

#[test]
fn motion_branch_preview_runtime_reports_preserve_lowering_block_truth() {
    let mut workspace = workspace("worth-kernel.motion-runtime.lowering");
    let report = prepare_primitive_construction_move_branch_preview_runtime_report(
        &mut workspace,
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .from(SpatialAnchorRef::world_origin())
        .to([10.0, 0.0, 3.0]),
    )
    .expect("branch preview runtime report");

    assert_eq!(
        report.motion_status(),
        PrimitiveConstructionMotionWitnessResolutionStatus::Admitted
    );
    assert_eq!(
        report.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::PlacementLoweringBlocked(
            SpatialPlacementMotionError::UnsupportedMoveAnchor
        )
    );
    assert!(report.runtime_report().is_none());
}

#[test]
fn motion_branch_preview_runtime_reports_cover_rotate_and_catalog_backed_points_toward_paths() {
    let mut rotate_workspace = workspace("worth-kernel.motion-runtime.rotate");
    let rotate = prepare_primitive_construction_rotate_branch_preview_runtime_report(
        &mut rotate_workspace,
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .around([0.0, 0.0, 1.0])
        .by_radians(0.5),
    )
    .expect("rotate runtime report");
    let mut points_workspace = workspace("worth-kernel.motion-runtime.points");
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-runtime",
        SpatialCarrierPointRole::Origin,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [2.0, 3.0, 4.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let points =
        prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog(
            &mut points_workspace,
            ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                RegularPyramidSpec {
                    sides: 3,
                    radius: 1.0,
                    height: 1.0,
                },
            ))
            .so(SpatialAnchorRef::shape_origin())
            .points_toward_witness(SpatialPointWitnessRef::feature_origin("feature-runtime")),
            &catalog,
        )
        .expect("points runtime report");

    assert_eq!(
        rotate.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(rotate.runtime_report().is_some());
    assert_eq!(
        points.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(points.runtime_report().is_some());
}
