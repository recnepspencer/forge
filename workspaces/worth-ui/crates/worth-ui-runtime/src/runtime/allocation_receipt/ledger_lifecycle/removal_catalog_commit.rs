use super::{
    ledger_state::UiAllocationCatalogLedgerTransition, UiAllocationReceiptLedger,
    UiAllocationReplanTransactionCounters, UiCommittedAllocationReplan,
};

impl UiAllocationReceiptLedger {
    pub(crate) fn seal_removal_only_catalog_activation(
        &self,
        catalog: crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog,
        frame_epoch: crate::runtime::UiAllocationFrameEpoch,
        reconciliation: &crate::runtime::WorthUiDurableStateReconciliationPlan,
        affected: &[crate::evidence::UiAllocationNeighborhoodScope],
    ) -> Result<
        crate::runtime::UiCommittedAllocationActivationAttempt,
        super::UiAllocationReceiptCommitOutcome,
    > {
        let state = self.state.borrow();
        let first_scope = affected.first().ok_or_else(binding_denial)?;
        let removed_identity = state
            .committed_by_scope
            .get(first_scope)
            .map(|receipt| {
                receipt
                    .committed_allocation()
                    .allocation_neighborhood()
                    .identity()
                    .clone()
            })
            .ok_or_else(binding_denial)?;
        let generation = state.checked_transaction_generation().map_err(|denial| {
            super::UiAllocationReceiptCommitOutcome::denied(
                super::UiAllocationReceiptCommitDenial::AuthorityCounterExhausted(denial),
            )
        })?;
        let transaction = super::UiAllocationReplanTransaction::for_catalog_removal_activation(
            removed_identity,
            state.runtime_generation,
            generation,
            frame_epoch,
        );
        let counters = UiAllocationReplanTransactionCounters::preflight(0).map_err(|()| {
            super::UiAllocationReceiptCommitOutcome::denied(
                super::UiAllocationReceiptCommitDenial::EvidenceCounterExhausted,
            )
        })?;
        let bindings = super::UiCommittedAllocationCatalogBindings::seal(&[], &[])
            .map_err(super::UiAllocationReceiptCommitOutcome::denied)?;
        let outcome =
            UiCommittedAllocationReplan::new(transaction.clone(), Vec::new(), counters, bindings)
                .map_err(|()| {
                super::UiAllocationReceiptCommitOutcome::denied(
                    super::UiAllocationReceiptCommitDenial::EvidenceCounterExhausted,
                )
            })?;
        let activation = super::UiCommittedAllocationCatalogActivation::seal(
            catalog.candidates_for_commit(),
            outcome.catalog_bindings(),
        )
        .map_err(|denial| {
            super::UiAllocationReceiptCommitOutcome::denied(
                super::UiAllocationReceiptCommitDenial::catalog_activation(denial),
            )
        })?;
        let predecessor = state.clone();
        let mut successor = predecessor.clone();
        let durable = reconciliation.allocation_durable_semantic_state();
        let durable_changed = successor.durable_semantic_state.as_ref() != Some(&durable);
        successor.truth_revision =
            successor
                .checked_truth_successor(0, false, true)
                .map_err(|denial| {
                    super::UiAllocationReceiptCommitOutcome::denied(
                        super::UiAllocationReceiptCommitDenial::AuthorityCounterExhausted(denial),
                    )
                })?;
        if durable_changed {
            successor.durable_semantic_state = Some(durable);
        }
        successor.next_transaction_generation = transaction.transaction_generation();
        successor.latest_frame_epoch = Some(frame_epoch);
        successor.completed_transactions.clear();
        successor
            .completed_transactions
            .insert(transaction.idempotency_key(), vec![outcome.clone()]);
        successor.denied_transactions.clear();
        drop(state);
        Ok(crate::runtime::UiCommittedAllocationActivationAttempt::new(
            catalog,
            UiAllocationCatalogLedgerTransition {
                predecessor,
                successor,
                outcome: outcome.clone(),
                durable_reconciliation: reconciliation.clone(),
                operational_meaning_changed: true,
            },
            outcome,
            activation,
        ))
    }
}

fn binding_denial() -> super::UiAllocationReceiptCommitOutcome {
    super::UiAllocationReceiptCommitOutcome::denied(
        super::UiAllocationReceiptCommitDenial::CatalogBindingCardinalityMismatch,
    )
}
