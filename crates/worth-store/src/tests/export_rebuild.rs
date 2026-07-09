use crate::WORTHStore;

use super::harness::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};

#[test]
fn canonical_export_rebuild_preserves_authoritative_truth() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);

    let mut store = crate::WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(first.clone()).unwrap();
    store.append_canonical_commit(second.clone()).unwrap();

    let export = store.export_authoritative_records();
    let rebuilt = WORTHStore::restore_from_authoritative_export(export.admit_restore()).unwrap();

    let rebuilt_first = rebuilt
        .fetch_canonical_commit(first.commit.commit_id)
        .unwrap();
    let rebuilt_second = rebuilt
        .fetch_canonical_commit(second.commit.commit_id)
        .unwrap();
    let rebuilt_head = rebuilt.fetch_branch_head(&second.branch_context).unwrap();

    assert_eq!(rebuilt_first.envelope(), &first);
    assert_eq!(rebuilt_second.envelope(), &second);
    assert_eq!(rebuilt_head.head_commit_id(), Some(second.commit.commit_id));
}

#[test]
fn rebuilt_store_continues_local_commit_sequence_monotonically() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);

    let mut store = crate::WORTHStoreBuilder::new().in_memory().build().unwrap();
    let persisted_first = store.append_canonical_commit(first).unwrap();
    let persisted_second = store.append_canonical_commit(second.clone()).unwrap();

    let export = store.export_authoritative_records();
    let mut rebuilt =
        WORTHStore::restore_from_authoritative_export(export.admit_restore()).unwrap();

    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let third = latest_envelope(&runtime);
    let persisted_third = rebuilt.append_canonical_commit(third).unwrap();

    assert_eq!(persisted_first.commit_sequence(), 1);
    assert_eq!(persisted_second.commit_sequence(), 2);
    assert_eq!(persisted_third.commit_sequence(), 3);
}

#[test]
fn duplicate_authoritative_export_records_are_rejected() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);

    let mut store = crate::WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(first).unwrap();

    let mut export = store.export_authoritative_records();
    let duplicate = export.commit_envelopes[0].clone();
    export.commit_envelopes.push(duplicate);

    let error = WORTHStore::restore_from_authoritative_export(export.admit_restore()).unwrap_err();
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::DuplicateArtifactIdentity
    );
}
