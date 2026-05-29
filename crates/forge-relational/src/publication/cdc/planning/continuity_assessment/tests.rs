use crate::facade::diagnostics::DiagnosticCode;
use crate::publication::cdc::data::{
    NormalizedContinuationProof, SubscriberContinuationClassSet, SubscriberContractDeclaration,
};
use crate::publication::cdc::planning::assess_subscriber_continuity;
use crate::schema::data::{
    DescriptorCanonicalizationVersion, DescriptorSemanticsVersion,
    HistoricalInterpretationSensitivity, SchemaBoundaryFingerprint, SchemaBridgeDescriptor,
    SchemaBridgeabilityClassification, SchemaContinuationClassification,
    SchemaContinuationDescriptor, SchemaStratum,
};
use crate::tests::support::{create_entity_outcome, runtime_with_test_schema};

#[test]
fn unsupported_continuation_failure_counts_current_boundary_when_no_prior_proof_exists() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "a");
    let fingerprint = SchemaBoundaryFingerprint::new([7_u8; 32]);
    let envelopes = vec![envelope_with_continuation(
        outcome.envelope().clone(),
        fingerprint,
        SchemaContinuationClassification::ContinueWithContractUpgrade,
    )];
    let contract = SubscriberContractDeclaration {
        accepted_upgrade_classes: SubscriberContinuationClassSet::new([]),
        ..SubscriberContractDeclaration::default()
    };

    let failure = assess_subscriber_continuity(
        &runtime,
        &envelopes,
        &contract,
        &NormalizedContinuationProof::default(),
        DescriptorSemanticsVersion::default(),
    )
    .unwrap_err();

    let rejection_entry = failure
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| entry.code == DiagnosticCode::SubscriberContractEvaluated)
        .unwrap();
    assert_eq!(
        rejection_entry.fields.root_value()["normalized_boundary_count_at_failure"],
        serde_json::json!(1)
    );
}

#[test]
fn unsupported_continuation_failure_deduplicates_boundary_already_present_in_prior_proof() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "a");
    let fingerprint = SchemaBoundaryFingerprint::new([9_u8; 32]);
    let envelopes = vec![envelope_with_continuation(
        outcome.envelope().clone(),
        fingerprint,
        SchemaContinuationClassification::ContinueWithContractUpgrade,
    )];
    let contract = SubscriberContractDeclaration {
        accepted_upgrade_classes: SubscriberContinuationClassSet::new([]),
        ..SubscriberContractDeclaration::default()
    };
    let prior_proof =
        NormalizedContinuationProof::new(vec![fingerprint], DescriptorSemanticsVersion::default());

    let failure = assess_subscriber_continuity(
        &runtime,
        &envelopes,
        &contract,
        &prior_proof,
        DescriptorSemanticsVersion::default(),
    )
    .unwrap_err();

    let rejection_entry = failure
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| entry.code == DiagnosticCode::SubscriberContractEvaluated)
        .unwrap();
    assert_eq!(
        rejection_entry.fields.root_value()["normalized_boundary_count_at_failure"],
        serde_json::json!(1)
    );
}

fn envelope_with_continuation(
    mut envelope: crate::replay::data::CanonicalCommitEnvelope,
    fingerprint: SchemaBoundaryFingerprint,
    continuation: SchemaContinuationClassification,
) -> crate::replay::data::CanonicalCommitEnvelope {
    envelope.schema_continuation_descriptor = Some(SchemaContinuationDescriptor::new(
        fingerprint,
        SchemaBridgeDescriptor::new(
            fingerprint,
            DescriptorSemanticsVersion::default(),
            DescriptorCanonicalizationVersion::default(),
            continuation,
            SchemaBridgeabilityClassification::SubscriberVisible,
            HistoricalInterpretationSensitivity::NotSensitive,
            vec![SchemaStratum::PublicationContract],
        ),
        1,
    ));
    envelope
}
