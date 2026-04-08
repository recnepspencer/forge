use std::sync::Arc;

use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeContinuityAuthorityBasis,
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, BridgeLineageContext,
    BridgeLineageSourceError, BridgePreparationMode, BridgeRouteRequest, ContinuityLineageSource,
    FineGrainedMatchStatus, SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind,
    TruthSnapshotIdentity,
};

use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime, build_runtime_with_aspects, committed_patch, field_aspect_registration,
    field_slice_snapshot, registration, snapshot, surface_fallback_registration,
};

#[derive(Debug, Clone, Default)]
struct ExplanationContinuityLineageSource;

impl ContinuityLineageSource for ExplanationContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![Arc::from("lineage:explanation-successor")],
            vec![Arc::from("entity:0:4:2")],
            vec![7],
        )
    }
}

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

#[test]
fn bridge_continuity_explanation_reconstructs_canonical_continuity_truth() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .with_continuity_lineage_source(ExplanationContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            crate::facade::BridgeMappingContext::default().with_lineage_context(
                BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                )),
            ),
        )
        .expect("route should plan");
    let result = runtime
        .deliver_invalidation(route)
        .expect("delivery should succeed");
    let route_record = runtime
        .diagnostics()
        .route_record_for_route_identity(result.result_summary().route_identity().as_str())
        .expect("route record should be retained");
    let requests = runtime
        .plan_continuity_requests(&route_record)
        .expect("continuity requests should plan");
    let packet = runtime
        .plan_historical_lineage_packet(&requests)
        .expect("historical lineage packet should plan");
    let resolved = runtime
        .resolve_lineage_continuity(&packet)
        .expect("continuity should resolve");
    let artifact = runtime.lower_continuity_artifact(&resolved);
    let canonical = runtime.canonicalize_continuity_record(&route_record, &requests, &artifact);

    let explanation = runtime
        .diagnostics()
        .explain_continuity_record(&canonical);

    assert_eq!(explanation.route_identity(), route_record.route_identity());
    assert_eq!(explanation.source_snapshot().as_str(), "snapshot-a");
    assert_eq!(explanation.source_branch().as_str(), "main");
    assert_eq!(explanation.continuity_outcomes().len(), 1);
    assert_eq!(
        explanation.continuity_outcomes()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor
    );
    assert_eq!(explanation.remapped_slices().len(), 1);
    assert_eq!(
        explanation.remapped_slices().slices()[0].entity_identity(),
        "entity:0:4:2"
    );
}

#[test]
fn bridge_bulk_explanation_reconstructs_canonical_bulk_plan_truth() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let plan = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("bulk workload should plan before explanation reconstruction");
    let record = runtime.canonicalize_bulk_workload_plan(&plan);

    let explanation = runtime
        .diagnostics()
        .explain_bulk_record(&record);

    assert_eq!(explanation.workload_identity(), plan.workload_identity());
    assert_eq!(
        explanation.canonical_planning_identity(),
        plan.canonical_planning_identity()
    );
    assert_eq!(
        explanation.admission_profile_identity(),
        plan.admission_profile_identity()
    );
    assert_eq!(explanation.selected_mode(), BridgePreparationMode::ParallelPreparation);
    assert_eq!(explanation.request_segment_count(), 2);
    assert_eq!(explanation.packet_set_digest(), plan.packet_set().digest());
    assert_eq!(
        explanation.execution_plan_digest(),
        plan.execution_plan().digest()
    );
    assert_eq!(
        explanation.reduced_artifact_digest(),
        plan.execution_plan().reduced_artifact().digest()
    );
    assert_eq!(
        explanation.decision_log_digest(),
        plan.execution_plan().decision_log().digest()
    );
    assert_eq!(
        explanation.decision_log(),
        plan.execution_plan().decision_log()
    );
    assert_eq!(
        explanation.counters().bulk_parallel_preparation_admitted_count(),
        1
    );
    assert!(explanation.planning_failures().is_empty());
    assert_eq!(explanation.planning_failure_count(), 0);
}

#[test]
fn bridge_bulk_explanation_retains_typed_parallel_fallback_failures() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let plan = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("bulk workload should plan before explanation reconstruction");
    let explanation = runtime
        .diagnostics()
        .explain_bulk_record(&runtime.canonicalize_bulk_workload_plan(&plan));

    assert_eq!(explanation.selected_mode(), BridgePreparationMode::Serial);
    assert_eq!(explanation.decision_log(), plan.execution_plan().decision_log());
    assert_eq!(explanation.planning_failures(), plan.execution_plan().planning_failures());
    assert_eq!(explanation.planning_failure_count(), 1);
    assert_eq!(
        explanation.planning_failures()[0].kind(),
        crate::facade::BridgeBulkPlanningFailureKind::ParallelPreparationNotProfitable
    );
}
