use crate::tests::support::*;
use crate::publication::cdc::data::{SubscriberContractDeclaration, SubscriberContinuationSummary};
use crate::schema::data::{
    DescriptorSemanticsVersion, HistoricalInterpretationSensitivity, ProposedSchemaTransition,
    SchemaBoundaryFingerprint, SchemaContinuationClassification, SchemaDiffAtom,
    SchemaDiffDetail, SchemaElementKind, SchemaElementRef, SchemaPublicationImpact,
    SchemaReconciliationPolicy, SchemaStratum, SchemaSubscriberImpact,
};

#[test]
fn subscriber_stream_rejects_zero_batch_size() {
    let runtime = runtime_with_test_schema();
    let error = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(0))
        .unwrap_err();

    assert_eq!(error.class, SubscriberStreamFailureClass::InvalidBatchSize);
}

#[test]
fn subscriber_stream_rejects_schema_incompatible_checkpoint() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "anchor");

    let mismatched_checkpoint =
        checkpoint_for_schema_version(PatchStreamPosition(1), SchemaVersionId(99));
    let error = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            mismatched_checkpoint,
            1,
        ))
        .unwrap_err();

    assert_eq!(
        error.class,
        SubscriberStreamFailureClass::SchemaIncompatible
    );
}

#[test]
fn subscriber_stream_rejects_checkpoint_without_history_or_durable_coverage() {
    let runtime = persisted_runtime_with_test_schema();
    let missing_checkpoint =
        checkpoint_for_schema_version(PatchStreamPosition(42), SchemaVersionId(1));
    let error = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(missing_checkpoint, 1))
        .unwrap_err();

    assert_eq!(
        error.class,
        SubscriberStreamFailureClass::DurableCoverageGap
    );
    assert!(error
        .diagnostics
        .iter()
        .all(|artifact| artifact.scope == DiagnosticsScope::Replay));
}

#[test]
fn subscriber_stream_rejects_checkpoint_with_mismatched_contract_identity() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "anchor");

    let batch = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(1))
        .unwrap();
    let checkpoint = batch.next_checkpoint.unwrap();
    let requested_contract = SubscriberContractDeclaration {
        contract_id: "subscriber.contract.geometry.v2".to_string(),
        ..SubscriberContractDeclaration::default()
    };

    let error = runtime
        .publication_access()
        .read_subscriber_stream(
            SubscriberResumeRequest::resume_after(checkpoint, 1)
                .with_subscriber_contract(requested_contract),
        )
        .unwrap_err();

    assert_eq!(
        error.class,
        SubscriberStreamFailureClass::SubscriberContractMismatch
    );
}

#[test]
fn subscriber_stream_rejects_when_normalized_continuation_proof_exceeds_complexity_ceiling() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "anchor");
    let batch = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(1))
        .unwrap();
    let checkpoint = batch
        .next_checkpoint
        .unwrap()
        .with_incoherent_continuation_for_test(
            "default.subscriber.contract".to_string(),
            crate::publication::cdc::data::NormalizedContinuationProof::from_raw_parts_for_test(
                (0_u8..64)
                .map(|value| SchemaBoundaryFingerprint::new([value; 32]))
                .collect(),
                DescriptorSemanticsVersion::default(),
                64,
            ),
            SubscriberContinuationSummary::new(
                "default.subscriber.contract".to_string(),
                SchemaContinuationClassification::ContinueWithVisibleBridge,
                64,
                64,
                DescriptorSemanticsVersion::default(),
                false,
            ),
            DescriptorSemanticsVersion::default(),
        );

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut txn = runtime.begin_transaction(
        TransactionOptions::default().with_schema_transition(
            ProposedSchemaTransition {
                source_schema_id: SchemaId("test".to_string()),
                source_schema_version_id: SchemaVersionId(1),
                target_schema_id: SchemaId("test".to_string()),
                target_schema_version_id: SchemaVersionId(2),
                diff_atoms: vec![SchemaDiffAtom::new(
                    SchemaElementRef::new(
                        SchemaElementKind::Field,
                        SchemaId("test".to_string()),
                        SchemaVersionId(2),
                        Some(KindId(1)),
                        "tag",
                    ),
                    vec![
                        SchemaStratum::StructuralShape,
                        SchemaStratum::PublicationContract,
                    ],
                    SchemaPublicationImpact::ObservableSurfaceChanged,
                    SchemaSubscriberImpact::ConsumableSurfaceChanged,
                    HistoricalInterpretationSensitivity::NotSensitive,
                    SchemaDiffDetail::AddedField {
                        field_name: "tag".into(),
                        required: false,
                        default_expression: Some("null".into()),
                    },
                )],
            },
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ),
    );
    txn.push_batch(batch_create("b"));
    txn.commit().unwrap();

    let error = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 10))
        .unwrap_err();

    assert_eq!(
        error.class,
        SubscriberStreamFailureClass::RenegotiationRequired
    );
    let rejection_entry = error
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| entry.code == DiagnosticCode::SubscriberContractEvaluated)
        .unwrap();
    assert_eq!(
        rejection_entry.fields["normalized_boundary_count_at_failure"],
        json!(65)
    );
    assert!(error
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::SubscriberRenegotiationDecision));
}

#[test]
fn subscriber_stream_rejects_checkpoint_with_inconsistent_continuation_summary() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "anchor");

    let batch = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(1))
        .unwrap();
    let checkpoint = batch.next_checkpoint.unwrap().with_incoherent_continuation_for_test(
        "default.subscriber.contract".to_string(),
        batch
            .latest_available_checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.normalized_continuation_proof().clone())
            .unwrap_or_default(),
        SubscriberContinuationSummary::new(
            "default.subscriber.contract".to_string(),
            SchemaContinuationClassification::ContinueWithVisibleBridge,
            1,
            99,
            DescriptorSemanticsVersion::default(),
            false,
        ),
        DescriptorSemanticsVersion::default(),
    );

    let error = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 1))
        .unwrap_err();

    assert_eq!(
        error.class,
        SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch
    );
}
