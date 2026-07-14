use crate::{
    IoSchedulerBackendCapabilityDenial, IoSchedulerBackendCapabilityRequirement,
    IoSchedulerIsolationAdmissionDenial,
};

use super::{ForegroundIoLaneKind, ForegroundResourceBudget, ForegroundResourceUnitKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForegroundReservationResourceShortfall {
    QueueSlot { requested: u64, available: u64 },
    BandwidthToken { requested: u64, available: u64 },
    FlushPermit { requested: u64, available: u64 },
    SyncDebt { requested: u64, available: u64 },
    ReadAheadWindow { requested: u64, available: u64 },
    WriteBackWindow { requested: u64, available: u64 },
    DirtyPageBudget { requested: u64, available: u64 },
    WorkerPermit { requested: u64, available: u64 },
    CacheResidencyHint { requested: u64, available: u64 },
    ReclaimPermit { requested: u64, available: u64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForegroundReservationAdmissionDenial {
    BackendCapabilityDenied(IoSchedulerBackendCapabilityDenial),
    StableReadinessDenied(IoSchedulerIsolationAdmissionDenial),
    LaneBackendRequirementMismatch {
        lane_required: IoSchedulerBackendCapabilityRequirement,
        admitted: IoSchedulerBackendCapabilityRequirement,
    },
    LaneBackendRequirementNotStoreOwned {
        lane: ForegroundIoLaneKind,
        backend_requirement: IoSchedulerBackendCapabilityRequirement,
    },
    SecureFrameReservationRequiresSecurityScope,
    SecureFrameBackendWasNotSecurityBound,
    MissingLaneEnvelope,
    MissingDeclaredResourceBudget,
    MissingRequiredResourceUnit {
        lane: ForegroundIoLaneKind,
        unit: ForegroundResourceUnitKind,
    },
    InsufficientCapacity(ForegroundReservationResourceShortfall),
    CapacityAdmissionLaneMismatch {
        requested: ForegroundIoLaneKind,
        admitted: ForegroundIoLaneKind,
    },
    CapacityAdmissionBudgetMismatch {
        lane_requested: ForegroundResourceBudget,
        capacity_requested: ForegroundResourceBudget,
    },
    CapacityAdmissionBackendMismatch,
    CapacityAdmissionEnvelopeMismatch,
    CapacityAdmissionArbitrationMismatch,
    CapacityAdmissionSecurityScopeMismatch,
    CapacityAdmissionReadinessCounterMismatch,
    CertificationOnlyEnvelopeCannotExecute,
    ForegroundPriorityLaundering {
        declared: ForegroundIoLaneKind,
        attempted: ForegroundIoLaneKind,
    },
    ReservationBasisRebindRequired,
    RawLaneLabelCannotReserve,
    SemanticPriorityCannotReserve,
    CopiedIsolationCountersCannotReserve,
    CopiedSecurityScopeFieldsCannotReserve,
    TerminalProjectionCannotReserve,
}
