use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
    maintenance::{MaintenanceDeclarationId, MaintenanceExecutionStatus},
};
pub(crate) fn persist_failed_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
    error: &StoreError,
) -> Result<(), StoreError> {
    let mut next = backend.state().clone();
    let execution = next
        .maintenance_execution_records
        .get_mut(declaration_id.as_str())
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::MaintenanceDeclarationMissing,
                format!(
                    "maintenance execution record for `{}` is not persisted",
                    declaration_id.as_str()
                ),
            )
        })?;
    execution.execution_status = MaintenanceExecutionStatus::Failed;
    execution.durable_error_kind = Some(format!("{:?}", error.kind()));
    execution.durable_error_message = Some(error.message().to_string());
    execution.pending_reason = None;
    execution.last_completed_phase = Some("failed".to_string());
    super::super::summaries::refresh_scheduler_summaries(&mut next);
    backend.commit_replacement_state(next)?;
    backend.counters().record_maintenance_failures(1);
    Ok(())
}
