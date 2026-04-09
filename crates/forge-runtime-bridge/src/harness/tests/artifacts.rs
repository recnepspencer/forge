use crate::facade::BridgeRouteRequest;

use super::support::{
    build_runtime, committed_patch_items, registration, snapshot, surface_fallback_registration,
};
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};

#[test]
fn bridge_artifact_identities_are_bounded_and_stable_for_identical_patchsets() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch_items(
        "commit-a",
        "patch-a",
        "snapshot-a",
        vec![
            crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
            crate::facade::BridgeCommittedPatchItem::new("user", "profile", "avatar"),
            crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
        ],
    ));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration(), surface_fallback_registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch_items(
        "commit-a",
        "patch-a",
        "snapshot-a",
        vec![
            crate::facade::BridgeCommittedPatchItem::new("user", "profile", "avatar"),
            crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
        ],
    ));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration(), surface_fallback_registration()],
    );

    let left_route = left_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan canonical route identity");
    let right_route = right_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan canonical route identity");

    let left_result = left_runtime
        .deliver_invalidation(left_route)
        .expect("bridge should lower and deliver canonical invalidation artifact");
    let right_result = right_runtime
        .deliver_invalidation(right_route)
        .expect("bridge should lower and deliver canonical invalidation artifact");

    assert_eq!(
        left_result.routing_summary().route_identity(),
        right_result.routing_summary().route_identity()
    );
    assert_eq!(
        left_result.artifact().invalidation_identity(),
        right_result.artifact().invalidation_identity()
    );
    assert_eq!(
        left_result.artifact().snapshot_token().token_value(),
        right_result.artifact().snapshot_token().token_value()
    );

    let route_identity = left_result.routing_summary().route_identity().as_str();
    let invalidation_identity = left_result.artifact().invalidation_identity().as_str();
    let snapshot_token = left_result.artifact().snapshot_token().token_value();
    assert!(route_identity.starts_with("route:sha256:"));
    assert!(invalidation_identity.starts_with("invalidation:sha256:"));
    assert!(snapshot_token.starts_with("snapshot-token:sha256:"));
    assert!(route_identity.len() < 90);
    assert!(invalidation_identity.len() < 100);
    assert!(snapshot_token.len() < 100);
}
