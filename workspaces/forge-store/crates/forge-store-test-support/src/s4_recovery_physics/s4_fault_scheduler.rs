#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaultSchedulerDriver {
    seed: u64,
    max_injected_faults: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledFault {
    seam: &'static str,
    seed: u64,
    ordinal: u16,
}

impl FaultSchedulerDriver {
    pub const fn deterministic(seed: u64) -> Self {
        Self {
            seed,
            max_injected_faults: 1,
        }
    }

    pub const fn with_fault_budget(mut self, max_injected_faults: u16) -> Self {
        self.max_injected_faults = max_injected_faults;
        self
    }

    pub const fn seed(&self) -> u64 {
        self.seed
    }

    pub const fn max_injected_faults(&self) -> u16 {
        self.max_injected_faults
    }

    pub const fn schedule_fault(&self, seam: &'static str) -> ScheduledFault {
        ScheduledFault {
            seam,
            seed: self.seed,
            ordinal: 1,
        }
    }
}

impl ScheduledFault {
    pub const fn seam(&self) -> &'static str {
        self.seam
    }

    pub const fn seed(&self) -> u64 {
        self.seed
    }

    pub const fn ordinal(&self) -> u16 {
        self.ordinal
    }
}
