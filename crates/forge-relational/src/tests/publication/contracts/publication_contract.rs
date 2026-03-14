use crate::tests::support::*;

#[test]
fn latest_bundle_remains_a_convenience_surface_not_the_subscriber_contract() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "anchor");

    let publication = runtime.publication_access();
    let bundle = publication.latest_bundle().unwrap();
    let subscriber = publication
        .read_subscriber_stream(SubscriberResumeRequest::from_head(8))
        .unwrap();

    assert_eq!(bundle.patch, subscriber.patches[0]);
    assert!(subscriber.latest_available_checkpoint.is_some());
}
