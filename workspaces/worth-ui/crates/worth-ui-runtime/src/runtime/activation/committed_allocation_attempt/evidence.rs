#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCommittedAllocationActivationDenialEvidence {
    attempt_identity_digest: u64,
    committed_row_count: usize,
    counters: super::UiCommittedAllocationActivationCounters,
    live_state_unchanged: bool,
}

impl UiCommittedAllocationActivationDenialEvidence {
    pub(super) fn unchanged(
        identity: &super::UiCommittedAllocationActivationIdentity,
        counters: super::UiCommittedAllocationActivationCounters,
    ) -> Self {
        Self {
            attempt_identity_digest: identity.structural_digest(),
            committed_row_count: identity.committed_row_count(),
            counters,
            live_state_unchanged: true,
        }
    }

    pub fn attempt_identity_digest(&self) -> u64 {
        self.attempt_identity_digest
    }
    pub fn committed_row_count(&self) -> usize {
        self.committed_row_count
    }
    pub fn counters(&self) -> super::UiCommittedAllocationActivationCounters {
        self.counters
    }
    pub fn live_state_unchanged(&self) -> bool {
        self.live_state_unchanged
    }
}
