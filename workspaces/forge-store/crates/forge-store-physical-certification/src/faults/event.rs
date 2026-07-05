use forge_store_recovery_physics::FreshRuntimeCrashRecoveryEvidence;

use super::denial::{require_unambiguous_locus, FaultDeliveryDenial};
use super::locus::PhysicalArtifactFaultLocus;
use crate::{PhysicalBoundarySeam, PhysicalBoundaryYieldpoint, S6IoPressureFaultKind};
use forge_store_physical_backend::ProductionStorageBoundarySeam;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalFaultEventKind {
    Crash,
    TornWrite,
    DroppedFlush,
    ReorderedPersistence,
    ByteCorruption,
    StaleGeneration,
    DelayedRelease,
    BlockedReclaim,
    ExecutionTimeReferenceDiscovery,
    UnboundedReadPlanFootprint,
    IoStall,
    NoFaultControl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalFaultEvent {
    Crash(CrashEvent),
    TornWrite(TornWriteEvent),
    DroppedFlush(DroppedFlushEvent),
    ReorderedPersistence(ReorderedPersistenceEvent),
    ByteCorruption(ByteCorruptionEvent),
    StaleGeneration(StaleGenerationEvent),
    DelayedRelease(DelayedReleaseEvent),
    BlockedReclaim(BlockedReclaimEvent),
    ExecutionTimeReferenceDiscovery(ExecutionTimeReferenceDiscoveryEvent),
    UnboundedReadPlanFootprint(UnboundedReadPlanFootprintEvent),
    IoStall(IoStallEvent),
    NoFaultControl(NoFaultControlEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashEvent {
    yieldpoint: PhysicalBoundaryYieldpoint,
    locus: PhysicalArtifactFaultLocus,
    fresh_runtime_evidence: FreshRuntimeCrashRecoveryEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TornWriteEvent {
    seam: ProductionStorageBoundarySeam,
    locus: PhysicalArtifactFaultLocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DroppedFlushEvent {
    seam: ProductionStorageBoundarySeam,
    locus: PhysicalArtifactFaultLocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReorderedPersistenceEvent {
    seam: ProductionStorageBoundarySeam,
    locus: PhysicalArtifactFaultLocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteCorruptionEvent {
    seam: ProductionStorageBoundarySeam,
    locus: PhysicalArtifactFaultLocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaleGenerationEvent {
    seam: ProductionStorageBoundarySeam,
    locus: PhysicalArtifactFaultLocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelayedReleaseEvent {
    seam: ProductionStorageBoundarySeam,
    locus: PhysicalArtifactFaultLocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockedReclaimEvent {
    seam: ProductionStorageBoundarySeam,
    locus: PhysicalArtifactFaultLocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionTimeReferenceDiscoveryEvent {
    seam: ProductionStorageBoundarySeam,
    locus: PhysicalArtifactFaultLocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnboundedReadPlanFootprintEvent {
    seam: ProductionStorageBoundarySeam,
    locus: PhysicalArtifactFaultLocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IoStallEvent {
    s6_pressure_fault_kind: Option<S6IoPressureFaultKind>,
    locus: PhysicalArtifactFaultLocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoFaultControlEvent {
    seam: ProductionStorageBoundarySeam,
    locus: PhysicalArtifactFaultLocus,
}

impl PhysicalFaultEvent {
    pub fn crash(event: CrashEvent) -> Self {
        Self::Crash(event)
    }

    pub fn torn_write(
        seam: ProductionStorageBoundarySeam,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::TornWrite(TornWriteEvent { seam, locus }))
    }

    pub fn dropped_flush(
        seam: ProductionStorageBoundarySeam,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::DroppedFlush(DroppedFlushEvent { seam, locus }))
    }

    pub fn reordered_persistence(
        seam: ProductionStorageBoundarySeam,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::ReorderedPersistence(ReorderedPersistenceEvent {
            seam,
            locus,
        }))
    }

    pub fn byte_corruption(
        seam: ProductionStorageBoundarySeam,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::ByteCorruption(ByteCorruptionEvent { seam, locus }))
    }

    pub fn stale_generation(
        seam: ProductionStorageBoundarySeam,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::StaleGeneration(StaleGenerationEvent { seam, locus }))
    }

    pub fn delayed_release(
        seam: ProductionStorageBoundarySeam,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::DelayedRelease(DelayedReleaseEvent { seam, locus }))
    }

    pub fn blocked_reclaim(
        seam: ProductionStorageBoundarySeam,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::BlockedReclaim(BlockedReclaimEvent { seam, locus }))
    }

    pub fn execution_time_reference_discovery(
        seam: ProductionStorageBoundarySeam,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::ExecutionTimeReferenceDiscovery(
            ExecutionTimeReferenceDiscoveryEvent { seam, locus },
        ))
    }

    pub fn unbounded_read_plan_footprint(
        seam: ProductionStorageBoundarySeam,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::UnboundedReadPlanFootprint(
            UnboundedReadPlanFootprintEvent { seam, locus },
        ))
    }

    pub fn io_stall(locus: PhysicalArtifactFaultLocus) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::IoStall(IoStallEvent {
            s6_pressure_fault_kind: None,
            locus,
        }))
    }

    pub fn s6_io_pressure_stall(
        fault_kind: S6IoPressureFaultKind,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::IoStall(IoStallEvent {
            s6_pressure_fault_kind: Some(fault_kind),
            locus,
        }))
    }

    pub fn no_fault_control(
        seam: ProductionStorageBoundarySeam,
        locus: PhysicalArtifactFaultLocus,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        Ok(Self::NoFaultControl(NoFaultControlEvent { seam, locus }))
    }

    pub const fn kind(&self) -> PhysicalFaultEventKind {
        match self {
            Self::Crash(_) => PhysicalFaultEventKind::Crash,
            Self::TornWrite(_) => PhysicalFaultEventKind::TornWrite,
            Self::DroppedFlush(_) => PhysicalFaultEventKind::DroppedFlush,
            Self::ReorderedPersistence(_) => PhysicalFaultEventKind::ReorderedPersistence,
            Self::ByteCorruption(_) => PhysicalFaultEventKind::ByteCorruption,
            Self::StaleGeneration(_) => PhysicalFaultEventKind::StaleGeneration,
            Self::DelayedRelease(_) => PhysicalFaultEventKind::DelayedRelease,
            Self::BlockedReclaim(_) => PhysicalFaultEventKind::BlockedReclaim,
            Self::ExecutionTimeReferenceDiscovery(_) => {
                PhysicalFaultEventKind::ExecutionTimeReferenceDiscovery
            }
            Self::UnboundedReadPlanFootprint(_) => {
                PhysicalFaultEventKind::UnboundedReadPlanFootprint
            }
            Self::IoStall(_) => PhysicalFaultEventKind::IoStall,
            Self::NoFaultControl(_) => PhysicalFaultEventKind::NoFaultControl,
        }
    }

    pub const fn locus(&self) -> Option<&PhysicalArtifactFaultLocus> {
        match self {
            Self::Crash(event) => Some(&event.locus),
            Self::TornWrite(event) => Some(&event.locus),
            Self::DroppedFlush(event) => Some(&event.locus),
            Self::ReorderedPersistence(event) => Some(&event.locus),
            Self::ByteCorruption(event) => Some(&event.locus),
            Self::StaleGeneration(event) => Some(&event.locus),
            Self::DelayedRelease(event) => Some(&event.locus),
            Self::BlockedReclaim(event) => Some(&event.locus),
            Self::ExecutionTimeReferenceDiscovery(event) => Some(&event.locus),
            Self::UnboundedReadPlanFootprint(event) => Some(&event.locus),
            Self::IoStall(event) => Some(&event.locus),
            Self::NoFaultControl(event) => Some(&event.locus),
        }
    }

    pub const fn required_seam(&self) -> PhysicalBoundarySeam {
        match self {
            Self::Crash(event) => event.yieldpoint.seam(),
            Self::TornWrite(event) => PhysicalBoundarySeam::ProductionStorage(event.seam),
            Self::DroppedFlush(event) => PhysicalBoundarySeam::ProductionStorage(event.seam),
            Self::ReorderedPersistence(event) => {
                PhysicalBoundarySeam::ProductionStorage(event.seam)
            }
            Self::ByteCorruption(event) => PhysicalBoundarySeam::ProductionStorage(event.seam),
            Self::StaleGeneration(event) => PhysicalBoundarySeam::ProductionStorage(event.seam),
            Self::DelayedRelease(event) => PhysicalBoundarySeam::ProductionStorage(event.seam),
            Self::BlockedReclaim(event) => PhysicalBoundarySeam::ProductionStorage(event.seam),
            Self::ExecutionTimeReferenceDiscovery(event) => {
                PhysicalBoundarySeam::ProductionStorage(event.seam)
            }
            Self::UnboundedReadPlanFootprint(event) => {
                PhysicalBoundarySeam::ProductionStorage(event.seam)
            }
            Self::IoStall(_) => PhysicalBoundarySeam::IoPressure,
            Self::NoFaultControl(event) => PhysicalBoundarySeam::ProductionStorage(event.seam),
        }
    }
}

impl IoStallEvent {
    pub const fn s6_pressure_fault_kind(&self) -> Option<S6IoPressureFaultKind> {
        self.s6_pressure_fault_kind
    }

    pub const fn locus(&self) -> &PhysicalArtifactFaultLocus {
        &self.locus
    }
}

impl CrashEvent {
    pub fn fresh_runtime_recovery(
        yieldpoint: PhysicalBoundaryYieldpoint,
        locus: PhysicalArtifactFaultLocus,
        fresh_runtime_evidence: FreshRuntimeCrashRecoveryEvidence,
    ) -> Result<Self, FaultDeliveryDenial> {
        require_unambiguous_locus(&locus)?;
        if yieldpoint.seam() != PhysicalBoundarySeam::FreshRuntimeRecovery {
            return Err(FaultDeliveryDenial::FaultYieldpointSeamMismatch {
                expected: PhysicalBoundarySeam::FreshRuntimeRecovery,
                actual: yieldpoint.seam(),
            });
        }
        Ok(Self {
            yieldpoint,
            locus,
            fresh_runtime_evidence,
        })
    }

    pub fn missing_fresh_runtime_evidence(
        yieldpoint: PhysicalBoundaryYieldpoint,
    ) -> Result<Self, FaultDeliveryDenial> {
        let _ = yieldpoint;
        Err(FaultDeliveryDenial::MissingFreshRuntimeEvidence)
    }

    pub const fn yieldpoint(&self) -> &PhysicalBoundaryYieldpoint {
        &self.yieldpoint
    }

    pub const fn locus(&self) -> &PhysicalArtifactFaultLocus {
        &self.locus
    }

    pub const fn fresh_runtime_evidence(&self) -> &FreshRuntimeCrashRecoveryEvidence {
        &self.fresh_runtime_evidence
    }
}
