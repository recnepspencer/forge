use crate::tests::support::*;

#[test]
fn subscriber_stream_rejects_zero_batch_size() {
    let runtime = runtime_with_test_schema();
    let error = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(0))
        .unwrap_err();

    assert_eq!(error.class, SubscriberStreamFailureClass::InvalidBatchSize);
}
