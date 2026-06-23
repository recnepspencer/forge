use crate::facade::tests::{canonical_envelope, runtime};
use crate::policy::BridgeRuntimePolicy;

#[test]
fn runtime_retains_stream_checkpoint_and_replay_records() {
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
            vec![canonical_envelope(
                crate::truth_identity_fixtures::truth_branch_fixture("main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
        )
        .expect("window should plan");
    let checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    let replay = runtime
        .canonicalize_stream_replay_record(&contract, &window, &checkpoint)
        .expect("stream replay record should canonicalize");

    assert_eq!(
        runtime
            .diagnostics()
            .stream_checkpoint_for_identity(checkpoint.checkpoint_token_identity())
            .expect("checkpoint should be retained")
            .checkpoint_token_identity(),
        checkpoint.checkpoint_token_identity()
    );
    assert_eq!(
        runtime
            .diagnostics()
            .stream_replay_record_for_identity(replay.replay_record_identity())
            .expect("replay record should be retained")
            .replay_record_identity(),
        replay.replay_record_identity()
    );
    assert_eq!(
        runtime
            .diagnostics()
            .stream_replay_record_for_checkpoint_identity(checkpoint.checkpoint_token_identity())
            .expect("checkpoint-to-replay lookup should be retained")
            .replay_record_identity(),
        replay.replay_record_identity()
    );
}

#[test]
fn runtime_explains_last_stream_checkpoint_and_replay_record() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
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
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("stream declarations should validate");
    let contract = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect("stream contract should resolve");
    let window = runtime
        .plan_change_stream_window(
            &contract,
            vec![canonical_envelope(
                crate::truth_identity_fixtures::truth_branch_fixture("main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
        )
        .expect("window should plan");
    let result = runtime
        .deliver_replay_audit_stream_window(&contract, &window)
        .expect("replay-audit windows should deliver");

    let checkpoint_explanation = runtime
        .diagnostics()
        .explain_last_stream_checkpoint()
        .expect("checkpoint explanation should be available");
    let replay_explanation = runtime
        .diagnostics()
        .explain_last_stream_replay_record()
        .expect("replay explanation should be available");

    assert_eq!(
        checkpoint_explanation.checkpoint_token_identity(),
        result
            .checkpoint()
            .checkpoint_token_identity_for_reporting()
    );
    assert_eq!(
        replay_explanation.replay_record_identity(),
        result.replay_record().replay_record_identity().as_str()
    );
    assert_eq!(
        replay_explanation.checkpoint_token_identity(),
        result
            .checkpoint()
            .checkpoint_token_identity_for_reporting()
    );
}
