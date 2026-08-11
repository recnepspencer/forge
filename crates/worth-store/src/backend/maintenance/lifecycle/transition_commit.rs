use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        records::StoreState,
    },
    failure::{StoreError, StoreErrorKind},
    maintenance::{MaintenanceExecutionStatus, MaintenanceForegroundImpact, MaintenancePlanFamily},
};

use super::{deferred::MaintenanceDispositionUpdate, reservation::MaintenanceReservationUpdate};

pub(super) fn commit_deferred_disposition<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: MaintenanceDispositionUpdate,
) -> Result<(), StoreError> {
    let mut next = backend.state().clone();
    apply_disposition_transition(
        &mut next,
        &update,
        MaintenanceExecutionStatus::Deferred,
        MaintenancePlanFamily::Deferred,
        "deferred",
    )?;
    commit_maintenance_lifecycle_state(backend, next)?;
    record_disposition_counters(backend, &update, false);
    Ok(())
}

pub(super) fn commit_cancelled_disposition<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: MaintenanceDispositionUpdate,
) -> Result<(), StoreError> {
    let mut next = backend.state().clone();
    apply_disposition_transition(
        &mut next,
        &update,
        MaintenanceExecutionStatus::Cancelled,
        MaintenancePlanFamily::Cancelled,
        "cancelled",
    )?;
    commit_maintenance_lifecycle_state(backend, next)?;
    record_disposition_counters(backend, &update, true);
    Ok(())
}

pub(super) fn commit_reservation<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: MaintenanceReservationUpdate,
) -> Result<(), StoreError> {
    let mut next = backend.state().clone();
    let execution = next
        .maintenance_execution_records
        .get_mut(update.declaration_id.as_str())
        .ok_or_else(|| missing_execution_error(&update.declaration_id))?;
    execution.execution_status = MaintenanceExecutionStatus::Reserved;
    execution.lane_key = Some(update.lane_key.clone());
    execution.plan_family = Some(update.plan_family);
    execution.pending_reason = None;
    execution.last_quantum_units = Some(update.quantum_units);
    execution.foreground_impact = if matches!(update.plan_family, MaintenancePlanFamily::Escalated)
    {
        MaintenanceForegroundImpact::escalated()
    } else {
        MaintenanceForegroundImpact::none()
    };
    execution.reservation_transition = Some(crate::MaintenanceReservationTransition::new(
        update.plan_family,
        update.quantum_units,
    ));
    execution.execution_transition = None;
    execution.coalescing_decision = Some(update.coalescing_decision);
    execution.supersession_source = update.supersession_source;
    execution.resource_budget_grant = update.resource_budget_grant.clone();
    execution.starvation_status = Some(update.starvation_status);
    execution.escalation_verdict = Some(update.escalation_verdict);
    execution.explicit_global_scope_debt = update.explicit_global_scope_debt;
    execution.last_completed_phase = Some("reserved".to_string());
    commit_maintenance_lifecycle_state(backend, next)?;
    record_disposition_counters(
        backend,
        &MaintenanceDispositionUpdate {
            declaration_id: update.declaration_id,
            reason: String::new(),
            lane_key: update.lane_key,
            coalescing_decision: update.coalescing_decision,
            supersession_source: update.supersession_source,
            starvation_status: update.starvation_status,
            escalation_verdict: update.escalation_verdict,
            explicit_global_scope_debt: update.explicit_global_scope_debt,
        },
        false,
    );
    if let Some(grant) = &update.resource_budget_grant {
        backend.counters().record_maintenance_budget_units_reserved(
            grant.granted_io().units(),
            grant.granted_cpu().units(),
            grant.granted_memory().units(),
            grant.granted_publication().units(),
        );
        backend.counters().record_maintenance_quantum_grants(1);
    }
    if matches!(update.plan_family, MaintenancePlanFamily::Escalated) {
        backend.counters().record_maintenance_foreground_borrow(1);
        backend.counters().record_maintenance_foreground_wait(1);
        backend.counters().record_maintenance_cutover_dependency(1);
    }
    Ok(())
}

fn apply_disposition_transition(
    state: &mut StoreState,
    update: &MaintenanceDispositionUpdate,
    execution_status: MaintenanceExecutionStatus,
    plan_family: MaintenancePlanFamily,
    completed_phase: &str,
) -> Result<(), StoreError> {
    let execution = state
        .maintenance_execution_records
        .get_mut(update.declaration_id.as_str())
        .ok_or_else(|| missing_execution_error(&update.declaration_id))?;
    execution.execution_status = execution_status;
    execution.lane_key = Some(update.lane_key.clone());
    execution.plan_family = Some(plan_family);
    execution.pending_reason = Some(update.reason.clone());
    execution.last_quantum_units = None;
    execution.reservation_transition = None;
    execution.execution_transition = None;
    execution.foreground_impact = MaintenanceForegroundImpact::none();
    execution.coalescing_decision = Some(update.coalescing_decision);
    execution.supersession_source = update.supersession_source.clone();
    execution.resource_budget_grant = None;
    execution.starvation_status = Some(update.starvation_status);
    execution.escalation_verdict = Some(update.escalation_verdict);
    execution.explicit_global_scope_debt = update.explicit_global_scope_debt;
    execution.last_completed_phase = Some(completed_phase.to_string());
    Ok(())
}

fn commit_maintenance_lifecycle_state<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    mut next: StoreState,
) -> Result<(), StoreError> {
    super::super::summaries::refresh_scheduler_summaries(&mut next);
    backend.commit_replacement_state(next)
}

fn record_disposition_counters<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    update: &MaintenanceDispositionUpdate,
    cancelled: bool,
) {
    if matches!(
        update.coalescing_decision,
        crate::MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember
    ) {
        backend.counters().record_maintenance_coalesced_work(1);
    }
    if matches!(
        update.coalescing_decision,
        crate::MaintenanceCoalescingDecision::CancelledAsSuperseded
    ) {
        backend
            .counters()
            .record_maintenance_cancelled_superseded_work(1);
        if cancelled
            && update
                .supersession_source
                .as_deref()
                .is_some_and(|source| source.contains("freshness window"))
        {
            backend
                .counters()
                .record_maintenance_freshness_rejections(1);
        }
    }
    if update.explicit_global_scope_debt {
        backend.counters().record_maintenance_store_global_scope(1);
    }
    if matches!(
        update.starvation_status,
        crate::MaintenanceStarvationStatus::DeferredLanePressure
    ) {
        backend.counters().record_maintenance_starvation_trigger(1);
    }
    if matches!(
        update.escalation_verdict,
        crate::MaintenanceEscalationVerdict::EscalatedForDebtPressure
    ) {
        backend.counters().record_maintenance_debt_escalation(1);
    }
}

fn missing_execution_error(declaration_id: &crate::MaintenanceDeclarationId) -> StoreError {
    StoreError::new(
        StoreErrorKind::MaintenanceDeclarationMissing,
        format!(
            "maintenance execution record for `{}` is not persisted",
            declaration_id.as_str()
        ),
    )
}
