use crate::facade::tests::{canonical_envelope, runtime};
use crate::policy::BridgeRuntimePolicy;

#[test]
fn runtime_resume_rejects_truncated_checkpoint_identity() {
    let runtime = runtime(
        BridgeRuntimePolicy::development()
            .with_route_record_limit(1)
            .with_failure_record_limit(1),
    );
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::Prefer(
            crate::stream::StreamCoalescingFamily::RoutingWindowCoalescing,
        ),
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::RouteInvalidations,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("stream declarations should validate");
    let contract = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect("stream contract should resolve");
    let first_window = runtime
        .plan_change_stream_window(
            &contract,
            vec![canonical_envelope(
                crate::truth_identity_fixtures::truth_branch_fixture("main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
        )
        .expect("first window should plan");
    let first_checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &first_window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    let second_window = runtime
        .plan_change_stream_window(
            &contract,
            vec![canonical_envelope(
                crate::truth_identity_fixtures::truth_branch_fixture("main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
        )
        .expect("second window should plan");
    let _second_checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &second_window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );

    let error = runtime
        .resume_stream_window_from_checkpoint(
            &contract,
            vec![canonical_envelope(
                crate::truth_identity_fixtures::truth_branch_fixture("main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
            first_checkpoint.checkpoint_token_identity(),
        )
        .expect_err("evicted checkpoints should be treated as truncated");

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::CheckpointTruncated
    );
}

#[test]
fn runtime_resume_reuses_retained_checkpoint_and_replay_truth() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::Prefer(
            crate::stream::StreamCoalescingFamily::RoutingWindowCoalescing,
        ),
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::RouteInvalidations,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("stream declarations should validate");
    let contract = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect("stream contract should resolve");
    let first_window = runtime
        .plan_change_stream_window(
            &contract,
            vec![canonical_envelope(
                crate::truth_identity_fixtures::truth_branch_fixture("main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
        )
        .expect("first window should plan");
    let checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &first_window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    let replay = runtime
        .canonicalize_stream_replay_record(&contract, &first_window, &checkpoint)
        .expect("replay record should canonicalize");

    let resumed = runtime
        .resume_stream_window_from_checkpoint(
            &contract,
            vec![canonical_envelope(
                crate::truth_identity_fixtures::truth_branch_fixture("main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
            checkpoint.checkpoint_token_identity(),
        )
        .expect("retained checkpoint should resume cleanly");

    assert_eq!(
        resumed.checkpoint().checkpoint_token_identity(),
        checkpoint.checkpoint_token_identity()
    );
    assert_eq!(
        resumed.replay_record().replay_record_identity(),
        replay.replay_record_identity()
    );
    assert_eq!(resumed.resumed_window().members().len(), 1);
    assert_eq!(
        resumed
            .resumed_window()
            .first_stream_position()
            .ordinal_position(),
        checkpoint.checkpoint_member_count()
    );
}
