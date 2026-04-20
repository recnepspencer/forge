use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        records::MaintenanceCheckpointRecord,
    },
    failure::{StoreError, StoreErrorKind},
    maintenance::{MaintenanceDeclarationId, MaintenanceExecutionStatus, MaintenanceStatusReport},
};

pub(crate) fn maintenance_status<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
) -> Result<MaintenanceStatusReport, StoreError> {
    let declaration = backend
        .state()
        .maintenance_declaration_records
        .get(declaration_id.as_str())
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::MaintenanceDeclarationMissing,
                format!(
                    "maintenance declaration `{}` is not persisted",
                    declaration_id.as_str()
                ),
            )
        })?;
    let execution = backend
        .state()
        .maintenance_execution_records
        .get(declaration_id.as_str())
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::MaintenanceDeclarationMissing,
                format!(
                    "maintenance execution record for `{}` is not persisted",
                    declaration_id.as_str()
                ),
            )
        })?;
    Ok(MaintenanceStatusReport::new(
        declaration.declaration.id().clone(),
        declaration.declaration_class,
        execution.execution_status,
        execution.last_completed_phase.clone(),
        execution.durable_error_kind.clone(),
        declaration.debt_link_artifact_id.clone(),
    ))
}

pub(crate) fn ensure_execution_status(
    current_status: MaintenanceExecutionStatus,
    declaration_id: &MaintenanceDeclarationId,
    allow_resume: bool,
) -> Result<(), StoreError> {
    let allowed = matches!(current_status, MaintenanceExecutionStatus::Admitted)
        || (allow_resume && matches!(current_status, MaintenanceExecutionStatus::Started));
    if allowed {
        Ok(())
    } else {
        Err(StoreError::new(
            StoreErrorKind::MaintenanceLifecycleViolation,
            format!(
                "maintenance declaration `{}` cannot execute from status {:?}",
                declaration_id.as_str(),
                current_status
            ),
        ))
    }
}

pub(crate) fn persist_started_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
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
    let is_resume = matches!(
        execution.execution_status,
        MaintenanceExecutionStatus::Started
    );
    execution.execution_status = MaintenanceExecutionStatus::Started;
    execution.last_completed_phase = Some("started".to_string());
    if is_resume {
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
    backend.commit_replacement_state(next)?;
    if is_resume {
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
    execution.execution_status = MaintenanceExecutionStatus::Completed;
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
    backend.commit_replacement_state(next)?;
    backend.counters().record_maintenance_completions(1);
    backend.counters().record_maintenance_checkpoints(1);
    Ok(())
}

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
    execution.last_completed_phase = Some("failed".to_string());
    backend.commit_replacement_state(next)?;
    backend.counters().record_maintenance_failures(1);
    Ok(())
}
