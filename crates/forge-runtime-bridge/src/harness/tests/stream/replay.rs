use forge_harness::facade::{parity_suite, ExecutionProfile, ExecutionRequest};

use crate::harness::adapter::BridgeHarnessAdapter;

use super::support::{replay_audit_target, stream_fixture};

#[test]
fn bridge_harness_stream_replay_audit_remains_parity_safe_across_candidate_profiles() {
    let report = parity_suite(
        BridgeHarnessAdapter,
        stream_fixture("bridge-stream-replay-parity"),
        ExecutionRequest::target("stream-replay-audit", replay_audit_target()),
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::operational("operational")])
    .compare()
    .expect("stream replay-audit parity should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);
}

#[test]
fn stream_replay_export_remains_deterministic_across_baseline_runs() {
    let report = parity_suite(
        BridgeHarnessAdapter,
        stream_fixture("bridge-stream-replay-determinism"),
        ExecutionRequest::target("stream-replay-audit", replay_audit_target()),
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::development("second-baseline")])
    .compare()
    .expect("stream replay-audit deterministic export parity should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);
}
