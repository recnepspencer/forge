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
    let maintenance_work_descriptor_count =
        backend.state().maintenance_declaration_records.len() as u64;
    let maintenance_admitted_plan_count = backend
        .state()
        .maintenance_execution_records
        .values()
        .filter(|record| {
            matches!(
                record.plan_family,
                Some(crate::MaintenancePlanFamily::BackgroundPaced)
                    | Some(crate::MaintenancePlanFamily::ForegroundReserved)
            )
        })
        .count() as u64;
    let maintenance_deferred_plan_count = backend
        .state()
        .maintenance_execution_records
        .values()
        .filter(|record| {
            matches!(
                record.plan_family,
                Some(crate::MaintenancePlanFamily::Deferred)
            )
        })
        .count() as u64;
    let maintenance_escalated_plan_count = backend
        .state()
        .maintenance_execution_records
        .values()
        .filter(|record| {
            matches!(
                record.plan_family,
                Some(crate::MaintenancePlanFamily::Escalated)
            )
        })
        .count() as u64;
    let maintenance_rejected_plan_count = backend
        .state()
        .maintenance_execution_records
        .values()
        .filter(|record| {
            matches!(
                record.plan_family,
                Some(crate::MaintenancePlanFamily::Cancelled)
            )
        })
        .count() as u64;
    let maintenance_queue_depth = backend
        .state()
        .maintenance_queue_summary_records
        .values()
        .map(|record| record.summary.admitted_count() + record.summary.deferred_count())
        .sum();
    let maintenance_queue_locality_scope_count = backend
        .state()
        .maintenance_queue_summary_records
        .values()
        .map(|record| record.summary.lane_key().locality_scope().clone())
        .collect::<std::collections::BTreeSet<_>>()
        .len() as u64;
    let debt_units_by_family = |family: crate::MaintenanceDebtFamily| -> u64 {
        backend
            .state()
            .maintenance_declaration_records
            .values()
            .filter(|record| record.work_descriptor.debt_family() == Some(family))
            .count() as u64
    };
    let maintenance_tier_work_execute_count = backend
        .state()
        .maintenance_execution_records
        .values()
        .filter(|record| {
            matches!(
                record.execution_status,
                crate::MaintenanceExecutionStatus::Completed
            ) && record.lane_key.as_ref().is_some_and(|lane_key| {
                matches!(
                    lane_key.work_class(),
                    crate::MaintenanceWorkClass::TierPlacementProposal
                        | crate::MaintenanceWorkClass::TierMoveExecution
                )
            })
        })
        .count() as u64;
    crate::Milestone11CounterContract {
        maintenance_work_descriptor_count,
        maintenance_declaration_count: snapshot.maintenance_declaration_count,
        maintenance_admission_count: snapshot.maintenance_admission_count,
        maintenance_rejection_count: snapshot.maintenance_rejection_count,
        maintenance_admitted_plan_count,
        maintenance_deferred_plan_count,
        maintenance_escalated_plan_count,
        maintenance_rejected_plan_count,
        maintenance_resume_count: snapshot.maintenance_resume_count,
        maintenance_restart_readmission_count: snapshot.maintenance_restart_readmission_count,
        maintenance_restart_rejection_count: snapshot.maintenance_restart_rejection_count,
        maintenance_restart_recovered_count: restart_recovered_descriptor_count,
        maintenance_checkpoint_count: snapshot.maintenance_checkpoint_count,
        maintenance_completion_count: snapshot.maintenance_completion_count,
        maintenance_failure_count: snapshot.maintenance_failure_count,
        maintenance_debt_link_count: snapshot.maintenance_debt_link_count,
        maintenance_compaction_debt_units: debt_units_by_family(
            crate::MaintenanceDebtFamily::CompactionDebt,
        ),
        maintenance_rebuild_debt_units: debt_units_by_family(
            crate::MaintenanceDebtFamily::RebuildDebt,
        ),
        maintenance_snapshot_debt_units: debt_units_by_family(
            crate::MaintenanceDebtFamily::SnapshotDebt,
        ),
        maintenance_replication_prep_debt_units: debt_units_by_family(
            crate::MaintenanceDebtFamily::ReplicationPreparationDebt,
        ),
        maintenance_tiering_debt_units: debt_units_by_family(
            crate::MaintenanceDebtFamily::TierPlacementDebt,
        ),
        maintenance_foreground_borrow_count: snapshot.maintenance_foreground_borrow_count,
        maintenance_foreground_wait_count: snapshot.maintenance_foreground_wait_count,
        maintenance_cutover_dependency_count: snapshot.maintenance_cutover_dependency_count,
        maintenance_coalesced_work_count: snapshot.maintenance_coalesced_work_count,
        maintenance_cancelled_superseded_work_count: snapshot
            .maintenance_cancelled_superseded_work_count,
        maintenance_store_global_scope_count: snapshot.maintenance_store_global_scope_count,
        maintenance_starvation_trigger_count: snapshot.maintenance_starvation_trigger_count,
        maintenance_debt_escalation_count: snapshot.maintenance_debt_escalation_count,
        maintenance_io_budget_units_reserved: snapshot.maintenance_io_budget_units_reserved,
        maintenance_cpu_budget_units_reserved: snapshot.maintenance_cpu_budget_units_reserved,
        maintenance_memory_budget_units_reserved: snapshot.maintenance_memory_budget_units_reserved,
        maintenance_publication_slot_budget_reserved: snapshot
            .maintenance_publication_slot_budget_reserved,
        maintenance_queue_depth,
        maintenance_queue_locality_scope_count,
        maintenance_quantum_grant_count: snapshot.maintenance_quantum_grant_count,
        maintenance_quantum_exhaustion_count: snapshot.maintenance_quantum_exhaustion_count,
        maintenance_background_unit_execute_count: snapshot
            .maintenance_background_unit_execute_count,
        maintenance_tier_work_execute_count,
        maintenance_foreground_interference_count: snapshot
            .maintenance_foreground_interference_count,
        maintenance_foreground_wait_on_cutover_count: snapshot
            .maintenance_foreground_wait_on_cutover_count,
        maintenance_foreground_broadened_count: snapshot.maintenance_foreground_broadened_count,
        maintenance_reservation_violation_count: snapshot.maintenance_reservation_violation_count,
        maintenance_cross_locality_escalation_count: snapshot
            .maintenance_cross_locality_escalation_count,
        maintenance_freshness_rejection_count: snapshot.maintenance_freshness_rejection_count,
        maintenance_locality_touch_count: snapshot.maintenance_locality_touch_count,
        maintenance_global_scope_fallback_count: snapshot.maintenance_global_scope_fallback_count,
        maintenance_cold_start_boot_count: snapshot.maintenance_cold_start_boot_count,
        maintenance_cold_start_summary_load_count: snapshot
            .maintenance_cold_start_summary_load_count,
        maintenance_cold_start_legacy_backfill_count: snapshot
            .maintenance_cold_start_legacy_backfill_count,
        maintenance_cold_start_recovery_backlog_count: snapshot
            .maintenance_cold_start_recovery_backlog_count,
        maintenance_cold_start_integrity_reject_count: snapshot
            .maintenance_cold_start_integrity_reject_count,
        maintenance_cold_start_global_scan_count: snapshot.maintenance_cold_start_global_scan_count,
        maintenance_plan_execute_without_descriptor_count: snapshot
            .maintenance_plan_execute_without_descriptor_count,
        maintenance_illegal_escalation_count: snapshot.maintenance_illegal_escalation_count,
        maintenance_truth_visibility_violation_count: snapshot
            .maintenance_truth_visibility_violation_count,
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
                *counts
                    .entry(record.work_descriptor.work_class())
                    .or_default() += 1;
                counts
            },
        )
        .into_iter()
        .map(
            |(work_class, declaration_count)| crate::Milestone11WorkClassCount {
                work_class,
                declaration_count,
            },
        )
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
        .map(|(reservation_family, declaration_count)| {
            let (reserved_count, deferred_count) = backend
                .state()
                .maintenance_reservation_summary_records
                .values()
                .filter(|record| record.summary.reservation_family() == reservation_family)
                .fold((0, 0), |(reserved, deferred), record| {
                    (
                        reserved + record.summary.reserved_count(),
                        deferred + record.summary.deferred_count(),
                    )
                });
            crate::Milestone11ReservationFamilyCount {
                reservation_family,
                declaration_count,
                reserved_count,
                deferred_count,
            }
        })
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
        .map(|(locality_scope, declaration_count)| {
            let (deferred_count, active_count) = backend
                .state()
                .maintenance_locality_summary_records
                .values()
                .filter(|record| record.summary.locality_scope() == &locality_scope)
                .fold((0, 0), |(deferred, active), record| {
                    (
                        deferred + record.summary.deferred_count(),
                        active + record.summary.active_count(),
                    )
                });
            crate::Milestone11LocalityScopeCount {
                locality_scope,
                declaration_count,
                deferred_count,
                active_count,
            }
        })
        .collect::<Vec<_>>();
    let scheduler_topology = crate::Milestone11SchedulerTopologyReport {
        queue_family_count: work_class_counts.len() as u64,
        locality_bucket_count: locality_scope_counts.len() as u64,
        has_restart_recovered_intake_lane: true,
        has_foreground_reservation_pool: true,
        has_background_reservation_pool: true,
    };
    let recovered_declaration_count = backend
        .state()
        .maintenance_declaration_records
        .values()
        .filter(|record| record.work_descriptor.recovered_from_restart())
        .count() as u64;
    let readmitted_recovered_declaration_count = backend
        .state()
        .maintenance_execution_records
        .values()
        .filter(|record| {
            matches!(
                record.restart_readmission_status,
                Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork)
            )
        })
        .count() as u64;
    let rejected_recovered_declaration_count = backend
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
        .count() as u64;
    let recovered_intake = recovered_maintenance_intake_report(backend.state());
    let snapshot = backend.counters().snapshot();
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
                matches!(
                    record.plan_family,
                    Some(crate::MaintenancePlanFamily::Escalated)
                )
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
        readmitted_recovered_declaration_count,
        rejected_recovered_declaration_count,
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
        recovered_declaration_count,
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
        coalesced_work_count: backend
            .state()
            .maintenance_queue_summary_records
            .values()
            .map(|record| record.summary.coalesced_count())
            .sum(),
        cancelled_superseded_work_count: backend
            .state()
            .maintenance_queue_summary_records
            .values()
            .map(|record| record.summary.cancelled_superseded_count())
            .sum(),
        store_global_scope_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| record.explicit_global_scope_debt)
            .count() as u64,
        starved_lane_count: backend
            .state()
            .maintenance_debt_summary_records
            .values()
            .filter(|record| {
                matches!(
                    record.summary.starvation_status(),
                    crate::MaintenanceStarvationStatus::DeferredLanePressure
                )
            })
            .count() as u64,
        debt_bearing_lane_count: backend
            .state()
            .maintenance_debt_summary_records
            .values()
            .filter(|record| {
                !matches!(
                    record.summary.pressure_class(),
                    crate::MaintenanceDebtPressureClass::None
                )
            })
            .count() as u64,
        foreground_interference_count: snapshot.maintenance_foreground_interference_count,
        foreground_broadened_count: snapshot.maintenance_foreground_broadened_count,
        reservation_violation_count: snapshot.maintenance_reservation_violation_count,
        recovered_intake,
        cold_start_boot: crate::MaintenanceColdStartBootReport::new(
            backend
                .state()
                .maintenance_loaded_persisted_summaries_on_boot,
            backend
                .state()
                .maintenance_used_legacy_summary_backfill_on_boot,
            backend.state().maintenance_recovered_backlog_on_boot,
            backend.state().maintenance_boot_integrity_reject_count,
        ),
        scheduler_topology,
        work_class_counts,
        reservation_family_counts,
        locality_scope_counts,
    }
}

