use crate::{
    backend::records::{MaintenanceDeclarationRecord, MaintenanceExecutionRecord},
    CpuBudgetUnits, ForegroundLatencyGuard, IoBudgetUnits, MaintenanceDescriptorDemand,
    MaintenanceEscalationDecision, MaintenanceExecutionStatus, MaintenanceLocalityScope,
    MemoryBudgetUnits, PublicationSlotBudget,
};

use super::resource_budget::fixture_budget_grant;

pub(super) fn force_sqlite_started(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    let payload_json: String = connection
        .query_row(
            "SELECT payload_json FROM maintenance_execution_records WHERE artifact_id = ?1",
            [declaration_id.as_str()],
            |row| row.get(0),
        )
        .expect("maintenance execution payload should exist");
    let mut record: MaintenanceExecutionRecord =
        serde_json::from_str(&payload_json).expect("maintenance execution payload should decode");
    record.execution_status = MaintenanceExecutionStatus::Started;
    record
        .plan_family
        .get_or_insert(crate::MaintenancePlanFamily::BackgroundPaced);
    record.last_quantum_units.get_or_insert(1);
    record
        .resource_budget_grant
        .get_or_insert_with(|| fixture_budget_grant(1));
    record.last_completed_phase = Some("started".to_string());
    record.resume_count = 1;
    connection
        .execute(
            "UPDATE maintenance_execution_records SET payload_json = ?1 WHERE artifact_id = ?2",
            [
                serde_json::to_string(&record)
                    .expect("maintenance execution payload should encode"),
                declaration_id.as_str().to_string(),
            ],
        )
        .expect("sqlite maintenance execution payload should update");
    clear_sqlite_scheduler_summaries(&connection);
}

pub(super) fn force_sqlite_reserved(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    plan_family: crate::MaintenancePlanFamily,
    quantum_units: u64,
) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    let payload_json: String = connection
        .query_row(
            "SELECT payload_json FROM maintenance_execution_records WHERE artifact_id = ?1",
            [declaration_id.as_str()],
            |row| row.get(0),
        )
        .expect("maintenance execution payload should exist");
    let mut record: MaintenanceExecutionRecord =
        serde_json::from_str(&payload_json).expect("maintenance execution payload should decode");
    record.execution_status = MaintenanceExecutionStatus::Reserved;
    record.plan_family = Some(plan_family);
    record.last_quantum_units = Some(quantum_units);
    record.resource_budget_grant = Some(fixture_budget_grant(quantum_units));
    record.reservation_transition = Some(crate::MaintenanceReservationTransition::new(
        plan_family,
        quantum_units,
    ));
    record.last_completed_phase = Some("reserved".to_string());
    if matches!(plan_family, crate::MaintenancePlanFamily::Escalated) {
        record.foreground_impact = crate::MaintenanceForegroundImpact::escalated();
    }
    connection
        .execute(
            "UPDATE maintenance_execution_records SET payload_json = ?1 WHERE artifact_id = ?2",
            [
                serde_json::to_string(&record)
                    .expect("maintenance execution payload should encode"),
                declaration_id.as_str().to_string(),
            ],
        )
        .expect("sqlite maintenance execution payload should update");
    clear_sqlite_scheduler_summaries(&connection);
    clear_sqlite_scheduler_summaries(&connection);
}

pub(super) fn force_sqlite_deferred(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_escalation_decision(MaintenanceEscalationDecision::DeferWithOperatorSignal);
    });
}

pub(super) fn force_sqlite_high_demand(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    io: u64,
    cpu: u64,
    memory: u64,
    publication: u64,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor =
            record
                .work_descriptor
                .clone()
                .with_demand(MaintenanceDescriptorDemand::new(
                    IoBudgetUnits::new(io),
                    CpuBudgetUnits::new(cpu),
                    MemoryBudgetUnits::new(memory),
                    PublicationSlotBudget::new(publication),
                    ForegroundLatencyGuard::new(1),
                ));
    });
}

pub(super) fn force_sqlite_escalated(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_escalation_decision(MaintenanceEscalationDecision::EscalateWithForegroundImpact);
    });
}

