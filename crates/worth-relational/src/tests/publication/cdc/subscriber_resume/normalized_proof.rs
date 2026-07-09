use super::fixtures::{install_schema_version, transaction_options_for_subscriber_impact};
use crate::schema::data::{SchemaSubscriberImpact, SchemaVersionId};
use crate::tests::support::*;

#[test]
fn subscriber_stream_composes_prior_and_new_boundaries_into_normalized_proof() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    install_schema_version(&mut runtime, SchemaVersionId(2));
    let mut txn = runtime.begin_transaction(transaction_options_for_subscriber_impact(
        SchemaVersionId(2),
        SchemaSubscriberImpact::ConsumableSurfaceChanged,
    ));
    txn.push_batch(batch_create("b"));
    txn.commit().unwrap();

    let first_batch = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(10))
        .unwrap();
    let checkpoint = first_batch.next_checkpoint.clone().unwrap();
    assert_eq!(
        checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        1
    );

    install_schema_version(&mut runtime, SchemaVersionId(3));
    let mut second_txn = runtime.begin_transaction(transaction_options_for_subscriber_impact(
        SchemaVersionId(3),
        SchemaSubscriberImpact::ConsumableSurfaceChanged,
    ));
    second_txn.push_batch(batch_create("c"));
    second_txn.commit().unwrap();

    let resumed = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 10))
        .unwrap();

    let next_checkpoint = resumed.next_checkpoint.unwrap();
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        2
    );
    assert_eq!(
        next_checkpoint
            .continuation_summary()
            .normalized_boundary_count,
        2
    );
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .boundary_fingerprints()
            .len(),
        2
    );
}

#[test]
fn resumed_subscriber_stream_preserves_prior_boundary_and_adds_new_boundary_trace() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    install_schema_version(&mut runtime, SchemaVersionId(2));
    let mut first_transition =
        runtime.begin_transaction(transaction_options_for_subscriber_impact(
            SchemaVersionId(2),
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
        ));
    first_transition.push_batch(batch_create("b"));
    first_transition.commit().unwrap();

    let first_batch = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(10))
        .unwrap();
    let checkpoint = first_batch.next_checkpoint.unwrap();
    assert_eq!(
        checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        1
    );

    install_schema_version(&mut runtime, SchemaVersionId(3));
    let mut second_transition =
        runtime.begin_transaction(transaction_options_for_subscriber_impact(
            SchemaVersionId(3),
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
        ));
    second_transition.push_batch(batch_create("c"));
    second_transition.commit().unwrap();

    let resumed = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 10))
        .unwrap();

    let next_checkpoint = resumed.next_checkpoint.unwrap();
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        2
    );
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .boundary_fingerprints()
            .len(),
        2
    );
    assert_eq!(resumed.continuation.crossed_boundaries().len(), 1);
    let boundary_entries = resumed
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .filter(|entry| entry.code == DiagnosticCode::SubscriberBoundaryEvaluated)
        .count();
    assert_eq!(boundary_entries, 1);
}
