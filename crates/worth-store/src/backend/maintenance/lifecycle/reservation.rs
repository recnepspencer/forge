use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::StoreError,
    maintenance::{
        MaintenanceCoalescingDecision, MaintenanceDeclarationId, MaintenanceEscalationVerdict,
        MaintenanceLaneKey, MaintenancePlanFamily, MaintenanceResourceBudgetGrant,
        MaintenanceStarvationStatus,
    },
};

#[derive(Debug, Clone)]
pub(crate) struct MaintenanceReservationUpdate {
    pub(crate) declaration_id: MaintenanceDeclarationId,
    pub(crate) plan_family: MaintenancePlanFamily,
    pub(crate) quantum_units: u64,
    pub(crate) lane_key: MaintenanceLaneKey,
    pub(crate) coalescing_decision: MaintenanceCoalescingDecision,
    pub(crate) supersession_source: Option<String>,
    pub(crate) resource_budget_grant: Option<MaintenanceResourceBudgetGrant>,
    pub(crate) starvation_status: MaintenanceStarvationStatus,
    pub(crate) escalation_verdict: MaintenanceEscalationVerdict,
    pub(crate) explicit_global_scope_debt: bool,
}

pub(crate) fn persist_reserved_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: MaintenanceReservationUpdate,
) -> Result<(), StoreError> {
    super::transition_commit::commit_reservation(backend, update)
}
