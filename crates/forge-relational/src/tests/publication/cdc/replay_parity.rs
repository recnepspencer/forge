use crate::tests::support::*;

#[test]
fn subscriber_stream_matches_patch_stream_for_committed_history() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "a");
    let _ = create_entity_outcome(&mut runtime, "b");

    let patch_batch = runtime
        .publication_access()
        .read_patch_stream(PatchStreamRequest {
            after_position: None,
            max_commits: 8,
        })
        .unwrap();
    let subscriber_batch = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(8))
        .unwrap();

    assert_eq!(subscriber_batch.patches, patch_batch.patches);
}

#[test]
fn durable_source_subscriber_stream_matches_recovered_runtime_patch_stream() {
    let mut runtime = persisted_runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "a");
    runtime.durability_authority().checkpoint().unwrap();
    let _second = create_entity_outcome(&mut runtime, "b");
    let _third = create_entity_outcome(&mut runtime, "c");

    assert!(runtime
        .history_authority()
        .remove_commit_envelope_for_test(first.commit.commit_id));

    let subscriber_batch = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            checkpoint_for_schema_version(PatchStreamPosition(1), SchemaVersionId(1)),
            8,
        ))
        .unwrap();

    let recovery_plan = runtime.durability_access().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered
        .durability_authority()
        .recover(recovery_plan)
        .unwrap();
    let recovered_patch_batch = recovered
        .publication_access()
        .read_patch_stream(PatchStreamRequest {
            after_position: Some(PatchStreamPosition(1)),
            max_commits: 8,
        })
        .unwrap();

    assert_eq!(subscriber_batch.patches, recovered_patch_batch.patches);
}
