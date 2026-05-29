use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, workspace_requires_historical_basis_context, TopologyRuntimeAdapters,
    TopologyRuntimePostureCapability, TopologyRuntimePostureStatus, TopologyRuntimeSupport,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use schema::facade::topology_authoring::seed_minimal_topology;

#[test]
fn current_head_runtime_posture_rows_freeze_admitted_and_denied_capabilities() {
    let support = TopologyRuntimeSupport::current_head_authoritative();

    assert_eq!(
        support.runtime_posture_rows().len(),
        TopologyRuntimePostureCapability::ALL.len()
    );
    for capability in TopologyRuntimePostureCapability::ALL {
        let row = support
            .runtime_posture_rows()
            .iter()
            .find(|row| row.capability() == capability)
            .expect("current-head posture row should exist");
        let expected_status = match capability {
            TopologyRuntimePostureCapability::CurrentHeadLiveReads
            | TopologyRuntimePostureCapability::PostWriteMaterialization
            | TopologyRuntimePostureCapability::BranchPreviewBasis
            | TopologyRuntimePostureCapability::AuthoritativeWrites => {
                TopologyRuntimePostureStatus::Admitted
            }
            TopologyRuntimePostureCapability::CurrentHeadMaterialization
            | TopologyRuntimePostureCapability::HistoricalBasis => {
                TopologyRuntimePostureStatus::Denied
            }
        };
        assert_eq!(row.status(), expected_status);
        assert!(!row.row_digest().is_empty());
    }
}

#[test]
fn snapshot_runtime_posture_rows_freeze_historical_read_only_capabilities() {
    let support = TopologyRuntimeSupport::snapshot_read_only();

    assert_eq!(
        support.runtime_posture_rows().len(),
        TopologyRuntimePostureCapability::ALL.len()
    );
    for capability in TopologyRuntimePostureCapability::ALL {
        let expected_status = match capability {
            TopologyRuntimePostureCapability::HistoricalBasis => {
                TopologyRuntimePostureStatus::Admitted
            }
            TopologyRuntimePostureCapability::CurrentHeadLiveReads
            | TopologyRuntimePostureCapability::CurrentHeadMaterialization
            | TopologyRuntimePostureCapability::PostWriteMaterialization
            | TopologyRuntimePostureCapability::BranchPreviewBasis
            | TopologyRuntimePostureCapability::AuthoritativeWrites => {
                TopologyRuntimePostureStatus::Denied
            }
        };
        assert_eq!(support.runtime_posture_status(capability), expected_status);
    }
}

#[test]
fn workspace_historical_basis_detection_tracks_topology_runtime_support_contract() {
    let current_runtime = build_milestone_one_runtime().expect("runtime");
    let current_adapters = TopologyRuntimeAdapters::current_head(current_runtime);
    let current_workspace =
        topology_runtime(current_adapters, ".runtime-posture.current-head").expect("workspace");
    assert!(!workspace_requires_historical_basis_context(
        &current_workspace
    ));

    let mut snapshot_runtime = build_milestone_one_runtime().expect("runtime");
    let seeded = seed_minimal_topology(&mut snapshot_runtime, "runtime-posture-snapshot")
        .expect("seed topology");
    let read_view = snapshot_runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot");
    let snapshot_adapters = TopologyRuntimeAdapters::snapshot_read_only(read_view, seeded.snapshot);
    let snapshot_workspace =
        topology_runtime(snapshot_adapters, ".runtime-posture.snapshot").expect("workspace");
    assert!(workspace_requires_historical_basis_context(
        &snapshot_workspace
    ));
}

#[test]
fn current_head_runtime_admits_preview_and_branch_sessions() {
    let runtime = build_milestone_one_runtime().expect("runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".runtime-posture.branch-preview").expect("workspace");

    let preview = workspace
        .preview("topology-preview")
        .expect("preview session");
    assert_eq!(preview.basis_admission().label(), "topology-preview");

    let branch = workspace.branch("topology-branch").expect("branch session");
    assert_eq!(branch.basis_admission().label(), "topology-branch");
}




