use crate::facade::BridgeRouteRequest;

use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime_with_aspects, committed_patch, field_aspect_registration, field_slice_snapshot,
    registration,
};

#[test]
fn bridge_counters_expose_digest_input_bytes() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = build_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let result = runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("route should plan before digest budget capture"),
        )
        .expect("delivery should succeed before digest budget capture");

    assert!(result.counters().digest_computation_count() >= 8);
    assert!(result.counters().digest_input_bytes() > 0);
}
