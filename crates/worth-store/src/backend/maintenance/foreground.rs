use crate::{
    ForegroundBroadeningCause, ForegroundInterferencePosture, ForegroundIsolationOutcome,
    ForegroundIsolationViolation, ForegroundReservationClass, ForegroundWaitDependency,
    MaintenanceExecutionStatus, MaintenanceLocalityScope, MaintenancePlanFamily,
};

use super::super::{
    engine::{StateBackedStoreBackend, StatePersistence},
    records::StoreState,
};

pub(crate) fn branch_locality_scope(
    branch_id: &worth_relational::facade::history::BranchId,
) -> MaintenanceLocalityScope {
    MaintenanceLocalityScope::BranchLocalityScope {
        branch_label: branch_id.0.clone(),
    }
}

pub(crate) fn assess_foreground_isolation<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    reservation_class: ForegroundReservationClass,
    locality_scope: &MaintenanceLocalityScope,
    allow_broadening: bool,
) -> ForegroundIsolationOutcome {
    let outcome = assess_foreground_isolation_from_state(
        backend.state(),
        reservation_class,
        locality_scope,
        allow_broadening,
    );
    record_outcome_counters(backend, &outcome);
    outcome
}

pub(crate) fn assess_write_foreground_isolation<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    locality_scope: &MaintenanceLocalityScope,
) -> ForegroundIsolationOutcome {
    assess_foreground_isolation(
        backend,
        ForegroundReservationClass::Write,
        locality_scope,
        false,
    )
}

fn assess_foreground_isolation_from_state(
    state: &StoreState,
    reservation_class: ForegroundReservationClass,
    locality_scope: &MaintenanceLocalityScope,
    allow_broadening: bool,
) -> ForegroundIsolationOutcome {
    let mut observed = false;

    for record in state.maintenance_execution_records.values() {
        if !matches!(
            record.execution_status,
            MaintenanceExecutionStatus::Admitted
                | MaintenanceExecutionStatus::Reserved
                | MaintenanceExecutionStatus::Started
                | MaintenanceExecutionStatus::Deferred
        ) {
            continue;
        }

        let Some(record_lane_key) = &record.lane_key else {
            continue;
        };

        if !locality_matches(record_lane_key.locality_scope(), locality_scope) {
            continue;
        }

        observed = true;

        if matches!(
            record.plan_family,
            Some(MaintenancePlanFamily::ForegroundReserved | MaintenancePlanFamily::Escalated)
        ) && matches!(reservation_class, ForegroundReservationClass::Write)
        {
            return ForegroundIsolationOutcome::violated(
                reservation_class,
                ForegroundIsolationViolation::SharedReservationConflict,
            );
        }

        if record.foreground_impact.cutover_dependency_required() {
            return ForegroundIsolationOutcome::waited(
                reservation_class,
                ForegroundWaitDependency::MaintenanceCutover,
            );
        }

        if record.foreground_impact.foreground_wait_required() {
            return ForegroundIsolationOutcome::waited(
                reservation_class,
                ForegroundWaitDependency::MaintenanceReservationRelease,
            );
        }

        if allow_broadening
            && (record.explicit_global_scope_debt
                || matches!(record.plan_family, Some(MaintenancePlanFamily::Escalated)))
        {
            return ForegroundIsolationOutcome::broadened(
                reservation_class,
                if record.explicit_global_scope_debt {
                    ForegroundBroadeningCause::GlobalDebtPromotion
                } else {
                    ForegroundBroadeningCause::MaintenanceBlockedIsolatedPath
                },
            );
        }
    }

    if observed {
        ForegroundIsolationOutcome::observed_maintenance(reservation_class)
    } else {
        ForegroundIsolationOutcome::stayed_isolated(reservation_class)
    }
}

fn locality_matches(
    maintenance_locality: &MaintenanceLocalityScope,
    foreground_locality: &MaintenanceLocalityScope,
) -> bool {
    maintenance_locality == foreground_locality
        || matches!(
            maintenance_locality,
            MaintenanceLocalityScope::StoreGlobalLocalityScope
        )
}

fn record_outcome_counters<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    outcome: &ForegroundIsolationOutcome,
) {
    if outcome.maintenance_interference() {
        backend
            .counters()
            .record_maintenance_foreground_interference(1);
    }
    if matches!(
        outcome.posture(),
        ForegroundInterferencePosture::BroadenedByMaintenance
    ) {
        backend
            .counters()
            .record_maintenance_foreground_broadened(1);
    }
    if matches!(
        outcome.wait_dependency(),
        Some(ForegroundWaitDependency::MaintenanceCutover)
    ) {
        backend
            .counters()
            .record_maintenance_foreground_wait_on_cutover(1);
    }
    if matches!(
        outcome.posture(),
        ForegroundInterferencePosture::ReservationViolation
    ) {
        backend
            .counters()
            .record_maintenance_reservation_violation(1);
    }
}
