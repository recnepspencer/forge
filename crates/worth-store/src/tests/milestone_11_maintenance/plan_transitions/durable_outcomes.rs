use super::super::*;
use super::compaction_receipt::admitted_compaction;

#[test]
fn deferred_plan_outcome_is_persisted_and_operator_visible_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-deferred-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_deferred(&local_path, local_compaction.declaration().id());

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let deferred = reopened_local
        .start_maintenance_declaration(&local_compaction)
        .unwrap_err();
    assert_eq!(
        deferred.failure_kind(),
        crate::MaintenanceFailureKind::Deferred
    );
    let local_status = reopened_local
        .maintenance_status(local_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        local_status.execution_status(),
        MaintenanceExecutionStatus::Deferred
    );
    assert_eq!(
        local_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Deferred)
    );
    assert_eq!(
        local_status.pending_reason(),
        Some("maintenance work was deferred pending an operator-visible signal")
    );
    let local_report = reopened_local.milestone_11_maintenance_report();
    assert_eq!(local_report.deferred_declaration_count, 1);
    assert_eq!(local_report.cancelled_declaration_count, 0);

    let sqlite_path = unique_test_sqlite_path("worth-store-m11-maintenance-deferred-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_deferred(&sqlite_path, sqlite_compaction.declaration().id());

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let deferred = reopened_sqlite
        .start_maintenance_declaration(&sqlite_compaction)
        .unwrap_err();
    assert_eq!(
        deferred.failure_kind(),
        crate::MaintenanceFailureKind::Deferred
    );
    let sqlite_status = reopened_sqlite
        .maintenance_status(sqlite_compaction.declaration().id())
        .unwrap();
    assert_eq!(
        sqlite_status.execution_status(),
        MaintenanceExecutionStatus::Deferred
    );
    assert_eq!(
        sqlite_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Deferred)
    );
    assert_eq!(
        sqlite_status.pending_reason(),
        Some("maintenance work was deferred pending an operator-visible signal")
    );
    let sqlite_report = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(sqlite_report.deferred_declaration_count, 1);
    assert_eq!(sqlite_report.cancelled_declaration_count, 0);
}

#[test]
fn cancelled_plan_outcome_is_persisted_before_execution_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-cancelled-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
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
        local_status.execution_status(),
        MaintenanceExecutionStatus::Cancelled
    );
    assert_eq!(
        local_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Cancelled)
    );
    assert_eq!(
        local_status.pending_reason(),
        Some("maintenance descriptor is stale and must be cancelled before execution")
    );
    let local_report = reopened_local.milestone_11_maintenance_report();
    assert_eq!(local_report.cancelled_declaration_count, 1);
    assert_eq!(local_report.deferred_declaration_count, 0);
    assert_eq!(
        reopened_local
            .milestone_11_counter_contract()
            .maintenance_freshness_rejection_count,
        1
    );
    assert_eq!(
        reopened_local
            .milestone_11_counter_contract()
            .maintenance_rejected_plan_count,
        1
    );

    let sqlite_path = unique_test_sqlite_path("worth-store-m11-maintenance-cancelled-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
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
        sqlite_status.execution_status(),
        MaintenanceExecutionStatus::Cancelled
    );
    assert_eq!(
        sqlite_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Cancelled)
    );
    assert_eq!(
        sqlite_status.pending_reason(),
        Some("maintenance descriptor is stale and must be cancelled before execution")
    );
    let sqlite_report = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(sqlite_report.cancelled_declaration_count, 1);
    assert_eq!(sqlite_report.deferred_declaration_count, 0);
    assert_eq!(
        reopened_sqlite
            .milestone_11_counter_contract()
            .maintenance_freshness_rejection_count,
        1
    );
    assert_eq!(
        reopened_sqlite
            .milestone_11_counter_contract()
            .maintenance_rejected_plan_count,
        1
    );
}

#[test]
fn escalated_plan_outcome_is_visible_after_execution_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-escalated-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = admitted_compaction(&local_receipt);
    drop(local_store);
    force_local_file_escalated(&local_path, local_compaction.declaration().id());

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
        local_status.execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    assert_eq!(
        local_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Escalated)
    );
    assert!(local_status
        .foreground_impact()
        .borrowed_foreground_reservation());
    assert!(local_status.foreground_impact().foreground_wait_required());
    assert!(local_status
        .foreground_impact()
        .cutover_dependency_required());
    let local_report = reopened_local.milestone_11_maintenance_report();
    assert_eq!(local_report.escalated_declaration_count, 1);
    assert_eq!(local_report.foreground_borrowed_declaration_count, 1);
    assert_eq!(local_report.foreground_waited_declaration_count, 1);
    assert_eq!(local_report.cutover_dependency_declaration_count, 1);

    let sqlite_path = unique_test_sqlite_path("worth-store-m11-maintenance-escalated-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = admitted_compaction(&sqlite_receipt);
    drop(sqlite_store);
    force_sqlite_escalated(&sqlite_path, sqlite_compaction.declaration().id());

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
        sqlite_status.execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    assert_eq!(
        sqlite_status.plan_family(),
        Some(crate::MaintenancePlanFamily::Escalated)
    );
    assert!(sqlite_status
        .foreground_impact()
        .borrowed_foreground_reservation());
    assert!(sqlite_status.foreground_impact().foreground_wait_required());
    assert!(sqlite_status
        .foreground_impact()
        .cutover_dependency_required());
    let sqlite_report = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(sqlite_report.escalated_declaration_count, 1);
    assert_eq!(sqlite_report.foreground_borrowed_declaration_count, 1);
    assert_eq!(sqlite_report.foreground_waited_declaration_count, 1);
    assert_eq!(sqlite_report.cutover_dependency_declaration_count, 1);
}
