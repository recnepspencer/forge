use super::super::*;
use super::compaction_receipt::equivalent_compaction_pair;

#[test]
fn cold_start_and_warm_start_recovered_backlog_have_equivalent_scheduler_shape() {
    let path = unique_test_store_path("worth-store-m11-maintenance-cold-warm-equivalence");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);
    force_local_file_recovered(&path, &declaration_id);

    let warm_reopen = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let warm_report = warm_reopen.milestone_11_maintenance_report();
    let warm_export = warm_reopen.export_authoritative_records();
    let warm_shape_digest = stable_digest(&(
        &warm_report.work_class_counts,
        &warm_report.reservation_family_counts,
        &warm_report.locality_scope_counts,
        warm_report.recovered_declaration_count,
        warm_report.recovered_intake.pending_recovered_count(),
    ));
    drop(warm_reopen);

    let cold_reopen = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let cold_report = cold_reopen.milestone_11_maintenance_report();
    let cold_export = cold_reopen.export_authoritative_records();
    let cold_shape_digest = stable_digest(&(
        &cold_report.work_class_counts,
        &cold_report.reservation_family_counts,
        &cold_report.locality_scope_counts,
        cold_report.recovered_declaration_count,
        cold_report.recovered_intake.pending_recovered_count(),
    ));

    assert_eq!(warm_shape_digest, cold_shape_digest);
    assert_eq!(warm_export.canonical_json(), cold_export.canonical_json());
    assert_eq!(
        cold_reopen
            .milestone_11_counter_contract()
            .maintenance_cold_start_global_scan_count,
        0
    );
}

#[test]
fn queue_summary_counts_survive_restart_in_both_durable_lanes() {
    let duplicate_id = "maintenance-compaction-restart-coalesced";

    let local_path = unique_test_store_path("worth-store-m11-maintenance-queue-summary-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_duplicate_batch = duplicate_compaction_batch(&local_batch, duplicate_id);
    let local_receipt = local_store
        .admit_maintenance_batch(local_duplicate_batch)
        .unwrap();
    let (_leader, local_duplicate) = equivalent_compaction_pair(&local_receipt, duplicate_id);
    let _ = local_store
        .start_maintenance_declaration(&local_duplicate)
        .unwrap_err();
    let local_report_before = local_store.milestone_11_maintenance_report();
    drop(local_store);

    let reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let local_report_after = reopened_local.milestone_11_maintenance_report();
    assert_eq!(
        local_report_after.coalesced_work_count,
        local_report_before.coalesced_work_count
    );
    assert_eq!(
        local_report_after.cancelled_superseded_work_count,
        local_report_before.cancelled_superseded_work_count
    );
    assert_eq!(
        reopened_local
            .maintenance_status(local_duplicate.declaration().id())
            .unwrap()
            .coalescing_decision(),
        Some(crate::MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember)
    );

    let sqlite_path = unique_test_sqlite_path("worth-store-m11-maintenance-queue-summary-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_duplicate_batch = duplicate_compaction_batch(&sqlite_batch, duplicate_id);
    let sqlite_receipt = sqlite_store
        .admit_maintenance_batch(sqlite_duplicate_batch)
        .unwrap();
    let (_leader, sqlite_duplicate) = equivalent_compaction_pair(&sqlite_receipt, duplicate_id);
    let _ = sqlite_store
        .start_maintenance_declaration(&sqlite_duplicate)
        .unwrap_err();
    let sqlite_report_before = sqlite_store.milestone_11_maintenance_report();
    drop(sqlite_store);

    let reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let sqlite_report_after = reopened_sqlite.milestone_11_maintenance_report();
    assert_eq!(
        sqlite_report_after.coalesced_work_count,
        sqlite_report_before.coalesced_work_count
    );
    assert_eq!(
        sqlite_report_after.cancelled_superseded_work_count,
        sqlite_report_before.cancelled_superseded_work_count
    );
    assert_eq!(
        reopened_sqlite
            .maintenance_status(sqlite_duplicate.declaration().id())
            .unwrap()
            .coalescing_decision(),
        Some(crate::MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember)
    );
}

#[test]
fn recovered_work_uses_same_lane_summary_path_as_fresh_work() {
    let duplicate_id = "maintenance-compaction-recovered-coalesced";

    let local_path = unique_test_store_path("worth-store-m11-maintenance-recovered-lane-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_duplicate_batch = duplicate_compaction_batch(&local_batch, duplicate_id);
    let local_receipt = local_store
        .admit_maintenance_batch(local_duplicate_batch)
        .unwrap();
    let (_leader, local_duplicate) = equivalent_compaction_pair(&local_receipt, duplicate_id);
    drop(local_store);
    force_local_file_recovered(&local_path, local_duplicate.declaration().id());

    let mut reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let cancelled = reopened_local
        .start_maintenance_declaration(&local_duplicate)
        .unwrap_err();
    assert_eq!(
        cancelled.failure_kind(),
        crate::MaintenanceFailureKind::Cancelled
    );
    let local_status = reopened_local
        .maintenance_status(local_duplicate.declaration().id())
        .unwrap();
    assert_eq!(
        local_status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork)
    );
    assert_eq!(
        local_status.coalescing_decision(),
        Some(crate::MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember)
    );

    let sqlite_path = unique_test_sqlite_path("worth-store-m11-maintenance-recovered-lane-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_duplicate_batch = duplicate_compaction_batch(&sqlite_batch, duplicate_id);
    let sqlite_receipt = sqlite_store
        .admit_maintenance_batch(sqlite_duplicate_batch)
        .unwrap();
    let (_leader, sqlite_duplicate) = equivalent_compaction_pair(&sqlite_receipt, duplicate_id);
    drop(sqlite_store);
    force_sqlite_recovered(&sqlite_path, sqlite_duplicate.declaration().id());

    let mut reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let cancelled = reopened_sqlite
        .start_maintenance_declaration(&sqlite_duplicate)
        .unwrap_err();
    assert_eq!(
        cancelled.failure_kind(),
        crate::MaintenanceFailureKind::Cancelled
    );
    let sqlite_status = reopened_sqlite
        .maintenance_status(sqlite_duplicate.declaration().id())
        .unwrap();
    assert_eq!(
        sqlite_status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork)
    );
    assert_eq!(
        sqlite_status.coalescing_decision(),
        Some(crate::MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember)
    );
}
