use super::super::*;
use super::compaction_receipt::admitted_compaction;

#[test]
fn recovered_stale_work_is_cancelled_during_readmission_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-readmission-stale-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_recovered(&local_path, local_compaction.declaration().id());
    force_local_file_cancelled(&local_path, local_compaction.declaration().id());

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let cancelled = reopened_local
        .start_maintenance_declaration(&local_compaction)
        .unwrap_err();
    assert_eq!(
        cancelled.failure_kind(),
        crate::MaintenanceFailureKind::Cancelled
    );
    let local_status = reopened_local
        .maintenance_status(local_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        local_status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::RejectedStaleRecoveredWork)
    );
    assert_eq!(
        local_status.execution_status(),
        MaintenanceExecutionStatus::Cancelled
    );
    let local_report = reopened_local.milestone_11_maintenance_report();
    assert_eq!(local_report.rejected_recovered_declaration_count, 1);

    let sqlite_path =
        unique_test_sqlite_path("worth-store-m11-maintenance-readmission-stale-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_recovered(&sqlite_path, sqlite_compaction.declaration().id());
    force_sqlite_cancelled(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let cancelled = reopened_sqlite
        .start_maintenance_declaration(&sqlite_compaction)
        .unwrap_err();
    assert_eq!(
        cancelled.failure_kind(),
        crate::MaintenanceFailureKind::Cancelled
    );
    let sqlite_status = reopened_sqlite
        .maintenance_status(sqlite_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        sqlite_status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::RejectedStaleRecoveredWork)
    );
    assert_eq!(
        sqlite_status.execution_status(),
        MaintenanceExecutionStatus::Cancelled
    );
    let sqlite_report = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(sqlite_report.rejected_recovered_declaration_count, 1);
}

#[test]
fn recovered_fresh_work_is_readmitted_before_execution_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-readmission-fresh-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_recovered(&local_path, local_compaction.declaration().id());

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let completed = reopened_local
        .start_maintenance_declaration(&local_compaction)
        .unwrap();
    assert_eq!(completed.last_completed_phase(), "compaction_cutover");
    let local_status = reopened_local
        .maintenance_status(local_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        local_status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork)
    );
    let local_report = reopened_local.milestone_11_maintenance_report();
    assert_eq!(local_report.readmitted_recovered_declaration_count, 1);
    assert_eq!(
        reopened_local
            .milestone_11_counter_contract()
            .maintenance_restart_readmission_count,
        1
    );

    let sqlite_path =
        unique_test_sqlite_path("worth-store-m11-maintenance-readmission-fresh-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_recovered(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let completed = reopened_sqlite
        .start_maintenance_declaration(&sqlite_compaction)
        .unwrap();
    assert_eq!(completed.last_completed_phase(), "compaction_cutover");
    let sqlite_status = reopened_sqlite
        .maintenance_status(sqlite_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        sqlite_status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork)
    );
    let sqlite_report = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(sqlite_report.readmitted_recovered_declaration_count, 1);
    assert_eq!(
        reopened_sqlite
            .milestone_11_counter_contract()
            .maintenance_restart_readmission_count,
        1
    );
}
