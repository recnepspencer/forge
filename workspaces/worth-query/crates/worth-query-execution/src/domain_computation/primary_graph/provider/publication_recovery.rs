use super::{WorthQueryPrimaryGraphProvider, WorthQueryProviderIdempotencyResolution};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationSettlementRecoveryError,
};

impl WorthQueryPrimaryGraphProvider {
    pub(in crate::domain_computation::primary_graph) fn recover_application_settlement(
        &self,
        settlement: &worth_relational::facade::publication::DeferredPublicationSettlement,
        branch: &worth_relational::facade::history::BranchId,
        idempotency: WorthQueryApplicationIdempotencyBinding,
    ) -> Result<
        worth_relational::facade::history::RelationalCommitReceipt,
        WorthQueryApplicationSettlementRecoveryError,
    > {
        let _serialization = self.serialize_application_commit();
        let repaired = self.graph.with_runtime_mut(|runtime| {
            let repaired = runtime
                .repair_deferred_publication_settlement(settlement)
                .map_err(WorthQueryApplicationSettlementRecoveryError::Durability)?;
            if &repaired.branch_id != branch {
                return Err(WorthQueryApplicationSettlementRecoveryError::Publication(
                    "deferred application settlement belongs to another branch",
                ));
            }
            self.graph
                .ensure_primary_indexes_current_for_branch(runtime, branch)
                .map_err(WorthQueryApplicationSettlementRecoveryError::Publication)?;
            let branch_identity = runtime.branch_identity(branch).map_err(|_| {
                WorthQueryApplicationSettlementRecoveryError::Publication(
                    "application publication branch is unavailable during recovery",
                )
            })?;
            let (_, basis) = runtime.observe_branch(&branch_identity).map_err(|_| {
                WorthQueryApplicationSettlementRecoveryError::Publication(
                    "application publication basis is unavailable during recovery",
                )
            })?;
            let current = runtime
                .history()
                .branch_head_for_observation(&basis.observation())
                .map_err(|_| {
                    WorthQueryApplicationSettlementRecoveryError::Publication(
                        "application publication basis is not owner-admitted",
                    )
                })?
                .ok_or(WorthQueryApplicationSettlementRecoveryError::Publication(
                    "application publication branch has no current commit",
                ))?;
            if !runtime
                .history()
                .ancestor_closure_by_commit_id_order(current.commit_id)
                .contains(&repaired.commit_id)
            {
                return Err(WorthQueryApplicationSettlementRecoveryError::Publication(
                    "settled application commit is not in the current branch ancestry",
                ));
            }
            self.graph
                .bind_truth_head_basis_in_runtime(runtime, &basis)
                .map_err(WorthQueryApplicationSettlementRecoveryError::Publication)?;
            Ok(repaired)
        })?;
        match self.resolve_idempotency_binding(idempotency, branch) {
            Ok(WorthQueryProviderIdempotencyResolution::Equivalent(_)) => Ok(repaired),
            Ok(WorthQueryProviderIdempotencyResolution::Absent) => {
                Err(WorthQueryApplicationSettlementRecoveryError::IdempotencyAbsent)
            }
            Ok(WorthQueryProviderIdempotencyResolution::Drift) => {
                Err(WorthQueryApplicationSettlementRecoveryError::IdempotencyDrift)
            }
            Err(detail) => Err(WorthQueryApplicationSettlementRecoveryError::Publication(
                detail,
            )),
        }
    }
}
