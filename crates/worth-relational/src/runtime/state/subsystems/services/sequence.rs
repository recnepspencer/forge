use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub(super) struct RuntimeSequenceState {
    runtime_instance_id: u64,
    next_transaction_id: AtomicU64,
    next_proposal_ordinal: u64,
    proposal_ordinals_exhausted: bool,
}

impl RuntimeSequenceState {
    pub(super) fn new() -> Self {
        static NEXT_RUNTIME_INSTANCE_ID: AtomicU64 = AtomicU64::new(1);
        Self {
            runtime_instance_id: NEXT_RUNTIME_INSTANCE_ID.fetch_add(1, Ordering::Relaxed),
            next_transaction_id: AtomicU64::new(1),
            next_proposal_ordinal: 1,
            proposal_ordinals_exhausted: false,
        }
    }

    pub(super) fn next_transaction_id(&self) -> crate::transactions::data::TransactionId {
        crate::transactions::data::TransactionId(
            self.next_transaction_id.fetch_add(1, Ordering::Relaxed),
        )
    }

    pub(super) fn next_proposal_ordinal(&mut self) -> Option<u64> {
        if self.proposal_ordinals_exhausted {
            return None;
        }
        let ordinal = self.next_proposal_ordinal;
        if let Some(next) = ordinal.checked_add(1) {
            self.next_proposal_ordinal = next;
        } else {
            self.proposal_ordinals_exhausted = true;
        }
        Some(ordinal)
    }

    pub(super) fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }
}

impl Default for RuntimeSequenceState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeSequenceState;

    #[test]
    fn proposal_ordinal_exhaustion_does_not_repeat_the_final_value() {
        let mut sequence = RuntimeSequenceState::new();
        sequence.next_proposal_ordinal = u64::MAX;

        assert_eq!(sequence.next_proposal_ordinal(), Some(u64::MAX));
        assert_eq!(sequence.next_proposal_ordinal(), None);
        assert_eq!(sequence.next_proposal_ordinal(), None);
    }
}
