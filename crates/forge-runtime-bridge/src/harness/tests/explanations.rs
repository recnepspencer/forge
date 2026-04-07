use crate::facade::{
    BridgeRouteRequest, FineGrainedMatchStatus, SliceFallbackPolicy, SubscriptionSliceKind,
    TruthDeltaSurfaceKind,
};

use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime, build_runtime_with_aspects, committed_patch, field_aspect_registration,
    field_slice_snapshot, registration, snapshot, surface_fallback_registration,
};

#[test]
fn bridge_route_explanation_reconstructs_patch_to_invalidation_mapping() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "avatar"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![surface_fallback_registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan route for explanation reconstruction");
    runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver route before explanation reconstruction");

    let explanation = runtime
        .diagnostics()
        .explain_last_route_record()
        .expect("bridge should explain the last canonical route record");

    assert_eq!(explanation.route_entries().len(), 1);
    assert_eq!(explanation.invalidation_targets().len(), 1);
    assert_eq!(explanation.snapshot_identity().as_str(), "snapshot-a");
    let entry = &explanation.route_entries()[0];
    assert_eq!(entry.entity_identity(), "user");
    assert_eq!(entry.aspect_label(), "profile");
    assert_eq!(entry.surface_label(), "avatar");
    assert_eq!(entry.mapping_id().as_str(), "profile-surface-fallback");
    assert_eq!(entry.signal_scope(), "signal.profile.fallback");
    assert_eq!(
        explanation.invalidation_targets()[0].signal_scope(),
        "signal.profile.fallback"
    );
}

#[test]
fn bridge_route_explanation_exposes_fine_grained_match_status() {
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
        .expect("bridge should deliver route before explanation reconstruction");

    let explanation = runtime
        .diagnostics()
        .explain_last_route_record()
        .expect("bridge should explain the last canonical route record");

    let entry = &explanation.route_entries()[0];
    assert_eq!(entry.truth_surface_kind(), TruthDeltaSurfaceKind::EntityField);
    assert_eq!(entry.fine_grained_match_status(), FineGrainedMatchStatus::Matched);
    assert_eq!(
        entry.aspect_registration_id().map(|id| id.as_str()),
        Some("profile-name-field")
    );
    assert_eq!(
        entry.subscription_slice_kind(),
        Some(&SubscriptionSliceKind::SignalField)
    );
    assert_eq!(entry.slice_fallback_policy(), Some(SliceFallbackPolicy::Disallow));
    assert_eq!(explanation.subscription_slices().len(), 1);
    assert_eq!(
        explanation.subscription_slices()[0].slice_kind(),
        &SubscriptionSliceKind::SignalField
    );
    assert_eq!(explanation.subscription_slices()[0].surface_label(), "name");
}
