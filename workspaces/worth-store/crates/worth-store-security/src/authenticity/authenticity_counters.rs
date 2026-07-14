#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StoreAuthenticityCheckCounterSnapshot {
    requirement_checks: u64,
    witness_observations: u64,
    verified_results: u64,
    missing_witness_denials: u64,
    stale_witness_denials: u64,
    wrong_scope_denials: u64,
    wrong_physical_identity_denials: u64,
    unavailable_denials: u64,
    unsupported_denials: u64,
    failed_denials: u64,
}

impl StoreAuthenticityCheckCounterSnapshot {
    pub const fn requirement_checks(self) -> u64 {
        self.requirement_checks
    }

    pub const fn witness_observations(self) -> u64 {
        self.witness_observations
    }

    pub const fn verified_results(self) -> u64 {
        self.verified_results
    }

    pub const fn missing_witness_denials(self) -> u64 {
        self.missing_witness_denials
    }

    pub const fn stale_witness_denials(self) -> u64 {
        self.stale_witness_denials
    }

    pub const fn wrong_scope_denials(self) -> u64 {
        self.wrong_scope_denials
    }

    pub const fn wrong_physical_identity_denials(self) -> u64 {
        self.wrong_physical_identity_denials
    }

    pub const fn unavailable_denials(self) -> u64 {
        self.unavailable_denials
    }

    pub const fn unsupported_denials(self) -> u64 {
        self.unsupported_denials
    }

    pub const fn failed_denials(self) -> u64 {
        self.failed_denials
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct StoreAuthenticityCheckCounterRecorder {
    snapshot: StoreAuthenticityCheckCounterSnapshot,
}

impl StoreAuthenticityCheckCounterRecorder {
    pub(crate) const fn new() -> Self {
        Self {
            snapshot: StoreAuthenticityCheckCounterSnapshot {
                requirement_checks: 0,
                witness_observations: 0,
                verified_results: 0,
                missing_witness_denials: 0,
                stale_witness_denials: 0,
                wrong_scope_denials: 0,
                wrong_physical_identity_denials: 0,
                unavailable_denials: 0,
                unsupported_denials: 0,
                failed_denials: 0,
            },
        }
    }

    pub(crate) const fn snapshot(self) -> StoreAuthenticityCheckCounterSnapshot {
        self.snapshot
    }

    pub(crate) fn record_requirement_check(&mut self) {
        self.snapshot.requirement_checks += 1;
    }

    pub(crate) fn record_witness_observation(&mut self) {
        self.snapshot.witness_observations += 1;
    }

    pub(crate) fn record_verified_result(&mut self) {
        self.snapshot.verified_results += 1;
    }

    pub(crate) fn record_missing_witness_denial(&mut self) {
        self.snapshot.missing_witness_denials += 1;
    }

    pub(crate) fn record_stale_witness_denial(&mut self) {
        self.snapshot.stale_witness_denials += 1;
    }

    pub(crate) fn record_wrong_scope_denial(&mut self) {
        self.snapshot.wrong_scope_denials += 1;
    }

    pub(crate) fn record_wrong_physical_identity_denial(&mut self) {
        self.snapshot.wrong_physical_identity_denials += 1;
    }

    pub(crate) fn record_unavailable_denial(&mut self) {
        self.snapshot.unavailable_denials += 1;
    }

    pub(crate) fn record_unsupported_denial(&mut self) {
        self.snapshot.unsupported_denials += 1;
    }

    pub(crate) fn record_failed_denial(&mut self) {
        self.snapshot.failed_denials += 1;
    }
}
