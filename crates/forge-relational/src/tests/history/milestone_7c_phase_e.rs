use crate::facade::diagnostics::{DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope};
use crate::facade::history::BranchId;
use crate::facade::merge::{MergeExecutionOutcome, MergeExecutionRequest, MergeIntent};
use crate::facade::transactions::TransactionOptions;
use crate::tests::support::{
    capture_aspect_truth_bundle, checkpoint_and_recover_with, create_branch_from_main,
    create_entity_outcome, create_entity_outcome_on_branch, persisted_runtime_with_test_schema,
};

fn execute_feature_into_main_merge() -> (
    crate::facade::runtime::RelationalRuntime,
    MergeExecutionOutcome,
    crate::facade::history::CommitId,
    crate::facade::history::CommitId,
) {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    let feature_head = create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );
    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");
    let main_head_commit_id = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .expect("main head before merge")
        .commit_id;
    let merge = runtime
        .execute_prepared_merge(prepared)
        .expect("executed prepared merge");
    (
        runtime,
        merge,
        main_head_commit_id,
        feature_head.commit.commit_id,
    )
}

#[test]
fn execute_prepared_merge_publishes_ordered_multi_parent_commit_through_canonical_envelope() {
    let (runtime, merge, main_head_commit_id, feature_head_commit_id) =
        execute_feature_into_main_merge();
    let replay = runtime.replay();

    assert_eq!(merge.commit.merge_parent_count(), 1);
    assert_eq!(
        merge.commit.commit.parents,
        vec![main_head_commit_id, feature_head_commit_id]
    );
    assert_eq!(
        merge.execution_summary.target_head_commit_id,
        main_head_commit_id
    );
    assert_eq!(
        merge.execution_summary.source_head_commit_id,
        feature_head_commit_id
    );
    assert_eq!(merge.execution_summary.executed_record_count, 1);

    let envelope = replay
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .expect("canonical merge envelope");
    assert_eq!(envelope.commit.parents, merge.commit.commit.parents);
    assert_eq!(
        envelope.merge_parent_branches,
        vec![BranchId("feature".to_string())]
    );
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("main".to_string()))
            .expect("main branch head")
            .commit_id,
        merge.commit.commit.commit_id
    );
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("feature".to_string()))
            .expect("feature branch head")
            .commit_id,
        feature_head_commit_id
    );
}

#[test]
fn execute_prepared_merge_survives_durability_append_and_recovery() {
    let (mut runtime, merge, _main_head_commit_id, _feature_head_commit_id) =
        execute_feature_into_main_merge();
    let before_bundle = capture_aspect_truth_bundle(&mut runtime, &[], &[], &[]);
    let merge_envelope = runtime
        .replay()
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .cloned()
        .expect("live merge envelope");

    let (_recovery, mut recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
    let recovered_bundle = capture_aspect_truth_bundle(&mut recovered, &[], &[], &[]);
    let recovered_envelope = recovered
        .replay()
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .cloned()
        .expect("recovered merge envelope");

    assert_eq!(before_bundle.visible_truth, recovered_bundle.visible_truth);
    assert_eq!(merge_envelope, recovered_envelope);
    assert!(merge_envelope
        .diagnostics_summary
        .entries
        .iter()
        .any(|entry| entry.code == DiagnosticCode::MergeExecutionPublished));
    let merge_execution_entry = merge_envelope
        .diagnostics_summary
        .entries
        .iter()
        .find(|entry| entry.code == DiagnosticCode::MergeExecutionPublished)
        .expect("merge execution summary entry");
    assert_eq!(
        merge_execution_entry.fields.root_value()["commit_id"],
        serde_json::json!(merge.commit.commit.commit_id.0)
    );
    assert_eq!(
        merge_execution_entry.fields.root_value()["execution_digest"],
        serde_json::json!(merge.execution_summary.execution_digest)
    );
    assert_eq!(
        merge_execution_entry.fields.root_value()["diagnostics_digest"],
        serde_json::json!(merge.execution_summary.diagnostics_digest)
    );
}

#[test]
fn execute_prepared_merge_produces_merge_ready_history_shape() {
    let (mut runtime, merge, _main_head_commit_id, feature_head_commit_id) =
        execute_feature_into_main_merge();

    assert_eq!(
        runtime.history().latest_common_ancestor_between_branches(
            &BranchId("main".to_string()),
            &BranchId("feature".to_string())
        ),
        Some(feature_head_commit_id)
    );

    let inspection = runtime.history().inspect_merge(
        &BranchId("feature".to_string()),
        &BranchId("main".to_string()),
    );
    assert!(inspection.source_only_commits.is_empty());
    assert_eq!(inspection.merge_base, Some(feature_head_commit_id));
    assert_eq!(
        runtime
            .replay_authority()
            .replay_commit(crate::facade::replay::RelationalReplayRequest {
                commit_id: merge.commit.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: crate::facade::replay::ReplayExecutionMode::SerialDeterministic,
                verification_mode:
                    crate::facade::replay::ReplayVerificationMode::NormalRecoveryVerification,
            })
            .commit
            .expect("replayed merge commit")
            .ordered_parents()
            .clone_inner(),
        merge.commit.commit.parents
    );
}

#[test]
fn execute_prepared_merge_reports_execution_counters_and_structural_summary() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");
    runtime.performance_access().reset_counters();

    let merge = runtime
        .execute_prepared_merge(prepared)
        .expect("executed prepared merge");
    let counters = runtime.performance_access().counters();

    assert_eq!(merge.structural_summary.executed_record_count, 1);
    assert_eq!(merge.structural_summary.adopted_source_record_count, 1);
    assert_eq!(merge.structural_summary.emitted_mutation_intent_count, 1);
    assert_eq!(counters.merge_execution_attempts, 1);
    assert_eq!(counters.merge_execution_requests, 1);
    assert_eq!(counters.merge_execution_records_admitted, 1);
    assert_eq!(counters.merge_execution_mutation_intents_emitted, 1);
    assert_eq!(
        merge
            .commit
            .execution
            .complexity_delta
            .merge_execution_attempts,
        1
    );
    assert_eq!(
        merge
            .commit
            .execution
            .complexity_delta
            .merge_execution_requests,
        1
    );
    assert_eq!(
        merge
            .commit
            .execution
            .complexity_delta
            .merge_execution_records_admitted,
        1
    );
    assert_eq!(
        merge
            .commit
            .execution
            .complexity_delta
            .merge_execution_mutation_intents_emitted,
        1
    );
    assert!(merge
        .commit
        .diagnostics()
        .iter()
        .any(
            |artifact| artifact.kind == DiagnosticsArtifactKind::MinimalSummary
                && artifact
                    .entries
                    .iter()
                    .any(|entry| entry.code == DiagnosticCode::MergeExecutionPublished)
        ));
    let execution_artifact = merge
        .commit
        .diagnostics()
        .iter()
        .find(|artifact| {
            artifact.kind == DiagnosticsArtifactKind::DetailedTrace
                && artifact
                    .entries
                    .iter()
                    .any(|entry| entry.code == DiagnosticCode::MergeExecutionPublished)
        })
        .expect("merge execution detailed artifact");
    assert!(execution_artifact.entries.iter().all(|entry| {
        entry.code == DiagnosticCode::MergeExecutionPublished
            && entry.fields.root_value().get("blocked_count").is_none()
            && entry.fields.root_value().get("rejected_count").is_none()
    }));
}

