use crate::facade::TruthSnapshotIdentity;
use forge_harness::facade::{
    ExecutionProfile, ExecutionRequest, MutationBatch, ReplayRequest, ScenarioPlan,
};
use forge_harness::runtime::{HarnessAdapter, ReplayHarnessAdapter};

use super::support::{committed_patch, committed_patch_on_branch, registration, snapshot};
use crate::facade::{BridgeHistoricalEvaluationFailureClass, BridgeHistoricalMaterializationPath};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessMutation, BridgeHarnessTargetId};
use crate::harness::fixtures::BridgeHarnessFixture;

use super::history_assertions::{
    assert_historical_record, assert_historical_replay_summary, last_historical_record,
    replay_historical_record,
};

#[test]
fn bridge_harness_executes_historical_commit_view() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-historical-commit-view",
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
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let _run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("historical execution should succeed");

    let record = last_historical_record(&session);
    assert_historical_record(
        &record,
        "snapshot-a",
        "main",
        "commit-a",
        BridgeHistoricalMaterializationPath::CommitEnvelopeSnapshot,
    );
}

#[test]
fn bridge_harness_executes_branch_head_view() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-branch-head-view",
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
    .declare_input("branch-head:main")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "branch-head-main",
        BridgeHarnessTargetId::branch_head(crate::facade::TruthBranchIdentity::new("main")),
    );
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let _run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("branch-head execution should succeed");

    let record = last_historical_record(&session);
    assert_historical_record(
        &record,
        "snapshot-a",
        "main",
        "commit-a",
        BridgeHistoricalMaterializationPath::BranchHeadEnvelopeSnapshot,
    );
}

#[test]
fn bridge_harness_replays_historical_record() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-historical-replay",
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
    let _replay = adapter
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

    let record = last_historical_record(&session);
    let replay_summary = replay_historical_record(&session, &record);
    assert_historical_replay_summary(&replay_summary, &record, "snapshot-a");
}

#[test]
fn bridge_harness_branch_divergence_changes_selected_truth_view_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("development");
    let request = ExecutionRequest::target(
        "branch-head-feature",
        BridgeHarnessTargetId::branch_head(crate::facade::TruthBranchIdentity::new("feature")),
    );

    let main_fixture = ScenarioPlan::new(
        "bridge-historical-main-head",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_on_branch(
                crate::facade::TruthBranchIdentity::new("main"),
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("branch-head:main")
    .declare_observation("historical")
    .compile();
    let feature_fixture = ScenarioPlan::new(
        "bridge-historical-feature-head",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_on_branch(
                crate::facade::TruthBranchIdentity::new("feature"),
                crate::facade::TruthCommitIdentity::new("commit-f"),
                crate::facade::TruthPatchIdentity::new("patch-f"),
                TruthSnapshotIdentity::new("snapshot-f"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-f"), "frank")),
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
    let _main_run = adapter
        .execute(
            &mut main_session,
            &main_fixture,
            &ExecutionRequest::target(
                "branch-head-main",
                BridgeHarnessTargetId::branch_head(crate::facade::TruthBranchIdentity::new("main")),
            ),
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
    let _feature_run = adapter
        .execute(&mut feature_session, &feature_fixture, &request, &profile)
        .expect("feature branch-head execution should succeed");

    let main_record = last_historical_record(&main_session);
    let feature_record = last_historical_record(&feature_session);
    assert_ne!(
        main_record.record_identity(),
        feature_record.record_identity()
    );
    assert_ne!(
        main_record.decision_log().snapshot_identity(),
        feature_record.decision_log().snapshot_identity()
    );
    assert_ne!(
        main_record.decision_log().branch_identity(),
        feature_record.decision_log().branch_identity()
    );
}

#[test]
fn bridge_harness_rejects_unavailable_historical_view_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-historical-missing-view",
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
    .declare_input("history-commit:main:missing-commit")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "historical-missing",
        BridgeHarnessTargetId::historical_commit(
            crate::facade::TruthBranchIdentity::new("main"),
            crate::facade::TruthCommitIdentity::new("missing-commit"),
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
        Some("missing-commit")
    );
    assert_eq!(failure.branch_identity().as_str(), "main");
}

#[test]
fn bridge_harness_replays_historical_record_after_newer_publication_arrives() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-historical-replay-stability",
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
            committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-b"),
                crate::facade::TruthPatchIdentity::new("patch-b"),
                TruthSnapshotIdentity::new("snapshot-b"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ),
        ))
        .push(BridgeHarnessMutation::PublishSnapshot(snapshot(
            TruthSnapshotIdentity::new("snapshot-b"),
            "bob",
        )));
    adapter
        .apply_mutation_batch(&mut session, &mutation)
        .expect("mutation batch should apply");

    let _replay = adapter
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

    let record = last_historical_record(&session);
    let replay_summary = replay_historical_record(&session, &record);
    assert_historical_replay_summary(&replay_summary, &record, "snapshot-a");
}
