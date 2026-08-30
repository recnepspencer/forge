#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RecoveryIntegrityIngressCounters {
    admitted: u64,
    rejected: u64,
}

impl RecoveryIntegrityIngressCounters {
    pub(crate) const fn admitted(self) -> u64 {
        self.admitted
    }

    pub(crate) const fn rejected(self) -> u64 {
        self.rejected
    }

    pub(crate) fn record_admitted(&mut self) {
        self.admitted = self.admitted.saturating_add(1);
    }

    pub(crate) fn record_rejected(&mut self) {
        self.rejected = self.rejected.saturating_add(1);
    }
}
