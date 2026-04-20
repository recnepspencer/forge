use super::*;

#[test]
fn maintenance_status_survives_restart() {
    let path = unique_test_store_path("forge-store-m10-5-maintenance-restart");
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

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let status = reopened.maintenance_status(&root_id).unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Admitted
    );
    assert_eq!(
        reopened
            .milestone_10_5_maintenance_report()
            .declared_batch_count,
        1
    );
}

#[test]
fn maintenance_status_survives_sqlite_restart() {
    let path = unique_test_sqlite_path("forge-store-m10-5-maintenance-sqlite-restart");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let root_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let status = reopened.maintenance_status(&root_id).unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Admitted
    );
    let report = reopened.milestone_10_5_maintenance_report();
    assert_eq!(report.declared_batch_count, 1);
    assert_eq!(
        report.persisted_declaration_count,
        receipt.admitted_declarations().len() as u64
    );
}

