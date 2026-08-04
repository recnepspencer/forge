use super::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceCostContractId,
    ResourceCostPosture, ResourceDensityStrategy,
};

impl ResourceBoundaryPerformanceEnvelope {
    pub(crate) fn completion_commit(lifecycle_transition_count: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionCommit,
            1,
            lifecycle_transition_count,
            1,
            0,
            0,
            0,
            0,
            0,
            lifecycle_transition_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(8),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_staging() -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionStaging,
            1,
            0,
            1,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(9),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_denial_staging() -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionDenialStaging,
            1,
            0,
            0,
            1,
            0,
            0,
            0,
            1,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(10),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_rollback(admitted_count: u32, denied_count: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionRollback,
            admitted_count.saturating_add(denied_count),
            0,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(11),
            ResourceCostPosture::Verified,
        )
    }
}
