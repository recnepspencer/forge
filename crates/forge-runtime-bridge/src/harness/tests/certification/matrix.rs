use crate::facade::TruthSnapshotIdentity;
use forge_harness::facade::{
    certification_matrix, ExecutionProfile, ExecutionRequest, ScenarioPlan,
};

use super::super::support::{committed_patch, registration, snapshot};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::BridgeHarnessFixture;

#[test]
fn bridge_certification_matrix_reports_diagnostics_for_candidate_profiles() {
    let fixture = ScenarioPlan::new(
        "bridge-certification",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(crate::facade::TruthCommitIdentity::new("commit-a")),
    );

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("bridge certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn bridge_historical_certification_matrix_reports_candidate_profile_parity() {
    let fixture = ScenarioPlan::new(
        "bridge-historical-certification",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("history-commit:main:commit-a")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "historical-commit-a",
        BridgeHarnessTargetId::historical_commit(
            crate::facade::TruthBranchIdentity::new("main"),
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ),
    );

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("bridge historical certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}
