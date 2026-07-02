#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LatchWaitCounterSnapshot {
    attempt_count: u64,
    wait_count: u64,
    denied_upgrade_count: u64,
    detected_cycle_count: u64,
    execution_time_discovery_denial_count: u64,
}

impl LatchWaitCounterSnapshot {
    pub const fn empty() -> Self {
        Self {
            attempt_count: 0,
            wait_count: 0,
            denied_upgrade_count: 0,
            detected_cycle_count: 0,
            execution_time_discovery_denial_count: 0,
        }
    }

    pub const fn from_exact_counts(
        attempt_count: u64,
        wait_count: u64,
        denied_upgrade_count: u64,
        detected_cycle_count: u64,
        execution_time_discovery_denial_count: u64,
    ) -> Self {
        Self {
            attempt_count,
            wait_count,
            denied_upgrade_count,
            detected_cycle_count,
            execution_time_discovery_denial_count,
        }
    }

    pub const fn with_attempts(self, attempt_count: u64) -> Self {
        Self {
            attempt_count: self.attempt_count + attempt_count,
            ..self
        }
    }

    pub const fn with_wait(self) -> Self {
        Self {
            wait_count: self.wait_count + 1,
            ..self
        }
    }

    pub const fn with_waits(self, wait_count: u64) -> Self {
        Self {
            wait_count: self.wait_count + wait_count,
            ..self
        }
    }

    pub const fn with_denied_upgrade(self) -> Self {
        Self {
            denied_upgrade_count: self.denied_upgrade_count + 1,
            ..self
        }
    }

    pub const fn with_detected_cycle(self) -> Self {
        Self {
            detected_cycle_count: self.detected_cycle_count + 1,
            ..self
        }
    }

    pub const fn with_execution_time_discovery_denial(self) -> Self {
        Self {
            execution_time_discovery_denial_count: self.execution_time_discovery_denial_count + 1,
            ..self
        }
    }

    pub const fn attempt_count(self) -> u64 {
        self.attempt_count
    }

    pub const fn wait_count(self) -> u64 {
        self.wait_count
    }

    pub const fn denied_upgrade_count(self) -> u64 {
        self.denied_upgrade_count
    }

    pub const fn detected_cycle_count(self) -> u64 {
        self.detected_cycle_count
    }

    pub const fn execution_time_discovery_denial_count(self) -> u64 {
        self.execution_time_discovery_denial_count
    }
}
