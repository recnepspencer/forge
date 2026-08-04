use super::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceCostContractId,
    ResourceCostPosture, ResourceDensityStrategy,
};

impl ResourceBoundaryPerformanceEnvelope {
    pub(super) fn new(
        boundary: ResourceBoundaryKind,
        input_width: u32,
        lifecycle_transition_count: u32,
        admitted_count: u32,
        denied_count: u32,
        broad_scan_denial_count: u32,
        retry_budget_scope_touch_count: u32,
        temporal_wake_footprint: u32,
        operational_allocation_count: u32,
        retained_history_allocation_count: u32,
        diagnostics_allocation_count: u32,
        facade_report_allocation_count: u32,
        density_strategy: ResourceDensityStrategy,
        cost_contract: ResourceCostContractId,
        cost_posture: ResourceCostPosture,
    ) -> Self {
        Self {
            boundary,
            input_width,
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            broad_scan_denial_count,
            coalescing_width: 0,
            output_continuity_classification_width: 0,
            retry_budget_scope_touch_count,
            temporal_wake_footprint,
            operational_allocation_count,
            retained_history_allocation_count,
            diagnostics_allocation_count,
            facade_report_allocation_count,
            density_strategy,
            cost_contract,
            cost_posture,
        }
    }

    pub(crate) fn with_density_strategy(
        mut self,
        density_strategy: ResourceDensityStrategy,
    ) -> Self {
        self.density_strategy = density_strategy;
        self
    }

    pub(crate) fn with_coalescing_width(mut self, coalescing_width: u32) -> Self {
        self.coalescing_width = coalescing_width;
        self
    }

    pub(crate) fn with_temporal_wake_footprint(mut self, temporal_wake_footprint: u32) -> Self {
        self.temporal_wake_footprint = temporal_wake_footprint;
        self
    }

    pub(crate) fn with_output_continuity_classification_width(
        mut self,
        output_continuity_classification_width: u32,
    ) -> Self {
        self.output_continuity_classification_width = output_continuity_classification_width;
        self
    }
}
