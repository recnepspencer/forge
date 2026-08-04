use super::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceCostContractId,
    ResourceCostPosture, ResourceDensityStrategy,
};

impl ResourceBoundaryPerformanceEnvelope {
    pub(crate) fn branch_restore(
        restored_in_flight_width: u32,
        retained_summary_width: u32,
        broad_rebuild_denial_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::BranchRestore,
            restored_in_flight_width.saturating_add(retained_summary_width),
            restored_in_flight_width,
            restored_in_flight_width,
            0,
            broad_rebuild_denial_count,
            0,
            0,
            restored_in_flight_width,
            retained_summary_width,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(13),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn cancellation(admitted_count: u32, denied_count: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::Cancellation,
            admitted_count.saturating_add(denied_count),
            admitted_count,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            0,
            admitted_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(3),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn rejection_admission(admitted_count: u32, denied_count: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::RejectionAdmission,
            admitted_count.saturating_add(denied_count),
            admitted_count,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            0,
            admitted_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(20),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn timeout_admission(
        admitted_count: u32,
        denied_count: u32,
        temporal_wake_footprint: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::TimeoutAdmission,
            admitted_count.saturating_add(denied_count),
            admitted_count,
            admitted_count,
            denied_count,
            0,
            0,
            temporal_wake_footprint,
            0,
            admitted_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(4),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn timeout_heartbeat_extension(
        admitted_count: u32,
        denied_count: u32,
        temporal_wake_footprint: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::TimeoutHeartbeatExtension,
            admitted_count.saturating_add(denied_count),
            0,
            admitted_count,
            denied_count,
            0,
            0,
            temporal_wake_footprint,
            admitted_count,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(4),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn retry_schedule(
        admitted_count: u32,
        denied_count: u32,
        retry_budget_scope_touch_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::RetrySchedule,
            admitted_count.saturating_add(denied_count),
            0,
            admitted_count,
            denied_count,
            0,
            retry_budget_scope_touch_count,
            admitted_count,
            admitted_count,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(5),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn retry_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
        temporal_wake_footprint: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::RetryAdmission,
            admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            0,
            0,
            temporal_wake_footprint,
            admitted_count,
            lifecycle_transition_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(6),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn revalidation_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
        temporal_wake_footprint: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::RevalidationAdmission,
            admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            0,
            0,
            temporal_wake_footprint,
            admitted_count,
            lifecycle_transition_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(7),
            ResourceCostPosture::Verified,
        )
    }
}
