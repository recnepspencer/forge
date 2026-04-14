use crate::{ForgeStoreBuilder, StoreErrorKind};

use super::support::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};

#[test]
fn append_and_fetch_preserves_authoritative_commit_truth() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let persisted = store.append_canonical_commit(envelope.clone()).unwrap();
    let fetched = store
        .fetch_canonical_commit(envelope.commit.commit_id)
        .unwrap();
    let branch_head = store.fetch_branch_head(&envelope.branch_context).unwrap();

    assert_eq!(persisted.envelope(), &envelope);
    assert_eq!(persisted.commit_sequence(), 1);
    assert_eq!(fetched.envelope(), &envelope);
    assert_eq!(fetched.digest().as_str(), persisted.digest().as_str());
    assert_eq!(
        branch_head.head_commit_id(),
        Some(envelope.commit.commit_id)
    );
}

#[test]
fn identical_duplicate_append_is_idempotent() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let first = store.append_canonical_commit(envelope.clone()).unwrap();
    let second = store.append_canonical_commit(envelope).unwrap();

    assert_eq!(first.commit_sequence(), second.commit_sequence());
    assert_eq!(first.digest().as_str(), second.digest().as_str());
}

#[test]
fn conflicting_duplicate_commit_identity_is_rejected() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let mut conflicting = latest_envelope(&runtime);
    conflicting.commit.commit_id = first.commit.commit_id;

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(first).unwrap();
    let error = store.append_canonical_commit(conflicting).unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::DuplicateArtifactIdentity);
}

#[test]
fn orphan_parent_reference_is_rejected() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let root = latest_envelope(&runtime);
    store.append_canonical_commit(root.clone()).unwrap();

    let mut envelope = root;
    envelope.commit.commit_id = forge_relational::facade::history::CommitId(999);
    envelope.commit.parents = vec![forge_relational::facade::history::CommitId(12345)];
    let error = store.append_canonical_commit(envelope).unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::OrphanParentReference);
}
