use super::*;

#[test]
fn stable_basis_persists_and_reopens_through_sqlite() {
    let path = unique_test_sqlite_path("worth-store-stable-basis");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let handle = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            envelope.branch_context.clone(),
            envelope.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let stable_basis_id = handle.stable_basis_id().clone();
    drop(store);

    let reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let fetched = reopened.fetch_stable_basis(&stable_basis_id).unwrap();

    assert_eq!(fetched.stable_basis_id(), &stable_basis_id);
    assert_eq!(fetched.frontier_commit_id(), envelope.commit.commit_id);
    assert_eq!(fetched.schema_boundary_artifact_id(), "schema-support:v1");

    let counters = reopened.counters();
    assert_eq!(counters.stable_basis_lookup_count, 1);
    assert_eq!(counters.stable_basis_read_count, 1);
    assert_eq!(counters.stable_basis_support_rows_read, 1);
    assert_eq!(counters.stable_basis_scope_lookup_count, 1);
}

#[test]
fn stable_basis_publication_and_fetch_preserve_descriptor_shape() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");

    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();

    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            branch_id,
            commit_id,
            "schema:v1",
            ContinuationRetentionStatus::Degraded {
                fallback_class: "authority_replay".to_string(),
            },
        ))
        .unwrap();
    let fetched = store.fetch_stable_basis(basis.stable_basis_id()).unwrap();

    assert_eq!(
        basis.retention_descriptor().minimum_retained_commit_id(),
        commit_id
    );
    assert_eq!(
        basis.retention_descriptor().required_support_artifact_set(),
        ["schema:v1"]
    );
    assert_eq!(
        basis.retention_descriptor().schema_boundary_dependency(),
        "schema:v1"
    );
    assert_eq!(
        basis
            .retention_descriptor()
            .authority_replay_fallback_class(),
        "authority_replay"
    );
    assert_eq!(basis.retention_descriptor(), fetched.retention_descriptor());
    assert_eq!(basis.fallback_class(), Some("authority_replay"));
    assert_eq!(basis.complexity_status(), LiveQueryComplexityStatus::Debt);
    assert_eq!(basis.required_support_artifact_set(), ["schema:v1"]);
}

#[test]
fn stable_basis_fetch_degrades_when_required_schema_support_is_missing() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();

    let published = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            envelope.branch_context.clone(),
            envelope.commit.commit_id,
            "schema-support:required",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    assert_eq!(
        published.retention_status(),
        &ContinuationRetentionStatus::Retained
    );

    let fetched = store
        .fetch_stable_basis(published.stable_basis_id())
        .unwrap();
    assert_eq!(
        fetched.retention_status(),
        &ContinuationRetentionStatus::Degraded {
            fallback_class: "authority_replay".to_string()
        }
    );
    assert_eq!(fetched.fallback_class(), Some("authority_replay"));
    assert_eq!(fetched.complexity_status(), LiveQueryComplexityStatus::Debt);

    let counters = store.counters();
    assert_eq!(counters.stable_basis_fallback_count, 1);
}
