#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiStateQueryResidueScan {
    query_installed: bool,
    scanned_state_receipts: usize,
    scanned_query_bindings: usize,
    scanned_plan_query_links: usize,
    scanned_settled_snapshots: usize,
    scanned_live_resources: usize,
    operation_live_subsystem_construction_count: usize,
    operation_live_succession_operation_count: usize,
    stale_installed_reference_count: usize,
    orphan_settlement_count: usize,
    orphan_live_resource_count: usize,
    foreign_plan_reference_count: usize,
    missing_plan_link_count: usize,
    mixed_application_generation_count: usize,
}

impl WorthUiStateQueryResidueScan {
    pub(crate) fn from_active_runtime(
        scanned_state_receipts: usize,
        query: worth_ui_query_binding::WorthUiRuntimeQueryStateObservation,
        plan: crate::runtime::active::WorthUiActiveQueryPlanObservation,
        generation_matches: bool,
    ) -> Self {
        let operation_live = query.operation_live();
        Self {
            query_installed: query.query_installed(),
            scanned_state_receipts,
            scanned_query_bindings: query.installed_reference_count(),
            scanned_plan_query_links: plan.query_binding_slot_count(),
            scanned_settled_snapshots: query.settled_snapshot_count(),
            scanned_live_resources: operation_live.retained_resource_count(),
            operation_live_subsystem_construction_count: operation_live
                .subsystem_construction_count(),
            operation_live_succession_operation_count: operation_live.succession_operation_count(),
            stale_installed_reference_count: query.stale_installed_reference_count(),
            orphan_settlement_count: query.orphan_settled_snapshot_count(),
            orphan_live_resource_count: operation_live.orphan_resource_count(),
            foreign_plan_reference_count: plan.foreign_installed_reference_count(),
            missing_plan_link_count: plan.missing_settled_fact_link_count(),
            mixed_application_generation_count: usize::from(!generation_matches),
        }
    }

    pub fn query_installed(&self) -> bool {
        self.query_installed
    }

    pub fn scanned_state_receipts(&self) -> usize {
        self.scanned_state_receipts
    }

    pub fn scanned_query_bindings(&self) -> usize {
        self.scanned_query_bindings
    }

    pub fn scanned_plan_query_links(&self) -> usize {
        self.scanned_plan_query_links
    }

    pub fn scanned_settled_snapshots(&self) -> usize {
        self.scanned_settled_snapshots
    }

    pub fn scanned_live_resources(&self) -> usize {
        self.scanned_live_resources
    }

    pub fn operation_live_subsystem_construction_count(&self) -> usize {
        self.operation_live_subsystem_construction_count
    }

    pub fn operation_live_succession_operation_count(&self) -> usize {
        self.operation_live_succession_operation_count
    }

    pub fn stale_installed_reference_count(&self) -> usize {
        self.stale_installed_reference_count
    }

    pub fn orphan_settlement_count(&self) -> usize {
        self.orphan_settlement_count
    }

    pub fn orphan_live_resource_count(&self) -> usize {
        self.orphan_live_resource_count
    }

    pub fn foreign_plan_reference_count(&self) -> usize {
        self.foreign_plan_reference_count
    }

    pub fn missing_plan_link_count(&self) -> usize {
        self.missing_plan_link_count
    }

    pub fn mixed_application_generation_count(&self) -> usize {
        self.mixed_application_generation_count
    }

    pub fn is_clean(&self) -> bool {
        self.stale_installed_reference_count == 0
            && self.orphan_settlement_count == 0
            && self.orphan_live_resource_count == 0
            && self.foreign_plan_reference_count == 0
            && self.missing_plan_link_count == 0
            && self.mixed_application_generation_count == 0
    }
}
