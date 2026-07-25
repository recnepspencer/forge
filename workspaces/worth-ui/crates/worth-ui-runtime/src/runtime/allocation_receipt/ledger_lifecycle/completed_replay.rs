impl super::UiAllocationReceiptLedger {
    pub(in crate::runtime) fn completed_replay(
        &self,
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
        resize: Option<&crate::runtime::UiResizeAllocationPlanningBasis>,
    ) -> Option<super::UiAllocationReplanTransactionOutcome> {
        let state = self.state.borrow();
        let transaction = super::UiAllocationReplanTransaction::from_graph_basis(
            selection.transaction_basis(),
            state.next_transaction_generation,
            state.runtime_generation,
        )
        .ok()?;
        let completed = state
            .completed_transactions
            .get(&transaction.idempotency_key())?
            .iter()
            .find(|completed| {
                completed.transaction().same_idempotency_basis(&transaction)
                    && receipts_match_resize_mode(completed.receipts(), resize)
            })?
            .clone();
        Some(super::UiAllocationReplanTransactionOutcome::Replayed(
            completed,
        ))
    }
}

fn receipts_match_resize_mode(
    receipts: &[super::UiAllocationReceipt],
    resize: Option<&crate::runtime::UiResizeAllocationPlanningBasis>,
) -> bool {
    !receipts.is_empty()
        && receipts.iter().all(|receipt| match resize {
            Some(expected) => receipt.resize_basis() == Some(expected),
            None => receipt.resize_basis().is_none(),
        })
}
