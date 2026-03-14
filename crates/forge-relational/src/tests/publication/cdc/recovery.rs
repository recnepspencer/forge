use crate::tests::support::*;

#[test]
fn subscriber_stream_recovers_from_durable_canonical_envelopes_when_checkpoint_is_not_in_memory() {
    let mut runtime = persisted_runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "a");
    runtime.durability_authority().checkpoint().unwrap();
    let _second = create_entity_outcome(&mut runtime, "b");

    assert!(runtime
        .history_authority()
        .remove_commit_envelope_for_test(first.commit.commit_id));

    let checkpoint = checkpoint_for_schema_version(PatchStreamPosition(1), SchemaVersionId(1));
    let resumed = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 8))
        .unwrap();

    assert_eq!(resumed.patches.len(), 1);
    assert_eq!(resumed.patches[0].position.0, 2);
    assert_eq!(
        resumed.recovery_decision.source,
        crate::facade::publication::SubscriberRecoverySource::DurableCanonicalRecovery
    );
}
