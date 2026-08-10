use crate::backend::records::StoreState;

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct ReservedResourceTotals {
    pub(super) io: u64,
    pub(super) cpu: u64,
    pub(super) memory: u64,
    pub(super) publication: u64,
    pub(super) foreground_latency_guard: u64,
}

pub(super) fn accumulate_reserved_resources(state: &StoreState) -> ReservedResourceTotals {
    let mut totals = ReservedResourceTotals::default();
    for execution in state.maintenance_execution_records.values() {
        if matches!(
            execution.execution_status,
            crate::MaintenanceExecutionStatus::Reserved
                | crate::MaintenanceExecutionStatus::Started
        ) {
            if let Some(grant) = &execution.resource_budget_grant {
                totals.io += grant.granted_io().units();
                totals.cpu += grant.granted_cpu().units();
                totals.memory += grant.granted_memory().units();
                totals.publication += grant.granted_publication().units();
                totals.foreground_latency_guard += grant.granted_foreground_latency_guard().units();
            }
        }
    }
    totals
}
