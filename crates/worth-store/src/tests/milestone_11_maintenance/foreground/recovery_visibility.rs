use super::super::*;

#[test]
fn recovered_intake_and_cold_start_boot_are_visible_before_execution() {
    let path = unique_test_store_path("worth-store-m11-recovered-intake");
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

    let reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let report = reopened.milestone_11_maintenance_report();
    assert!(report.cold_start_boot.used_legacy_summary_backfill());
    assert_eq!(report.cold_start_boot.recovered_backlog_count(), 1);
    assert_eq!(report.recovered_intake.pending_recovered_count(), 1);
    assert_eq!(report.recovered_intake.lane_intake().len(), 1);
    assert_eq!(
        report.recovered_intake.lane_intake()[0].pending_recovered_count(),
        1
    );
    let counters = reopened.milestone_11_counter_contract();
    assert_eq!(counters.maintenance_cold_start_boot_count, 1);
    assert_eq!(counters.maintenance_cold_start_summary_load_count, 0);
    assert_eq!(counters.maintenance_cold_start_legacy_backfill_count, 1);
    assert_eq!(counters.maintenance_cold_start_recovery_backlog_count, 1);
    assert_eq!(counters.maintenance_restart_recovered_count, 1);
    assert_eq!(counters.maintenance_cold_start_global_scan_count, 0);
}

#[test]
fn maintenance_recovery_report_summarizes_scheduler_backlog_counts() {
    let path = unique_test_store_path("worth-store-m11-maintenance-recovery-summary");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let active_declaration_count = receipt.admitted_declarations().len() as u64;
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);

    force_local_file_recovered(&path, &declaration_id);
    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let report = reopened.maintenance_recovery_report().unwrap();
    assert_eq!(report.active_declaration_count(), active_declaration_count);
    assert_eq!(report.escalated_declaration_count(), 1);
    assert_eq!(report.recovered_backlog_count(), 1);
}
