use forge_harness::facade::{ExecutionProfile, ExecutionRequest};
use forge_harness::runtime::HarnessAdapter;

use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};

use super::super::support::{build_runtime, committed_patch, registration, snapshot};
use super::support::{replay_audit_target, routing_target, stream_fixture};

#[test]
fn bridge_harness_stream_consumers_preserve_canonical_member_truth() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("baseline");
    let fixture = stream_fixture("bridge-stream-multi-consumer");

    let mut routing_runtime = adapter.create_runtime().expect("routing harness runtime");
    adapter
        .prepare_runtime(&mut routing_runtime, &profile)
        .expect("routing harness prepare");
    adapter
        .load_fixture(&mut routing_runtime, &fixture)
        .expect("routing harness load fixture");
    let routing_run = adapter
        .execute(
            &mut routing_runtime,
            &fixture,
            &ExecutionRequest::target("stream-routing", routing_target()),
            &profile,
        )
        .expect("routing stream execution");

    let mut replay_runtime = adapter.create_runtime().expect("replay harness runtime");
    adapter
        .prepare_runtime(&mut replay_runtime, &profile)
        .expect("replay harness prepare");
    adapter
        .load_fixture(&mut replay_runtime, &fixture)
        .expect("replay harness load fixture");
    let replay_run = adapter
        .execute(
            &mut replay_runtime,
            &fixture,
            &ExecutionRequest::target("stream-replay-audit", replay_audit_target()),
            &profile,
        )
        .expect("replay stream execution");

    assert_eq!(
        routing_run.summary["stream_digest"],
        replay_run.summary["stream_digest"]
    );
    assert_eq!(
        routing_run.summary["first_stream_member_identity"],
        replay_run.summary["first_stream_member_identity"]
    );
    assert_eq!(
        routing_run.summary["last_stream_member_identity"],
        replay_run.summary["last_stream_member_identity"]
    );
    assert_eq!(routing_run.summary["stream_member_count"], 2);
    assert_eq!(replay_run.summary["stream_member_count"], 2);
    assert!(routing_run.summary["window_digest"].as_str().is_some());
    assert!(replay_run.summary["replay_digest"].as_str().is_some());
    assert_eq!(
        routing_run.summary["counter_snapshot"]["stream_member_count"],
        serde_json::json!(2)
    );
    assert_eq!(
        replay_run.summary["counter_snapshot"]["stream_replay_count"],
        serde_json::json!(1)
    );
}

#[test]
fn consumer_pacing_differences_do_not_change_stream_meaning() {
    bridge_harness_stream_consumers_preserve_canonical_member_truth();
}

#[test]
fn illegal_coalescing_boundary_fails_explicitly() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    source.insert_snapshot(snapshot("snapshot-b", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
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
        .expect("declaration should validate");
    let contract = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect("contract should resolve");

    let error = runtime
        .plan_change_stream_window(
            &contract,
            vec![
                runtime
                    .ingest_committed_patch(crate::facade::BridgeRouteRequest::for_commit(
                        "commit-a",
                    ))
                    .expect("first envelope should ingest"),
                runtime
                    .ingest_committed_patch(crate::facade::BridgeRouteRequest::for_commit(
                        "commit-b",
                    ))
                    .expect("second envelope should ingest"),
            ],
        )
        .expect_err("illegal coalescing boundaries must fail explicitly");

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::IllegalCoalescingBoundary
    );
}
