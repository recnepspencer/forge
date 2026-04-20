use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        records::StoreState,
    },
    evidence::StoreCounterSnapshot,
};

pub(crate) fn milestone_11_counter_contract<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone11CounterContract {
    let snapshot = backend.counters().snapshot();
    let declaration_records = backend.state().maintenance_declaration_records.values();
    let queue_family_count = declaration_records
        .clone()
        .map(|record| record.work_descriptor.work_class())
        .collect::<std::collections::BTreeSet<_>>()
        .len() as u64;
    let locality_bucket_count = declaration_records
        .clone()
        .map(|record| record.work_descriptor.locality_scope().clone())
        .collect::<std::collections::BTreeSet<_>>()
        .len() as u64;
    let explicit_foreground_reservation_count = declaration_records
        .clone()
        .filter(|record| {
            matches!(
                record.work_descriptor.reservation_family(),
                crate::MaintenanceReservationFamily::Foreground(_)
            )
        })
        .count() as u64;
    let explicit_background_reservation_count = declaration_records
        .clone()
        .filter(|record| {
            matches!(
                record.work_descriptor.reservation_family(),
                crate::MaintenanceReservationFamily::Background(_)
            )
        })
        .count() as u64;
    let restart_recovered_descriptor_count = declaration_records
        .filter(|record| record.work_descriptor.recovered_from_restart())
        .count() as u64;
    crate::Milestone11CounterContract {
        maintenance_declaration_count: snapshot.maintenance_declaration_count,
        maintenance_admission_count: snapshot.maintenance_admission_count,
        maintenance_rejection_count: snapshot.maintenance_rejection_count,
        maintenance_resume_count: snapshot.maintenance_resume_count,
        maintenance_restart_readmission_count: snapshot.maintenance_restart_readmission_count,
        maintenance_restart_rejection_count: snapshot.maintenance_restart_rejection_count,
        maintenance_checkpoint_count: snapshot.maintenance_checkpoint_count,
        maintenance_completion_count: snapshot.maintenance_completion_count,
        maintenance_failure_count: snapshot.maintenance_failure_count,
        maintenance_debt_link_count: snapshot.maintenance_debt_link_count,
        maintenance_foreground_borrow_count: snapshot.maintenance_foreground_borrow_count,
        maintenance_foreground_wait_count: snapshot.maintenance_foreground_wait_count,
        maintenance_cutover_dependency_count: snapshot.maintenance_cutover_dependency_count,
        scheduler_work_class_lane_count: queue_family_count,
        scheduler_locality_bucket_count: locality_bucket_count,
        explicit_foreground_reservation_count,
        explicit_background_reservation_count,
        restart_recovered_descriptor_count,
    }
}

pub(crate) fn milestone_11_complexity_surface<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone11ComplexitySurface {
    complexity_surface_from_parts(backend.state(), &backend.counters().snapshot())
}

