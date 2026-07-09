use super::*;

#[test]
fn stable_basis_fetch_rejects_missing_artifact() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();

    let error = store
        .fetch_stable_basis(&crate::StableBasisId::from_string("stable-basis|missing"))
        .expect_err("missing stable basis artifact must fail typed");

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::StableBasisArtifactMissing
    );
}

#[test]
fn stable_basis_rejects_branch_frontier_mismatch() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();

    let error = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            BranchId("other".to_string()),
            envelope.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .expect_err("cross-branch frontier basis must fail typed");

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::StableBasisShapeViolation
    );
}

#[test]
fn stable_basis_rejects_authority_digest_drift() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();

    let error = store
        .read_stable_basis(StableBasisReadRequest::new(
            envelope.branch_context.clone(),
            envelope.commit.commit_id,
            StableBasisReadScope::SingleEntity(crate::SingleEntityAspectScope::new("entity-alpha")),
            stable_digest(
                store
                    .export_authoritative_records()
                    .into_canonicalized()
                    .commit_support_summaries
                    .iter()
                    .find(|summary| summary.commit_id == envelope.commit.commit_id)
                    .expect("support summary"),
            ),
            "schema-support:v1",
            StableBasisLayoutPosture::ProofOnly,
            "authority:basis:drifted",
            ContinuationRetentionStatus::Retained,
        ))
        .expect_err("authority digest drift must fail typed");

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::StableBasisShapeViolation
    );
}

#[test]
fn stable_basis_rejects_support_context_digest_drift() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let export = store.export_authoritative_records().into_canonicalized();
    let commit = export
        .commit_envelopes
        .iter()
        .find(|stored| stored.envelope.commit.commit_id == envelope.commit.commit_id)
        .expect("frontier commit");

    let error = store
        .read_stable_basis(StableBasisReadRequest::new(
            envelope.branch_context.clone(),
            envelope.commit.commit_id,
            StableBasisReadScope::SingleEntity(crate::SingleEntityAspectScope::new("entity-alpha")),
            "support:ctx:drifted",
            "schema-support:v1",
            StableBasisLayoutPosture::ProofOnly,
            commit.envelope_digest.clone(),
            ContinuationRetentionStatus::Retained,
        ))
        .expect_err("support-context digest drift must fail typed");

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::StableBasisSupportContextMismatch
    );
}
