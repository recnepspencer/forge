use crate::construction::certification::motion::prepare_primitive_construction_reorient_witness_resolution_report;
use crate::construction::certification::motion::PrimitiveConstructionMotionWitnessResolutionStatus;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::motion_branch_runtime::{
    prepare_primitive_construction_reorient_branch_preview_runtime_report,
    PrimitiveConstructionMotionRuntimeSurfaceStatus,
};
use crate::construction::motion_replay::prepare_primitive_construction_reorient_replay_parity_report;
use crate::construction::runtime_basis::prepare_primitive_construction_branch_preview_runtime_report;
use crate::construction::specs::RegularPyramidSpec;
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
fn motion_reports_support_directional_reorient_anchor_paths() {
    let intents = [
        (
            shape_u_intent(),
            "worth-kernel.motion.directional.shape-u",
            "worth-kernel.motion.directional.shape-u.direct",
        ),
        (
            shape_v_intent(),
            "worth-kernel.motion.directional.shape-v",
            "worth-kernel.motion.directional.shape-v.direct",
        ),
        (
            frame_axis_intent(),
            "worth-kernel.motion.directional.frame-axis",
            "worth-kernel.motion.directional.frame-axis.direct",
        ),
    ];

    for (intent, runtime_name, direct_name) in intents {
        let witness =
            prepare_primitive_construction_reorient_witness_resolution_report(intent.clone());
        let replay = prepare_primitive_construction_reorient_replay_parity_report(intent.clone());
        let mut runtime_workspace = workspace(runtime_name);
        let branch = prepare_primitive_construction_reorient_branch_preview_runtime_report(
            &mut runtime_workspace,
            intent.clone(),
        )
        .expect("branch runtime");
        let mut direct_workspace = workspace(direct_name);
        let direct = prepare_primitive_construction_branch_preview_runtime_report(
            &mut direct_workspace,
            intent.finish().expect("finish"),
        )
        .expect("direct runtime");

        assert_eq!(
            witness.status(),
            PrimitiveConstructionMotionWitnessResolutionStatus::Admitted
        );
        assert!(replay.parity_verified());
        assert_eq!(
            branch.runtime_surface_status(),
            PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
        );
        assert_eq!(
            branch.runtime_report().expect("branch runtime").outcome(),
            direct.outcome()
        );
    }
}
