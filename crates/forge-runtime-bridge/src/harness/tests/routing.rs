use crate::facade::{BridgeRouteErrorKind, BridgeRouteRequest};

use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime, build_runtime_with_aspects, committed_patch, field_aspect_registration,
    field_slice_snapshot, registration, surface_fallback_registration,
};

#[test]
fn bridge_routes_registered_fallback_deterministically() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "avatar"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![surface_fallback_registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge planning should admit registered fallback routing");

    assert_eq!(route.routing_summary().routing_entry_count(), 1);
    assert_eq!(route.lowering_summary().invalidation_target_count(), 1);
}

#[test]
fn bridge_rejects_unmapped_surface_without_registration() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "avatar"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let error = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect_err("bridge planning should reject unmapped committed patch surfaces");

    assert_eq!(error.kind(), BridgeRouteErrorKind::MissingMappingRegistration);
    assert!(error.to_string().contains("No bridge mapping registration matched"));
}

#[test]
fn bridge_slice_identity_is_stable_for_identical_slice_sets() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let left_runtime = build_runtime_with_aspects(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "field:name"));
    right_source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let right_runtime = build_runtime_with_aspects(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![surface_fallback_registration()],
        vec![field_aspect_registration()],
    );

    let left_result = left_runtime
        .deliver_invalidation(
            left_runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("left route should plan"),
        )
        .expect("left route should deliver");
    let right_result = right_runtime
        .deliver_invalidation(
            right_runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("right route should plan"),
        )
        .expect("right route should deliver");

    assert_eq!(
        left_result.result_summary().subscription_slice_identity(),
        right_result.result_summary().subscription_slice_identity()
    );
    assert_eq!(left_result.result_summary().subscription_slice_count(), 1);
    assert_eq!(right_result.result_summary().subscription_slice_count(), 1);
}
