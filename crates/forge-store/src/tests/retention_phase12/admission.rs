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
    let report = store.milestone_10_5_maintenance_report();
    assert_eq!(report.declared_batch_count, 1);
    assert_eq!(
        report.persisted_declaration_count,
        receipt.admitted_declarations().len() as u64
    );
    let counters = store.milestone_10_5_counter_contract();
    assert_eq!(
        counters.maintenance_admission_count,
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
    let counters = store.milestone_10_5_counter_contract();
    assert_eq!(counters.maintenance_resume_count, 0);
    assert_eq!(counters.maintenance_completion_count, 1);
    assert_eq!(counters.maintenance_checkpoint_count, 2);
}

