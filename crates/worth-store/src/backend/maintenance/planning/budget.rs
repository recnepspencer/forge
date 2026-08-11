use crate::maintenance::{MaintenanceQuantum, MaintenanceResourceBudgetGrant, PacingWindow};

pub(super) fn deterministic_budget_grant(
    descriptor: &crate::MaintenanceWorkDescriptor,
) -> MaintenanceResourceBudgetGrant {
    let demand = descriptor.demand();
    let cap = match descriptor.work_class() {
        crate::MaintenanceWorkClass::RetentionAudit => 1,
        crate::MaintenanceWorkClass::CompactionMaintenance => 3,
        crate::MaintenanceWorkClass::DerivedArtifactReclaim
        | crate::MaintenanceWorkClass::AuthoritativeReclaim => 2,
        crate::MaintenanceWorkClass::RetainedRangeRebuild => 2,
        _ => 1,
    };
    let quantum_units = demand
        .predicted_io()
        .units()
        .max(demand.predicted_cpu().units())
        .max(demand.predicted_memory().units())
        .max(demand.predicted_publication().units().max(1))
        .min(cap);
    let pacing_units = quantum_units.max(demand.foreground_latency_guard().units());
    MaintenanceResourceBudgetGrant::new(
        demand.predicted_io(),
        demand.predicted_cpu(),
        demand.predicted_memory(),
        demand.predicted_publication(),
        demand.foreground_latency_guard(),
        MaintenanceQuantum::new(quantum_units),
        PacingWindow::new(pacing_units),
    )
}
