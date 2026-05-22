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
    SpatialAnchorRef, SpatialAxis, SpatialCarrierDirectionRole, SpatialCarrierPointRole,
    SpatialCatalogResolvedDirectionWitness, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass, SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog,
    SpatialPlacementMotionError, SpatialPointWitnessRef, SpatialWitnessFailureClass,
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
        .from(SpatialAnchorRef::shape_axis(SpatialAxis::W))
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
fn motion_branch_preview_runtime_reports_cover_external_pivot_rotate_and_points_toward_paths() {
    let mut rotate_workspace = workspace("worth-kernel.motion-runtime.rotate");
    let rotate = prepare_primitive_construction_rotate_branch_preview_runtime_report(
        &mut rotate_workspace,
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .about(SpatialAnchorRef::frame_origin(
            worth_spatial::facade::SpatialFrameRef::workplane(
                "pivot",
                [4.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
            ),
        ))
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
            .so(SpatialAnchorRef::world_origin())
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

#[test]
fn motion_branch_preview_runtime_reports_cover_feature_owned_anchor_paths_with_catalog() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-anchor",
        SpatialCarrierPointRole::Anchor,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 0.0, 0.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let mut rotate_workspace = workspace("worth-kernel.motion-runtime.feature-anchor.rotate");
    let rotate =
        super::prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog(
            &mut rotate_workspace,
            RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                edge_count: 6,
            }))
            .about(SpatialAnchorRef::feature_owned("feature-anchor"))
            .around([0.0, 0.0, 1.0])
            .by_radians(0.5),
            &catalog,
        )
        .expect("rotate runtime report");
    let mut points_workspace = workspace("worth-kernel.motion-runtime.feature-anchor.points");
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
            .so(SpatialAnchorRef::feature_owned("feature-anchor"))
            .points_toward([4.0, 3.0, 4.0]),
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

#[test]
fn motion_branch_preview_runtime_reports_cover_feature_owned_move_anchor_paths_with_catalog() {
    let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
        "feature-anchor",
        SpatialCarrierPointRole::Anchor,
        Ok(SpatialCatalogResolvedPointWitness::new(
            [4.0, 1.0, 0.0],
            SpatialCatalogWitnessResolutionClass::FallbackDerived,
        )),
    );
    let mut workspace = workspace("worth-kernel.motion-runtime.feature-anchor.move");
    let report =
        super::prepare_primitive_construction_move_branch_preview_runtime_report_with_catalog(
            &mut workspace,
            MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                edge_count: 6,
            }))
            .from(SpatialAnchorRef::feature_owned("feature-anchor"))
            .to([10.0, 0.0, 3.0]),
            &catalog,
        )
        .expect("move runtime report");

    assert_eq!(
        report.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(report.runtime_report().is_some());
}

#[test]
fn motion_branch_preview_runtime_reports_cover_external_reference_translation_paths() {
    let mut workspace = workspace("worth-kernel.motion-runtime.external-reference");
    let moved = prepare_primitive_construction_move_branch_preview_runtime_report(
        &mut workspace,
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .from(SpatialAnchorRef::world_origin())
        .to([10.0, 0.0, 3.0]),
    )
    .expect("move runtime report");

    assert_eq!(
        moved.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert!(moved.runtime_report().is_some());
}

#[test]
fn motion_branch_preview_runtime_reports_preserve_feature_owned_anchor_witness_failure_truth() {
    let mut workspace = workspace("worth-kernel.motion-runtime.feature-anchor.failure");
    let report =
        super::prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog(
            &mut workspace,
            RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                edge_count: 6,
            }))
            .about(SpatialAnchorRef::feature_owned("feature-anchor"))
            .around([0.0, 0.0, 1.0])
            .by_radians(0.5),
            &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
                "feature-anchor",
                SpatialCarrierPointRole::Anchor,
                Err(SpatialWitnessFailureClass::Exhausted),
            ),
        )
        .expect("rotate runtime report");

    assert_eq!(
        report.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::PlacementLoweringBlocked(
            SpatialPlacementMotionError::AnchorWitnessFailure(
                SpatialWitnessFailureClass::Exhausted
            )
        )
    );
    assert!(report.runtime_report().is_none());
}

#[test]
fn motion_branch_preview_runtime_reports_preserve_directional_anchor_ambiguity_truth() {
    let mut workspace = workspace("worth-kernel.motion-runtime.directional.ambiguity");
    let report =
        super::prepare_primitive_construction_reorient_branch_preview_runtime_report_with_catalog(
            &mut workspace,
            ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                RegularPyramidSpec {
                    sides: 4,
                    radius: 1.0,
                    height: 2.0,
                },
            ))
            .about(SpatialAnchorRef::feature_owned("feature-axis"))
            .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0])),
            &SpatialFixtureWitnessCatalog::new()
                .with_feature_owned_point(
                    "feature-axis",
                    SpatialCarrierPointRole::Anchor,
                    Ok(SpatialCatalogResolvedPointWitness::new(
                        [0.0, 0.0, 0.0],
                        SpatialCatalogWitnessResolutionClass::FallbackDerived,
                    )),
                )
                .with_feature_owned_direction(
                    "feature-axis",
                    SpatialCarrierDirectionRole::Axis,
                    Ok(SpatialCatalogResolvedDirectionWitness::new(
                        [1.0, 0.0, 0.0],
                        SpatialCatalogWitnessResolutionClass::CarrierDerived,
                    )),
                ),
        )
        .expect("runtime report");

    assert_eq!(
        report.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::PlacementLoweringBlocked(
            SpatialPlacementMotionError::AmbiguousReorientAnchorMeaning
        )
    );
    assert!(report.runtime_report().is_none());
}
