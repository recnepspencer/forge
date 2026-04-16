use crate::{DurableMutationRequest, ForgeStoreBuilder, SnapshotCaptureRequest, StoreErrorKind};

use super::harness::fixtures::runtime::{
    create_entity, create_entity_commit, latest_envelope, runtime_with_demo_schema,
};

fn emitted_support_record_count(
    envelope: &forge_relational::facade::replay::CanonicalCommitEnvelope,
) -> u64 {
    let emits_schema_support = envelope.schema_transition.is_some()
        || envelope.schema_continuation_descriptor.is_some()
        || envelope.schema_reconciliation_descriptor.is_some();
    let emits_lineage_support =
        !envelope.lineage_event_ids().is_empty() || !envelope.lineage_events().is_empty();

    1 + u64::from(emits_schema_support) + u64::from(emits_lineage_support)
}

fn authoritative_append_family_count(
    envelope: &forge_relational::facade::replay::CanonicalCommitEnvelope,
) -> u64 {
    5 + (emitted_support_record_count(envelope) - 1)
}

fn authoritative_append_record_count(
    envelope: &forge_relational::facade::replay::CanonicalCommitEnvelope,
) -> u64 {
    envelope.commit.parents.len() as u64 + 3 + emitted_support_record_count(envelope)
}

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
        envelope.commit.parents.len() as u64 + 4 + emitted_support_record_count(&envelope)
    );
    assert_eq!(counters.commit_support_publication_count, 1);
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
        envelope.commit.parents.len() as u64 + 4 + emitted_support_record_count(&envelope)
    );
    assert_eq!(counters.commit_support_publication_count, 1);
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
    assert_eq!(
        counters.state_delta_touched_family_count,
        authoritative_append_family_count(&envelope)
    );
    assert_eq!(
        counters.state_delta_touched_record_count,
        authoritative_append_record_count(&envelope)
    );
    assert_eq!(counters.state_clone_fallback_count, 0);
}

#[test]
fn durable_hot_path_avoids_clone_fallback_and_reports_delta_breadth() {
    let mut expectation_runtime = runtime_with_demo_schema();
    create_entity(&mut expectation_runtime, "alpha");
    let expected_envelope = latest_envelope(&expectation_runtime);

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
    assert_eq!(
        counters.state_delta_touched_family_count,
        5 + authoritative_append_family_count(&expected_envelope)
    );
    assert_eq!(
        counters.state_delta_touched_record_count,
        5 + authoritative_append_record_count(&expected_envelope)
    );
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

#[test]
fn lineage_fetch_counters_reflect_present_or_missing_support_artifacts() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let emits_lineage_support =
        !envelope.lineage_event_ids().is_empty() || !envelope.lineage_events().is_empty();

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();

    match store.fetch_lineage_support(envelope.commit.commit_id) {
        Ok(_) => assert!(emits_lineage_support),
        Err(error) => {
            assert!(!emits_lineage_support);
            assert_eq!(error.kind(), &StoreErrorKind::LineageArtifactMissing);
        }
    }

    let counters = store.counters();
    assert_eq!(counters.schema_boundary_fetch_count, 0);
    assert_eq!(
        counters.lineage_lookup_count,
        u64::from(emits_lineage_support)
    );
    assert_eq!(
        counters.commit_support_publication_gap_count,
        u64::from(!emits_lineage_support)
    );
}
