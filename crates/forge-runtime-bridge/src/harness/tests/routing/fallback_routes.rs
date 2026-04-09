use crate::facade::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeRouteRequest, MappingSelector,
    SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
};

use super::super::support::{
    build_runtime_with_aspects, committed_patch, surface_fallback_registration,
};
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};

#[test]
fn registered_partition_fallback_routes_deterministically() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch(
        "commit-a",
        "patch-a",
        "snapshot-a",
        "partition:inventory",
    ));
    let left_runtime = build_runtime_with_aspects(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![surface_fallback_registration()],
        vec![BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::new("partition-fallback"),
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("inventory"),
            ),
            TruthDeltaSurfaceKind::EntityPartition,
            SubscriptionSliceKind::SignalPartition,
            SliceFallbackPolicy::RegisteredPartitionFallback,
        )],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch(
        "commit-a",
        "patch-a",
        "snapshot-a",
        "partition:inventory",
    ));
    let right_runtime = build_runtime_with_aspects(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![surface_fallback_registration()],
        vec![BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::new("partition-fallback"),
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("inventory"),
            ),
            TruthDeltaSurfaceKind::EntityPartition,
            SubscriptionSliceKind::SignalPartition,
            SliceFallbackPolicy::RegisteredPartitionFallback,
        )],
    );

    let left_route = left_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("partition fallback route should plan");
    let right_route = right_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("partition fallback route should plan deterministically");

    assert_eq!(left_route.subscription_slices().len(), 1);
    assert_eq!(
        left_route.subscription_slices().slices()[0].slice_kind(),
        &SubscriptionSliceKind::SignalPartition
    );
    assert_eq!(
        left_route.subscription_slices().slices()[0].match_status(),
        crate::facade::FineGrainedMatchStatus::FallbackAdmitted
    );
    assert_eq!(left_route.counters().planned_slice_match_count(), 1);
    assert_eq!(left_route.counters().slice_fallback_count(), 1);
    assert_eq!(left_route.counters().slice_suppression_count(), 0);
    assert_eq!(
        left_route.lowering_summary().subscription_slice_identity(),
        right_route.lowering_summary().subscription_slice_identity()
    );
}
