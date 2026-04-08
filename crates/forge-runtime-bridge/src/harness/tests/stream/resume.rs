use crate::facade::{
    BridgeRouteRequest, ChangeStreamDeclaration, StreamCheckpointFrontierKind,
    StreamCheckpointPublicationMode, StreamCoalescingFamily, StreamCoalescingIntent,
    StreamConsumerShape, StreamDeliveryIntent, StreamDiagnosticsPolicyClass, StreamReplayMode,
    StreamResumeMode,
};
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};

use super::super::support::{build_runtime, committed_patch, registration, snapshot};

#[test]
fn resume_from_checkpoint_preserves_routing_semantics_against_control_run() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(source, sink, vec![registration()]);
    let declaration = ChangeStreamDeclaration::new(
        StreamConsumerShape::RoutingConsumer,
        StreamResumeMode::FromCheckpointOnly,
        StreamCheckpointPublicationMode::PublishEveryWindow,
        StreamCoalescingIntent::Prefer(StreamCoalescingFamily::RoutingWindowCoalescing),
        StreamReplayMode::Enabled,
        StreamDeliveryIntent::RouteInvalidations,
        StreamDiagnosticsPolicyClass::Standard,
    );
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("declaration should validate");
    let contract = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect("contract should resolve");
    let first_window = runtime
        .plan_change_stream_window(
            &contract,
            vec![runtime
                .ingest_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("first envelope should ingest")],
        )
        .expect("first window should plan");
    let control = runtime
        .plan_change_stream_window(
            &contract,
            vec![runtime
                .ingest_committed_patch(BridgeRouteRequest::for_commit("commit-b"))
                .expect("second envelope should ingest")],
        )
        .and_then(|window| runtime.deliver_change_stream_window(&contract, &window))
        .expect("control delivery should succeed");
    let checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &first_window,
        StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    let replay = runtime
        .canonicalize_stream_replay_record(&contract, &first_window, &checkpoint)
        .expect("replay record should canonicalize");
    let resumed = runtime
        .resume_stream_window_from_checkpoint(
            &contract,
            vec![runtime
                .ingest_committed_patch(BridgeRouteRequest::for_commit("commit-b"))
                .expect("second envelope should ingest")],
            checkpoint.checkpoint_token_identity(),
        )
        .expect("resume should succeed");
    let resumed_delivery = runtime
        .deliver_change_stream_window(&contract, resumed.resumed_window())
        .expect("resumed delivery should succeed");

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
        control.summary().stream_digest(),
        resumed_delivery.summary().stream_digest()
    );
    assert_eq!(
        control
            .route_results()
            .iter()
            .map(|result| result.result_summary().route_identity().as_str().to_owned())
            .collect::<Vec<_>>(),
        resumed_delivery
            .route_results()
            .iter()
            .map(|result| result.result_summary().route_identity().as_str().to_owned())
            .collect::<Vec<_>>()
    );
}
