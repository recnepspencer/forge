use crate::{DurableMutationRequest, ForgeStoreBuilder, SnapshotCaptureRequest};

use super::harness::fixtures::runtime::{
    create_entity, create_entity_commit, latest_envelope, runtime_with_demo_schema,
};

fn create_alpha_commit_for_durable(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "alpha"))
}

#[test]
fn append_and_fetch_counters_match_admitted_work() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .fetch_canonical_commit(envelope.commit.commit_id)
        .unwrap();

    let counters = store.counters();
    assert_eq!(counters.authoritative_commit_append_count, 1);
    assert_eq!(counters.authoritative_commit_fetch_count, 1);
    assert_eq!(
        counters.commit_parent_record_write_count,
        envelope.commit.parents.len() as u64
    );
    assert_eq!(counters.branch_head_write_count, 2);
    assert_eq!(
        counters.authoritative_digest_write_count,
        envelope.commit.parents.len() as u64 + 4
    );
    assert_eq!(counters.authoritative_fetch_verification_count, 1);
    assert_eq!(counters.authoritative_fetch_verification_failure_count, 0);
    assert_eq!(
        counters.canonicalization_item_count,
        envelope.commit.parents.len() as u64
    );
}

#[test]
fn duplicate_idempotent_append_does_not_increment_authoritative_append_counters_twice() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();

    let counters = store.counters();
    assert_eq!(counters.authoritative_commit_append_count, 1);
    assert_eq!(
        counters.commit_parent_record_write_count,
        envelope.commit.parents.len() as u64
    );
    assert_eq!(counters.branch_head_write_count, 2);
    assert_eq!(
        counters.authoritative_digest_write_count,
        envelope.commit.parents.len() as u64 + 4
    );
    assert_eq!(
        counters.canonicalization_item_count,
        (envelope.commit.parents.len() as u64) * 2
    );
    assert_eq!(counters.canonicalization_duplicate_collapse_count, 0);
}

#[test]
fn authoritative_append_records_delta_scope_without_clone_fallback() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();

    let counters = store.counters();
    assert_eq!(counters.state_delta_apply_count, 1);
    assert_eq!(counters.state_delta_touched_family_count, 4);
    assert_eq!(
        counters.state_delta_touched_record_count,
        envelope.commit.parents.len() as u64 + 3
    );
    assert_eq!(counters.state_clone_fallback_count, 0);
}

#[test]
fn durable_hot_path_avoids_clone_fallback_and_reports_delta_breadth() {
    let mut durable = ForgeStoreBuilder::new()
        .in_memory()
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();

    durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit_for_durable,
        ))
        .unwrap();

    let counters = durable.store().counters();
    assert_eq!(counters.state_delta_apply_count, 6);
    assert_eq!(counters.state_delta_touched_family_count, 9);
    assert_eq!(counters.state_delta_touched_record_count, 8);
    assert_eq!(counters.state_clone_fallback_count, 0);
    assert_eq!(counters.wal_record_append_count, 5);
}

#[test]
fn snapshot_capture_records_delta_scope_without_clone_fallback() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let before = store.counters();

    store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            envelope.commit.commit_id,
        ))
        .unwrap();

    let after = store.counters();
    assert_eq!(
        after.state_delta_apply_count - before.state_delta_apply_count,
        1
    );
    assert_eq!(
        after.state_delta_touched_family_count - before.state_delta_touched_family_count,
        2
    );
    assert_eq!(
        after.state_delta_touched_record_count - before.state_delta_touched_record_count,
        2
    );
    assert_eq!(
        after.state_clone_fallback_count - before.state_clone_fallback_count,
        0
    );
}
