use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    maintenance::{
        AdmittedMaintenanceWork, CompletedMaintenance, ExecutingMaintenanceWork,
        FailedMaintenance, MaintenanceDeclaration, MaintenanceDeclarationId,
        MaintenanceExecutionStatus,
    },
};

pub(crate) fn start_maintenance_declaration<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
) -> Result<CompletedMaintenance, FailedMaintenance> {
    execute_declared_maintenance(backend, declaration_id, false)
}

pub(crate) fn resume_maintenance_declaration<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
) -> Result<CompletedMaintenance, FailedMaintenance> {
    execute_declared_maintenance(backend, declaration_id, true)
}

fn execute_declared_maintenance<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    declaration_id: &MaintenanceDeclarationId,
    allow_resume: bool,
) -> Result<CompletedMaintenance, FailedMaintenance> {
    let (declaration, descriptor) = match backend
        .state()
        .maintenance_declaration_records
        .get(declaration_id.as_str())
        .map(|record| (record.declaration.clone(), record.work_descriptor.clone()))
    {
        Some(declaration) => declaration,
        None => {
            return Err(FailedMaintenance::new(
                MaintenanceDeclaration::retention(
                    declaration_id.clone(),
                    crate::RetentionMaintenanceDeclaration::new("missing", 0, 0),
                ),
                None,
                crate::MaintenanceFailureKind::RestartAdmissionFailure,
                "MaintenanceDeclarationMissing",
                format!(
                    "maintenance declaration `{}` is not persisted",
                    declaration_id.as_str()
                ),
            ))
        }
    };

    let current_status = backend
        .state()
        .maintenance_execution_records
        .get(declaration_id.as_str())
        .map(|record| record.execution_status)
        .unwrap_or(MaintenanceExecutionStatus::Declared);
    if let Err(error) =
        super::lifecycle::ensure_execution_status(current_status, declaration_id, allow_resume)
    {
        return Err(FailedMaintenance::new(
            declaration,
            Some(descriptor),
            crate::MaintenanceFailureKind::ReservationViolation,
            format!("{:?}", error.kind()),
            error.message().to_string(),
        ));
    }
    if let Err(error) = super::lifecycle::perform_restart_readmission(
        backend,
        &declaration,
        declaration_id,
        &descriptor,
    ) {
        return Err(error);
    }
    if let Err(error) =
        super::declaration_execution::ensure_execution_eligibility(backend, &declaration)
    {
        return Err(FailedMaintenance::new(
            declaration,
            Some(descriptor),
            crate::MaintenanceFailureKind::ExecutionFailure,
            format!("{:?}", error.kind()),
            error.message().to_string(),
        ));
    }

    let admitted_work = AdmittedMaintenanceWork::new(declaration.clone(), descriptor.clone());
    let lowered_plan = match super::planning::lower_maintenance_plan(
        &admitted_work,
        allow_resume,
        current_status,
    ) {
        Ok(plan) => plan,
        Err(error) => {
            return Err(FailedMaintenance::new(
                declaration,
                Some(descriptor),
                crate::MaintenanceFailureKind::ReservationViolation,
                format!("{:?}", error.kind()),
                error.message().to_string(),
            ))
        }
    };

    match &lowered_plan {
        super::planning::LoweredMaintenancePlan::Deferred(_) => {
            let reason = lowered_plan
                .reason()
                .expect("deferred maintenance plans must carry a reason");
            let _ = super::lifecycle::persist_deferred_state(backend, declaration_id, reason);
            return Err(FailedMaintenance::new(
                declaration,
                Some(descriptor),
                crate::MaintenanceFailureKind::Deferred,
                "MaintenanceDeferred",
                reason.to_string(),
            ));
        }
        super::planning::LoweredMaintenancePlan::Cancelled { .. } => {
            let reason = lowered_plan
                .reason()
                .expect("cancelled maintenance plans must carry a reason");
            let _ = super::lifecycle::persist_cancelled_state(backend, declaration_id, reason);
            return Err(FailedMaintenance::new(
                declaration,
                Some(descriptor),
                crate::MaintenanceFailureKind::Cancelled,
                "MaintenanceCancelled",
                reason.to_string(),
            ));
        }
        super::planning::LoweredMaintenancePlan::ForegroundReserved(_)
        | super::planning::LoweredMaintenancePlan::BackgroundPaced(_)
        | super::planning::LoweredMaintenancePlan::Escalated(_) => {
            let quantum_units = lowered_plan
                .quantum_units()
                .expect("reserved maintenance plans must carry a quantum receipt");
            if let Err(error) = super::lifecycle::persist_reserved_state(
                backend,
                declaration_id,
                lowered_plan.family(),
                quantum_units,
            ) {
                return Err(FailedMaintenance::new(
                    declaration,
                    Some(descriptor),
                    crate::MaintenanceFailureKind::ReservationViolation,
                    format!("{:?}", error.kind()),
                    error.message().to_string(),
                ));
            }
        }
    }

    let reserved_work = lowered_plan
        .clone()
        .into_reserved_work(admitted_work)
        .expect("reserved maintenance plans must lower into reserved work");
    let resumed_from_started =
        allow_resume && matches!(current_status, MaintenanceExecutionStatus::Started);
    if let Err(error) =
        super::lifecycle::persist_started_state(backend, declaration_id, resumed_from_started)
    {
        return Err(FailedMaintenance::new(
            declaration,
            Some(descriptor),
            crate::MaintenanceFailureKind::ExecutionFailure,
            format!("{:?}", error.kind()),
            error.message().to_string(),
        ));
    }
    let executing_work = ExecutingMaintenanceWork::new(reserved_work);
    let execution =
        super::declaration_execution::execute_started_declaration(backend, &executing_work);
    match execution {
        Ok(completed_phase) => {
            if let Err(error) =
                super::lifecycle::persist_completed_state(backend, declaration_id, &completed_phase)
            {
                return Err(FailedMaintenance::new(
                    declaration,
                    Some(descriptor),
                    crate::MaintenanceFailureKind::ExecutionFailure,
                    format!("{:?}", error.kind()),
                    error.message().to_string(),
                ));
            }
            Ok(CompletedMaintenance::new(declaration, descriptor, completed_phase))
        }
        Err(error) => {
            let _ = super::lifecycle::persist_failed_state(backend, declaration_id, &error);
            Err(FailedMaintenance::new(
                declaration,
                Some(descriptor),
                crate::MaintenanceFailureKind::ExecutionFailure,
                format!("{:?}", error.kind()),
                error.message().to_string(),
            ))
        }
    }
}
