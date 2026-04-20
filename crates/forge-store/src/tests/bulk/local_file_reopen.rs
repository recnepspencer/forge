use super::*;

#[test]
fn bulk_witness_index_highest_ordinal_regression_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-witness-index-regression");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-witness-index-regression",
        "source-witness-index-regression",
    );
    drop(store);

    force_bulk_witness_index_highest_ordinal_regression(
        &path,
        "program-witness-index-regression",
        plan.plan_id(),
        0,
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err(
            "reopen should fail when witness index regresses below the persisted witness set",
        );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_witness_index_witness_count_drift_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-witness-index-count-drift");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-witness-index-count-drift",
        "source-witness-index-count-drift",
    );
    drop(store);

    force_bulk_witness_index_witness_count_drift(
        &path,
        "program-witness-index-count-drift",
        plan.plan_id(),
        1,
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("reopen should fail when witness index witness count drifts below the persisted witness set");
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_witness_index_highest_ordinal_regression_fails_sqlite_reopen_closed() {
    let path = unique_test_sqlite_path("forge-store-bulk-witness-index-regression-sqlite");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-witness-index-regression-sqlite",
        "source-witness-index-regression-sqlite",
    );
    drop(store);

    regress_sqlite_bulk_witness_index_highest_ordinal(
        &path,
        "program-witness-index-regression-sqlite",
        plan.plan_id(),
        0,
    );

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err(
        "reopen should fail when a sqlite witness index regresses below the persisted witness set",
    );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_witness_index_witness_count_drift_fails_sqlite_reopen_closed() {
    let path = unique_test_sqlite_path("forge-store-bulk-witness-index-count-drift-sqlite");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-witness-index-count-drift-sqlite",
        "source-witness-index-count-drift-sqlite",
    );
    drop(store);

    drift_sqlite_bulk_witness_index_witness_count(
        &path,
        "program-witness-index-count-drift-sqlite",
        plan.plan_id(),
        1,
    );

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("reopen should fail when a sqlite witness index witness count drifts below the persisted witness set");
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn frozen_transform_basis_payload_drift_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-transform-basis-drift");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (basis, _partition, _plan) =
        persist_transform_artifacts(&mut store, "program-transform-drift", "transform-drift");
    drop(store);

    force_frozen_transform_basis_payload_scope_drift(
        &path,
        "program-transform-drift",
        basis.basis_digest(),
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err(
            "reopen should fail when a frozen transform basis payload drifts under a stale digest",
        );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn frozen_transform_partition_payload_drift_fails_sqlite_reopen_closed() {
    let path = unique_test_sqlite_path("forge-store-transform-partition-drift-sqlite");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let (_basis, partition, _plan) = persist_transform_artifacts(
        &mut store,
        "program-transform-partition-drift",
        "transform-partition-drift",
    );
    drop(store);

    drift_sqlite_frozen_transform_partition_payload_member_width(
        &path,
        "program-transform-partition-drift",
        partition.partition_digest(),
    );

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err(
        "reopen should fail when a frozen transform partition payload drifts under a stale digest",
    );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_plan_payload_drift_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-plan-drift");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) =
        persist_two_checkpoint_bulk_family(&mut store, "program-plan-drift", "source-plan-drift");
    drop(store);

    force_bulk_plan_payload_chunk_width_drift(&path, "program-plan-drift", plan.plan_id());

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err(
            "reopen should fail when a persisted bulk plan payload drifts under a stale plan id",
        );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

