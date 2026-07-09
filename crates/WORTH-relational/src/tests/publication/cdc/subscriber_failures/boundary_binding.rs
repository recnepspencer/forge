use super::fixtures::{install_schema_version, visible_bridge_transition_options};
use crate::schema::data::{
    SchemaBoundaryFingerprint, SchemaContinuationClassification, SchemaVersionId,
};
use crate::tests::support::*;

#[test]
fn subscriber_stream_rejects_checkpoint_with_mismatched_authoritative_boundary_binding() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    install_schema_version(&mut runtime, SchemaVersionId(2));
    let mut txn = runtime.begin_transaction(visible_bridge_transition_options(SchemaVersionId(2)));
    txn.push_batch(batch_create("b"));
    let _ = txn.commit().unwrap();

    let batch = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(2))
        .unwrap();
    let WORTHd = batch
        .next_checkpoint
        .unwrap()
        .with_authoritative_boundary_binding_for_test(
            Some(SchemaBoundaryFingerprint::ZERO),
            Some(SchemaContinuationClassification::ContinueUnchanged),
            Some(SchemaContinuationClassification::ContinueUnchanged),
            true,
        );

    let error = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(WORTHd, 1))
        .unwrap_err();

    assert_eq!(
        error.class,
        SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch
    );
}
