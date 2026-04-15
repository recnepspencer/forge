use crate::{
    AbsentRuntimeWitness, DurableMutationRequest, EmbeddedCheckpointClassification,
    ExternalRuntimeCheckpointEnvelope, ExternalRuntimeCommitEnvelope, ForgeStoreBuilder,
    StoreErrorKind,
};
use serde_json::json;

use super::harness::{
    fixtures::runtime::{create_entity_commit, latest_envelope, runtime_with_demo_schema},
    scenarios::modes::create_alpha_commit,
};

#[test]
fn durable_and_embedded_modes_persist_equivalent_canonical_artifacts() {
    let mut embedded_runtime = runtime_with_demo_schema();
    create_entity_commit(&mut embedded_runtime, "alpha");
    let embedded_envelope = latest_envelope(&embedded_runtime);

    let mut embedded = ForgeStoreBuilder::new()
        .in_memory()
        .embedded_mode()
        .build()
        .expect("embedded mode should build");
    embedded
        .persist_external_commit(ExternalRuntimeCommitEnvelope::new(
            "embedded-runtime",
            embedded_envelope,
        ))
        .expect("embedded mode should persist external commit");

    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
        .in_memory()
        .durable_mode(durable_runtime)
        .build()
        .expect("durable mode should build");
    durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("durable mode should execute hosted mutation");

    assert_eq!(
        embedded
            .store()
            .export_authoritative_records()
            .canonical_json(),
        durable
            .store()
            .export_authoritative_records()
            .canonical_json()
    );

    let embedded_counters = embedded.store().counters();
    assert_eq!(embedded_counters.embedded_mode_selection_count, 1);
    assert_eq!(embedded_counters.durable_mode_selection_count, 0);
    assert_eq!(embedded_counters.external_commit_intake_count, 1);
    assert_eq!(
        embedded_counters.cross_mode_canonical_boundary_reuse_count,
        1
    );

    let durable_counters = durable.store().counters();
    assert_eq!(durable_counters.durable_mode_selection_count, 1);
    assert_eq!(durable_counters.embedded_mode_selection_count, 0);
    assert_eq!(durable_counters.hosted_runtime_start_count, 1);
    assert_eq!(durable_counters.external_commit_intake_count, 0);
}

#[test]
fn derived_embedded_checkpoint_persists_without_changing_authority() {
    let mut runtime = runtime_with_demo_schema();
    create_entity_commit(&mut runtime, "beta");
    let envelope = latest_envelope(&runtime);

    let mut embedded = ForgeStoreBuilder::new()
        .in_memory()
        .embedded_mode()
        .build()
        .expect("embedded mode should build");
    embedded
        .persist_external_commit(ExternalRuntimeCommitEnvelope::new(
            "embedded-runtime",
            envelope,
        ))
        .expect("embedded mode should persist commit");

    let before_export = embedded
        .store()
        .export_authoritative_records()
        .canonical_json();
    let before_head = embedded
        .store()
        .fetch_branch_head(&forge_relational::facade::history::BranchId(
            "main".to_string(),
        ))
        .expect("main branch head before checkpoint");

    let receipt = embedded
        .persist_external_checkpoint(
            ExternalRuntimeCheckpointEnvelope::new(
                "checkpoint-1",
                "embedded-runtime",
                EmbeddedCheckpointClassification::DerivedDurable,
            )
            .with_basis_branch(forge_relational::facade::history::BranchId(
                "main".to_string(),
            ))
            .with_basis_commit(before_head.head_commit_id().expect("head commit"))
            .with_metadata(json!({"kind":"session-checkpoint"})),
        )
        .expect("derived checkpoint should persist");

    assert_eq!(receipt.checkpoint_id(), "checkpoint-1");
    assert!(receipt.contained_commit_ids().is_empty());

    let after_export = embedded
        .store()
        .export_authoritative_records()
        .canonical_json();
    let after_head = embedded
        .store()
        .fetch_branch_head(&forge_relational::facade::history::BranchId(
            "main".to_string(),
        ))
        .expect("main branch head after checkpoint");

    assert_eq!(before_export, after_export);
    assert_eq!(before_head, after_head);

    let fetched = embedded
        .fetch_persisted_checkpoint("checkpoint-1")
        .expect("stored checkpoint should round-trip");
    assert_eq!(fetched.checkpoint_id(), "checkpoint-1");

    let counters = embedded.store().counters();
    assert_eq!(counters.external_checkpoint_intake_count, 1);
    assert_eq!(counters.embedded_checkpoint_authority_rejection_count, 0);
    assert_eq!(counters.mode_misuse_rejection_count, 0);
    assert_eq!(counters.authoritative_commit_append_count, 1);
}

