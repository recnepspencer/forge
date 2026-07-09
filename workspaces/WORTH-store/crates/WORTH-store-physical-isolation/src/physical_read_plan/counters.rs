#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReadPlanCounterSnapshot {
    protected_references: u64,
    protected_ranges: u64,
    latch_requirements: u64,
    epoch_validations: u64,
    retry_decisions: u64,
    resident_bytes: u64,
    release_obligations: u64,
    reachability_barriers: u64,
    scratch_capacity: u64,
    scratch_allocations: u64,
    allocation_events: u64,
}

impl ReadPlanCounterSnapshot {
    pub const fn new(
        protected_references: u64,
        protected_ranges: u64,
        latch_requirements: u64,
        epoch_validations: u64,
        retry_decisions: u64,
        resident_bytes: u64,
        release_obligations: u64,
        reachability_barriers: u64,
        scratch_capacity: u64,
        scratch_allocations: u64,
        allocation_events: u64,
    ) -> Self {
        Self {
            protected_references,
            protected_ranges,
            latch_requirements,
            epoch_validations,
            retry_decisions,
            resident_bytes,
            release_obligations,
            reachability_barriers,
            scratch_capacity,
            scratch_allocations,
            allocation_events,
        }
    }

    pub const fn protected_references(self) -> u64 {
        self.protected_references
    }

    pub const fn protected_ranges(self) -> u64 {
        self.protected_ranges
    }

    pub const fn latch_requirements(self) -> u64 {
        self.latch_requirements
    }

    pub const fn epoch_validations(self) -> u64 {
        self.epoch_validations
    }

    pub const fn retry_decisions(self) -> u64 {
        self.retry_decisions
    }

    pub const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }

    pub const fn release_obligations(self) -> u64 {
        self.release_obligations
    }

    pub const fn reachability_barriers(self) -> u64 {
        self.reachability_barriers
    }

    pub const fn scratch_capacity(self) -> u64 {
        self.scratch_capacity
    }

    pub const fn scratch_allocations(self) -> u64 {
        self.scratch_allocations
    }

    pub const fn allocation_events(self) -> u64 {
        self.allocation_events
    }
}
