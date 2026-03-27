use crate::facade::history::BranchId;
use crate::facade::merge::{MergeExecutionOutcome, MergeExecutionRequest, MergeIntent};
use crate::tests::support::{
    capture_aspect_truth_bundle, certification_digest, checkpoint_and_recover_with,
    create_branch_from_main, create_entity_outcome, create_entity_outcome_on_branch,
    persisted_runtime_with_test_schema,
};

fn execute_feature_into_main_merge(
) -> (
    crate::facade::runtime::RelationalRuntime,
    MergeExecutionOutcome,
    crate::facade::history::CommitId,
    crate::facade::history::CommitId,
) {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    let feature_head =
        create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));
    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");
    let main_head_commit_id = runtime
        .history_access()
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
    let replay = runtime.replay_access();

    assert_eq!(merge.commit.merge_parent_count(), 1);
    assert_eq!(
        merge.commit.commit.parents,
        vec![main_head_commit_id, feature_head_commit_id]
    );
    assert_eq!(merge.execution_summary.target_head_commit_id, main_head_commit_id);
    assert_eq!(merge.execution_summary.source_head_commit_id, feature_head_commit_id);
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
            .history_access()
            .branch_head(&BranchId("main".to_string()))
            .expect("main branch head")
            .commit_id,
        merge.commit.commit.commit_id
    );
    assert_eq!(
        runtime
            .history_access()
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
        .replay_access()
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .cloned()
        .expect("live merge envelope");

    let (_recovery, mut recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
    let recovered_bundle = capture_aspect_truth_bundle(&mut recovered, &[], &[], &[]);
    let recovered_envelope = recovered
        .replay_access()
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .cloned()
        .expect("recovered merge envelope");

    assert_eq!(before_bundle.visible_truth, recovered_bundle.visible_truth);
    assert_eq!(merge_envelope, recovered_envelope);
    assert_eq!(
        certification_digest(&merge_envelope.merge_parent_branches),
        certification_digest(&recovered_envelope.merge_parent_branches)
    );
}

#[test]
fn execute_prepared_merge_produces_merge_ready_history_shape() {
    let (mut runtime, merge, _main_head_commit_id, feature_head_commit_id) =
        execute_feature_into_main_merge();

    assert_eq!(
        runtime
            .history_access()
            .latest_common_ancestor_between_branches(
                &BranchId("main".to_string()),
                &BranchId("feature".to_string())
            ),
        Some(feature_head_commit_id)
    );

    let inspection = runtime
        .history_access()
        .inspect_merge(&BranchId("feature".to_string()), &BranchId("main".to_string()));
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
