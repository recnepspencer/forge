use super::super::support::{committed_patch, registration, snapshot};
use crate::facade::BridgeHistoricalEvaluationFailureClass;
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::BridgeHarnessFixture;
use worth_harness::facade::{ExecutionProfile, ExecutionRequest, ScenarioPlan};
use worth_harness::runtime::HarnessAdapter;

#[test]
fn bridge_harness_rejects_unavailable_historical_view_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-historical-missing-view",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                "alice",
            )),
    )
    .declare_input("history-commit:main:missing-commit")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "historical-missing",
        BridgeHarnessTargetId::historical_commit(
            crate::truth_identity_fixtures::truth_branch_fixture("main"),
            crate::truth_identity_fixtures::truth_commit_fixture("missing-commit"),
        ),
    );
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let _error = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect_err("missing historical view should fail explicitly");

    let failure = session
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_historical_evaluation_failure()
        .expect("historical failure should be recorded");
    assert_eq!(
        failure.failure_class(),
        BridgeHistoricalEvaluationFailureClass::TruthViewUnavailable
    );
    assert_eq!(
        failure.commit_identity().map(|commit| commit.as_str()),
        Some(crate::truth_identity_fixtures::truth_commit_fixture("missing-commit").as_str())
    );
    assert_eq!(
        failure.branch_identity().as_str(),
        crate::truth_identity_fixtures::truth_branch_fixture("main").as_str()
    );
}
