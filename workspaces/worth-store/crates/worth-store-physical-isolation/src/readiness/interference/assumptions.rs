use crate::PhysicalIsolationCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PhysicalStabilityAssumption {
    StableReadPlansAreStorePublished,
    LatchOrderingPreventsExecutionTimeRaceDiscovery,
    EpochScopesBoundPublishedRoots,
    ReclaimWaitsForReachabilitySafety,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForegroundInterferenceSurface {
    wait_count: u64,
    retry_count: u64,
    protected_byte_footprint: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundMaintenanceIsolationAssumption {
    blocked_maintenance_count: u64,
    reclaim_block_count: u64,
}

impl PhysicalStabilityAssumption {
    pub const fn required() -> [Self; 4] {
        [
            Self::StableReadPlansAreStorePublished,
            Self::LatchOrderingPreventsExecutionTimeRaceDiscovery,
            Self::EpochScopesBoundPublishedRoots,
            Self::ReclaimWaitsForReachabilitySafety,
        ]
    }
}

impl ForegroundInterferenceSurface {
    pub const fn from_counters(counters: PhysicalIsolationCounterSnapshot) -> Self {
        Self {
            wait_count: counters.wait_count(),
            retry_count: counters.retry_count(),
            protected_byte_footprint: counters.protected_byte_footprint(),
        }
    }

    pub const fn wait_count(self) -> u64 {
        self.wait_count
    }

    pub const fn retry_count(self) -> u64 {
        self.retry_count
    }

    pub const fn protected_byte_footprint(self) -> u64 {
        self.protected_byte_footprint
    }
}

impl BackgroundMaintenanceIsolationAssumption {
    pub const fn from_counters(counters: PhysicalIsolationCounterSnapshot) -> Self {
        Self {
            blocked_maintenance_count: counters.blocked_maintenance_count(),
            reclaim_block_count: counters.reclaim_block_count(),
        }
    }

    pub const fn blocked_maintenance_count(self) -> u64 {
        self.blocked_maintenance_count
    }

    pub const fn reclaim_block_count(self) -> u64 {
        self.reclaim_block_count
    }
}
