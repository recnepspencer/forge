use super::*;

#[test]
fn stable_basis_identity_equality_and_inequality_rules_hold() {
    let branch_id = BranchId("main".to_string());
    let request = stable_basis_request(
        branch_id.clone(),
        CommitId(7),
        "schema-support:7",
        "support:ctx:v1",
        ContinuationRetentionStatus::Retained,
    );
    let same = stable_basis_request(
        branch_id.clone(),
        CommitId(7),
        "schema-support:7",
        "support:ctx:v1",
        ContinuationRetentionStatus::Retained,
    );
    let different = StableBasisReadRequest::new(
        branch_id,
        CommitId(7),
        StableBasisReadScope::SingleEntity(crate::SingleEntityAspectScope::new("entity-alpha")),
        "support:ctx:v2",
        "schema-support:v1",
        StableBasisLayoutPosture::ProofOnly,
        "authority:basis:v1",
        ContinuationRetentionStatus::Retained,
    );

    let left = crate::StableBasisId::from_request(&request);
    let right = crate::StableBasisId::from_request(&same);
    let changed = crate::StableBasisId::from_request(&different);

    assert_eq!(left, right);
    assert_ne!(left, changed);
}

#[test]
fn continuation_batch_identity_is_canonicalized_from_all_required_inputs() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;
    store.append_canonical_commit(envelope).unwrap();
    let basis = planned_basis_handle(
        &store,
        branch_id,
        commit_id,
        ContinuationRetentionStatus::Retained,
    );

    let first = ContinuationBatchId::from_parts(
        &basis,
        "cursor-main",
        "subscriber-a",
        (CommitId(9), CommitId(10)),
        basis.read_scope(),
        1,
    );
    let same = ContinuationBatchId::from_parts(
        &basis,
        "cursor-main",
        "subscriber-a",
        (CommitId(9), CommitId(10)),
        basis.read_scope(),
        1,
    );
    let different = ContinuationBatchId::from_parts(
        &basis,
        "cursor-main",
        "subscriber-a",
        (CommitId(9), CommitId(11)),
        basis.read_scope(),
        1,
    );

    assert_eq!(first, same);
    assert_ne!(first, different);
}

#[test]
fn retention_status_families_are_distinct() {
    let retained = ContinuationRetentionStatus::Retained;
    let degraded = ContinuationRetentionStatus::Degraded {
        fallback_class: "authority_replay".to_string(),
    };
    let rejected = ContinuationRetentionStatus::Rejected {
        reason: "retention gap".to_string(),
    };

    assert!(retained.is_retained());
    assert!(!matches!(degraded, ContinuationRetentionStatus::Retained));
    assert!(!matches!(rejected, ContinuationRetentionStatus::Retained));
}
