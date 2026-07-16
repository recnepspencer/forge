/// Ledger-issued allocation truth revision. External callers may inspect but
/// cannot construct canonical revision truth.
///
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::UiAllocationTruthRevision;
/// let _forged = UiAllocationTruthRevision::default();
/// ```
///
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::UiAllocationTruthRevision;
/// let _forged = UiAllocationTruthRevision {
///     revision: 0,
///     committed_receipt_publications: 0,
///     durable_resize_mutations: 0,
///     durable_state_replacements: 0,
/// };
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationTruthRevision {
    revision: u64,
    committed_receipt_publications: u64,
    durable_resize_mutations: u64,
    durable_state_replacements: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationTruthDelta {
    committed_receipt_publications: u64,
    durable_resize_mutations: u64,
    durable_state_replacements: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationAuthorityCounter {
    TransactionGeneration,
    TruthRevision,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationAuthorityCounterExhaustion {
    counter: UiAllocationAuthorityCounter,
    increment: u64,
}

impl UiAllocationTruthRevision {
    pub(super) fn initial() -> Self {
        Self {
            revision: 0,
            committed_receipt_publications: 0,
            durable_resize_mutations: 0,
            durable_state_replacements: 0,
        }
    }
    pub(super) fn checked_successor(
        self,
        receipt_publications: usize,
        durable_resize_mutation: bool,
        durable_state_replacement: bool,
    ) -> Result<Self, UiAllocationAuthorityCounterExhaustion> {
        debug_assert!(self.invariant_holds());
        let receipts = u64::try_from(receipt_publications)
            .map_err(|_| exhaustion(UiAllocationAuthorityCounter::TruthRevision, u64::MAX))?;
        let resize = u64::from(durable_resize_mutation);
        let replacement = u64::from(durable_state_replacement);
        let total = receipts
            .checked_add(resize)
            .and_then(|v| v.checked_add(replacement))
            .ok_or_else(|| exhaustion(UiAllocationAuthorityCounter::TruthRevision, u64::MAX))?;
        let revision = self
            .revision
            .checked_add(total)
            .ok_or_else(|| exhaustion(UiAllocationAuthorityCounter::TruthRevision, total))?;
        let successor = Self {
            revision,
            committed_receipt_publications: self
                .committed_receipt_publications
                .checked_add(receipts)
                .expect("aggregate admission proves receipt counter capacity"),
            durable_resize_mutations: self
                .durable_resize_mutations
                .checked_add(resize)
                .expect("aggregate admission proves durable mutation counter capacity"),
            durable_state_replacements: self
                .durable_state_replacements
                .checked_add(replacement)
                .expect("aggregate admission proves replacement counter capacity"),
        };
        debug_assert!(successor.invariant_holds());
        Ok(successor)
    }
    pub(crate) fn delta_since(self, before: Self) -> Option<UiAllocationTruthDelta> {
        Some(UiAllocationTruthDelta {
            committed_receipt_publications: self
                .committed_receipt_publications
                .checked_sub(before.committed_receipt_publications)?,
            durable_resize_mutations: self
                .durable_resize_mutations
                .checked_sub(before.durable_resize_mutations)?,
            durable_state_replacements: self
                .durable_state_replacements
                .checked_sub(before.durable_state_replacements)?,
        })
    }
    pub fn revision(self) -> u64 {
        self.revision
    }
    pub fn committed_receipt_publications(self) -> u64 {
        self.committed_receipt_publications
    }
    pub fn durable_resize_mutations(self) -> u64 {
        self.durable_resize_mutations
    }
    pub fn durable_state_replacements(self) -> u64 {
        self.durable_state_replacements
    }
    pub(super) fn invariant_holds(self) -> bool {
        self.committed_receipt_publications
            .checked_add(self.durable_resize_mutations)
            .and_then(|sum| sum.checked_add(self.durable_state_replacements))
            == Some(self.revision)
    }
}

impl UiAllocationAuthorityCounterExhaustion {
    pub fn counter(self) -> UiAllocationAuthorityCounter {
        self.counter
    }
    pub fn increment(self) -> u64 {
        self.increment
    }
    pub(super) fn transaction_generation() -> Self {
        exhaustion(UiAllocationAuthorityCounter::TransactionGeneration, 1)
    }
}

fn exhaustion(
    counter: UiAllocationAuthorityCounter,
    increment: u64,
) -> UiAllocationAuthorityCounterExhaustion {
    UiAllocationAuthorityCounterExhaustion { counter, increment }
}

impl UiAllocationTruthDelta {
    pub(super) fn is_zero(self) -> bool {
        self.committed_receipt_publications == 0
            && self.durable_resize_mutations == 0
            && self.durable_state_replacements == 0
    }
    pub fn committed_receipt_publications(self) -> u64 {
        self.committed_receipt_publications
    }
    pub fn durable_resize_mutations(self) -> u64 {
        self.durable_resize_mutations
    }
    pub fn durable_state_replacements(self) -> u64 {
        self.durable_state_replacements
    }
}

#[cfg(test)]
impl UiAllocationTruthRevision {
    pub(super) fn position_with_remaining_capacity(self, remaining: u64) -> Option<Self> {
        if !self.invariant_holds() {
            return None;
        }
        let target = u64::MAX.checked_sub(remaining)?;
        let historical_receipts = target.checked_sub(self.revision)?;
        let positioned = Self {
            revision: target,
            committed_receipt_publications: self
                .committed_receipt_publications
                .checked_add(historical_receipts)?,
            ..self
        };
        positioned.invariant_holds().then_some(positioned)
    }
}

impl super::ledger_state::UiAllocationReceiptLedgerState {
    pub(super) fn checked_transaction_generation(
        &self,
    ) -> Result<u64, UiAllocationAuthorityCounterExhaustion> {
        self.next_transaction_generation
            .checked_add(1)
            .ok_or_else(UiAllocationAuthorityCounterExhaustion::transaction_generation)
    }
    pub(super) fn checked_truth_successor(
        &self,
        receipts: usize,
        resize: bool,
        replacement: bool,
    ) -> Result<UiAllocationTruthRevision, UiAllocationAuthorityCounterExhaustion> {
        self.truth_revision
            .checked_successor(receipts, resize, replacement)
    }
}
