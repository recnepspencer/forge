use crate::facade::tests::{canonical_envelope, runtime};
use crate::policy::BridgeRuntimePolicy;

#[test]
fn runtime_publishes_checkpoint_from_window() {
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
    let window = runtime
        .plan_change_stream_window(
            &contract,
            vec![
                canonical_envelope(
                    crate::truth_identity_fixtures::truth_branch_fixture("main"),
                    crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                    crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                ),
                canonical_envelope(
                    crate::truth_identity_fixtures::truth_branch_fixture("main"),
                    crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
                    crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                ),
            ],
        )
        .expect("window should plan");

    let checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );

    assert_eq!(
        checkpoint.consumer_contract_identity(),
        contract.consumer_contract_identity()
    );
    assert_eq!(
        checkpoint.stream_protocol_identity(),
        contract.stream_protocol_identity()
    );
    assert_eq!(checkpoint.checkpoint_member_count(), 2);
    assert_eq!(
        checkpoint.checkpoint_frontier_kind(),
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier
    );
    assert_eq!(
        checkpoint.contiguous_acknowledged_through_position(),
        window.last_stream_position().stream_position_identity()
    );
}

#[test]
fn runtime_checkpoint_member_count_tracks_cumulative_frontier_width() {
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
    let first_checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &first_window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    let _first_replay = runtime
        .canonicalize_stream_replay_record(&contract, &first_window, &first_checkpoint)
        .expect("first replay record should canonicalize");
    let resumed = runtime
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
        .expect("resume should succeed");
    let second_checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        resumed.resumed_window(),
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );

    assert_eq!(first_checkpoint.checkpoint_member_count(), 1);
    assert_eq!(second_checkpoint.checkpoint_member_count(), 2);
}
