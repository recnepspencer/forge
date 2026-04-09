use forge_harness::facade::{parity_suite, ExecutionProfile, ExecutionRequest};
use forge_harness::runtime::HarnessAdapter;

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
fn stream_replay_matches_original_canonical_records() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("baseline");
    let fixture = stream_fixture("bridge-stream-replay-parity");

    let mut first_runtime = adapter.create_runtime().expect("first harness runtime");
    adapter
        .prepare_runtime(&mut first_runtime, &profile)
        .expect("first harness prepare");
    adapter
        .load_fixture(&mut first_runtime, &fixture)
        .expect("first harness fixture load");
    let first_run = adapter
        .execute(
            &mut first_runtime,
            &fixture,
            &ExecutionRequest::target("stream-replay-audit", replay_audit_target()),
            &profile,
        )
        .expect("first replay execution");

    let mut second_runtime = adapter.create_runtime().expect("second harness runtime");
    adapter
        .prepare_runtime(&mut second_runtime, &profile)
        .expect("second harness prepare");
    adapter
        .load_fixture(&mut second_runtime, &fixture)
        .expect("second harness fixture load");
    let second_run = adapter
        .execute(
            &mut second_runtime,
            &fixture,
            &ExecutionRequest::target("stream-replay-audit", replay_audit_target()),
            &profile,
        )
        .expect("second replay execution");

    assert_eq!(
        first_run.summary["stream_digest"],
        second_run.summary["stream_digest"]
    );
    assert_eq!(
        first_run.summary["window_digest"],
        second_run.summary["window_digest"]
    );
    assert_eq!(
        first_run.summary["checkpoint_digest"],
        second_run.summary["checkpoint_digest"]
    );
    assert_eq!(
        first_run.summary["replay_digest"],
        second_run.summary["replay_digest"]
    );
}
