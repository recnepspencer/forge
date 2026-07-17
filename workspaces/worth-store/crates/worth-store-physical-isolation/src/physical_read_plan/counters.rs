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
    pub(crate) fn from_plan(
        footprint: &super::PhysicalReadPlanFootprint,
        latch_plan: &crate::LatchAcquisitionPlan,
        retry_posture: super::PhysicalReadPlanRetryPosture,
    ) -> Self {
        let protected = footprint.protected();
        let scratch_usage = protected.scratch_usage().with_latch_lowering();
        Self {
            protected_references: protected.references().len() as u64,
            protected_ranges: protected.ranges().ranges().len() as u64,
            latch_requirements: latch_plan.steps().len() as u64,
            epoch_validations: 1 + protected.references().len() as u64,
            retry_decisions: retry_posture.retry_decisions(),
            resident_bytes: footprint.resident_bytes(),
            release_obligations: protected.references().len() as u64,
            reachability_barriers: 1,
            scratch_capacity: scratch_usage.protected_reference_capacity() as u64,
            scratch_allocations: scratch_usage.scratch_allocations(),
            allocation_events: scratch_usage.allocation_events(),
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
