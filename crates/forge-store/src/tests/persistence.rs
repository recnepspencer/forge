use crate::{ForgeStoreBuilder, StoreErrorKind};

use super::harness::{
    corruption::authoritative::{
        corrupt_local_file_branch_head_digest, corrupt_local_file_commit_digest,
        corrupt_sqlite_authoritative_digest, corrupt_sqlite_branch_head_digest,
        corrupt_sqlite_envelope_payload, delete_sqlite_parent_row,
    },
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::unique_test_store_path,
    },
};

#[test]
fn file_backed_backend_reloads_authoritative_commits() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store");

    {
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope.clone()).unwrap();
    }

    let store = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let fetched = store
        .fetch_canonical_commit(envelope.commit.commit_id)
        .unwrap();
    let branch_head = store.fetch_branch_head(&envelope.branch_context).unwrap();

    assert_eq!(fetched.envelope(), &envelope);
    assert_eq!(
        branch_head.head_commit_id(),
        Some(envelope.commit.commit_id)
    );
}

#[test]
fn sqlite_backend_reloads_authoritative_commits() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store-sqlite");

    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope.clone()).unwrap();
    }

    let store = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let fetched = store
        .fetch_canonical_commit(envelope.commit.commit_id)
        .unwrap();
    let branch_head = store.fetch_branch_head(&envelope.branch_context).unwrap();

    assert_eq!(fetched.envelope(), &envelope);
    assert_eq!(
        branch_head.head_commit_id(),
        Some(envelope.commit.commit_id)
    );
}

#[test]
fn sqlite_corrupted_digest_record_is_rejected_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store-sqlite-corrupt").with_extension("sqlite");

    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    corrupt_sqlite_authoritative_digest(&path);

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn sqlite_branch_head_digest_drift_is_rejected_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store-sqlite-head-drift").with_extension("sqlite");

    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    corrupt_sqlite_branch_head_digest(&path);

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn sqlite_missing_parent_row_is_rejected_on_open() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store-sqlite-missing-parent").with_extension("sqlite");

    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(first).unwrap();
        store.append_canonical_commit(second).unwrap();
    }

    delete_sqlite_parent_row(&path);

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn sqlite_malformed_envelope_payload_is_rejected_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store-sqlite-malformed").with_extension("sqlite");

    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    corrupt_sqlite_envelope_payload(&path);

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn corrupted_persisted_digest_record_is_rejected_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store-corrupt");

    {
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    corrupt_local_file_commit_digest(&path);

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn branch_head_digest_drift_is_rejected_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store-head-drift");

    {
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    corrupt_local_file_branch_head_digest(&path);

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}
