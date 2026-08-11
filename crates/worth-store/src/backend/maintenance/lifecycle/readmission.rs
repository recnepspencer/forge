use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
    maintenance::{MaintenanceDeclarationId, MaintenanceExecutionStatus},
};
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
        execution.lane_key = Some(descriptor.lane_key());
        execution.plan_family = Some(crate::MaintenancePlanFamily::Cancelled);
        execution.pending_reason =
            Some("recovered maintenance descriptor became stale before readmission".to_string());
        execution.restart_readmission_status =
            Some(crate::MaintenanceReadmissionStatus::RejectedStaleRecoveredWork);
        super::super::summaries::refresh_scheduler_summaries(&mut next);
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
        super::super::summaries::refresh_scheduler_summaries(&mut next);
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
    super::super::summaries::refresh_scheduler_summaries(&mut next);
    backend
        .counters()
        .record_maintenance_restart_readmissions(1);
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
