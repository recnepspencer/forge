use super::{
    prepare_primitive_construction_reorient_branch_preview_runtime_report,
    PrimitiveConstructionMotionRuntimeSurfaceStatus,
};
use crate::construction::runtime_basis::prepare_primitive_construction_branch_preview_runtime_report;
use crate::construction::{PrimitiveConstructionIntent, RegularPyramidSpec};
use crate::facade::authoring::intents::ReorientSpatialIntent;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_spatial::facade::refs::{SpatialAnchorRef, SpatialAxis, SpatialDirectionWitnessRef};

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

fn shape_u_intent() -> ReorientSpatialIntent<PrimitiveConstructionIntent> {
    ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 1.0,
            height: 2.0,
        },
    ))
    .about(SpatialAnchorRef::shape_axis(SpatialAxis::U))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 0.0]))
}

fn shape_v_intent() -> ReorientSpatialIntent<PrimitiveConstructionIntent> {
    ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 1.0,
            height: 2.0,
        },
    ))
    .about(SpatialAnchorRef::shape_axis(SpatialAxis::V))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([1.0, 0.0, 0.0]))
}

fn shape_w_intent() -> ReorientSpatialIntent<PrimitiveConstructionIntent> {
    ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 1.0,
            height: 2.0,
        },
    ))
    .about(SpatialAnchorRef::shape_axis(SpatialAxis::W))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0]))
}

fn frame_axis_intent() -> ReorientSpatialIntent<PrimitiveConstructionIntent> {
    ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 1.0,
            height: 2.0,
        },
    ))
    .about(SpatialAnchorRef::frame_axis(
        worth_spatial::facade::refs::SpatialFrameRef::world(),
        SpatialAxis::U,
    ))
    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
}

#[test]
fn motion_branch_preview_runtime_reports_cover_directional_reorient_anchor_paths() {
    let mut shape_u_workspace = workspace("worth-kernel.motion-runtime.directional.shape-u");
    let shape_u = prepare_primitive_construction_reorient_branch_preview_runtime_report(
        &mut shape_u_workspace,
        shape_u_intent(),
    )
    .expect("shape-u runtime report");
    let mut shape_v_workspace = workspace("worth-kernel.motion-runtime.directional.shape-v");
    let shape_v = prepare_primitive_construction_reorient_branch_preview_runtime_report(
        &mut shape_v_workspace,
        shape_v_intent(),
    )
    .expect("shape-v runtime report");
    let mut shape_w_workspace = workspace("worth-kernel.motion-runtime.directional.shape-w");
    let shape_w = prepare_primitive_construction_reorient_branch_preview_runtime_report(
        &mut shape_w_workspace,
        shape_w_intent(),
    )
    .expect("shape-w runtime report");
    let mut frame_workspace = workspace("worth-kernel.motion-runtime.directional.frame-axis");
    let frame_axis = prepare_primitive_construction_reorient_branch_preview_runtime_report(
        &mut frame_workspace,
        frame_axis_intent(),
    )
    .expect("frame-axis runtime report");
    let mut direct_shape_u_workspace =
        workspace("worth-kernel.motion-runtime.directional.shape-u.direct");
    let direct_shape_u = prepare_primitive_construction_branch_preview_runtime_report(
        &mut direct_shape_u_workspace,
        shape_u_intent().finish().expect("shape-u finish"),
    )
    .expect("shape-u direct runtime report");
    let mut direct_shape_v_workspace =
        workspace("worth-kernel.motion-runtime.directional.shape-v.direct");
    let direct_shape_v = prepare_primitive_construction_branch_preview_runtime_report(
        &mut direct_shape_v_workspace,
        shape_v_intent().finish().expect("shape-v finish"),
    )
    .expect("shape-v direct runtime report");
    let mut direct_shape_w_workspace =
        workspace("worth-kernel.motion-runtime.directional.shape-w.direct");
    let direct_shape_w = prepare_primitive_construction_branch_preview_runtime_report(
        &mut direct_shape_w_workspace,
        shape_w_intent().finish().expect("shape-w finish"),
    )
    .expect("shape-w direct runtime report");
    let mut direct_frame_workspace =
        workspace("worth-kernel.motion-runtime.directional.frame-axis.direct");
    let direct_frame_axis = prepare_primitive_construction_branch_preview_runtime_report(
        &mut direct_frame_workspace,
        frame_axis_intent().finish().expect("frame-axis finish"),
    )
    .expect("frame-axis direct runtime report");

    assert_eq!(
        shape_u.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert_eq!(
        shape_v.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert_eq!(
        shape_w.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert_eq!(
        frame_axis.runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert_eq!(
        shape_u
            .runtime_report()
            .expect("shape-u branch runtime")
            .outcome(),
        direct_shape_u.outcome()
    );
    assert_eq!(
        shape_v
            .runtime_report()
            .expect("shape-v branch runtime")
            .outcome(),
        direct_shape_v.outcome()
    );
    assert_eq!(
        shape_w
            .runtime_report()
            .expect("shape-w branch runtime")
            .outcome(),
        direct_shape_w.outcome()
    );
    assert_eq!(
        frame_axis
            .runtime_report()
            .expect("frame-axis branch runtime")
            .outcome(),
        direct_frame_axis.outcome()
    );
}
