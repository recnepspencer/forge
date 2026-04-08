use crate::facade::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeRouteRequest, MappingSelector,
    SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
};

use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::super::support::{
    build_runtime_with_aspects, committed_patch, field_aspect_registration, field_slice_snapshot,
    registration,
};

#[test]
fn field_surface_invalidates_only_registered_field_slice() {
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
        .expect("field-scoped route should plan");

    assert_eq!(route.subscription_slices().len(), 1);
    assert_eq!(
        route.subscription_slices().slices()[0].slice_kind(),
        &SubscriptionSliceKind::SignalField
    );
    assert_eq!(route.subscription_slices().slices()[0].surface_label(), "name");
    assert_eq!(route.counters().truth_delta_surface_count(), 1);
    assert_eq!(route.counters().normalized_truth_delta_surface_count(), 1);
    assert_eq!(route.counters().planned_slice_match_count(), 1);
    assert_eq!(route.counters().slice_fallback_count(), 0);
    assert_eq!(route.counters().slice_suppression_count(), 0);
    assert_eq!(route.counters().mapping_fallback_count(), 0);
    assert_eq!(route.lowering_summary().subscription_slice_count(), 1);
    assert_eq!(route.routing_summary().invalidation_target_count(), 1);
}

#[test]
fn region_surface_invalidates_only_registered_region_slice() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        "commit-a",
        "patch-a",
        "snapshot-a",
        "region:name",
    ));
    let runtime = build_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::new("profile-name-region"),
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityRegion,
            SubscriptionSliceKind::SignalRegion,
            SliceFallbackPolicy::Disallow,
        )],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("region-scoped route should plan");

    assert_eq!(route.subscription_slices().len(), 1);
    assert_eq!(
        route.subscription_slices().slices()[0].slice_kind(),
        &SubscriptionSliceKind::SignalRegion
    );
    assert_eq!(route.subscription_slices().slices()[0].surface_label(), "name");
    assert_eq!(route.counters().truth_delta_surface_count(), 1);
    assert_eq!(route.counters().normalized_truth_delta_surface_count(), 1);
    assert_eq!(route.counters().planned_slice_match_count(), 1);
    assert_eq!(route.counters().slice_fallback_count(), 0);
    assert_eq!(route.counters().slice_suppression_count(), 0);
    assert_eq!(route.counters().mapping_fallback_count(), 0);
    assert_eq!(route.lowering_summary().subscription_slice_count(), 1);
    assert_eq!(route.routing_summary().invalidation_target_count(), 1);
}
