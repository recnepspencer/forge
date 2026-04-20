use super::*;

#[test]
fn started_maintenance_can_resume_after_restart_in_both_durable_lanes() {
    let local_path = unique_test_store_path("forge-store-m10-5-maintenance-resume-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(local_path.clone()),
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
    force_local_file_started(&local_path, &local_compaction);

    let mut reopened_local = ForgeStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let completed_local = reopened_local
        .resume_maintenance_declaration(&local_compaction)
        .unwrap();
    assert_eq!(completed_local.last_completed_phase(), "compaction_cutover");
    assert_eq!(
        reopened_local
            .maintenance_status(&local_compaction)
            .unwrap()
            .execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    assert_eq!(
        reopened_local
            .milestone_10_5_counter_contract()
            .maintenance_resume_count,
        1
    );

    let sqlite_path = unique_test_sqlite_path("forge-store-m10-5-maintenance-resume-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(sqlite_path.clone()),
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
    force_sqlite_started(&sqlite_path, &sqlite_compaction);

    let mut reopened_sqlite = ForgeStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let completed_sqlite = reopened_sqlite
        .resume_maintenance_declaration(&sqlite_compaction)
        .unwrap();
    assert_eq!(
        completed_sqlite.last_completed_phase(),
        "compaction_cutover"
    );
    assert_eq!(
        reopened_sqlite
            .maintenance_status(&sqlite_compaction)
            .unwrap()
            .execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    assert_eq!(
        reopened_sqlite
            .milestone_10_5_counter_contract()
            .maintenance_resume_count,
        1
    );
}
