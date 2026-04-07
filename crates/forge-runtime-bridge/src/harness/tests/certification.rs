use forge_harness::facade::{certification_matrix, ExecutionProfile, ExecutionRequest, ScenarioPlan};

use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::BridgeHarnessFixture;
use super::support::{committed_patch, registration, snapshot};

#[test]
fn bridge_certification_matrix_reports_diagnostics_for_candidate_profiles() {
    let fixture = ScenarioPlan::new(
        "bridge-certification",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

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
