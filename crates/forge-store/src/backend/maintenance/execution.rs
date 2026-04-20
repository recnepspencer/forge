use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    maintenance::{
        CompletedMaintenance, FailedMaintenance, MaintenanceDeclaration, MaintenanceDeclarationId,
        MaintenanceExecutionStatus, StartedMaintenance,
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
    let declaration = match backend
        .state()
        .maintenance_declaration_records
        .get(declaration_id.as_str())
        .map(|record| record.declaration.clone())
    {
        Some(declaration) => declaration,
        None => {
            return Err(FailedMaintenance::new(
                MaintenanceDeclaration::retention(
                    declaration_id.clone(),
                    crate::RetentionMaintenanceDeclaration::new("missing", 0, 0),
                ),
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
            format!("{:?}", error.kind()),
            error.message().to_string(),
        ));
    }
    if let Err(error) =
        super::declaration_execution::ensure_execution_eligibility(backend, &declaration)
    {
        return Err(FailedMaintenance::new(
            declaration,
            format!("{:?}", error.kind()),
            error.message().to_string(),
        ));
    }

    if let Err(error) = super::lifecycle::persist_started_state(backend, declaration_id) {
        return Err(FailedMaintenance::new(
            declaration,
            format!("{:?}", error.kind()),
            error.message().to_string(),
        ));
    }
    let started = StartedMaintenance::new(declaration.clone());
    let execution = super::declaration_execution::execute_started_declaration(backend, &started);
    match execution {
        Ok(completed_phase) => {
            if let Err(error) =
                super::lifecycle::persist_completed_state(backend, declaration_id, &completed_phase)
            {
                return Err(FailedMaintenance::new(
                    declaration,
                    format!("{:?}", error.kind()),
                    error.message().to_string(),
                ));
            }
            Ok(CompletedMaintenance::new(declaration, completed_phase))
        }
        Err(error) => {
            let _ = super::lifecycle::persist_failed_state(backend, declaration_id, &error);
            Err(FailedMaintenance::new(
                declaration,
                format!("{:?}", error.kind()),
                error.message().to_string(),
            ))
        }
    }
}
