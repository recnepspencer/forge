use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::StoreError,
    maintenance::{
        MaintenanceCoalescingDecision, MaintenanceDeclarationId, MaintenanceEscalationVerdict,
        MaintenanceLaneKey, MaintenanceStarvationStatus,
    },
};

#[derive(Debug, Clone)]
pub(crate) struct MaintenanceDispositionUpdate {
    pub(crate) declaration_id: MaintenanceDeclarationId,
    pub(crate) reason: String,
    pub(crate) lane_key: MaintenanceLaneKey,
    pub(crate) coalescing_decision: MaintenanceCoalescingDecision,
    pub(crate) supersession_source: Option<String>,
    pub(crate) starvation_status: MaintenanceStarvationStatus,
    pub(crate) escalation_verdict: MaintenanceEscalationVerdict,
    pub(crate) explicit_global_scope_debt: bool,
}

pub(crate) fn persist_deferred_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: MaintenanceDispositionUpdate,
) -> Result<(), StoreError> {
    super::transition_commit::commit_deferred_disposition(backend, update)
}

pub(crate) fn persist_cancelled_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: MaintenanceDispositionUpdate,
) -> Result<(), StoreError> {
    super::transition_commit::commit_cancelled_disposition(backend, update)
}