fn recovered_maintenance_intake_report(
    state: &StoreState,
) -> crate::RecoveredMaintenanceIntakeReport {
    let mut lane_rollup = std::collections::BTreeMap::<
        crate::MaintenanceLaneKey,
        (u64, u64, u64, u64, u64, bool),
    >::new();
    for declaration in state.maintenance_declaration_records.values() {
        if !declaration.work_descriptor.recovered_from_restart() {
            continue;
        }
        let lane_key = declaration.work_descriptor.lane_key();
        let execution = state
            .maintenance_execution_records
            .get(declaration.artifact_id.as_str());
        let entry = lane_rollup
            .entry(lane_key.clone())
            .or_insert((0, 0, 0, 0, 0, false));
        match execution.and_then(|record| record.restart_readmission_status) {
            None | Some(crate::MaintenanceReadmissionStatus::PendingRecoveredReadmission) => {
                entry.0 += 1;
            }
            Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork) => {
                entry.1 += 1;
            }
            Some(crate::MaintenanceReadmissionStatus::RejectedStaleRecoveredWork) => {
                entry.2 += 1;
                entry.3 += 1;
            }
            Some(crate::MaintenanceReadmissionStatus::RejectedSupersededRecoveredWork) => {
                entry.2 += 1;
            }
        }
        if matches!(
            execution.and_then(|record| record.coalescing_decision),
            Some(crate::MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember)
        ) {
            entry.4 += 1;
        }
        entry.5 = state
            .maintenance_debt_summary_records
            .get(&lane_key.artifact_id())
            .map(|record| {
                !matches!(
                    record.summary.pressure_class(),
                    crate::MaintenanceDebtPressureClass::None
                )
            })
            .unwrap_or(false);
    }

    let lane_intake = lane_rollup
        .into_iter()
        .map(
            |(lane_key, (pending, readmitted, rejected, stale, coalesced, debt_bearing))| {
                crate::RecoveredMaintenanceLaneIntake::new(
                    lane_key,
                    pending,
                    readmitted,
                    rejected,
                    stale,
                    coalesced,
                    debt_bearing,
                )
            },
        )
        .collect::<Vec<_>>();

    crate::RecoveredMaintenanceIntakeReport::new(
        lane_intake
            .iter()
            .map(|lane| lane.pending_recovered_count())
            .sum(),
        lane_intake
            .iter()
            .map(|lane| lane.readmitted_recovered_count())
            .sum(),
        lane_intake
            .iter()
            .map(|lane| lane.rejected_recovered_count())
            .sum(),
        lane_intake
            .iter()
            .map(|lane| lane.stale_recovered_count())
            .sum(),
        lane_intake
            .iter()
            .map(|lane| lane.coalesced_recovered_count())
            .sum(),
        lane_intake,
    )
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
    } else if !state.maintenance_queue_summary_records.is_empty() {
        crate::Milestone11ComplexityPathStatus::verified(
            "batch admission maintains durable scheduler lane summaries before planning begins",
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
