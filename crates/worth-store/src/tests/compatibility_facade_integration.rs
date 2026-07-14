use crate::{
    CompatibilityFamilyKind, DurableCursorAcknowledgeRequest, WORTHStoreBuilder, StoreErrorKind,
};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

#[test]
fn fetch_commit_rejects_when_runtime_commit_manifest_basis_is_missing() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store.remove_compatibility_manifest_record_for_test(CompatibilityFamilyKind::CommitEnvelope);

    let error = store
        .fetch_canonical_commit(envelope.commit.commit_id)
        .expect_err("commit fetch must fail when the live manifest publication is missing");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityManifestPublicationGap
    );
}

#[test]
fn append_rejects_when_runtime_commit_manifest_basis_is_missing() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(first).unwrap();

    create_entity(&mut runtime, "beta");
    let second = latest_envelope(&runtime);
    store.remove_compatibility_manifest_record_for_test(CompatibilityFamilyKind::CommitEnvelope);

    let error = store
        .append_canonical_commit(second)
        .expect_err("append must fail when commit compatibility publication is missing");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityManifestPublicationGap
    );
}

#[test]
fn fetch_branch_head_rejects_when_runtime_branch_manifest_basis_is_missing() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store.remove_compatibility_manifest_record_for_test(
        CompatibilityFamilyKind::BranchVersionDagRecord,
    );

    let error = store
        .fetch_branch_head(&envelope.branch_context)
        .expect_err("branch head fetch must fail when branch compatibility publication is missing");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityManifestPublicationGap
    );
}

#[test]
fn acknowledge_cursor_rejects_when_runtime_support_manifest_basis_is_missing() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store.remove_compatibility_manifest_record_for_test(
        CompatibilityFamilyKind::SchemaLineageCursorCheckpointSupport,
    );

    let error = store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .expect_err("cursor acknowledgment must fail when support-family compatibility is missing");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityManifestPublicationGap
    );
}

#[test]
fn fetch_cursor_identity_rejects_when_runtime_support_manifest_basis_is_missing() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .unwrap();
    store.remove_compatibility_manifest_record_for_test(
        CompatibilityFamilyKind::SchemaLineageCursorCheckpointSupport,
    );

    let error = store
        .fetch_durable_cursor_identity("cursor-main")
        .expect_err("cursor identity fetch must fail when support-family compatibility is missing");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityManifestPublicationGap
    );
}
