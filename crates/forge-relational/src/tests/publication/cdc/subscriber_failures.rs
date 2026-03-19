use crate::tests::support::*;

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
