use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    maintenance::{
        AdmittedMaintenanceWork, CompletedMaintenance, ExecutingMaintenanceWork, FailedMaintenance,
        MaintenanceDeclaration, MaintenanceDeclarationId, MaintenanceExecutionStatus,
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
        .cloned();
    let current_execution = current_status.clone();
    let current_status = current_execution
        .as_ref()
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
    let context =
        super::summaries::scheduler_admission_context(backend.state(), &descriptor.lane_key());
    let resumed_execution = current_execution.as_ref().and_then(|record| {
        if allow_resume && matches!(record.execution_status, MaintenanceExecutionStatus::Started) {
            Some(super::planning::ResumedExecutionState {
                plan_family: record.plan_family?,
                resource_budget_grant: record.resource_budget_grant.clone()?,
                starvation_status: record
                    .starvation_status
                    .unwrap_or(crate::MaintenanceStarvationStatus::NotStarved),
                escalation_verdict: record
                    .escalation_verdict
                    .unwrap_or(crate::MaintenanceEscalationVerdict::NoEscalation),
                explicit_global_scope_debt: record.explicit_global_scope_debt,
            })
        } else {
            None
        }
    });
    let planning_decision = match super::planning::lower_maintenance_plan(
        &admitted_work,
        allow_resume,
        current_status,
        resumed_execution.as_ref(),
        &context,
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

    let resumed_from_started =
        allow_resume && matches!(current_status, MaintenanceExecutionStatus::Started);
    if !resumed_from_started {
        match planning_decision.family() {
            crate::MaintenancePlanFamily::Deferred => {
                let reason = planning_decision
                    .reason()
                    .expect("deferred maintenance plans must carry a reason");
                let _ = super::lifecycle::persist_deferred_state(
                    backend,
                    declaration_id,
                    reason,
                    planning_decision.lane_key().clone(),
                    planning_decision.coalescing_decision(),
                    planning_decision
                        .supersession_source()
                        .map(ToString::to_string),
                    planning_decision.starvation_status(),
                    planning_decision.escalation_verdict(),
                    planning_decision.explicit_global_scope_debt(),
                );
                return Err(FailedMaintenance::new(
                    declaration,
                    Some(descriptor),
                    crate::MaintenanceFailureKind::Deferred,
                    "MaintenanceDeferred",
                    reason.to_string(),
                ));
            }
            crate::MaintenancePlanFamily::Cancelled => {
                let reason = planning_decision
                    .reason()
                    .expect("cancelled maintenance plans must carry a reason");
                let _ = super::lifecycle::persist_cancelled_state(
                    backend,
                    declaration_id,
                    reason,
                    planning_decision.lane_key().clone(),
                    planning_decision.coalescing_decision(),
                    planning_decision
                        .supersession_source()
                        .map(ToString::to_string),
                    planning_decision.starvation_status(),
                    planning_decision.escalation_verdict(),
                    planning_decision.explicit_global_scope_debt(),
                );
                return Err(FailedMaintenance::new(
                    declaration,
                    Some(descriptor),
                    crate::MaintenanceFailureKind::Cancelled,
                    "MaintenanceCancelled",
                    reason.to_string(),
                ));
            }
            crate::MaintenancePlanFamily::ForegroundReserved
            | crate::MaintenancePlanFamily::BackgroundPaced
            | crate::MaintenancePlanFamily::Escalated => {
                let quantum_units = planning_decision
                    .quantum_units()
                    .expect("reserved maintenance plans must carry a quantum receipt");
                if let Err(error) = super::lifecycle::persist_reserved_state(
                    backend,
                    declaration_id,
                    planning_decision.family(),
                    quantum_units,
                    planning_decision.lane_key().clone(),
                    planning_decision.coalescing_decision(),
                    planning_decision
                        .supersession_source()
                        .map(ToString::to_string),
                    planning_decision.resource_budget_grant().cloned(),
                    planning_decision.starvation_status(),
                    planning_decision.escalation_verdict(),
                    planning_decision.explicit_global_scope_debt(),
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
    }

    let reserved_work = planning_decision
        .into_reserved_work(admitted_work)
        .expect("reserved maintenance plans must lower into reserved work");
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
            Ok(CompletedMaintenance::new(
                declaration,
                descriptor,
                completed_phase,
            ))
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
