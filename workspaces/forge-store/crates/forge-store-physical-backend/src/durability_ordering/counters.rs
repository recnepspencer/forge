#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreDurabilityCounterStrength {
    Exact,
    Bounded,
    Sampled,
    Derived,
    CertificationOnly,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreDurabilityCounterSnapshot {
    writes_submitted: u64,
    writes_accepted: u64,
    flushes_completed: u64,
    fdatasyncs_completed: u64,
    fsyncs_completed: u64,
    directory_syncs_completed: u64,
    renames_completed: u64,
    ordering_barriers_completed: u64,
    delayed_syncs: u64,
    failed_syncs: u64,
    denied_claims: u64,
    unsupported_claims: u64,
    unknown_claims: u64,
    stale_claims: u64,
    rebind_required_claims: u64,
    strength: StoreDurabilityCounterStrength,
}

impl StoreDurabilityCounterSnapshot {
    pub const fn new(strength: StoreDurabilityCounterStrength) -> Self {
        Self {
            writes_submitted: 0,
            writes_accepted: 0,
            flushes_completed: 0,
            fdatasyncs_completed: 0,
            fsyncs_completed: 0,
            directory_syncs_completed: 0,
            renames_completed: 0,
            ordering_barriers_completed: 0,
            delayed_syncs: 0,
            failed_syncs: 0,
            denied_claims: 0,
            unsupported_claims: 0,
            unknown_claims: 0,
            stale_claims: 0,
            rebind_required_claims: 0,
            strength,
        }
    }

    pub const fn writes_submitted(self) -> u64 {
        self.writes_submitted
    }

    pub const fn writes_accepted(self) -> u64 {
        self.writes_accepted
    }

    pub const fn flushes_completed(self) -> u64 {
        self.flushes_completed
    }

    pub const fn fdatasyncs_completed(self) -> u64 {
        self.fdatasyncs_completed
    }

    pub const fn fsyncs_completed(self) -> u64 {
        self.fsyncs_completed
    }

    pub const fn directory_syncs_completed(self) -> u64 {
        self.directory_syncs_completed
    }

    pub const fn renames_completed(self) -> u64 {
        self.renames_completed
    }

    pub const fn ordering_barriers_completed(self) -> u64 {
        self.ordering_barriers_completed
    }

    pub const fn delayed_syncs(self) -> u64 {
        self.delayed_syncs
    }

    pub const fn failed_syncs(self) -> u64 {
        self.failed_syncs
    }

    pub const fn denied_claims(self) -> u64 {
        self.denied_claims
    }

    pub const fn unsupported_claims(self) -> u64 {
        self.unsupported_claims
    }

    pub const fn unknown_claims(self) -> u64 {
        self.unknown_claims
    }

    pub const fn stale_claims(self) -> u64 {
        self.stale_claims
    }

    pub const fn rebind_required_claims(self) -> u64 {
        self.rebind_required_claims
    }

    pub const fn strength(self) -> StoreDurabilityCounterStrength {
        self.strength
    }

    pub(crate) const fn with_write_submitted(mut self) -> Self {
        self.writes_submitted += 1;
        self
    }

    pub(crate) const fn with_write_accepted(mut self) -> Self {
        self.writes_accepted += 1;
        self
    }

    pub(crate) const fn with_flush_completed(mut self) -> Self {
        self.flushes_completed += 1;
        self
    }

    pub(crate) const fn with_file_sync_completed(
        mut self,
        kind: super::StoreDurabilityFileSyncKind,
    ) -> Self {
        match kind {
            super::StoreDurabilityFileSyncKind::Fdatasync => {
                self.fdatasyncs_completed += 1;
            }
            super::StoreDurabilityFileSyncKind::Fsync => {
                self.fsyncs_completed += 1;
            }
        }
        self
    }

    pub(crate) const fn with_directory_sync_completed(mut self) -> Self {
        self.directory_syncs_completed += 1;
        self
    }

    pub(crate) const fn with_rename_completed(mut self) -> Self {
        self.renames_completed += 1;
        self
    }

    pub(crate) const fn with_ordering_barrier_completed(mut self) -> Self {
        self.ordering_barriers_completed += 1;
        self
    }

    pub const fn with_delayed_syncs(mut self, count: u64) -> Self {
        self.delayed_syncs += count;
        self
    }

    pub const fn with_delayed_sync(self) -> Self {
        self.with_delayed_syncs(1)
    }

    pub(crate) const fn with_failed_syncs(mut self, count: u64) -> Self {
        self.failed_syncs += count;
        self
    }

    pub(crate) const fn with_denied_claim(mut self) -> Self {
        self.denied_claims += 1;
        self
    }

    pub(crate) const fn with_unsupported_claim(mut self) -> Self {
        self.unsupported_claims += 1;
        self
    }

    pub(crate) const fn with_unknown_claim(mut self) -> Self {
        self.unknown_claims += 1;
        self
    }

    pub(crate) const fn with_stale_claim(mut self) -> Self {
        self.stale_claims += 1;
        self
    }

    pub(crate) const fn with_rebind_required_claim(mut self) -> Self {
        self.rebind_required_claims += 1;
        self
    }
}
