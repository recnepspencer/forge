use crate::facade::{BridgeDeliveryErrorKind, BridgeRouteRequest};

use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime_with_aspects, committed_patch, field_aspect_registration, field_slice_snapshot,
    registration, snapshot,
};

#[test]
fn bridge_diagnostics_respect_route_record_retention_budget() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_committed_patch(committed_patch("commit-c", "patch-c", "snapshot-c", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));
    source.insert_snapshot(snapshot("snapshot-c", "carol"));
    let runtime = crate::facade::RuntimeBridge::builder()
        .with_relational_source(source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .with_policy(
            crate::facade::BridgeRuntimePolicy::development()
                .with_route_record_limit(2)
                .with_failure_record_limit(2),
        )
        .register_mapping(registration())
        .build()
        .expect("bridge runtime with bounded diagnostics retention");

    for commit in ["commit-a", "commit-b", "commit-c"] {
        let route = runtime
            .plan_committed_patch(BridgeRouteRequest::for_commit(commit))
            .expect("bridge should plan route for retention test");
        runtime
            .deliver_invalidation(route)
            .expect("bridge should deliver route for retention test");
    }

    let route_records = runtime.diagnostics().route_records();
    assert_eq!(runtime.diagnostics().route_record_limit(), 2);
    assert_eq!(route_records.len(), 2);
    assert_eq!(route_records[0].source_commit().as_str(), "commit-b");
    assert_eq!(
        route_records
            .last()
            .expect("retained route record")
            .source_commit()
            .as_str(),
        "commit-c"
    );
}

#[test]
fn bridge_diagnostics_respect_failure_record_retention_budget() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_committed_patch(committed_patch("commit-c", "patch-c", "snapshot-c", "name"));
    let runtime = crate::facade::RuntimeBridge::builder()
        .with_relational_source(source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .with_policy(
            crate::facade::BridgeRuntimePolicy::development()
                .with_route_record_limit(2)
                .with_failure_record_limit(2),
        )
        .register_mapping(registration())
        .build()
        .expect("bridge runtime with bounded diagnostics retention");

    for commit in ["commit-a", "commit-b", "commit-c"] {
        let route = runtime
            .plan_committed_patch(BridgeRouteRequest::for_commit(commit))
            .expect("bridge should plan route for failure retention test");
        let error = runtime
            .deliver_invalidation(route)
            .expect_err("bridge should fail delivery when the planned snapshot is absent");
        assert_eq!(error.kind(), BridgeDeliveryErrorKind::SnapshotAcquisitionFailure);
    }

    let failure_records = runtime.diagnostics().failure_records();
    assert_eq!(runtime.diagnostics().failure_record_limit(), 2);
    assert_eq!(failure_records.len(), 2);
    assert_eq!(failure_records[0].source_commit().as_str(), "commit-b");
    assert_eq!(
        failure_records
            .last()
            .expect("retained failure record")
            .source_commit()
            .as_str(),
        "commit-c"
    );
}

#[test]
fn bridge_route_record_captures_slice_counters_and_slice_entries() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = build_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan route with fine-grained aspect registration");
    runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver route before diagnostics capture");

    let record = runtime
        .diagnostics()
        .last_route_record()
        .expect("bridge should capture a route record");

    assert_eq!(record.subscription_slices().len(), 1);
    assert_eq!(record.counters().planned_slice_match_count(), 1);
    assert_eq!(record.counters().slice_fallback_count(), 0);
    assert_eq!(record.counters().slice_suppression_count(), 0);
}
