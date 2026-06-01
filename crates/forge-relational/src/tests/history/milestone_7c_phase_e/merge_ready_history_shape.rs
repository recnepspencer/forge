use crate::facade::history::BranchId;
use crate::facade::replay::{RelationalReplayRequest, ReplayExecutionMode, ReplayVerificationMode};

use super::fixtures::execute_feature_into_main_merge;

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
            .replay_commit(RelationalReplayRequest {
                commit_id: merge.commit.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
            })
            .commit
            .expect("replayed merge commit")
            .ordered_parents()
            .clone_inner(),
        merge.commit.commit.parents
    );
}
