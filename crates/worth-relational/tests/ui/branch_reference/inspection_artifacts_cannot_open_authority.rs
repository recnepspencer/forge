use worth_relational::facade::history::BranchId;
use worth_relational::facade::inspection::{
    RelationalBranchSharingObservation, RelationalMvccCostObservation,
    RelationalOwnerAllocationLedgerObservation, RelationalVisibilityCommitmentObservation,
};
use worth_relational::facade::runtime::RelationalRuntime;

fn sharing_cannot_fork(
    runtime: &mut RelationalRuntime,
    observation: RelationalBranchSharingObservation,
) {
    let _ = runtime.fork_branch(BranchId("forged".to_owned()), observation);
}

fn allocation_cannot_begin_mutation(
    runtime: &mut RelationalRuntime,
    observation: RelationalOwnerAllocationLedgerObservation,
) {
    let _ = runtime.begin_transaction(observation);
}

fn cost_cannot_publish(
    runtime: &RelationalRuntime,
    observation: RelationalMvccCostObservation,
) {
    let _ = runtime.publish_commit_for_bridge(observation, "model");
}

fn visibility_cannot_retain(
    runtime: &mut RelationalRuntime,
    observation: RelationalVisibilityCommitmentObservation,
) {
    let _ = runtime
        .history_authority()
        .retain_version_for_replay(observation);
}

fn main() {}
