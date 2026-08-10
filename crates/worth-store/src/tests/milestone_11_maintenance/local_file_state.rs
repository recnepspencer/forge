use crate::{
    backend::records::StoreState, CpuBudgetUnits, ForegroundLatencyGuard, IoBudgetUnits,
    MaintenanceDescriptorDemand, MaintenanceEscalationDecision, MaintenanceExecutionStatus,
    MaintenanceLocalityScope, MemoryBudgetUnits, PublicationSlotBudget,
};

use super::resource_budget::fixture_budget_grant;

pub(super) fn force_local_file_started(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let execution = state
        .maintenance_execution_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance execution record should exist");
    execution.execution_status = MaintenanceExecutionStatus::Started;
    execution
        .plan_family
        .get_or_insert(crate::MaintenancePlanFamily::BackgroundPaced);
    execution.last_quantum_units.get_or_insert(1);
    execution
        .resource_budget_grant
        .get_or_insert_with(|| fixture_budget_grant(1));
    execution.last_completed_phase = Some("started".to_string());
    execution.resume_count = 1;
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("started maintenance state should write");
}

pub(super) fn force_local_file_reserved(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    plan_family: crate::MaintenancePlanFamily,
    quantum_units: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let execution = state
        .maintenance_execution_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance execution record should exist");
    execution.execution_status = MaintenanceExecutionStatus::Reserved;
    execution.plan_family = Some(plan_family);
    execution.last_quantum_units = Some(quantum_units);
    execution.resource_budget_grant = Some(fixture_budget_grant(quantum_units));
    execution.reservation_transition = Some(crate::MaintenanceReservationTransition::new(
        plan_family,
        quantum_units,
    ));
    execution.last_completed_phase = Some("reserved".to_string());
    if matches!(plan_family, crate::MaintenancePlanFamily::Escalated) {
        execution.foreground_impact = crate::MaintenanceForegroundImpact::escalated();
    }
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("reserved maintenance state should write");
}

pub(super) fn force_local_file_deferred(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor = declaration
        .work_descriptor
        .clone()
        .with_escalation_decision(MaintenanceEscalationDecision::DeferWithOperatorSignal);
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("deferred maintenance state should write");
}

pub(super) fn force_local_file_high_demand(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    io: u64,
    cpu: u64,
    memory: u64,
    publication: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor =
        declaration
            .work_descriptor
            .clone()
            .with_demand(MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(io),
                CpuBudgetUnits::new(cpu),
                MemoryBudgetUnits::new(memory),
                PublicationSlotBudget::new(publication),
                ForegroundLatencyGuard::new(1),
            ));
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("high demand maintenance state should write");
}

pub(super) fn force_local_file_high_latency_guard(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    guard_units: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    let demand = declaration.work_descriptor.demand().clone();
    declaration.work_descriptor =
        declaration
            .work_descriptor
            .clone()
            .with_demand(MaintenanceDescriptorDemand::new(
                demand.predicted_io(),
                demand.predicted_cpu(),
                demand.predicted_memory(),
                demand.predicted_publication(),
                ForegroundLatencyGuard::new(guard_units),
            ));
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("high latency guard maintenance state should write");
}

pub(super) fn force_local_file_escalated(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor = declaration
        .work_descriptor
        .clone()
        .with_escalation_decision(MaintenanceEscalationDecision::EscalateWithForegroundImpact);
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("escalated maintenance state should write");
}

pub(super) fn force_local_file_recovered(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor = declaration
        .work_descriptor
        .clone()
        .with_recovered_from_restart(true);
    let execution = state
        .maintenance_execution_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance execution record should exist");
    execution.restart_readmission_status =
        Some(crate::MaintenanceReadmissionStatus::PendingRecoveredReadmission);
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("recovered maintenance state should write");
}

pub(super) fn force_local_file_cancelled(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor = declaration
        .work_descriptor
        .clone()
        .with_freshness_window(crate::FreshnessWindow::new(0));
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("cancelled maintenance state should write");
}

pub(super) fn force_local_file_supersession_epoch(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    epoch: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor = declaration
        .work_descriptor
        .clone()
        .with_supersession_epoch(crate::SupersessionEpoch::new(epoch));
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("supersession maintenance state should write");
}

pub(super) fn force_local_file_global_scope_escalated(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor = declaration
        .work_descriptor
        .clone()
        .with_locality_scope(MaintenanceLocalityScope::StoreGlobalLocalityScope)
        .with_escalation_decision(MaintenanceEscalationDecision::EscalateWithForegroundImpact);
    let execution = state
        .maintenance_execution_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance execution record should exist");
    execution.lane_key = Some(declaration.work_descriptor.lane_key());
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("global scope escalated maintenance state should write");
}

fn clear_state_scheduler_summaries(state: &mut StoreState) {
    state.maintenance_queue_summary_records.clear();
    state.maintenance_locality_summary_records.clear();
    state.maintenance_reservation_summary_records.clear();
    state.maintenance_resource_budget_summary_records.clear();
    state.maintenance_debt_summary_records.clear();
}
