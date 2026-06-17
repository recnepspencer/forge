use crate::facade::tests::{canonical_envelope, runtime};
use crate::policy::BridgeRuntimePolicy;

#[test]
fn runtime_canonicalizes_stream_replay_record_from_matching_checkpoint() {
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

    runtime
        .validate_consumer_checkpoint(&contract, &window, &checkpoint)
        .expect("matching checkpoints should validate");
    let replay = runtime
        .canonicalize_stream_replay_record(&contract, &window, &checkpoint)
        .expect("matching stream facts should canonicalize a replay record");

    assert_eq!(
        replay.consumer_contract_identity(),
        contract.consumer_contract_identity()
    );
    assert_eq!(
        replay.stream_window_identity(),
        window.stream_window_identity()
    );
    assert_eq!(
        replay.checkpoint_token_identity(),
        checkpoint.checkpoint_token_identity()
    );
    assert!(replay
        .digest()
        .starts_with("canonical-stream-replay-record:sha256:"));
}

#[test]
fn runtime_rejects_checkpoint_reuse_across_different_contracts() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let routing_declaration = crate::stream::ChangeStreamDeclaration::new(
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
    let replay_declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::ReplayAuditConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::Prefer(
            crate::stream::StreamCoalescingFamily::ReplayAuditWindowCoalescing,
        ),
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::ReplayAudit,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let routing_protocol = runtime
        .validate_change_stream_declaration(routing_declaration)
        .expect("routing declaration should validate");
    let routing_contract = runtime
        .resolve_change_stream_consumer_contract(&routing_protocol)
        .expect("routing contract should resolve");
    let replay_protocol = runtime
        .validate_change_stream_declaration(replay_declaration)
        .expect("replay declaration should validate");
    let replay_contract = runtime
        .resolve_change_stream_consumer_contract(&replay_protocol)
        .expect("replay contract should resolve");
    let window = runtime
        .plan_change_stream_window(
            &routing_contract,
            vec![canonical_envelope(
                crate::truth_identity_fixtures::truth_branch_fixture("main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
        )
        .expect("window should plan");
    let checkpoint = runtime.publish_consumer_checkpoint(
        &routing_contract,
        &window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );

    let error = runtime
        .validate_consumer_checkpoint(&replay_contract, &window, &checkpoint)
        .expect_err("checkpoints should not be reusable across different contracts");

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::CheckpointContractMismatch
    );
}
