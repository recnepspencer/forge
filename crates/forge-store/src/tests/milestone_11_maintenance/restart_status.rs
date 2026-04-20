use super::*;

#[test]
fn maintenance_status_survives_restart() {
    let path = unique_test_store_path("forge-store-m11-maintenance-restart");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let initial = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(initial).unwrap();
    let head = update_entity_on_branch_with_commit(&mut runtime, entity_id, "main-v2");
    store.append_canonical_commit(head.clone()).unwrap();
    store
        .materialize_milestone_6_layout_support(layout_request(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        vec![PinnedSnapshotPolicy::new(snapshot.snapshot_id)],
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );
    let batch = store
        .plan_retention_maintenance_batch(RetentionPolicyClass::Conservative(policy))
        .unwrap();
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let root_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);
    force_local_file_recovered(&path, &root_id);

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let status = reopened.maintenance_status(&root_id).unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Admitted
    );
    assert!(status.recovered_from_restart());
    assert_eq!(
        status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::PendingRecoveredReadmission)
    );
    assert_eq!(
        reopened
            .milestone_11_maintenance_report()
            .declared_batch_count,
        1
    );
    assert_eq!(
        reopened
            .milestone_11_maintenance_report()
            .recovered_declaration_count,
        1
    );
}

#[test]
fn maintenance_status_survives_sqlite_restart() {
    let path = unique_test_sqlite_path("forge-store-m11-maintenance-sqlite-restart");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let root_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);
    force_sqlite_recovered(&path, &root_id);

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let status = reopened.maintenance_status(&root_id).unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Admitted
    );
    assert!(status.recovered_from_restart());
    assert_eq!(
        status.restart_readmission_status(),
        Some(crate::MaintenanceReadmissionStatus::PendingRecoveredReadmission)
    );
    let report = reopened.milestone_11_maintenance_report();
    assert_eq!(report.declared_batch_count, 1);
    assert_eq!(
        report.persisted_declaration_count,
        receipt.admitted_declarations().len() as u64
    );
    assert_eq!(report.recovered_declaration_count, 1);
}

#[test]
fn reserved_transition_survives_restart_in_both_durable_lanes() {
    let local_path = unique_test_store_path("forge-store-m11-maintenance-reserved-restart-local");
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
    force_local_file_reserved(
        &local_path,
        &local_compaction,
        crate::MaintenancePlanFamily::BackgroundPaced,
        3,
    );

    let reopened_local = ForgeStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let local_status = reopened_local.maintenance_status(&local_compaction).unwrap();
    assert_eq!(local_status.execution_status(), MaintenanceExecutionStatus::Reserved);
    assert_eq!(
        local_status
            .reservation_transition()
            .expect("reserved transition should persist")
            .quantum_units(),
        3
    );

    let sqlite_path =
        unique_test_sqlite_path("forge-store-m11-maintenance-reserved-restart-sqlite");
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
    force_sqlite_reserved(
        &sqlite_path,
        &sqlite_compaction,
        crate::MaintenancePlanFamily::BackgroundPaced,
        3,
    );

    let reopened_sqlite = ForgeStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let sqlite_status = reopened_sqlite.maintenance_status(&sqlite_compaction).unwrap();
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

