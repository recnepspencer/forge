/// Read-only eligibility input for bounded history reclamation. It carries no
/// authority to remove a commit; the future catalog owner decides execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HistoryReclamationEligibility {
    age_ticks: u64,
    reachable: bool,
}

impl HistoryReclamationEligibility {
    pub(crate) const fn new(age_ticks: u64, reachable: bool) -> Self {
        Self {
            age_ticks,
            reachable,
        }
    }

    pub(crate) const fn is_eligible(self) -> bool {
        !self.reachable && self.age_ticks > 0
    }
}
