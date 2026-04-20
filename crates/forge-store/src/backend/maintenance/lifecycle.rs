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
        declaration.work_descriptor.work_class(),
        declaration.work_descriptor.execution_posture(),
        declaration.work_descriptor.locality_scope().clone(),
        declaration.work_descriptor.reservation_family(),
        declaration.work_descriptor.plan_generation(),
        declaration.work_descriptor.supersession_epoch(),
        declaration.work_descriptor.freshness_window(),
        declaration.work_descriptor.debt_family(),
        declaration.work_descriptor.escalation_decision(),
        declaration.work_descriptor.recovered_from_restart(),
        execution.restart_readmission_status,
        execution.reservation_transition.clone(),
        execution.execution_transition.clone(),
        execution.foreground_impact.clone(),
        execution.plan_family,
        execution.pending_reason.clone(),
        execution.execution_status,
        execution.last_completed_phase.clone(),
        execution.durable_error_kind.clone(),
        declaration.debt_link_artifact_id.clone(),
    ))
}

pub(crate) fn perform_restart_readmission<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration: &crate::MaintenanceDeclaration,
    declaration_id: &MaintenanceDeclarationId,
    descriptor: &crate::MaintenanceWorkDescriptor,
) -> Result<(), crate::FailedMaintenance> {
    if !descriptor.recovered_from_restart() {
        return Ok(());
    }

    let mut next = backend.state().clone();
    let execution = next
        .maintenance_execution_records
        .get_mut(declaration_id.as_str())
        .ok_or_else(|| {
            crate::FailedMaintenance::new(
                declaration.clone(),
                Some(descriptor.clone()),
                crate::MaintenanceFailureKind::RestartAdmissionFailure,
                "MaintenanceDeclarationMissing",
                format!(
                    "maintenance execution record for `{}` is not persisted",
                    declaration_id.as_str()
                ),
            )
        })?;

    let readmission_status = if descriptor.freshness_window().value() == 0 {
        execution.execution_status = MaintenanceExecutionStatus::Cancelled;
        execution.plan_family = Some(crate::MaintenancePlanFamily::Cancelled);
        execution.pending_reason =
            Some("recovered maintenance descriptor became stale before readmission".to_string());
        execution.restart_readmission_status =
            Some(crate::MaintenanceReadmissionStatus::RejectedStaleRecoveredWork);
        backend.counters().record_maintenance_restart_rejections(1);
        backend.commit_replacement_state(next).map_err(|error| {
            crate::FailedMaintenance::new(
                declaration.clone(),
                Some(descriptor.clone()),
                crate::MaintenanceFailureKind::RestartAdmissionFailure,
                format!("{:?}", error.kind()),
                error.message().to_string(),
            )
        })?;
        return Err(crate::FailedMaintenance::new(
            declaration.clone(),
            Some(descriptor.clone()),
            crate::MaintenanceFailureKind::Cancelled,
            "RecoveredMaintenanceRejected",
            "recovered maintenance descriptor became stale before readmission",
        ));
    } else if execution.execution_status == MaintenanceExecutionStatus::Cancelled {
        execution.restart_readmission_status =
            Some(crate::MaintenanceReadmissionStatus::RejectedSupersededRecoveredWork);
        backend.counters().record_maintenance_restart_rejections(1);
        backend.commit_replacement_state(next).map_err(|error| {
            crate::FailedMaintenance::new(
                declaration.clone(),
                Some(descriptor.clone()),
                crate::MaintenanceFailureKind::RestartAdmissionFailure,
                format!("{:?}", error.kind()),
                error.message().to_string(),
            )
        })?;
        return Err(crate::FailedMaintenance::new(
            declaration.clone(),
            Some(descriptor.clone()),
            crate::MaintenanceFailureKind::Cancelled,
            "RecoveredMaintenanceRejected",
            "recovered maintenance declaration was already cancelled before readmission",
        ));
    } else {
        crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork
    };

    execution.restart_readmission_status = Some(readmission_status);
    backend.counters().record_maintenance_restart_readmissions(1);
    backend.commit_replacement_state(next).map_err(|error| {
        crate::FailedMaintenance::new(
            declaration.clone(),
            Some(descriptor.clone()),
            crate::MaintenanceFailureKind::RestartAdmissionFailure,
            format!("{:?}", error.kind()),
            error.message().to_string(),
        )
    })?;
    Ok(())
}

pub(crate) fn ensure_execution_status(
    current_status: MaintenanceExecutionStatus,
    declaration_id: &MaintenanceDeclarationId,
    allow_resume: bool,
) -> Result<(), StoreError> {
    let allowed = matches!(current_status, MaintenanceExecutionStatus::Admitted)
        || matches!(current_status, MaintenanceExecutionStatus::Reserved)
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

pub(crate) fn persist_reserved_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
    plan_family: crate::MaintenancePlanFamily,
    quantum_units: u64,
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
    execution.execution_status = MaintenanceExecutionStatus::Reserved;
    execution.plan_family = Some(plan_family);
    execution.pending_reason = None;
    execution.last_quantum_units = Some(quantum_units);
    execution.foreground_impact = if matches!(plan_family, crate::MaintenancePlanFamily::Escalated)
    {
        crate::MaintenanceForegroundImpact::escalated()
    } else {
        crate::MaintenanceForegroundImpact::none()
    };
    execution.reservation_transition = Some(crate::MaintenanceReservationTransition::new(
        plan_family,
        quantum_units,
    ));
    execution.last_completed_phase = Some("reserved".to_string());
    backend.commit_replacement_state(next)?;
    if matches!(plan_family, crate::MaintenancePlanFamily::Escalated) {
        backend.counters().record_maintenance_foreground_borrow(1);
        backend.counters().record_maintenance_foreground_wait(1);
        backend.counters().record_maintenance_cutover_dependency(1);
    }
    Ok(())
}

pub(crate) fn persist_deferred_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
    reason: &str,
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
    execution.execution_status = MaintenanceExecutionStatus::Deferred;
    execution.plan_family = Some(crate::MaintenancePlanFamily::Deferred);
    execution.pending_reason = Some(reason.to_string());
    execution.last_quantum_units = None;
    execution.reservation_transition = None;
    execution.execution_transition = None;
    execution.foreground_impact = crate::MaintenanceForegroundImpact::none();
    execution.last_completed_phase = Some("deferred".to_string());
    backend.commit_replacement_state(next)?;
    Ok(())
}

pub(crate) fn persist_cancelled_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
    reason: &str,
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
    execution.execution_status = MaintenanceExecutionStatus::Cancelled;
    execution.plan_family = Some(crate::MaintenancePlanFamily::Cancelled);
    execution.pending_reason = Some(reason.to_string());
    execution.last_quantum_units = None;
    execution.reservation_transition = None;
    execution.execution_transition = None;
    execution.foreground_impact = crate::MaintenanceForegroundImpact::none();
    execution.last_completed_phase = Some("cancelled".to_string());
    backend.commit_replacement_state(next)?;
    Ok(())
}

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
    execution.pending_reason = None;
    execution.last_completed_phase = Some("failed".to_string());
    backend.commit_replacement_state(next)?;
    backend.counters().record_maintenance_failures(1);
    Ok(())
}
