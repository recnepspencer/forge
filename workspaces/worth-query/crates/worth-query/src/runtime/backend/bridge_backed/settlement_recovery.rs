use super::super::WorthQuerySettlementRecoveryBackend;
use super::WorthQueryBridgeBackedRuntimeBackend;

impl WorthQuerySettlementRecoveryBackend for WorthQueryBridgeBackedRuntimeBackend {
    fn repair_deferred_branch_merge_settlement(
        &mut self,
        deferred: &crate::ordinary::workflow::WorthQueryBranchMergeSettlementDeferred,
    ) -> Result<
        worth_relational::facade::history::RelationalCommitReceipt,
        crate::runtime::WorthQuerySettlementRepairError,
    > {
        let settlement = deferred.settlement();
        match self.primary_graph_runtime.as_ref() {
            Some(primary_graph) => primary_graph.repair_deferred_publication_settlement(settlement),
            None => self
                .relational_runtime
                .as_mut()
                .ok_or(crate::runtime::WorthQuerySettlementRepairError::RelationalOwnerUnavailable)?
                .repair_deferred_publication_settlement(settlement)
                .map_err(Into::into),
        }
    }

    fn repair_pending_branch_merge_settlement(
        &mut self,
        commit_id: worth_relational::facade::history::CommitId,
    ) -> Result<
        worth_relational::facade::history::RelationalCommitReceipt,
        crate::runtime::WorthQuerySettlementRepairError,
    > {
        match self.primary_graph_runtime.as_ref() {
            Some(primary_graph) => primary_graph.repair_pending_publication_settlement(commit_id),
            None => self
                .relational_runtime
                .as_mut()
                .ok_or(crate::runtime::WorthQuerySettlementRepairError::RelationalOwnerUnavailable)?
                .repair_pending_publication_settlement(commit_id)
                .map_err(Into::into),
        }
    }
}