#[test]
fn authoritative_checkpoint_classification_is_rejected() {
    let mut embedded = ForgeStoreBuilder::new()
        .in_memory()
        .embedded_mode()
        .build()
        .expect("embedded mode should build");

    let error = embedded
        .persist_external_checkpoint(ExternalRuntimeCheckpointEnvelope::new(
            "checkpoint-authoritative",
            "embedded-runtime",
            EmbeddedCheckpointClassification::AuthoritativeCommitBundle,
        ))
        .expect_err("authoritative checkpoint classification must be rejected");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::EmbeddedCheckpointAuthorityViolation
    );
    let counters = embedded.store().counters();
    assert_eq!(counters.external_checkpoint_intake_count, 1);
    assert_eq!(counters.embedded_checkpoint_authority_rejection_count, 1);
    assert_eq!(counters.mode_misuse_rejection_count, 1);
}

#[test]
fn external_commit_requires_non_empty_runtime_identity() {
    let mut runtime = runtime_with_demo_schema();
    create_entity_commit(&mut runtime, "delta");
    let envelope = latest_envelope(&runtime);

    let mut embedded = ForgeStoreBuilder::new()
        .in_memory()
        .embedded_mode()
        .build()
        .expect("embedded mode should build");

    let error = embedded
        .persist_external_commit(ExternalRuntimeCommitEnvelope::new("", envelope))
        .expect_err("empty source runtime identity must be rejected");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::ExternalRuntimeArtifactRejection
    );
    let counters = embedded.store().counters();
    assert_eq!(counters.external_commit_intake_count, 0);
    assert_eq!(counters.authoritative_commit_append_count, 0);
}

#[test]
fn embedded_checkpoint_requires_non_empty_identity_fields() {
    let mut embedded = ForgeStoreBuilder::new()
        .in_memory()
        .embedded_mode()
        .build()
        .expect("embedded mode should build");

    let empty_checkpoint_id = embedded
        .persist_external_checkpoint(ExternalRuntimeCheckpointEnvelope::new(
            "",
            "embedded-runtime",
            EmbeddedCheckpointClassification::DerivedDurable,
        ))
        .expect_err("empty checkpoint identity must be rejected");
    assert_eq!(
        empty_checkpoint_id.kind(),
        &StoreErrorKind::ExternalRuntimeCheckpointRejection
    );

    let empty_runtime_id = embedded
        .persist_external_checkpoint(ExternalRuntimeCheckpointEnvelope::new(
            "checkpoint-empty-runtime",
            "",
            EmbeddedCheckpointClassification::DerivedDurable,
        ))
        .expect_err("empty source runtime identity must be rejected");
    assert_eq!(
        empty_runtime_id.kind(),
        &StoreErrorKind::ExternalRuntimeCheckpointRejection
    );

    let counters = embedded.store().counters();
    assert_eq!(counters.external_checkpoint_intake_count, 2);
    assert_eq!(counters.embedded_checkpoint_authority_rejection_count, 0);
    assert_eq!(counters.mode_misuse_rejection_count, 0);
}

#[test]
fn absent_runtime_witness_produces_semantic_evidence_without_store() {
    let mut runtime = runtime_with_demo_schema();
    let commit_id = create_entity_commit(&mut runtime, "gamma");

    let witness = AbsentRuntimeWitness::new(runtime);
    let evidence = witness.semantic_evidence();

    assert_eq!(evidence.latest_commit_id(), Some(commit_id));
    assert_eq!(
        evidence
            .latest_commit_envelope()
            .expect("absent mode evidence should carry latest envelope")
            .commit
            .commit_id,
        commit_id
    );
}
