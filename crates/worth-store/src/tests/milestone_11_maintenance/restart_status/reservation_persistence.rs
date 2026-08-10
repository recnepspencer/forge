use super::super::*;

#[test]
fn reserved_transition_survives_restart_in_both_durable_lanes() {
    let local_path = unique_test_store_path("worth-store-m11-maintenance-reserved-restart-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = local_receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Compaction { .. }
            )
        })
        .expect("local compaction declaration")
        .declaration()
        .id()
        .clone();
    drop(local_store);
    force_local_file_reserved(
        &local_path,
        &local_compaction,
        crate::MaintenancePlanFamily::BackgroundPaced,
        3,
    );

    let reopened_local = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let local_status = reopened_local
        .maintenance_status(&local_compaction)
        .unwrap();
    assert_eq!(
        local_status.execution_status(),
        MaintenanceExecutionStatus::Reserved
    );
    assert_eq!(
        local_status
            .reservation_transition()
            .expect("reserved transition should persist")
            .quantum_units(),
        3
    );

    let sqlite_path =
        unique_test_sqlite_path("worth-store-m11-maintenance-reserved-restart-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = sqlite_receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Compaction { .. }
            )
        })
        .expect("sqlite compaction declaration")
        .declaration()
        .id()
        .clone();
    drop(sqlite_store);
    force_sqlite_reserved(
        &sqlite_path,
        &sqlite_compaction,
        crate::MaintenancePlanFamily::BackgroundPaced,
        3,
    );

    let reopened_sqlite = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let sqlite_status = reopened_sqlite
        .maintenance_status(&sqlite_compaction)
        .unwrap();
    assert_eq!(
        sqlite_status.execution_status(),
        MaintenanceExecutionStatus::Reserved
    );
    assert_eq!(
        sqlite_status
            .reservation_transition()
            .expect("reserved transition should persist")
            .quantum_units(),
        3
    );
}
