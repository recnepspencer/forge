use super::IoSchedulerS6ReadinessDenial;
use forge_store_physical_isolation::{
    PhysicalIsolationCounterSnapshot, PhysicalStabilityAssumption, S6IoQosIsolationReadiness,
    UnsupportedQoSClaim,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IoSchedulerPhysicalStabilityAssumption {
    StableReadPlansAreStorePublished,
    LatchOrderingPreventsExecutionTimeRaceDiscovery,
    EpochScopesBoundPublishedRoots,
    ReclaimWaitsForReachabilitySafety,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IoSchedulerUnsupportedQosNonClaim {
    P99Latency,
    P999Latency,
    HardwareQueueDepth,
    MediaQoS,
    BackgroundWorkPacing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoSchedulerS6CounterSnapshot {
    wait_count: u64,
    retry_count: u64,
    latch_counter_rows: u64,
    reclaim_counter_rows: u64,
    blocked_maintenance_count: u64,
    protected_byte_footprint: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoSchedulerForegroundInterferenceSurface {
    wait_count: u64,
    retry_count: u64,
    protected_byte_footprint: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoSchedulerBackgroundMaintenanceAssumption {
    blocked_maintenance_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoSchedulerS6ReadinessRequest {
    assumptions: [IoSchedulerPhysicalStabilityAssumption; 4],
    counters: IoSchedulerS6CounterSnapshot,
    non_claims: [IoSchedulerUnsupportedQosNonClaim; 5],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IoSchedulerS6ReadinessAdmission {
    assumptions: [IoSchedulerPhysicalStabilityAssumption; 4],
    foreground_interference: IoSchedulerForegroundInterferenceSurface,
    background_maintenance: IoSchedulerBackgroundMaintenanceAssumption,
    counters: IoSchedulerS6CounterSnapshot,
}

pub fn admit_s6_io_qos_isolation_readiness(
    request: IoSchedulerS6ReadinessRequest,
) -> Result<IoSchedulerS6ReadinessAdmission, IoSchedulerS6ReadinessDenial> {
    require_counters(request.counters)?;
    require_non_claims(&request.non_claims)?;
    Ok(IoSchedulerS6ReadinessAdmission {
        assumptions: request.assumptions,
        foreground_interference: IoSchedulerForegroundInterferenceSurface::from_counters(
            request.counters,
        ),
        background_maintenance: IoSchedulerBackgroundMaintenanceAssumption::from_counters(
            request.counters,
        ),
        counters: request.counters,
    })
}

pub fn admit_store_published_s6_io_qos_isolation_readiness(
    readiness: &S6IoQosIsolationReadiness,
) -> Result<IoSchedulerS6ReadinessAdmission, IoSchedulerS6ReadinessDenial> {
    admit_s6_io_qos_isolation_readiness(
        IoSchedulerS6ReadinessRequest::from_store_published_readiness(readiness),
    )
}

pub const fn reject_log_or_metric_projection_as_s6_readiness(
) -> Result<(), IoSchedulerS6ReadinessDenial> {
    Err(IoSchedulerS6ReadinessDenial::LogOrMetricProjection)
}

pub const fn reject_hardware_queue_depth_claim_as_s6_readiness(
) -> Result<(), IoSchedulerS6ReadinessDenial> {
    Err(IoSchedulerS6ReadinessDenial::HardwareQueueDepthClaim)
}

pub const fn reject_media_qos_claim_as_s6_readiness() -> Result<(), IoSchedulerS6ReadinessDenial> {
    Err(IoSchedulerS6ReadinessDenial::MediaQosClaim)
}

impl IoSchedulerPhysicalStabilityAssumption {
    pub const fn required_from_s5() -> [Self; 4] {
        [
            Self::StableReadPlansAreStorePublished,
            Self::LatchOrderingPreventsExecutionTimeRaceDiscovery,
            Self::EpochScopesBoundPublishedRoots,
            Self::ReclaimWaitsForReachabilitySafety,
        ]
    }
}

impl IoSchedulerUnsupportedQosNonClaim {
    pub const fn required_from_s5() -> [Self; 5] {
        [
            Self::P99Latency,
            Self::P999Latency,
            Self::HardwareQueueDepth,
            Self::MediaQoS,
            Self::BackgroundWorkPacing,
        ]
    }
}

impl IoSchedulerS6ReadinessRequest {
    fn from_store_published_readiness(readiness: &S6IoQosIsolationReadiness) -> Self {
        Self {
            assumptions: scheduler_assumptions(readiness.assumptions()),
            counters: IoSchedulerS6CounterSnapshot::from_store_published_counters(
                readiness.counters(),
            ),
            non_claims: scheduler_non_claims(readiness.unsupported_qos_claims()),
        }
    }
}

impl IoSchedulerS6CounterSnapshot {
    const fn from_store_published_counters(counters: PhysicalIsolationCounterSnapshot) -> Self {
        Self {
            wait_count: counters.wait_count(),
            retry_count: counters.retry_count(),
            latch_counter_rows: counters.latch_counter_rows(),
            reclaim_counter_rows: counters.reclaim_counter_rows(),
            blocked_maintenance_count: counters.blocked_maintenance_count(),
            protected_byte_footprint: counters.protected_byte_footprint(),
        }
    }

    pub const fn wait_count(self) -> u64 {
        self.wait_count
    }

    pub const fn retry_count(self) -> u64 {
        self.retry_count
    }

    pub const fn latch_counter_rows(self) -> u64 {
        self.latch_counter_rows
    }

    pub const fn reclaim_counter_rows(self) -> u64 {
        self.reclaim_counter_rows
    }

    pub const fn blocked_maintenance_count(self) -> u64 {
        self.blocked_maintenance_count
    }

    pub const fn protected_byte_footprint(self) -> u64 {
        self.protected_byte_footprint
    }
}

impl IoSchedulerS6ReadinessAdmission {
    pub const fn physical_stability_assumptions(
        &self,
    ) -> &[IoSchedulerPhysicalStabilityAssumption; 4] {
        &self.assumptions
    }

    pub const fn foreground_interference(&self) -> IoSchedulerForegroundInterferenceSurface {
        self.foreground_interference
    }

    pub const fn background_maintenance(&self) -> IoSchedulerBackgroundMaintenanceAssumption {
        self.background_maintenance
    }

    pub const fn counters(&self) -> IoSchedulerS6CounterSnapshot {
        self.counters
    }
}

const fn scheduler_assumptions(
    assumptions: &[PhysicalStabilityAssumption; 4],
) -> [IoSchedulerPhysicalStabilityAssumption; 4] {
    [
        scheduler_assumption(assumptions[0]),
        scheduler_assumption(assumptions[1]),
        scheduler_assumption(assumptions[2]),
        scheduler_assumption(assumptions[3]),
    ]
}

const fn scheduler_assumption(
    assumption: PhysicalStabilityAssumption,
) -> IoSchedulerPhysicalStabilityAssumption {
    match assumption {
        PhysicalStabilityAssumption::StableReadPlansAreStorePublished => {
            IoSchedulerPhysicalStabilityAssumption::StableReadPlansAreStorePublished
        }
        PhysicalStabilityAssumption::LatchOrderingPreventsExecutionTimeRaceDiscovery => {
            IoSchedulerPhysicalStabilityAssumption::LatchOrderingPreventsExecutionTimeRaceDiscovery
        }
        PhysicalStabilityAssumption::EpochScopesBoundPublishedRoots => {
            IoSchedulerPhysicalStabilityAssumption::EpochScopesBoundPublishedRoots
        }
        PhysicalStabilityAssumption::ReclaimWaitsForReachabilitySafety => {
            IoSchedulerPhysicalStabilityAssumption::ReclaimWaitsForReachabilitySafety
        }
    }
}

const fn scheduler_non_claims(
    non_claims: &[UnsupportedQoSClaim; 5],
) -> [IoSchedulerUnsupportedQosNonClaim; 5] {
    [
        scheduler_non_claim(non_claims[0]),
        scheduler_non_claim(non_claims[1]),
        scheduler_non_claim(non_claims[2]),
        scheduler_non_claim(non_claims[3]),
        scheduler_non_claim(non_claims[4]),
    ]
}

const fn scheduler_non_claim(non_claim: UnsupportedQoSClaim) -> IoSchedulerUnsupportedQosNonClaim {
    match non_claim {
        UnsupportedQoSClaim::P99Latency => IoSchedulerUnsupportedQosNonClaim::P99Latency,
        UnsupportedQoSClaim::P999Latency => IoSchedulerUnsupportedQosNonClaim::P999Latency,
        UnsupportedQoSClaim::HardwareQueueDepth => {
            IoSchedulerUnsupportedQosNonClaim::HardwareQueueDepth
        }
        UnsupportedQoSClaim::MediaQoS => IoSchedulerUnsupportedQosNonClaim::MediaQoS,
        UnsupportedQoSClaim::BackgroundWorkPacing => {
            IoSchedulerUnsupportedQosNonClaim::BackgroundWorkPacing
        }
    }
}

impl IoSchedulerForegroundInterferenceSurface {
    const fn from_counters(counters: IoSchedulerS6CounterSnapshot) -> Self {
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

impl IoSchedulerBackgroundMaintenanceAssumption {
    const fn from_counters(counters: IoSchedulerS6CounterSnapshot) -> Self {
        Self {
            blocked_maintenance_count: counters.blocked_maintenance_count(),
        }
    }

    pub const fn blocked_maintenance_count(self) -> u64 {
        self.blocked_maintenance_count
    }
}

fn require_counters(
    counters: IoSchedulerS6CounterSnapshot,
) -> Result<(), IoSchedulerS6ReadinessDenial> {
    if counters.latch_counter_rows() == 0 {
        return Err(IoSchedulerS6ReadinessDenial::MissingLatchCounters);
    }
    if counters.reclaim_counter_rows() == 0 {
        return Err(IoSchedulerS6ReadinessDenial::MissingReclaimCounters);
    }
    if counters.protected_byte_footprint() == 0 {
        return Err(IoSchedulerS6ReadinessDenial::MissingProtectedByteFootprint);
    }
    Ok(())
}

fn require_non_claims(
    non_claims: &[IoSchedulerUnsupportedQosNonClaim; 5],
) -> Result<(), IoSchedulerS6ReadinessDenial> {
    for required in IoSchedulerUnsupportedQosNonClaim::required_from_s5() {
        if !non_claims.contains(&required) {
            return Err(IoSchedulerS6ReadinessDenial::MissingUnsupportedQosNonClaim);
        }
    }
    Ok(())
}
