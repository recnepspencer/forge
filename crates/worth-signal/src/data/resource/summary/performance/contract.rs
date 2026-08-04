use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceBoundaryKind {
    DeclarationLowering,
    RequestAdmission,
    RejectionAdmission,
    Cancellation,
    TimeoutAdmission,
    TimeoutHeartbeatExtension,
    RetrySchedule,
    RetryAdmission,
    RevalidationAdmission,
    CompletionAdmission,
    CompletionBatchAdmission,
    CompletionStaging,
    CompletionDenialStaging,
    CompletionCommit,
    CompletionRollback,
    BranchRestore,
    ReplayReconstruction,
    PolicyCompatibility,
    SummaryRead,
    DiagnosticsExpansion,
    LifecycleRetentionCompaction,
    ObservationMaterialization,
    ReplayAvailability,
    AsyncNodeGateState,
    AsyncNodeHierarchyReplay,
    AsyncNodeHierarchyCancellation,
    AsyncNodeHistoricalParity,
    AsyncNodeCapabilityEquivalence,
    AsyncKeyedNodeHistoricalParity,
    AsyncKeyedNodeCapabilityEquivalence,
    AsyncNodeHierarchyHistoricalParity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceCostPosture {
    Verified,
    Debt,
    DeniedFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceDensityStrategy {
    NotApplicable,
    SparseIndexedLookup,
    BurstySortedDeduplicated,
    DenseSortedDeduplicated,
}

impl ResourceDensityStrategy {
    pub(crate) fn request_pressure(in_flight_width: u32) -> Self {
        if in_flight_width <= 1 {
            Self::SparseIndexedLookup
        } else if in_flight_width <= 8 {
            Self::BurstySortedDeduplicated
        } else {
            Self::DenseSortedDeduplicated
        }
    }

    pub(crate) fn scalar_completion() -> Self {
        Self::SparseIndexedLookup
    }

    pub(crate) fn completion_batch(input_width: u32, in_flight_width: u32) -> Self {
        if input_width <= 1 {
            Self::SparseIndexedLookup
        } else if input_width >= 4 && input_width >= in_flight_width.max(1) {
            Self::DenseSortedDeduplicated
        } else {
            Self::BurstySortedDeduplicated
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceCostContractId(u64);

impl ResourceCostContractId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}
