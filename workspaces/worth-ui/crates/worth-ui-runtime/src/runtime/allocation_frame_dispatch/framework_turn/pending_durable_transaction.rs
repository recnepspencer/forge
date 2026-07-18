#[derive(Debug)]
pub(in crate::runtime::allocation_frame_dispatch) struct UiPendingDurableResizeCommitPort<'runtime>
{
    ledger: &'runtime crate::runtime::allocation_receipt::UiAllocationReceiptLedger,
    invalidation_authority: &'runtime std::cell::RefCell<
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    >,
    pending: super::allocation_transaction::UiPendingAllocationTransaction,
}

impl<'runtime> UiPendingDurableResizeCommitPort<'runtime> {
    pub(super) fn from_planned_transaction(
        ledger: &'runtime crate::runtime::allocation_receipt::UiAllocationReceiptLedger,
        invalidation_authority: &'runtime std::cell::RefCell<
            crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
        >,
        pending: super::allocation_transaction::UiPendingAllocationTransaction,
    ) -> Self {
        Self {
            ledger,
            invalidation_authority,
            pending,
        }
    }

    pub(super) fn commit(
        self,
    ) -> (
        crate::runtime::UiAllocationReplanTransactionOutcome,
        Option<crate::runtime::UiAllocationDurableSemanticState>,
        bool,
    ) {
        let previous = self.ledger.durable_semantic_state();
        let outcome = super::allocation_transaction::publish_pending(
            self.ledger,
            &mut self.invalidation_authority.borrow_mut(),
            self.pending,
        );
        let state = self.ledger.durable_semantic_state();
        let mutated = matches!(
            outcome,
            crate::runtime::UiAllocationReplanTransactionOutcome::Committed(_)
        ) && previous != state;
        (outcome, state, mutated)
    }
}
