use super::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceCostContractId,
    ResourceCostPosture, ResourceDensityStrategy,
};

impl ResourceBoundaryPerformanceEnvelope {
    pub(crate) fn request_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::RequestAdmission,
            admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            admitted_count,
            lifecycle_transition_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(1),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionAdmission,
            admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            0,
            denied_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(2),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_batch_admission(
        input_width: u32,
        admitted_count: u32,
        denied_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionBatchAdmission,
            input_width,
            admitted_count,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            denied_count,
            0,
            admitted_count.saturating_add(denied_count),
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(12),
            ResourceCostPosture::Verified,
        )
    }
}
