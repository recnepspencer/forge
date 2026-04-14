use crate::ForgeStoreBuilder;

use super::support::{create_entity, latest_envelope, runtime_with_demo_schema};

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
