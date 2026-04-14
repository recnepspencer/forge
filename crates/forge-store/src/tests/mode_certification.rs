use crate::{
    AbsentRuntimeWitness, CheckpointAuthorityReport, DurableMutationRequest,
    EmbeddedCheckpointClassification, ExternalRuntimeCheckpointEnvelope,
    ExternalRuntimeCommitEnvelope, ForgeStoreBuilder, Milestone2CertificationBundle,
    ObservedModeFailure,
};
use serde_json::json;

use super::support::{create_entity_commit, latest_envelope, runtime_with_demo_schema};

fn create_alpha_commit(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "alpha"))
}

#[test]
fn milestone_2_certification_bundle_proves_mode_contract_parity() {
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

    let before_checkpoint_artifact_digest = embedded.milestone_2_lane_evidence().artifact_digest;
    let checkpoint_receipt = embedded
        .persist_external_checkpoint(
            ExternalRuntimeCheckpointEnvelope::new(
                "checkpoint-certified",
                "embedded-runtime",
                EmbeddedCheckpointClassification::DerivedDurable,
            )
            .with_metadata(json!({"kind":"certified-checkpoint"})),
        )
        .expect("checkpoint should persist");
    let embedded_lane = embedded.milestone_2_lane_evidence();
    let checkpoint_authority_report = CheckpointAuthorityReport::from_checkpoint(
        before_checkpoint_artifact_digest,
        embedded_lane.artifact_digest.clone(),
        &checkpoint_receipt,
    );

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
    let durable_lane = durable.milestone_2_lane_evidence();

    let absent_runtime = {
        let mut runtime = runtime_with_demo_schema();
        create_entity_commit(&mut runtime, "alpha");
        runtime
    };
    let absent_lane = AbsentRuntimeWitness::new(absent_runtime).milestone_2_lane_evidence();

    let bundle = Milestone2CertificationBundle::new(
        durable_lane.clone(),
        embedded_lane.clone(),
        absent_lane.clone(),
        checkpoint_authority_report.clone(),
        &[],
    );

    assert_eq!(durable_lane.artifact_digest, embedded_lane.artifact_digest);
    assert!(bundle.mode_contract_matrix.durable_embedded_artifact_parity);
    assert!(bundle.mode_contract_matrix.absent_mode_is_no_store);
    assert!(bundle.mode_contract_matrix.zero_forbidden_cross_mode_work);
    assert!(!bundle.artifact_digest.is_empty());
    assert!(!bundle.diagnostics_digest.is_empty());
    assert!(!bundle.failure_digest.is_empty());
    assert_eq!(
        bundle
            .checkpoint_authority_report
            .authoritative_artifact_digest_before,
        bundle
            .checkpoint_authority_report
            .authoritative_artifact_digest_after
    );
    assert!(!bundle.canonical_json().is_empty());
    assert_eq!(
        bundle
            .counter_snapshot
            .absent_lane
            .absent_mode_selection_count,
        1
    );
    assert_eq!(
        bundle
            .counter_snapshot
            .absent_lane
            .absent_mode_store_touch_count,
        0
    );
}

#[test]
fn milestone_2_certification_bundle_captures_typed_mode_failures() {
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

    let mut embedded_runtime = runtime_with_demo_schema();
    create_entity_commit(&mut embedded_runtime, "alpha");
    let envelope = latest_envelope(&embedded_runtime);
    embedded
        .persist_external_commit(ExternalRuntimeCommitEnvelope::new(
            "embedded-runtime",
            envelope,
        ))
        .expect("embedded commit should persist");

    let embedded_lane = embedded.milestone_2_lane_evidence();

    let durable_runtime = runtime_with_demo_schema();
    let durable = ForgeStoreBuilder::new()
        .in_memory()
        .durable_mode(durable_runtime)
        .build()
        .expect("durable mode should build");
    let durable_lane = durable.milestone_2_lane_evidence();

    let absent_lane =
        AbsentRuntimeWitness::new(runtime_with_demo_schema()).milestone_2_lane_evidence();

    let checkpoint_authority_report = CheckpointAuthorityReport::from_checkpoint(
        embedded_lane.artifact_digest.clone(),
        embedded_lane.artifact_digest.clone(),
        &embedded
            .persist_external_checkpoint(ExternalRuntimeCheckpointEnvelope::new(
                "checkpoint-derived",
                "embedded-runtime",
                EmbeddedCheckpointClassification::DerivedDurable,
            ))
            .expect("derived checkpoint should persist"),
    );

    let bundle = Milestone2CertificationBundle::new(
        durable_lane,
        embedded_lane,
        absent_lane,
        checkpoint_authority_report,
        &[ObservedModeFailure::from_error(&error)],
    );

    assert!(!bundle.failure_digest.is_empty());
    assert_eq!(
        bundle
            .counter_snapshot
            .embedded_lane
            .mode_misuse_rejection_count,
        1
    );
    assert_eq!(
        bundle
            .counter_snapshot
            .embedded_lane
            .embedded_checkpoint_authority_rejection_count,
        1
    );
}
