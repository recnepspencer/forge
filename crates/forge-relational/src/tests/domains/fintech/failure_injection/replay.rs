use crate::facade::{
    BranchId, RelationalReplayOutcome, RelationalReplayRequest, RelationalRuntime,
    ReplayExecutionMode,
};

pub(crate) fn drop_latest_parent_envelope_for_replay(
    runtime: &mut RelationalRuntime,
) -> Option<u64> {
    let latest = runtime.history_access().latest_commit()?.commit_id;
    let chain = runtime.history_access().ancestor_chain(latest);
    let parent = *chain.get(1)?;
    runtime
        .history_authority()
        .remove_commit_envelope_for_test(parent)
        .then_some(parent.0)
}

pub(crate) fn replay_latest_commit_on_wrong_branch(
    runtime: &mut RelationalRuntime,
) -> Option<RelationalReplayOutcome> {
    let latest = runtime.history_access().latest_commit()?.commit_id;
    Some(runtime.replay_authority().replay_commit(RelationalReplayRequest {
        commit_id: latest,
        branch_id: BranchId("wrong".to_string()),
        execution_mode: ReplayExecutionMode::SerialDeterministic,
    }))
}
