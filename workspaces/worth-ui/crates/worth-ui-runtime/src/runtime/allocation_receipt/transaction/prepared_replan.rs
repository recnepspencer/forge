#[derive(Debug)]
pub(in crate::runtime) enum UiAllocationLedgerPreparation {
    Resolved(super::UiAllocationReplanTransactionOutcome),
    Prepared(UiPreparedAllocationLedgerTransition),
}

#[derive(Debug)]
pub(in crate::runtime) struct UiPreparedAllocationLedgerTransition {
    pub(super) predecessor: super::ledger_state::UiAllocationReceiptLedgerState,
    pub(super) successor: super::ledger_state::UiAllocationReceiptLedgerState,
    pub(super) committed: super::UiCommittedAllocationReplan,
}

impl From<super::UiAllocationReplanTransactionOutcome> for UiAllocationLedgerPreparation {
    fn from(outcome: super::UiAllocationReplanTransactionOutcome) -> Self {
        Self::Resolved(outcome)
    }
}

impl UiPreparedAllocationLedgerTransition {
    pub(super) fn new(
        predecessor: super::ledger_state::UiAllocationReceiptLedgerState,
        successor: super::ledger_state::UiAllocationReceiptLedgerState,
        committed: super::UiCommittedAllocationReplan,
    ) -> Self {
        Self {
            predecessor,
            successor,
            committed,
        }
    }

    pub(in crate::runtime) fn committed(&self) -> &super::UiCommittedAllocationReplan {
        &self.committed
    }

    pub(in crate::runtime) fn with_committed(
        mut self,
        committed: super::UiCommittedAllocationReplan,
    ) -> Self {
        let key = committed.transaction().idempotency_key();
        if let Some(bucket) = self.successor.completed_transactions.get_mut(&key) {
            if let Some(retained) = bucket.iter_mut().find(|retained| {
                retained
                    .transaction()
                    .same_idempotency_basis(committed.transaction())
            }) {
                *retained = committed.clone();
            }
        }
        self.committed = committed;
        self
    }
}
