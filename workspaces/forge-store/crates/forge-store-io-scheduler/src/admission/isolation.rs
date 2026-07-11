use crate::IoSchedulerIsolationAdmissionDenial;
use forge_store_physical_isolation::{
    PhysicalIsolationCounterSnapshot, PhysicalStabilityAssumption, SchedulerIsolationCapability,
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
pub struct IoSchedulerIsolationCounterSnapshot {
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
pub struct IoSchedulerIsolationAdmissionRequest {
    assumptions: [IoSchedulerPhysicalStabilityAssumption; 4],
    counters: IoSchedulerIsolationCounterSnapshot,
    non_claims: [IoSchedulerUnsupportedQosNonClaim; 5],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IoSchedulerIsolationAdmission {
    assumptions: [IoSchedulerPhysicalStabilityAssumption; 4],
    foreground_interference: IoSchedulerForegroundInterferenceSurface,
    background_maintenance: IoSchedulerBackgroundMaintenanceAssumption,
    counters: IoSchedulerIsolationCounterSnapshot,
}

#[cfg(any(test, feature = "certification-test-authority"))]
pub fn admit_isolation_for_certification_test(
    request: IoSchedulerIsolationAdmissionRequest,
) -> Result<IoSchedulerIsolationAdmission, IoSchedulerIsolationAdmissionDenial> {
    require_counters(request.counters)?;
    verify_s6_non_claims(&request.non_claims)?;
    Ok(IoSchedulerIsolationAdmission {
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

pub fn admit_store_published_isolation_capability(
    readiness: &SchedulerIsolationCapability,
) -> Result<IoSchedulerIsolationAdmission, IoSchedulerIsolationAdmissionDenial> {
    let request = collect_store_published_readiness_evidence(readiness);
    require_counters(request.counters)?;
    verify_s6_non_claims(&request.non_claims)?;
    let foreground_interference = project_scheduler_foreground_interference(request.counters);
    let background_maintenance = project_scheduler_background_maintenance(request.counters);
    Ok(assemble_s6_readiness_admission(
        request.assumptions,
        foreground_interference,
        background_maintenance,
        request.counters,
    ))
}

pub const fn reject_log_or_metric_projection_as_isolation_admission(
) -> Result<(), IoSchedulerIsolationAdmissionDenial> {
    Err(IoSchedulerIsolationAdmissionDenial::LogOrMetricProjection)
}

pub const fn reject_hardware_queue_depth_claim_as_isolation_admission(
) -> Result<(), IoSchedulerIsolationAdmissionDenial> {
    Err(IoSchedulerIsolationAdmissionDenial::HardwareQueueDepthClaim)
}

pub const fn reject_media_qos_claim_as_isolation_admission(
) -> Result<(), IoSchedulerIsolationAdmissionDenial> {
    Err(IoSchedulerIsolationAdmissionDenial::MediaQosClaim)
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

impl IoSchedulerIsolationAdmissionRequest {
    fn from_store_published_readiness(readiness: &SchedulerIsolationCapability) -> Self {
        Self {
            assumptions: scheduler_assumptions(readiness.assumptions()),
            counters: IoSchedulerIsolationCounterSnapshot::from_store_published_counters(
                readiness.counters(),
            ),
            non_claims: scheduler_non_claims(readiness.unsupported_qos_claims()),
        }
    }
}

impl IoSchedulerIsolationCounterSnapshot {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_test(
        wait_count: u64,
        retry_count: u64,
        latch_counter_rows: u64,
        reclaim_counter_rows: u64,
        blocked_maintenance_count: u64,
        protected_byte_footprint: u64,
    ) -> Self {
        Self {
            wait_count,
            retry_count,
            latch_counter_rows,
            reclaim_counter_rows,
            blocked_maintenance_count,
            protected_byte_footprint,
        }
    }

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

impl IoSchedulerIsolationAdmission {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_test() -> Self {
        Self {
            assumptions: IoSchedulerPhysicalStabilityAssumption::required_from_s5(),
            foreground_interference:
                IoSchedulerForegroundInterferenceSurface::for_certification_test(1, 1, 1),
            background_maintenance:
                IoSchedulerBackgroundMaintenanceAssumption::for_certification_test(1),
            counters: IoSchedulerIsolationCounterSnapshot::for_certification_test(1, 1, 1, 1, 1, 1),
        }
    }

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

    pub const fn counters(&self) -> IoSchedulerIsolationCounterSnapshot {
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
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_test(
        wait_count: u64,
        retry_count: u64,
        protected_byte_footprint: u64,
    ) -> Self {
        Self {
            wait_count,
            retry_count,
            protected_byte_footprint,
        }
    }

    const fn from_counters(counters: IoSchedulerIsolationCounterSnapshot) -> Self {
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
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn for_certification_test(blocked_maintenance_count: u64) -> Self {
        Self {
            blocked_maintenance_count,
        }
    }

    const fn from_counters(counters: IoSchedulerIsolationCounterSnapshot) -> Self {
        Self {
            blocked_maintenance_count: counters.blocked_maintenance_count(),
        }
    }

    pub const fn blocked_maintenance_count(self) -> u64 {
        self.blocked_maintenance_count
    }
}

fn require_counters(
    counters: IoSchedulerIsolationCounterSnapshot,
) -> Result<(), IoSchedulerIsolationAdmissionDenial> {
    if counters.latch_counter_rows() == 0 {
        return Err(IoSchedulerIsolationAdmissionDenial::MissingLatchCounters);
    }
    if counters.reclaim_counter_rows() == 0 {
        return Err(IoSchedulerIsolationAdmissionDenial::MissingReclaimCounters);
    }
    if counters.protected_byte_footprint() == 0 {
        return Err(IoSchedulerIsolationAdmissionDenial::MissingProtectedByteFootprint);
    }
    Ok(())
}

fn collect_store_published_readiness_evidence(
    readiness: &SchedulerIsolationCapability,
) -> IoSchedulerIsolationAdmissionRequest {
    IoSchedulerIsolationAdmissionRequest::from_store_published_readiness(readiness)
}

fn verify_s6_non_claims(
    non_claims: &[IoSchedulerUnsupportedQosNonClaim; 5],
) -> Result<(), IoSchedulerIsolationAdmissionDenial> {
    for required in IoSchedulerUnsupportedQosNonClaim::required_from_s5() {
        if !non_claims.contains(&required) {
            return Err(IoSchedulerIsolationAdmissionDenial::MissingUnsupportedQosNonClaim);
        }
    }
    Ok(())
}

fn project_scheduler_foreground_interference(
    counters: IoSchedulerIsolationCounterSnapshot,
) -> IoSchedulerForegroundInterferenceSurface {
    IoSchedulerForegroundInterferenceSurface::from_counters(counters)
}

fn project_scheduler_background_maintenance(
    counters: IoSchedulerIsolationCounterSnapshot,
) -> IoSchedulerBackgroundMaintenanceAssumption {
    IoSchedulerBackgroundMaintenanceAssumption::from_counters(counters)
}

fn assemble_s6_readiness_admission(
    assumptions: [IoSchedulerPhysicalStabilityAssumption; 4],
    foreground_interference: IoSchedulerForegroundInterferenceSurface,
    background_maintenance: IoSchedulerBackgroundMaintenanceAssumption,
    counters: IoSchedulerIsolationCounterSnapshot,
) -> IoSchedulerIsolationAdmission {
    IoSchedulerIsolationAdmission {
        assumptions,
        foreground_interference,
        background_maintenance,
        counters,
    }
}
