use super::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceCostContractId,
    ResourceCostPosture, ResourceDensityStrategy,
};

impl ResourceBoundaryPerformanceEnvelope {
    pub(crate) fn replay_reconstruction(
        descriptor_width: u32,
        lifecycle_summary_width: u32,
        denied_completion_width: u32,
        in_flight_width: u32,
        retained_history_unavailable_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::ReplayReconstruction,
            descriptor_width
                .saturating_add(lifecycle_summary_width)
                .saturating_add(denied_completion_width)
                .saturating_add(in_flight_width),
            lifecycle_summary_width,
            in_flight_width,
            denied_completion_width,
            retained_history_unavailable_count,
            0,
            0,
            0,
            0,
            descriptor_width
                .saturating_add(lifecycle_summary_width)
                .saturating_add(denied_completion_width)
                .saturating_add(in_flight_width),
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(14),
            ResourceCostPosture::Debt,
        )
    }

    pub(crate) fn summary_read() -> Self {
        Self::new(
            ResourceBoundaryKind::SummaryRead,
            1,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(15),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn observation_materialization(
        input_width: u32,
        admitted_count: u32,
        denied_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::ObservationMaterialization,
            input_width,
            0,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            input_width,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(19),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn replay_availability(
        input_width: u32,
        admitted_count: u32,
        denied_count: u32,
        diagnostics_allocation_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::ReplayAvailability,
            input_width,
            0,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            0,
            0,
            diagnostics_allocation_count,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(21),
            ResourceCostPosture::Verified,
        )
    }
}