pub(crate) fn milestone_11_maintenance_report<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone11MaintenanceReport {
    let work_class_counts = backend
        .state()
        .maintenance_declaration_records
        .values()
        .fold(
            std::collections::BTreeMap::<crate::MaintenanceWorkClass, u64>::new(),
            |mut counts, record| {
                *counts.entry(record.work_descriptor.work_class()).or_default() += 1;
                counts
            },
        )
        .into_iter()
        .map(|(work_class, declaration_count)| crate::Milestone11WorkClassCount {
            work_class,
            declaration_count,
        })
        .collect::<Vec<_>>();
    let reservation_family_counts = backend
        .state()
        .maintenance_declaration_records
        .values()
        .fold(
            std::collections::BTreeMap::<crate::MaintenanceReservationFamily, u64>::new(),
            |mut counts, record| {
                *counts
                    .entry(record.work_descriptor.reservation_family())
                    .or_default() += 1;
                counts
            },
        )
        .into_iter()
        .map(
            |(reservation_family, declaration_count)| crate::Milestone11ReservationFamilyCount {
                reservation_family,
                declaration_count,
            },
        )
        .collect::<Vec<_>>();
    let locality_scope_counts = backend
        .state()
        .maintenance_declaration_records
        .values()
        .fold(
            std::collections::BTreeMap::<crate::MaintenanceLocalityScope, u64>::new(),
            |mut counts, record| {
                *counts
                    .entry(record.work_descriptor.locality_scope().clone())
                    .or_default() += 1;
                counts
            },
        )
        .into_iter()
        .map(|(locality_scope, declaration_count)| crate::Milestone11LocalityScopeCount {
            locality_scope,
            declaration_count,
        })
        .collect::<Vec<_>>();
    let scheduler_topology = crate::Milestone11SchedulerTopologyReport {
        queue_family_count: work_class_counts.len() as u64,
        locality_bucket_count: locality_scope_counts.len() as u64,
        has_restart_recovered_intake_lane: true,
        has_foreground_reservation_pool: true,
        has_background_reservation_pool: true,
    };
    crate::Milestone11MaintenanceReport {
        declared_batch_count: backend.state().maintenance_batch_records.len() as u64,
        persisted_declaration_count: backend.state().maintenance_declaration_records.len() as u64,
        active_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Admitted
                        | crate::MaintenanceExecutionStatus::Reserved
                        | crate::MaintenanceExecutionStatus::Started
                )
            })
            .count() as u64,
        reserved_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Reserved
                )
            })
            .count() as u64,
        deferred_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Deferred
                )
            })
            .count() as u64,
        escalated_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(record.plan_family, Some(crate::MaintenancePlanFamily::Escalated))
            })
            .count() as u64,
        cancelled_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Cancelled
                )
            })
            .count() as u64,
        readmitted_recovered_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.restart_readmission_status,
                    Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork)
                )
            })
            .count() as u64,
        rejected_recovered_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.restart_readmission_status,
                    Some(crate::MaintenanceReadmissionStatus::RejectedStaleRecoveredWork)
                        | Some(crate::MaintenanceReadmissionStatus::RejectedSupersededRecoveredWork)
                )
            })
            .count() as u64,
        completed_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Completed
                )
            })
            .count() as u64,
        failed_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Failed
                )
            })
            .count() as u64,
        checkpoint_count: backend.state().maintenance_checkpoint_records.len() as u64,
        recovered_declaration_count: backend
            .state()
            .maintenance_declaration_records
            .values()
            .filter(|record| record.work_descriptor.recovered_from_restart())
            .count() as u64,
        foreground_borrowed_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| record.foreground_impact.borrowed_foreground_reservation())
            .count() as u64,
        foreground_waited_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| record.foreground_impact.foreground_wait_required())
            .count() as u64,
        cutover_dependency_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| record.foreground_impact.cutover_dependency_required())
            .count() as u64,
        scheduler_topology,
        work_class_counts,
        reservation_family_counts,
        locality_scope_counts,
    }
}

fn complexity_surface_from_parts(
    state: &StoreState,
    snapshot: &StoreCounterSnapshot,
) -> crate::Milestone11ComplexitySurface {
    let declaration_lowering = if snapshot.maintenance_declaration_count > 0 {
        crate::Milestone11ComplexityPathStatus::verified(
            "retention maintenance lowering publishes deterministic declaration identities before execution",
        )
    } else {
        crate::Milestone11ComplexityPathStatus::verified(
            "maintenance lowering surface is compiled and awaiting the first declared batch",
        )
    };
    let batch_admission = if snapshot.maintenance_rejection_count > 0 {
        crate::Milestone11ComplexityPathStatus::debt(
            "batch admission has already encountered duplicate or conflicting declarations",
        )
    } else {
        crate::Milestone11ComplexityPathStatus::verified(
            "batch admission persists declarations and execution status records before work starts",
        )
    };
    let maintenance_resume = if state.maintenance_execution_records.values().any(|record| {
        matches!(
            record.execution_status,
            crate::MaintenanceExecutionStatus::Started
        )
    }) {
        crate::Milestone11ComplexityPathStatus::debt(
            "there are started maintenance declarations waiting to resume after interruption",
        )
    } else {
        crate::Milestone11ComplexityPathStatus::verified(
            "no interrupted maintenance declarations are waiting for resume",
        )
    };
    let durable_status_lookup = crate::Milestone11ComplexityPathStatus::verified(
        "durable status lookup is keyed directly by persisted declaration identity",
    );
    crate::Milestone11ComplexitySurface {
        declaration_lowering,
        batch_admission,
        maintenance_resume,
        durable_status_lookup,
    }
}
