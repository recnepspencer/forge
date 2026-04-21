use super::*;

#[test]
fn retention_maintenance_batch_lowers_and_admits_durably() {
    let (mut store, batch) = build_maintenance_ready_store();

    assert!(!batch.declarations().is_empty());
    assert_eq!(batch.batch_class(), crate::MaintenanceBatchClass::Retention);
    let receipt = store.admit_maintenance_batch(batch.clone()).unwrap();
    assert!(!receipt.admitted_declarations().is_empty());
    assert_eq!(
        receipt.batch_summary().declaration_count(),
        receipt.admitted_declarations().len() as u64
    );

    let status = store
        .maintenance_status(receipt.admitted_declarations()[0].declaration().id())
        .unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Admitted
    );
    assert_eq!(
        receipt.admitted_declarations()[0]
            .descriptor()
            .declaration_id()
            .as_str(),
        receipt.admitted_declarations()[0]
            .declaration()
            .id()
            .as_str()
    );
    assert!(matches!(
        receipt.admitted_declarations()[0]
            .descriptor()
            .reservation_family(),
        crate::MaintenanceReservationFamily::Background(_)
    ));
    let report = store.milestone_11_maintenance_report();
    assert_eq!(report.declared_batch_count, 1);
    assert_eq!(
        report.persisted_declaration_count,
        receipt.admitted_declarations().len() as u64
    );
    assert_eq!(report.reserved_declaration_count, 0);
    assert_eq!(report.deferred_declaration_count, 0);
    assert_eq!(report.escalated_declaration_count, 0);
    assert_eq!(report.cancelled_declaration_count, 0);
    assert_eq!(report.readmitted_recovered_declaration_count, 0);
    assert_eq!(report.rejected_recovered_declaration_count, 0);
    assert_eq!(report.recovered_declaration_count, 0);
    assert_eq!(report.foreground_borrowed_declaration_count, 0);
    assert_eq!(report.foreground_waited_declaration_count, 0);
    assert_eq!(report.cutover_dependency_declaration_count, 0);
    assert_eq!(
        report.scheduler_topology.queue_family_count,
        report.work_class_counts.len() as u64
    );
    assert!(report.scheduler_topology.queue_family_count >= 2);
    assert!(report
        .work_class_counts
        .iter()
        .any(|entry| entry.work_class == crate::MaintenanceWorkClass::CompactionMaintenance));
    assert!(report.locality_scope_counts.iter().any(|entry| matches!(
        entry.locality_scope,
        crate::MaintenanceLocalityScope::ArtifactFamilyLocalityScope { .. }
    )));
    let counters = store.milestone_11_counter_contract();
    assert_eq!(
        counters.maintenance_work_descriptor_count,
        receipt.admitted_declarations().len() as u64
    );
    assert_eq!(
        counters.maintenance_admission_count,
        receipt.admitted_declarations().len() as u64
    );
    assert_eq!(
        counters.maintenance_queue_depth,
        receipt.admitted_declarations().len() as u64
    );
    assert_eq!(
        counters.maintenance_queue_locality_scope_count,
        report.locality_scope_counts.len() as u64
    );
    assert_eq!(
        counters.maintenance_locality_touch_count,
        receipt.admitted_declarations().len() as u64
    );
    assert_eq!(counters.explicit_foreground_reservation_count, 0);
    assert_eq!(
        counters.explicit_background_reservation_count,
        receipt.admitted_declarations().len() as u64
    );
}

#[test]
fn admitted_maintenance_declarations_execute_and_persist_status() {
    let (mut store, batch) = build_maintenance_ready_store();
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let compaction = receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Compaction { .. }
            )
        })
        .expect("compaction declaration")
        .clone();

    let completed = store.start_maintenance_declaration(&compaction).unwrap();
    assert_eq!(completed.last_completed_phase(), "compaction_cutover");
    let status = store
        .maintenance_status(compaction.declaration().id())
        .unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    assert_eq!(
        status.work_class(),
        crate::MaintenanceWorkClass::CompactionMaintenance
    );
    assert_eq!(
        status.execution_posture(),
        crate::MaintenanceExecutionPosture::ForegroundAware
    );
    assert!(matches!(
        status.locality_scope(),
        crate::MaintenanceLocalityScope::ArtifactFamilyLocalityScope { .. }
    ));
    let counters = store.milestone_11_counter_contract();
    assert_eq!(counters.maintenance_resume_count, 0);
    assert_eq!(counters.maintenance_restart_readmission_count, 0);
    assert_eq!(counters.maintenance_restart_rejection_count, 0);
    assert_eq!(counters.maintenance_completion_count, 1);
    assert_eq!(counters.maintenance_checkpoint_count, 2);
    assert_eq!(counters.maintenance_admitted_plan_count, 1);
    assert_eq!(counters.maintenance_deferred_plan_count, 0);
    assert_eq!(counters.maintenance_escalated_plan_count, 0);
    assert_eq!(counters.maintenance_rejected_plan_count, 0);
    assert_eq!(counters.maintenance_restart_recovered_count, 0);
    assert_eq!(counters.maintenance_foreground_borrow_count, 0);
    assert_eq!(counters.maintenance_quantum_grant_count, 1);
    assert_eq!(counters.maintenance_background_unit_execute_count, 1);
    assert_eq!(counters.maintenance_tier_work_execute_count, 0);
    assert_eq!(counters.maintenance_quantum_exhaustion_count, 0);
    assert_eq!(counters.maintenance_cross_locality_escalation_count, 0);
    assert_eq!(counters.maintenance_global_scope_fallback_count, 0);
    assert_eq!(
        counters.maintenance_plan_execute_without_descriptor_count,
        0
    );
    assert_eq!(counters.maintenance_illegal_escalation_count, 0);
    assert_eq!(counters.maintenance_truth_visibility_violation_count, 0);
}

#[test]
fn maintenance_status_exposes_persisted_scheduler_descriptor_metadata() {
    let (mut store, batch) = build_maintenance_ready_store();
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let compaction = receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Compaction { .. }
            )
        })
        .expect("compaction declaration");

    let status = store
        .maintenance_status(compaction.declaration().id())
        .unwrap();
    assert_eq!(
        status.work_class(),
        crate::MaintenanceWorkClass::CompactionMaintenance
    );
    assert_eq!(
        status.execution_posture(),
        crate::MaintenanceExecutionPosture::ForegroundAware
    );
    assert_eq!(
        status.debt_family(),
        Some(crate::MaintenanceDebtFamily::CompactionDebt)
    );
    assert_eq!(status.plan_generation().value(), 0);
    assert_eq!(status.supersession_epoch().value(), 0);
    assert_eq!(status.freshness_window().value(), 1);
    assert!(matches!(
        status.reservation_family(),
        crate::MaintenanceReservationFamily::Background(_)
    ));
    assert!(!status.recovered_from_restart());
}
