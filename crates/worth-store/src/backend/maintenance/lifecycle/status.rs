use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
    maintenance::{
        MaintenanceDeclarationId, MaintenanceStatusReport, MaintenanceStatusReportBasis,
    },
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
    Ok(MaintenanceStatusReport::new(MaintenanceStatusReportBasis {
        declaration_id: declaration.declaration.id().clone(),
        declaration_class: declaration.declaration_class,
        work_class: declaration.work_descriptor.work_class(),
        execution_posture: declaration.work_descriptor.execution_posture(),
        locality_scope: declaration.work_descriptor.locality_scope().clone(),
        lane_key: declaration.work_descriptor.lane_key(),
        reservation_family: declaration.work_descriptor.reservation_family(),
        plan_generation: declaration.work_descriptor.plan_generation(),
        supersession_epoch: declaration.work_descriptor.supersession_epoch(),
        freshness_window: declaration.work_descriptor.freshness_window(),
        debt_family: declaration.work_descriptor.debt_family(),
        escalation_decision: declaration.work_descriptor.escalation_decision(),
        tier_work_container_class: declaration.work_descriptor.tier_work_container_class(),
        recovered_from_restart: declaration.work_descriptor.recovered_from_restart(),
        restart_readmission_status: execution.restart_readmission_status,
        reservation_transition: execution.reservation_transition.clone(),
        execution_transition: execution.execution_transition.clone(),
        foreground_impact: execution.foreground_impact.clone(),
        coalescing_decision: execution.coalescing_decision,
        supersession_source: execution.supersession_source.clone(),
        resource_budget_grant: execution.resource_budget_grant.clone(),
        starvation_status: execution.starvation_status,
        escalation_verdict: execution.escalation_verdict,
        explicit_global_scope_debt: execution.explicit_global_scope_debt,
        plan_family: execution.plan_family,
        pending_reason: execution.pending_reason.clone(),
        execution_status: execution.execution_status,
        last_completed_phase: execution.last_completed_phase.clone(),
        durable_error_kind: execution.durable_error_kind.clone(),
        debt_link_artifact_id: declaration.debt_link_artifact_id.clone(),
    }))
}
