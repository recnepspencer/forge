use crate::construction::certification::motion::representative_evidence::prepare_primitive_construction_motion_representative_evidence;
use crate::construction::certification::motion::representative_inputs::required_motion_representative_cases;
use crate::construction::certification::motion::{
    PrimitiveConstructionMotionDxSurface, PrimitiveConstructionMotionResolutionPolicyCase,
};
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};

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
fn motion_representative_evidence_covers_required_cases() {
    let cases = required_motion_representative_cases();
    let evidence = cases
        .iter()
        .enumerate()
        .map(|(index, case)| {
            let mut workspace = workspace(&format!("worth-kernel.motion-evidence.{index}"));
            prepare_primitive_construction_motion_representative_evidence(&mut workspace, *case)
                .expect("representative evidence")
        })
        .collect::<Vec<_>>();

    assert_eq!(evidence.len(), cases.len());
    assert!(evidence.iter().all(|row| row.parity_verified()));
}

#[test]
fn motion_representative_evidence_replaces_bundle_truth_for_direct_move() {
    let mut workspace = workspace("worth-kernel.motion-evidence.direct-move");
    let evidence = prepare_primitive_construction_motion_representative_evidence(
        &mut workspace,
        PrimitiveConstructionMotionResolutionPolicyCase::DirectMove,
    )
    .expect("representative evidence");

    assert_eq!(
        evidence.dx_row().dx_surface(),
        PrimitiveConstructionMotionDxSurface::CommonPath
    );
    assert!(evidence.replay_report().parity_verified());
    assert_ne!(evidence.report_digest(), evidence.policy_row().row_digest());
    assert_ne!(
        evidence.report_digest(),
        evidence.branch_runtime_report().report_digest()
    );
}
