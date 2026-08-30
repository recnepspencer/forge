use super::{WorthQueryPrimaryGraphProvider, WorthQueryProviderIdempotencyResolution};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationSettlementRecoveryError,
};

impl WorthQueryPrimaryGraphProvider {
    pub(super) fn repair_equivalent_publication_settlement(
        &self,
        runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
        committed: &super::WorthQueryPrimaryGraphCommittedApplication,
    ) -> Result<(), super::WorthQueryProviderIdempotencyResolutionDenial> {
        runtime
            .repair_pending_publication_settlement(committed.commit_reference().commit_id)
            .map(|_| ())
            .map_err(|_| super::WorthQueryProviderIdempotencyResolutionDenial::Unavailable)
    }

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
                .map_err(settlement_index_currency_denial)?;
            let branch_identity = runtime.branch_identity(branch).map_err(|_| {
                WorthQueryApplicationSettlementRecoveryError::Publication(
                    "application publication branch is unavailable during recovery",
                )
            })?;
            let (_, basis) = runtime.observe_branch(&branch_identity).map_err(|denial| match denial {
                worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionCapacityExhausted => {
                    WorthQueryApplicationSettlementRecoveryError::RetentionCapacityExhausted
                }
                worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionIdentityExhausted => {
                    WorthQueryApplicationSettlementRecoveryError::RetentionIdentityExhausted
                }
                worth_relational::facade::branch::RelationalBranchBasisDenial::SnapshotIdentityExhausted => {
                    WorthQueryApplicationSettlementRecoveryError::SnapshotIdentityExhausted
                }
                _ => WorthQueryApplicationSettlementRecoveryError::Publication(
                    "application publication basis is unavailable during recovery",
                ),
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
                .map_err(|denial| match denial {
                    worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionCapacityExhausted => {
                        WorthQueryApplicationSettlementRecoveryError::RetentionCapacityExhausted
                    }
                    worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionIdentityExhausted => {
                        WorthQueryApplicationSettlementRecoveryError::RetentionIdentityExhausted
                    }
                    worth_relational::facade::branch::RelationalBranchBasisDenial::SnapshotIdentityExhausted => {
                        WorthQueryApplicationSettlementRecoveryError::SnapshotIdentityExhausted
                    }
                    _ => WorthQueryApplicationSettlementRecoveryError::Publication(
                        "application publication head could not bind to Bridge during recovery",
                    ),
                })?;
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
            Err(super::WorthQueryProviderIdempotencyResolutionDenial::ActiveSnapshotCapacityExhausted {
                maximum_active_snapshots,
            }) => Err(WorthQueryApplicationSettlementRecoveryError::ActiveSnapshotCapacityExhausted {
                maximum_active_snapshots,
            }),
            Err(super::WorthQueryProviderIdempotencyResolutionDenial::Unavailable) => {
                Err(WorthQueryApplicationSettlementRecoveryError::Publication(
                    "application idempotency evidence is unavailable during settlement recovery",
                ))
            }
            Err(super::WorthQueryProviderIdempotencyResolutionDenial::RetentionCapacityExhausted) => {
                Err(WorthQueryApplicationSettlementRecoveryError::RetentionCapacityExhausted)
            }
            Err(super::WorthQueryProviderIdempotencyResolutionDenial::RetentionIdentityExhausted) => {
                Err(WorthQueryApplicationSettlementRecoveryError::RetentionIdentityExhausted)
            }
            Err(super::WorthQueryProviderIdempotencyResolutionDenial::SnapshotIdentityExhausted) => {
                Err(WorthQueryApplicationSettlementRecoveryError::SnapshotIdentityExhausted)
            }
        }
    }
}

fn settlement_index_currency_denial(
    denial: crate::domain_computation::primary_graph::index_currency::WorthQueryPrimaryIndexCurrencyDenial,
) -> WorthQueryApplicationSettlementRecoveryError {
    match denial {
        crate::domain_computation::primary_graph::index_currency::WorthQueryPrimaryIndexCurrencyDenial::Basis(
            crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::RetentionCapacityExhausted,
        ) => WorthQueryApplicationSettlementRecoveryError::RetentionCapacityExhausted,
        crate::domain_computation::primary_graph::index_currency::WorthQueryPrimaryIndexCurrencyDenial::Basis(
            crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::RetentionIdentityExhausted,
        ) => WorthQueryApplicationSettlementRecoveryError::RetentionIdentityExhausted,
        crate::domain_computation::primary_graph::index_currency::WorthQueryPrimaryIndexCurrencyDenial::Basis(
            crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::SnapshotIdentityExhausted,
        ) => WorthQueryApplicationSettlementRecoveryError::SnapshotIdentityExhausted,
        crate::domain_computation::primary_graph::index_currency::WorthQueryPrimaryIndexCurrencyDenial::IndexUnavailable(detail) => {
            WorthQueryApplicationSettlementRecoveryError::Publication(detail)
        }
        _ => WorthQueryApplicationSettlementRecoveryError::Publication(
            "application publication branch basis is unavailable during recovery",
        ),
    }
}
