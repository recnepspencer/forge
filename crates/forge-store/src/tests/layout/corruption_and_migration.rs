use super::*;

#[test]
fn milestone_6_layout_materialization_fails_reopen_when_persisted_key_drifts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-materialization-corrupt");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_layout_materialization_key_mismatch(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::BackendIntegrityViolation
    );
    assert!(error
        .message()
        .contains("milestone 6 layout materialization map key"));
}

#[test]
fn milestone_6_layout_materialization_fails_reopen_when_payload_witness_drifts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-materialization-payload-corrupt");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_layout_materialization_chunk_member_count_drift(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::BackendIntegrityViolation
    );
    assert!(error
        .message()
        .contains("canonical Milestone 9 physical chunk reference"));
}

#[test]
fn milestone_6_commit_coupled_layout_seed_fails_reopen_when_authority_basis_digest_drifts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-published-request-authority-drift");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_commit_coupled_layout_seed_authority_digest_drift(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::BackendIntegrityViolation
    );
    assert!(error.message().contains("authority basis digest"));
}

#[test]
fn milestone_6_commit_coupled_layout_seed_fails_reopen_when_commit_support_summary_loses_seed_link()
{
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-published-request-summary-gap");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_commit_support_summary_seed_gap(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::CommitSupportPublicationGap
    );
    assert!(error.message().contains("commit-coupled layout seed set"));
}

#[test]
fn milestone_6_commit_coupled_layout_seed_fails_reopen_when_payload_support_artifact_is_missing() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-published-request-payload-gap");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_commit_coupled_layout_seed_payload_gap(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::CommitSupportPublicationGap
    );
    assert!(error.message().contains("commit-coupled layout seed"));
}

#[test]
fn milestone_6_commit_coupled_layout_seed_fails_reopen_when_payload_body_drifts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-published-request-payload-drift");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_commit_coupled_layout_seed_payload_drift(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::CommitSupportPublicationGap
    );
    assert!(error
        .message()
        .contains("non-canonical milestone 6 commit-coupled layout seed"));
}

#[test]
fn milestone_6_chunk_membership_fails_reopen_when_boundary_reference_drifts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_store_path("layout-chunk-membership-boundary-drift");

    let mut store = crate::ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    drop(store);

    force_milestone_6_chunk_membership_boundary_drift(&path);

    let error = crate::ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::BackendIntegrityViolation
    );
    assert!(error.message().contains("chunk membership"));
}

#[test]
fn sqlite_legacy_commit_coupled_layout_seed_table_migrates_forward_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    let request = entity_set_request(branch_id, commit_id);
    let path = unique_test_sqlite_path("layout-legacy-seed-migration");

    let mut store = crate::ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    let materialized = store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    drop(store);

    simulate_legacy_milestone_6_commit_coupled_layout_seed_storage(&path);

    let reopened = crate::ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let fetched = reopened.fetch_milestone_6_layout_support(request).unwrap();
    assert_eq!(fetched, materialized);

    let connection = rusqlite::Connection::open(path).unwrap();
    let migrated_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM milestone_6_commit_coupled_layout_seed_records",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let legacy_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM milestone_6_published_layout_request_records",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(migrated_count, 1);
    assert_eq!(legacy_count, 1);
}