pub(super) fn force_sqlite_recovered(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_recovered_from_restart(true);
    });
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    let payload_json: String = connection
        .query_row(
            "SELECT payload_json FROM maintenance_execution_records WHERE artifact_id = ?1",
            [declaration_id.as_str()],
            |row| row.get(0),
        )
        .expect("maintenance execution payload should exist");
    let mut record: MaintenanceExecutionRecord =
        serde_json::from_str(&payload_json).expect("maintenance execution payload should decode");
    record.restart_readmission_status =
        Some(crate::MaintenanceReadmissionStatus::PendingRecoveredReadmission);
    connection
        .execute(
            "UPDATE maintenance_execution_records SET payload_json = ?1 WHERE artifact_id = ?2",
            [
                serde_json::to_string(&record)
                    .expect("maintenance execution payload should encode"),
                declaration_id.as_str().to_string(),
            ],
        )
        .expect("sqlite maintenance execution payload should update");
    clear_sqlite_scheduler_summaries(&connection);
    clear_sqlite_scheduler_summaries(&connection);
}

pub(super) fn force_sqlite_cancelled(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_freshness_window(crate::FreshnessWindow::new(0));
    });
}

pub(super) fn force_sqlite_global_scope_escalated(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_locality_scope(MaintenanceLocalityScope::StoreGlobalLocalityScope)
            .with_escalation_decision(MaintenanceEscalationDecision::EscalateWithForegroundImpact);
    });
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    let payload_json: String = connection
        .query_row(
            "SELECT payload_json FROM maintenance_execution_records WHERE artifact_id = ?1",
            [declaration_id.as_str()],
            |row| row.get(0),
        )
        .expect("maintenance execution payload should exist");
    let mut record: MaintenanceExecutionRecord =
        serde_json::from_str(&payload_json).expect("maintenance execution payload should decode");
    let declaration_json: String = connection
        .query_row(
            "SELECT payload_json FROM maintenance_declaration_records WHERE artifact_id = ?1",
            [declaration_id.as_str()],
            |row| row.get(0),
        )
        .expect("maintenance declaration payload should exist");
    let declaration: MaintenanceDeclarationRecord = serde_json::from_str(&declaration_json)
        .expect("maintenance declaration payload should decode");
    record.lane_key = Some(declaration.work_descriptor.lane_key());
    connection
        .execute(
            "UPDATE maintenance_execution_records SET payload_json = ?1 WHERE artifact_id = ?2",
            [
                serde_json::to_string(&record)
                    .expect("maintenance execution payload should encode"),
                declaration_id.as_str().to_string(),
            ],
        )
        .expect("sqlite maintenance execution payload should update");
    clear_sqlite_scheduler_summaries(&connection);
}

fn clear_sqlite_scheduler_summaries(connection: &rusqlite::Connection) {
    connection
        .execute_batch(
            "
            DELETE FROM maintenance_queue_summary_records;
            DELETE FROM maintenance_locality_summary_records;
            DELETE FROM maintenance_reservation_summary_records;
            DELETE FROM maintenance_resource_budget_summary_records;
            DELETE FROM maintenance_debt_summary_records;
            ",
        )
        .expect("sqlite maintenance summary tables should clear");
}

fn mutate_sqlite_declaration<F>(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    mut mutate: F,
) where
    F: FnMut(&mut MaintenanceDeclarationRecord),
{
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    let payload_json: String = connection
        .query_row(
            "SELECT payload_json FROM maintenance_declaration_records WHERE artifact_id = ?1",
            [declaration_id.as_str()],
            |row| row.get(0),
        )
        .expect("maintenance declaration payload should exist");
    let mut record: MaintenanceDeclarationRecord =
        serde_json::from_str(&payload_json).expect("maintenance declaration payload should decode");
    mutate(&mut record);
    connection
        .execute(
            "UPDATE maintenance_declaration_records SET payload_json = ?1 WHERE artifact_id = ?2",
            [
                serde_json::to_string(&record)
                    .expect("maintenance declaration payload should encode"),
                declaration_id.as_str().to_string(),
            ],
        )
        .expect("sqlite maintenance declaration payload should update");
    clear_sqlite_scheduler_summaries(&connection);
}
