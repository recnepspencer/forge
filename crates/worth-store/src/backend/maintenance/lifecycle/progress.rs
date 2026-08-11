use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        records::MaintenanceCheckpointRecord,
    },
    failure::{StoreError, StoreErrorKind},
    maintenance::{MaintenanceDeclarationId, MaintenanceExecutionStatus},
};
pub(crate) fn persist_started_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
    resumed_from_started: bool,
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
    execution.execution_status = MaintenanceExecutionStatus::Started;
    execution.pending_reason = None;
    execution.execution_transition = Some(crate::MaintenanceExecutionTransition::new(
        resumed_from_started,
        execution.last_quantum_units,
    ));
    execution.last_completed_phase = Some("started".to_string());
    if resumed_from_started {
        execution.resume_count += 1;
    }
    next.next_maintenance_checkpoint_order += 1;
    let checkpoint_order = next.next_maintenance_checkpoint_order;
    next.maintenance_checkpoint_records.insert(
        format!(
            "maintenance-checkpoint:{}:{}",
            declaration_id.as_str(),
            checkpoint_order
        ),
        MaintenanceCheckpointRecord {
            artifact_id: format!(
                "maintenance-checkpoint:{}:{}",
                declaration_id.as_str(),
                checkpoint_order
            ),
            family_version: 1,
            declaration_id: declaration_id.as_str().to_string(),
            completed_phase: "started".to_string(),
            checkpoint_order,
        },
    );
    super::super::summaries::refresh_scheduler_summaries(&mut next);
    backend.commit_replacement_state(next)?;
    if resumed_from_started {
        backend.counters().record_maintenance_resumes(1);
    }
    backend.counters().record_maintenance_checkpoints(1);
    Ok(())
}

pub(crate) fn persist_completed_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
    completed_phase: &str,
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
    let completed_plan_family = execution.plan_family;
    execution.execution_status = MaintenanceExecutionStatus::Completed;
    execution.pending_reason = None;
    execution.last_completed_phase = Some(completed_phase.to_string());
    execution.durable_error_kind = None;
    execution.durable_error_message = None;
    next.next_maintenance_checkpoint_order += 1;
    let checkpoint_order = next.next_maintenance_checkpoint_order;
    next.maintenance_checkpoint_records.insert(
        format!(
            "maintenance-checkpoint:{}:{}",
            declaration_id.as_str(),
            checkpoint_order
        ),
        MaintenanceCheckpointRecord {
            artifact_id: format!(
                "maintenance-checkpoint:{}:{}",
                declaration_id.as_str(),
                checkpoint_order
            ),
            family_version: 1,
            declaration_id: declaration_id.as_str().to_string(),
            completed_phase: completed_phase.to_string(),
            checkpoint_order,
        },
    );
    super::super::summaries::refresh_scheduler_summaries(&mut next);
    backend.commit_replacement_state(next)?;
    backend.counters().record_maintenance_completions(1);
    backend.counters().record_maintenance_checkpoints(1);
    if matches!(
        completed_plan_family,
        Some(crate::MaintenancePlanFamily::BackgroundPaced)
    ) {
        backend
            .counters()
            .record_maintenance_background_unit_execution(1);
    }
    Ok(())
}
