use worth_harness::facade::{certification_matrix, ExecutionProfile, ExecutionRequest};

use crate::harness::adapter::BridgeHarnessAdapter;

use super::support::{routing_target, stream_fixture};

#[test]
fn bridge_harness_stream_certification_reports_candidate_profile_parity() {
    let report = certification_matrix(
        BridgeHarnessAdapter,
        stream_fixture("bridge-stream-certification"),
        ExecutionRequest::target("stream-routing", routing_target()),
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("stream certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}