#[test]
fn execute_prepared_merge_records_attempt_without_success_on_failure() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");
    create_entity_outcome(&mut runtime, "head-drift");
    runtime.performance_access().reset_counters();

    let error = runtime
        .execute_prepared_merge(prepared)
        .expect_err("stale prepared merge should fail");
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        crate::facade::merge::MergeExecutionError::StaleBranchHead { .. }
    ));
    assert_eq!(counters.merge_execution_attempts, 1);
    assert_eq!(counters.merge_execution_requests, 0);
    assert_eq!(counters.merge_execution_records_admitted, 0);
    assert_eq!(counters.merge_execution_mutation_intents_emitted, 0);
    assert!(runtime
        .publication()
        .diagnostics()
        .artifacts()
        .iter()
        .any(|artifact| artifact.scope == DiagnosticsScope::History
            && artifact.kind == DiagnosticsArtifactKind::Failure
            && artifact
                .entries
                .iter()
                .any(|entry| entry.code == DiagnosticCode::DeterministicMergeViolation)));
    assert!(!runtime
        .publication()
        .diagnostics()
        .artifacts()
        .iter()
        .any(|artifact| artifact.scope == DiagnosticsScope::History
            && artifact.kind == DiagnosticsArtifactKind::DetailedTrace
            && artifact
                .entries
                .iter()
                .any(|entry| entry.code == DiagnosticCode::MergeExecutionPublished)));
}

#[test]
fn merge_commit_context_rejects_mismatched_parent_branch_metadata() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");
    let mutation_plan = runtime
        .merge()
        .derive_merge_commit_mutation_plan(
            crate::facade::transactions::TransactionId(999),
            &prepared,
        )
        .expect("merge mutation plan");

    let error = crate::authority::commit::pipeline::AuthoritativeCommitContext::from_merge(
        TransactionOptions {
            target_branch: Some(BranchId("main".to_string())),
            merge_parent_branches: vec![BranchId("wrong".to_string())],
            ..TransactionOptions::default()
        },
        mutation_plan,
    )
    .expect_err("mismatched merge context should be rejected");

    match error {
        crate::facade::transactions::TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                crate::facade::transactions::ConflictClass::InvalidMergeParent { .. }
            ));
        }
        other => panic!("expected conflict error, got {other:?}"),
    }
}

#[test]
fn execute_prepared_merge_preserves_reserved_summary_when_optional_diagnostics_budget_is_zero() {
    let mut runtime = persisted_runtime_with_test_schema();
    runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
    create_entity_outcome(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");

    let merge = runtime
        .execute_prepared_merge(prepared)
        .expect("executed prepared merge");
    let replay = runtime.replay();
    let envelope = replay
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .expect("canonical merge envelope");

    assert_eq!(envelope.diagnostics_summary.entries.len(), 1);
    assert_eq!(
        envelope.diagnostics_summary.entries[0].code,
        DiagnosticCode::MergeExecutionPublished
    );
}
