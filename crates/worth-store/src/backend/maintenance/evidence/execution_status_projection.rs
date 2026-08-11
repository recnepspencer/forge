use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

#[derive(Debug, Clone)]
pub(super) struct MaintenanceExecutionStatusProjection {
    pub(super) active_declaration_count: u64,
    pub(super) reserved_declaration_count: u64,
    pub(super) deferred_declaration_count: u64,
    pub(super) escalated_declaration_count: u64,
    pub(super) cancelled_declaration_count: u64,
    pub(super) readmitted_recovered_declaration_count: u64,
    pub(super) rejected_recovered_declaration_count: u64,
    pub(super) completed_declaration_count: u64,
    pub(super) failed_declaration_count: u64,
}

pub(super) fn project_execution_status<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> MaintenanceExecutionStatusProjection {
    let executions = backend.state().maintenance_execution_records.values();
    MaintenanceExecutionStatusProjection {
        active_declaration_count: executions
            .clone()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Admitted
                        | crate::MaintenanceExecutionStatus::Reserved
                        | crate::MaintenanceExecutionStatus::Started
                )
            })
            .count() as u64,
        reserved_declaration_count: executions
            .clone()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Reserved
                )
            })
            .count() as u64,
        deferred_declaration_count: executions
            .clone()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Deferred
                )
            })
            .count() as u64,
        escalated_declaration_count: executions
            .clone()
            .filter(|record| {
                matches!(
                    record.plan_family,
                    Some(crate::MaintenancePlanFamily::Escalated)
                )
            })
            .count() as u64,
        cancelled_declaration_count: executions
            .clone()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Cancelled
                )
            })
            .count() as u64,
        readmitted_recovered_declaration_count: executions
            .clone()
            .filter(|record| {
                matches!(
                    record.restart_readmission_status,
                    Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork)
                )
            })
            .count() as u64,
        rejected_recovered_declaration_count: executions
            .clone()
            .filter(|record| {
                matches!(
                    record.restart_readmission_status,
                    Some(crate::MaintenanceReadmissionStatus::RejectedStaleRecoveredWork)
                        | Some(
                            crate::MaintenanceReadmissionStatus::RejectedSupersededRecoveredWork
                        )
                )
            })
            .count() as u64,
        completed_declaration_count: executions
            .clone()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Completed
                )
            })
            .count() as u64,
        failed_declaration_count: executions
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Failed
                )
            })
            .count() as u64,
    }
}
