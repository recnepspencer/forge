use super::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceCostContractId,
    ResourceCostPosture, ResourceDensityStrategy,
};

impl ResourceBoundaryPerformanceEnvelope {
    pub fn boundary(self) -> ResourceBoundaryKind {
        self.boundary
    }

    pub fn input_width(self) -> u32 {
        self.input_width
    }

    pub fn admitted_count(self) -> u32 {
        self.admitted_count
    }

    pub fn lifecycle_transition_count(self) -> u32 {
        self.lifecycle_transition_count
    }

    pub fn denied_count(self) -> u32 {
        self.denied_count
    }

    pub fn broad_scan_denial_count(self) -> u32 {
        self.broad_scan_denial_count
    }

    pub fn coalescing_width(self) -> u32 {
        self.coalescing_width
    }

    pub fn output_continuity_classification_width(self) -> u32 {
        self.output_continuity_classification_width
    }

    pub fn retry_budget_scope_touch_count(self) -> u32 {
        self.retry_budget_scope_touch_count
    }

    pub fn temporal_wake_footprint(self) -> u32 {
        self.temporal_wake_footprint
    }

    pub fn operational_allocation_count(self) -> u32 {
        self.operational_allocation_count
    }

    pub fn retained_history_allocation_count(self) -> u32 {
        self.retained_history_allocation_count
    }

    pub fn diagnostics_allocation_count(self) -> u32 {
        self.diagnostics_allocation_count
    }

    pub fn facade_report_allocation_count(self) -> u32 {
        self.facade_report_allocation_count
    }

    pub fn density_strategy(self) -> ResourceDensityStrategy {
        self.density_strategy
    }

    pub fn cost_contract(self) -> ResourceCostContractId {
        self.cost_contract
    }

    pub fn cost_posture(self) -> ResourceCostPosture {
        self.cost_posture
    }
}
