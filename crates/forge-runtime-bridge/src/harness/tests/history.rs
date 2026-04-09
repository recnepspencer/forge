use forge_harness::facade::{
    ExecutionProfile, ExecutionRequest, MutationBatch, ReplayRequest, ScenarioPlan,
};
use forge_harness::runtime::{HarnessAdapter, ReplayHarnessAdapter};

use super::support::{committed_patch, committed_patch_on_branch, registration, snapshot};
use crate::facade::BridgeHistoricalEvaluationFailureClass;
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessMutation};
use crate::harness::fixtures::BridgeHarnessFixture;

#[test]
fn bridge_harness_executes_historical_commit_view() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-historical-commit-view",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("history-commit:main:commit-a")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "historical-commit-a",
        "history-commit:main:commit-a".to_string(),
    );
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("historical execution should succeed");

    assert_eq!(run.summary["snapshot_identity"], "snapshot-a");
    assert_eq!(run.summary["commit_identity"], "commit-a");
    assert!(run.summary["historical_artifact_identity"].is_string());
    assert_eq!(
        run.extensions["bridge_historical_evaluation_record"]["materialization_path"],
        "CommitEnvelopeSnapshot"
    );
}

#[test]
fn bridge_harness_executes_branch_head_view() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-branch-head-view",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("branch-head:main")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target("branch-head-main", "branch-head:main".to_string());
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("branch-head execution should succeed");

    assert_eq!(run.summary["snapshot_identity"], "snapshot-a");
    assert_eq!(run.summary["branch_identity"], "main");
    assert!(run.extensions["bridge_historical_evaluation_record"]["artifact_identity"].is_string());
    assert_eq!(
        run.extensions["bridge_historical_evaluation_record"]["materialization_path"],
        "BranchHeadEnvelopeSnapshot"
    );
}

#[test]
fn bridge_harness_replays_historical_record() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-historical-replay",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("history-commit:main:commit-a")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "historical-commit-a",
        "history-commit:main:commit-a".to_string(),
    );
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("historical execution should succeed");
    let replay = adapter
        .capture_replay(
            &session,
            &fixture,
            &ReplayRequest {
                name: "historical-replay".to_string(),
                source_run: run,
                request: request.clone(),
                profile: profile.clone(),
            },
        )
        .expect("historical replay capture should succeed");

    assert_eq!(replay.summary["source_snapshot"], "snapshot-a");
    assert!(replay.summary["historical_record_identity"].is_string());
}

#[test]
fn bridge_harness_branch_divergence_changes_selected_truth_view_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("development");
    let request =
        ExecutionRequest::target("branch-head-feature", "branch-head:feature".to_string());

    let main_fixture = ScenarioPlan::new(
        "bridge-historical-main-head",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_on_branch(
                "main",
                "commit-a",
                "patch-a",
                "snapshot-a",
                "name",
            ))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("branch-head:main")
    .declare_observation("historical")
    .compile();
    let feature_fixture = ScenarioPlan::new(
        "bridge-historical-feature-head",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_on_branch(
                "feature",
                "commit-f",
                "patch-f",
                "snapshot-f",
                "name",
            ))
            .with_snapshot(snapshot("snapshot-f", "frank")),
    )
    .declare_input("branch-head:feature")
    .declare_observation("historical")
    .compile();

    let mut main_session = adapter
        .create_runtime()
        .expect("main bridge harness runtime");
    adapter
        .prepare_runtime(&mut main_session, &profile)
        .expect("main bridge harness prepare");
    adapter
        .load_fixture(&mut main_session, &main_fixture)
        .expect("main bridge harness load fixture");
    let main_run = adapter
        .execute(
            &mut main_session,
            &main_fixture,
            &ExecutionRequest::target("branch-head-main", "branch-head:main".to_string()),
            &profile,
        )
        .expect("main branch-head execution should succeed");

    let mut feature_session = adapter
        .create_runtime()
        .expect("feature bridge harness runtime");
    adapter
        .prepare_runtime(&mut feature_session, &profile)
        .expect("feature bridge harness prepare");
    adapter
        .load_fixture(&mut feature_session, &feature_fixture)
        .expect("feature bridge harness load fixture");
    let feature_run = adapter
        .execute(&mut feature_session, &feature_fixture, &request, &profile)
        .expect("feature branch-head execution should succeed");

    assert_ne!(
        main_run.summary["historical_record_identity"],
        feature_run.summary["historical_record_identity"]
    );
    assert_ne!(
        main_run.summary["snapshot_identity"],
        feature_run.summary["snapshot_identity"]
    );
    assert_ne!(
        main_run.summary["branch_identity"],
        feature_run.summary["branch_identity"]
    );
}

#[test]
fn bridge_harness_rejects_unavailable_historical_view_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-historical-missing-view",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("history-commit:main:missing-commit")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "historical-missing",
        "history-commit:main:missing-commit".to_string(),
    );
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let error = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect_err("missing historical view should fail explicitly");

    let detail = error.to_string();
    assert!(detail.contains("historical planning failed"));
    assert!(detail.contains("missing-commit"));
    assert_eq!(
        session
            .runtime
            .as_ref()
            .expect("bridge runtime")
            .diagnostics()
            .last_historical_evaluation_failure()
            .expect("historical failure should be recorded")
            .failure_class(),
        BridgeHistoricalEvaluationFailureClass::TruthViewUnavailable
    );
}

#[test]
fn bridge_harness_replays_historical_record_after_newer_publication_arrives() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-historical-replay-stability",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("history-commit:main:commit-a")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "historical-commit-a",
        "history-commit:main:commit-a".to_string(),
    );
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("historical execution should succeed");

    let mutation = MutationBatch::new("publish-newer-history")
        .push(BridgeHarnessMutation::PublishCommittedPatch(
            committed_patch("commit-b", "patch-b", "snapshot-b", "name"),
        ))
        .push(BridgeHarnessMutation::PublishSnapshot(snapshot(
            "snapshot-b",
            "bob",
        )));
    adapter
        .apply_mutation_batch(&mut session, &mutation)
        .expect("mutation batch should apply");

    let replay = adapter
        .capture_replay(
            &session,
            &fixture,
            &ReplayRequest {
                name: "historical-replay-after-newer-publication".to_string(),
                source_run: run,
                request,
                profile,
            },
        )
        .expect("historical replay should remain pinned to the original record");

    assert_eq!(replay.summary["source_snapshot"], "snapshot-a");
}
