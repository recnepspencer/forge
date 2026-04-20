use super::*;

#[test]
fn bulk_frozen_manifest_and_checkpoint_fetch_survive_sqlite_reopen() {
    let path = unique_test_sqlite_path("forge-store-bulk-fetch");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-fetch-authority");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-fetch",
            "source-fetch",
            envelope.branch_context.clone(),
            vec![
                BulkSourceMember::new("a", 1),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let witness = store
        .publish_bulk_chunk_witness(&admitted, envelope.commit.commit_id)
        .unwrap();
    store.publish_bulk_progress_checkpoint(&witness).unwrap();
    drop(store);

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let fetched_manifest = reopened
        .fetch_frozen_bulk_manifest("program-fetch", manifest.manifest_digest())
        .unwrap();
    let fetched_checkpoint = reopened
        .fetch_bulk_progress_checkpoint("program-fetch", plan.plan_id())
        .unwrap();

    assert_eq!(
        fetched_manifest.manifest_digest(),
        manifest.manifest_digest()
    );
    assert_eq!(fetched_checkpoint.checkpoint_sequence(), 1);
}

#[test]
fn bulk_witness_with_missing_canonical_commit_fails_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-witness-missing-commit");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-witness-corruption");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-witness-integrity",
            "source-witness-integrity",
            envelope.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(1))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, envelope.clone())
        .unwrap();
    store
        .execute_bulk_canonical_chunk(request, BulkCheckpointPolicy::Publish)
        .unwrap();
    drop(store);

    force_bulk_witness_missing_commit(&path, "program-witness-integrity", plan.plan_id(), 0);

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("reopen should fail when a bulk witness references a missing canonical commit");
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_checkpoint_gap_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-checkpoint-gap");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-checkpoint-gap",
        "source-checkpoint-gap",
    );
    drop(store);

    force_bulk_checkpoint_gap(&path, "program-checkpoint-gap", plan.plan_id(), 1);

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("reopen should fail when a bulk checkpoint family has a sequence gap");
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_checkpoint_completed_chunk_regression_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-checkpoint-regression");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-checkpoint-regression",
        "source-checkpoint-regression",
    );
    drop(store);

    force_bulk_checkpoint_completed_chunk_regression(
        &path,
        "program-checkpoint-regression",
        plan.plan_id(),
        2,
        0,
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err(
            "reopen should fail when a bulk checkpoint regresses the completed chunk ordinal",
        );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_checkpoint_index_reference_missing_checkpoint_fails_sqlite_reopen_closed() {
    let path = unique_test_sqlite_path("forge-store-bulk-checkpoint-index-gap");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-checkpoint-index-gap",
        "source-checkpoint-index-gap",
    );
    drop(store);

    delete_sqlite_bulk_checkpoint(&path, "program-checkpoint-index-gap", plan.plan_id(), 2);

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("reopen should fail when witness index points at a missing bulk checkpoint");
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_checkpoint_completed_chunk_regression_fails_sqlite_reopen_closed() {
    let path = unique_test_sqlite_path("forge-store-bulk-checkpoint-regression-sqlite");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-checkpoint-regression-sqlite",
        "source-checkpoint-regression-sqlite",
    );
    drop(store);

    regress_sqlite_bulk_checkpoint_completed_chunk(
        &path,
        "program-checkpoint-regression-sqlite",
        plan.plan_id(),
        2,
        0,
    );

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err(
        "reopen should fail when a sqlite bulk checkpoint regresses the completed chunk ordinal",
    );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

