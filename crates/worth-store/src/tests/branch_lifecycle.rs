use crate::{WORTHStoreBuilder, StoreErrorKind};
use worth_relational::facade::history::BranchId;

use super::harness::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};

#[test]
fn branch_append_requires_registered_branch_and_then_fast_forwards() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let root_branch = root.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());

    runtime
        .history_authority()
        .create_branch(feature_branch.clone(), &root_branch)
        .unwrap();
    update_entity_on_branch(
        &mut runtime,
        entity_id,
        "feature-update",
        Some(feature_branch.clone()),
    );
    let feature_commit = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();

    let unknown_branch = store
        .append_canonical_commit(feature_commit.clone())
        .unwrap_err();
    assert_eq!(unknown_branch.kind(), &StoreErrorKind::UnknownBranch);

    store
        .create_branch(feature_branch.clone(), Some(&root_branch))
        .unwrap();
    let persisted = store
        .append_canonical_commit(feature_commit.clone())
        .unwrap();
    let branch_head = store.fetch_branch_head(&feature_branch).unwrap();

    assert_eq!(persisted.envelope(), &feature_commit);
    assert_eq!(
        branch_head.head_commit_id(),
        Some(feature_commit.commit.commit_id)
    );
}

#[test]
fn non_root_commit_is_rejected_for_empty_branch() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let error = store.append_canonical_commit(envelope).unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::UnknownBranch);
}
