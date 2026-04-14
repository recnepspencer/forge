use crate::{ForgeStoreBuilder, StoreErrorKind};
use rusqlite::Connection;

use super::support::{
    create_entity, latest_envelope, runtime_with_demo_schema, unique_test_store_path,
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

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "
            UPDATE authoritative_artifact_digests
            SET artifact_digest = 'corrupted-digest'
            WHERE rowid = (
                SELECT rowid
                FROM authoritative_artifact_digests
                WHERE artifact_family = 'CommitEnvelope'
                LIMIT 1
            )
            ",
            [],
        )
        .unwrap();

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

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "
            UPDATE branch_head_records
            SET head_commit_digest = 'drifted-digest'
            WHERE rowid = (
                SELECT rowid
                FROM branch_head_records
                LIMIT 1
            )
            ",
            [],
        )
        .unwrap();

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
    super::support::update_entity_on_branch(&mut runtime, entity_id, "beta", None);
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

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "
            DELETE FROM commit_parent_records
            WHERE rowid = (
                SELECT rowid
                FROM commit_parent_records
                LIMIT 1
            )
            ",
            [],
        )
        .unwrap();

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

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "
            UPDATE commit_envelopes
            SET envelope_payload = '{not-json'
            WHERE rowid = (
                SELECT rowid
                FROM commit_envelopes
                LIMIT 1
            )
            ",
            [],
        )
        .unwrap();

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

    let raw = std::fs::read_to_string(&path).unwrap();
    let persisted: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let digests = persisted
        .get("authoritative_artifact_digests")
        .and_then(serde_json::Value::as_object)
        .unwrap();
    let commit_digest_key = digests
        .keys()
        .find(|key| key.starts_with("CommitEnvelope:commit:"))
        .cloned()
        .unwrap();
    let artifact_digest = digests[&commit_digest_key]
        .get("artifact_digest")
        .and_then(serde_json::Value::as_str)
        .unwrap();
    let corrupted = raw.replacen(
        &format!("\"artifact_digest\": \"{artifact_digest}\""),
        "\"artifact_digest\": \"corrupted-digest\"",
        1,
    );
    std::fs::write(&path, corrupted).unwrap();

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

    let raw = std::fs::read_to_string(&path).unwrap();
    let corrupted = raw.replacen(
        "\"head_commit_digest\": \"",
        "\"head_commit_digest\": \"drifted-",
        1,
    );
    std::fs::write(&path, corrupted).unwrap();

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}
