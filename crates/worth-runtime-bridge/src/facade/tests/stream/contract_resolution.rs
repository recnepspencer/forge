use crate::facade::tests::runtime;
use crate::policy::BridgeRuntimePolicy;

#[test]
fn runtime_validates_and_resolves_routing_stream_contract() {
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
        .validate_change_stream_declaration(declaration.clone())
        .expect("stream declarations should validate");
    let contract = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect("routing declaration should resolve into a routing contract");

    assert_eq!(protocol.declaration(), &declaration);
    assert_eq!(
        contract.consumer_shape(),
        crate::stream::StreamConsumerShape::RoutingConsumer
    );
    assert_eq!(
        contract.admitted_coalescing_family(),
        crate::stream::StreamCoalescingFamily::RoutingWindowCoalescing
    );
    assert_eq!(
        contract.admitted_checkpoint_mode(),
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow
    );
}

#[test]
fn runtime_rejects_replay_disabled_checkpoint_resume_contract_during_resolution() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::None,
        crate::stream::StreamReplayMode::Disabled,
        crate::stream::StreamDeliveryIntent::RouteInvalidations,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("stream declarations should validate");

    let error = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect_err(
        "replay-disabled checkpoint resume contracts must be rejected during contract resolution",
    );

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::UnsupportedResumeMode
    );
}

#[test]
fn runtime_rejects_checkpoint_resume_contract_when_replay_mode_is_disabled() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::None,
        crate::stream::StreamReplayMode::Disabled,
        crate::stream::StreamDeliveryIntent::RouteInvalidations,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("stream declarations should validate");

    let error = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect_err(
            "checkpoint resume without replay records must be rejected during contract resolution",
        );

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::UnsupportedResumeMode
    );
}
