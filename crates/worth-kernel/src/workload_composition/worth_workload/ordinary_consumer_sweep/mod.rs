mod boolean_split_batch_execution_cluster;
mod closeout;
mod cluster_ledger;
mod current_cutover;
mod current_cutover_proof;
mod current_ledgers;
pub(crate) mod current_replay_undo_boundary_proof;
mod current_route_authority;
mod current_route_witness;
mod error;
mod lookup_consumed_cluster;
mod residue;
mod residue_manifest;
mod workload_composition_explainer_ledger;

pub use boolean_split_batch_execution_cluster::CompletedBooleanSplitBatchExecutionCluster;
pub use closeout::current_worth_workload_ordinary_consumer_sweep_closeout;
pub use cluster_ledger::{
    WorthWorkloadOrdinaryConsumerClusterKind,
    WorthWorkloadOrdinaryConsumerClusterLedger, WorthWorkloadOrdinaryConsumerClusterRowDisposition,
    WorthWorkloadOrdinaryConsumerSweepResidueRow,
};
#[cfg(test)]
pub(crate) use current_cutover::{
    current_worth_workload_ordinary_consumer_cutover, WorthWorkloadOrdinaryConsumerCutover,
    WorthWorkloadOrdinaryConsumerCutoverPosture, WorthWorkloadOrdinaryConsumerCutoverRow,
};
#[cfg(not(test))]
pub(crate) use current_cutover::{
    WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverPosture,
    WorthWorkloadOrdinaryConsumerCutoverRow,
};
#[cfg(test)]
pub(crate) use current_replay_undo_boundary_proof::current_replay_undo_boundary_proof;
pub use error::{
    WorthWorkloadOrdinaryConsumerSweepCloseoutError,
    WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind,
};
pub use lookup_consumed_cluster::LookupConsumedBatchExecutionCluster;
pub use residue_manifest::{
    worth_workload_ordinary_consumer_residue_rows, WorthWorkloadOrdinaryConsumerResidueBoundary,
    WorthWorkloadOrdinaryConsumerResidueRow, WorthWorkloadOrdinaryConsumerResidueSurface,
};
pub use workload_composition_explainer_ledger::{
    WorthWorkloadCompositionExplainerDisposition, WorthWorkloadCompositionExplainerLedger,
    WorthWorkloadCompositionExplainerRow,
};

#[cfg(test)]
mod closeout_hostile_tests;
#[cfg(test)]
mod closeout_test_support;
#[cfg(test)]
mod closeout_tests;
#[cfg(test)]
mod current_cutover_tests;
#[cfg(test)]
mod current_replay_undo_boundary_proof_tests;
#[cfg(test)]
mod current_route_authority_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_support_completed_split;
#[cfg(test)]
mod tests_support_replay_undo_scope;
#[cfg(test)]
mod workload_composition_explainer_cutover_tests;

#[cfg(test)]
pub(crate) fn ordinary_consumer_cutover_from_inventory_for_tests(
    inventory: &crate::workload_composition::ConflictBatchAdmissionInventory,
) -> Result<
    current_cutover::WorthWorkloadOrdinaryConsumerCutover,
    crate::workload_composition::planner_owned_routing::WorthWorkloadOrdinaryConsumerCutoverError,
> {
    current_cutover::ordinary_consumer_cutover_from_inventory_for_tests(inventory)
}

#[cfg(test)]
pub(crate) fn ordinary_consumer_cutover_from_inventory_with_test_replay_undo_identity_override(
    inventory: &crate::workload_composition::ConflictBatchAdmissionInventory,
    boundary_proof_digest: &str,
    transaction_packet_identity: &str,
    replay_scope_identity: &str,
    undo_scope_identity: &str,
) -> Result<
    current_cutover::WorthWorkloadOrdinaryConsumerCutover,
    crate::workload_composition::planner_owned_routing::WorthWorkloadOrdinaryConsumerCutoverError,
> {
    current_cutover::ordinary_consumer_cutover_from_inventory_with_test_replay_undo_identity_override(
        inventory,
        boundary_proof_digest,
        transaction_packet_identity,
        replay_scope_identity,
        undo_scope_identity,
    )
}
