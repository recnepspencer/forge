use crate::branch::RelationalBranchRoot;
use crate::history::data::BranchId;

use super::{
    HistorySubsystem, RelationalBranchSharingCostCounters, RelationalForkMaterializationCost,
};

impl HistorySubsystem {
    pub(crate) fn record_fork_root_acquisition(&mut self, branch_id: &BranchId) {
        self.record_branch_sharing_operation(branch_id, |costs| {
            costs.shared_root_acquisitions = costs.shared_root_acquisitions.saturating_add(1);
        });
    }

    pub(crate) fn record_snapshot_root_read(&mut self, branch_id: &BranchId) {
        self.record_branch_sharing_operation(branch_id, |costs| {
            costs.snapshot_root_reads = costs.snapshot_root_reads.saturating_add(1);
        });
    }

    pub(crate) fn record_transaction_validation_attempt(&mut self, branch_id: &BranchId) {
        self.record_branch_sharing_operation(branch_id, |costs| {
            costs.transaction_validation_attempts =
                costs.transaction_validation_attempts.saturating_add(1);
        });
    }

    pub(crate) fn record_retained_history_head_lookup(&mut self, branch_id: &BranchId) {
        self.record_branch_sharing_operation(branch_id, |costs| {
            costs.retained_history_head_lookups =
                costs.retained_history_head_lookups.saturating_add(1);
        });
    }

    pub(crate) fn record_candidate_preparation(&mut self, branch_id: &BranchId) {
        self.record_branch_sharing_operation(branch_id, |costs| {
            costs.candidate_preparations = costs.candidate_preparations.saturating_add(1);
        });
    }

    pub(crate) fn record_candidate_discard(&mut self, branch_id: &BranchId) {
        self.record_branch_sharing_operation(branch_id, |costs| {
            costs.candidate_discards = costs.candidate_discards.saturating_add(1);
        });
    }

    pub(crate) fn record_publication_attempt(&mut self, branch_id: &BranchId) {
        self.record_branch_sharing_operation(branch_id, |costs| {
            costs.publication_attempts = costs.publication_attempts.saturating_add(1);
        });
    }

    pub(crate) fn record_fork_materialization(
        &mut self,
        branch_id: &BranchId,
        cost: RelationalForkMaterializationCost,
    ) {
        self.record_branch_sharing_operation(branch_id, |costs| {
            costs.copied_truth_bytes = costs
                .copied_truth_bytes
                .saturating_add(cost.authoritative_bytes);
            costs.copied_commit_envelopes = costs
                .copied_commit_envelopes
                .saturating_add(cost.copied_commit_envelopes);
            costs.fork_materialized_entity_count = costs
                .fork_materialized_entity_count
                .saturating_add(cost.entity_count);
            costs.fork_materialized_relation_count = costs
                .fork_materialized_relation_count
                .saturating_add(cost.relation_count);
            costs.fork_materialized_authoritative_bytes = costs
                .fork_materialized_authoritative_bytes
                .saturating_add(cost.authoritative_bytes);
        });
    }

    pub(crate) fn record_root_publication(
        &mut self,
        branch_id: &BranchId,
        root: &RelationalBranchRoot,
        new_authoritative_bytes: u64,
    ) {
        let cost = root.publication_cost();
        self.record_branch_sharing_operation(branch_id, |costs| {
            costs.copied_truth_bytes = costs
                .copied_truth_bytes
                .saturating_add(cost.copied_truth_bytes);
            costs.copied_commit_envelopes = costs
                .copied_commit_envelopes
                .saturating_add(cost.copied_commit_envelopes);
            costs.publication_touched_region_count = costs
                .publication_touched_region_count
                .saturating_add(cost.touched_regions);
            costs.publication_reused_region_count = costs
                .publication_reused_region_count
                .saturating_add(cost.reused_regions);
            costs.publication_persistent_index_path_nodes = costs
                .publication_persistent_index_path_nodes
                .saturating_add(cost.persistent_index_path_nodes);
            costs.publication_new_authoritative_bytes = costs
                .publication_new_authoritative_bytes
                .saturating_add(new_authoritative_bytes);
        });
    }

    pub(crate) fn sharing_costs_for_branch(
        &self,
        branch_id: &BranchId,
    ) -> RelationalBranchSharingCostCounters {
        let Some(identity) = self.branch_cell(branch_id).map(|cell| cell.identity()) else {
            return RelationalBranchSharingCostCounters::default();
        };
        self.branch_sharing_costs
            .get(identity)
            .copied()
            .unwrap_or_default()
    }

    fn record_branch_sharing_operation(
        &mut self,
        branch_id: &BranchId,
        mut record: impl FnMut(&mut RelationalBranchSharingCostCounters),
    ) {
        record(&mut self.sharing_costs);
        let Some(identity) = self
            .branch_cell(branch_id)
            .map(|cell| cell.identity().clone())
        else {
            return;
        };
        record(self.branch_sharing_costs.entry(identity).or_default());
    }
}
