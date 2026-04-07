use forge_harness::facade::{ExecutionProfile, ExecutionRequest, ScenarioPlan};
use forge_harness::runtime::HarnessAdapter;
use std::sync::Arc;

use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeContinuityAuthorityBasis,
    BridgeDiagnosticsTier, BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest,
    BridgeLineageContext, BridgeLineageSourceError, BridgeMappingContext, BridgeRouteRequest,
    BridgeBulkDecisionRecordKind, BridgeBulkPlanningFailureKind, BridgeParallelAdmissionClass,
    BridgeParallelAdmissionReason, BridgeParallelLegalityClass, BridgeParallelLegalityReason,
    BridgeParallelProfitabilityClass, BridgeParallelProfitabilityReason, BridgePreparationMode,
    BridgeRuntimePolicy, ContinuityLineageSource, FineGrainedMatchStatus, SubscriptionSliceKind,
    TruthSnapshotIdentity,
};

use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::{BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime, committed_patch, committed_patch_items, field_aspect_registration,
    field_slice_snapshot, registration, snapshot, surface_fallback_registration,
};

#[derive(Debug, Clone, Default)]
struct TestContinuityLineageSource;

impl ContinuityLineageSource for TestContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![Arc::from("lineage:test-successor")],
            vec![Arc::from("entity:0:4:2")],
            vec![7],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct TestMismatchedAuthorityLineageSource;

impl ContinuityLineageSource for TestMismatchedAuthorityLineageSource {
    fn historical_lineage(
        &self,
        _request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            BridgeContinuityAuthorityBasis::new(
                crate::facade::TruthBranchIdentity::new("wrong-branch"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            vec![Arc::from("lineage:test-successor")],
            vec![Arc::from("entity:0:4:2")],
            vec![7],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct TestUnsupportedContinuityLineageSource;

impl ContinuityLineageSource for TestUnsupportedContinuityLineageSource {
    fn historical_lineage(
        &self,
        _request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        Err(crate::facade::BridgeLineageSourceError::new(
            crate::facade::BridgeLineageSourceErrorKind::UnsupportedContinuityClass,
            "unsupported continuity class",
        ))
    }
}

#[derive(Debug, Clone, Default)]
struct TestSplitContinuityLineageSource;

impl ContinuityLineageSource for TestSplitContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![
                Arc::from("lineage:test-split-a"),
                Arc::from("lineage:test-split-b"),
            ],
            vec![Arc::from("entity:0:4:2"), Arc::from("entity:0:5:2")],
            vec![7, 8],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct TestMergeLikeContinuityLineageSource;

impl ContinuityLineageSource for TestMergeLikeContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![
                Arc::from("lineage:test-merge-a"),
                Arc::from("lineage:test-merge-b"),
            ],
            vec![Arc::from("entity:0:9:3")],
            vec![7, 8],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct TestAmbiguousContinuityLineageSource;

impl ContinuityLineageSource for TestAmbiguousContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![
                Arc::from("lineage:test-ambiguous-a"),
                Arc::from("lineage:test-ambiguous-b"),
                Arc::from("lineage:test-ambiguous-c"),
            ],
            vec![Arc::from("entity:0:4:2"), Arc::from("entity:0:5:2")],
            vec![7, 8, 9],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct TestNoAuthoritativeSuccessorLineageSource;

impl ContinuityLineageSource for TestNoAuthoritativeSuccessorLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            Vec::new(),
            Vec::new(),
            vec![7],
        )
    }
}

#[test]
fn bridge_prepared_delivery_is_equivalent_to_one_shot_delivery() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let one_shot = left_runtime
        .deliver_invalidation(
            left_runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("one-shot route should plan"),
        )
        .expect("one-shot delivery should succeed");
    let prepared = right_runtime.prepare_delivery(
        right_runtime
            .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
            .expect("prepared route should plan"),
    );
    let staged = right_runtime
        .deliver_prepared(prepared)
        .expect("prepared delivery should succeed");

    assert_eq!(
        one_shot.result_summary().route_identity(),
        staged.result_summary().route_identity()
    );
    assert_eq!(
        one_shot.result_summary().invalidation_identity(),
        staged.result_summary().invalidation_identity()
    );
    assert_eq!(
        one_shot.result_summary().subscription_slice_identity(),
        staged.result_summary().subscription_slice_identity()
    );
    assert_eq!(one_shot.counters(), staged.counters());
}

#[test]
fn bridge_empty_mapping_context_is_equivalent_to_default_planning_path() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let default_route = left_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("default planning should succeed");
    let explicit_route = right_runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::empty(),
        )
        .expect("explicit empty mapping context planning should succeed");

    assert_eq!(default_route.route_identity(), explicit_route.route_identity());
    assert_eq!(default_route.source_digest(), explicit_route.source_digest());
    assert_eq!(
        default_route.planning_provenance().digest(),
        explicit_route.planning_provenance().digest()
    );
    assert_eq!(
        default_route.lowering_provenance().digest(),
        explicit_route.lowering_provenance().digest()
    );
    assert_eq!(default_route.read_packet(), explicit_route.read_packet());
    assert_eq!(default_route.counters(), explicit_route.counters());
}

#[test]
fn bridge_route_identity_is_stable_across_equivalent_surface_spellings() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "field:name"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left_route = left_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("unprefixed field route should plan");
    let right_route = right_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("prefixed field route should plan");

    assert_eq!(left_route.route_identity(), right_route.route_identity());
    assert_eq!(left_route.read_packet(), right_route.read_packet());
    assert_eq!(
        left_route.lowering_summary().subscription_slice_identity(),
        right_route.lowering_summary().subscription_slice_identity()
    );
}

#[test]
fn bridge_route_identity_is_stable_when_patch_items_arrive_out_of_order_with_duplicates() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-canonical-patch-order",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_items(
                "commit-a",
                "patch-a",
                "snapshot-a",
                vec![
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                ],
            ))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let profile = ExecutionProfile::development("development");

    let mut left = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut left, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut left, &fixture)
        .expect("bridge harness load fixture");
    let left_run = adapter
        .execute(&mut left, &fixture, &request, &profile)
        .expect("bridge harness execute");

    let reordered_fixture = ScenarioPlan::new(
        "bridge-canonical-patch-order-reordered",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_items(
                "commit-a",
                "patch-a",
                "snapshot-a",
                vec![
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                ],
            ))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let mut right = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut right, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut right, &reordered_fixture)
        .expect("bridge harness load fixture");
    let right_run = adapter
        .execute(&mut right, &reordered_fixture, &request, &profile)
        .expect("bridge harness execute");

    assert_eq!(
        left_run.summary["route_identity"],
        right_run.summary["route_identity"]
    );
}

#[test]
fn bridge_bulk_planning_rejects_empty_workloads() {
    let source = InMemoryRelationalBridgeSource::default();
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let error = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![]))
        .expect_err("empty bulk workload should be rejected");

    assert_eq!(
        error.kind(),
        crate::error::BridgeRouteErrorKind::EmptyBulkWorkloadRequest
    );
}

#[test]
fn bridge_bulk_planning_identity_is_stable_across_input_order() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    left_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    right_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("left bulk workload should plan");
    let right = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("right bulk workload should plan");

    assert_eq!(left.workload_identity(), right.workload_identity());
    assert_eq!(
        left.canonical_planning_identity(),
        right.canonical_planning_identity()
    );
    assert_eq!(
        left.admission_profile_identity(),
        right.admission_profile_identity()
    );
    assert_eq!(left.canonical_request().digest(), right.canonical_request().digest());
    assert_eq!(
        left.normalized_summary().digest(),
        right.normalized_summary().digest()
    );
    assert_eq!(left.summary().digest(), right.summary().digest());
    assert_eq!(
        left.planned_routes()
            .iter()
            .map(|route| route.route_identity().as_str())
            .collect::<Vec<_>>(),
        vec![
            right.planned_routes()[0].route_identity().as_str(),
            right.planned_routes()[1].route_identity().as_str(),
        ]
    );
}

#[test]
fn bridge_bulk_planning_separates_canonical_plan_identity_from_admission_profile_identity() {
    let standard_source = InMemoryRelationalBridgeSource::default();
    standard_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    standard_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let standard_runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_policy(
            BridgeRuntimePolicy::development()
                .with_route_record_limit(128)
                .with_failure_record_limit(128),
        )
        .with_relational_source(standard_source.clone())
        .with_truth_branch_head_source(standard_source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .register_mapping(registration())
        .build()
        .expect("standard runtime should build");

    let exhaustive_source = InMemoryRelationalBridgeSource::default();
    exhaustive_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    exhaustive_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let exhaustive_runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_policy(
            BridgeRuntimePolicy::forensic()
                .with_route_record_limit(128)
                .with_failure_record_limit(128),
        )
        .with_relational_source(exhaustive_source.clone())
        .with_truth_branch_head_source(exhaustive_source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .register_mapping(registration())
        .build()
        .expect("exhaustive runtime should build");

    let request = BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
        BridgeRouteRequest::for_commit("commit-a"),
    )]);
    let standard = standard_runtime
        .plan_bulk_workload(request.clone())
        .expect("standard workload should plan");
    let exhaustive = exhaustive_runtime
        .plan_bulk_workload(request)
        .expect("exhaustive workload should plan");

    assert_eq!(
        standard.canonical_planning_identity(),
        exhaustive.canonical_planning_identity()
    );
    assert_eq!(standard.workload_identity(), exhaustive.workload_identity());
    assert_eq!(
        standard.canonical_request().digest(),
        exhaustive.canonical_request().digest()
    );
    assert_eq!(
        standard.normalized_summary().digest(),
        exhaustive.normalized_summary().digest()
    );
    assert_ne!(
        standard.admission_profile_identity(),
        exhaustive.admission_profile_identity()
    );
    assert_eq!(
        standard.planned_routes()[0].source_commit().as_str(),
        exhaustive.planned_routes()[0].source_commit().as_str()
    );
    assert_eq!(BridgeDiagnosticsTier::Standard, standard_runtime.policy().diagnostics_tier());
    assert_eq!(
        BridgeDiagnosticsTier::Exhaustive,
        exhaustive_runtime.policy().diagnostics_tier()
    );
}

#[test]
fn bridge_bulk_execution_plan_carries_canonical_legality_proof() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    left_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    right_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("left bulk workload should plan");
    let right = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("right bulk workload should plan");

    assert_eq!(
        left.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::ParallelPreparationAdmitted
    );
    assert_eq!(
        left.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::AdmittedOperational
    );
    assert_eq!(
        left.execution_plan().legality_decision().class(),
        BridgeParallelLegalityClass::ParallelPreparationLegal
    );
    assert_eq!(
        left.execution_plan().legality_decision().reason(),
        BridgeParallelLegalityReason::DisjointPacketRegionsCertified
    );
    assert_eq!(
        left.execution_plan().profitability_decision().class(),
        BridgeParallelProfitabilityClass::Profitable
    );
    assert_eq!(
        left.execution_plan().profitability_decision().reason(),
        BridgeParallelProfitabilityReason::AdmittedOperational
    );
    assert_eq!(
        left.execution_plan().selected_mode(),
        BridgePreparationMode::ParallelPreparation
    );
    assert_eq!(
        left.execution_plan()
            .legality_proof()
            .canonical_planning_identity(),
        left.canonical_planning_identity()
    );
    assert_eq!(
        left.execution_plan().legality_proof().digest(),
        right.execution_plan().legality_proof().digest()
    );
    assert_eq!(
        left.execution_plan()
            .legality_proof()
            .disjoint_packet_regions()
            .regions()
            .len(),
        4
    );
    assert_eq!(
        left.execution_plan()
            .legality_proof()
            .admitted_partitions()
            .partitions()
            .len(),
        4
    );
    assert_eq!(
        left.execution_plan().reduced_artifact().digest(),
        right.execution_plan().reduced_artifact().digest()
    );
    assert_eq!(left.packet_set().digest(), right.packet_set().digest());
    assert_eq!(left.packet_set().routing_packets().len(), 2);
    assert_eq!(left.packet_set().truth_view_packets().len(), 2);
    assert_eq!(left.packet_set().reduction_packets().len(), 2);
    assert_eq!(left.packet_set().counters().bulk_packet_count(), 4);
    assert_eq!(left.execution_plan().counters().bulk_parallel_legal_count(), 1);
    assert_eq!(
        left.execution_plan().counters().bulk_parallel_profitable_count(),
        1
    );
    assert!(left.execution_plan().planning_failures().is_empty());
    assert_eq!(left.execution_plan().decision_log().records().len(), 3);
    assert_eq!(
        left.execution_plan().decision_log().records()[0].kind(),
        BridgeBulkDecisionRecordKind::ParallelLegality
    );
    assert_eq!(
        left.execution_plan().decision_log().records()[1].kind(),
        BridgeBulkDecisionRecordKind::ParallelProfitability
    );
    assert_eq!(
        left.execution_plan().decision_log().records()[2].kind(),
        BridgeBulkDecisionRecordKind::ParallelAdmission
    );
    assert_eq!(
        left.execution_plan()
            .locality_footprint()
            .publication_scope_count(),
        2
    );
    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_publications()
            .len(),
        2
    );
}

#[test]
fn bridge_bulk_canonical_workload_request_carries_canonical_member_sets() {
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

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("bulk workload should plan");

    assert_eq!(planned.canonical_request().workload_identity(), planned.workload_identity());
    assert_eq!(planned.canonical_request().route_members().len(), 2);
    assert_eq!(planned.canonical_request().subscription_slice_members().len(), 2);
    assert_eq!(planned.canonical_request().truth_view_members().len(), 2);
    assert_eq!(planned.canonical_request().commit_members().len(), 2);
    assert_eq!(planned.canonical_request().snapshot_members().len(), 2);
    assert_eq!(planned.canonical_request().branch_members().len(), 1);
}

#[test]
fn bridge_bulk_normalized_summary_derives_shared_workload_facts_once() {
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

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("bulk workload should plan");

    assert_eq!(planned.normalized_summary().workload_identity(), planned.workload_identity());
    assert_eq!(planned.normalized_summary().route_count(), 2);
    assert_eq!(planned.normalized_summary().subscription_slice_count(), 2);
    assert_eq!(planned.normalized_summary().snapshot_read_count(), 2);
    assert_eq!(planned.normalized_summary().truth_view_member_count(), 2);
    assert_eq!(planned.normalized_summary().continuity_member_count(), 0);
    assert_eq!(planned.normalized_summary().branch_scope_count(), 1);
    assert_eq!(planned.normalized_summary().snapshot_scope_count(), 2);
}

#[test]
fn bridge_bulk_execution_plan_falls_back_to_serial_for_single_route_workload() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
            BridgeRouteRequest::for_commit("commit-a"),
        )]))
        .expect("single-route bulk workload should plan");

    assert_eq!(
        planned.execution_plan().selected_mode(),
        BridgePreparationMode::Serial
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::SerialRequired
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::BelowMinWorkloadWidth
    );
    assert_eq!(
        planned.execution_plan().legality_decision().class(),
        BridgeParallelLegalityClass::SerialOnly
    );
    assert_eq!(
        planned.execution_plan().legality_decision().reason(),
        BridgeParallelLegalityReason::BelowMinWorkloadWidth
    );
    assert_eq!(
        planned.execution_plan().profitability_decision().class(),
        BridgeParallelProfitabilityClass::NotApplicable
    );
    assert_eq!(
        planned.execution_plan().profitability_decision().reason(),
        BridgeParallelProfitabilityReason::SerialOnlyWorkload
    );
    assert_eq!(
        planned
            .execution_plan()
            .legality_proof()
            .admitted_partitions()
            .partitions()
            .len(),
        0
    );
    assert_eq!(
        planned.execution_plan().reduced_artifact().reduction_input_count(),
        2
    );
    assert_eq!(
        planned.execution_plan().reduced_artifact().reduction_output_count(),
        2
    );
    assert!(
        planned
            .execution_plan()
            .legality_proof()
            .disjoint_packet_regions()
            .regions()
            .is_empty()
    );
    assert_eq!(planned.packet_set().routing_packets().len(), 1);
    assert_eq!(planned.packet_set().truth_view_packets().len(), 1);
    assert_eq!(planned.packet_set().reduction_packets().len(), 1);
    assert_eq!(planned.execution_plan().counters().bulk_serial_required_count(), 1);
    assert_eq!(
        planned.execution_plan().counters().bulk_parallel_profitable_count(),
        0
    );
    assert!(planned.execution_plan().planning_failures().is_empty());
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::BelowMinWorkloadWidth
    );
}

#[test]
fn bridge_bulk_execution_plan_falls_back_when_parallel_is_legal_but_not_profitable() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("legal-but-unprofitable workload should plan");

    assert_eq!(
        planned.execution_plan().selected_mode(),
        BridgePreparationMode::Serial
    );
    assert_eq!(
        planned.execution_plan().legality_decision().class(),
        BridgeParallelLegalityClass::ParallelPreparationLegal
    );
    assert_eq!(
        planned.execution_plan().profitability_decision().class(),
        BridgeParallelProfitabilityClass::Unprofitable
    );
    assert_eq!(
        planned.execution_plan().profitability_decision().reason(),
        BridgeParallelProfitabilityReason::SharedPublicationReductionTarget
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::SerialRequired
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::SharedPublicationReductionTarget
    );
    assert_eq!(
        planned.execution_plan().counters().bulk_parallel_legal_count(),
        1
    );
    assert_eq!(
        planned.execution_plan().counters().bulk_parallel_profitable_count(),
        0
    );
    assert_eq!(
        planned
            .execution_plan()
            .counters()
            .bulk_parallel_fallback_to_serial_count(),
        1
    );
    assert_eq!(planned.execution_plan().planning_failures().len(), 1);
    assert_eq!(
        planned.execution_plan().planning_failures()[0].kind(),
        BridgeBulkPlanningFailureKind::LegalButUnprofitableParallelFallback
    );
    assert_eq!(
        planned.execution_plan().decision_log().records()[1].kind(),
        BridgeBulkDecisionRecordKind::ParallelProfitability
    );
}

#[test]
fn bridge_bulk_reduction_collapses_duplicate_publications_deterministically() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("duplicate bulk workload should plan");
    let reduced_artifact = planned.execution_plan().reduced_artifact();

    assert_eq!(reduced_artifact.reduction_input_count(), 2);
    assert_eq!(reduced_artifact.reduction_output_count(), 2);
    assert_eq!(reduced_artifact.counters().bulk_reduction_input_count(), 2);
    assert_eq!(reduced_artifact.reduced_truth_views().len(), 1);
    assert_eq!(reduced_artifact.reduced_publications().len(), 1);
    assert_eq!(
        reduced_artifact.reduced_publications()[0]
            .reduced_route_identities()
            .len(),
        2
    );
    assert_eq!(
        reduced_artifact.reduced_publications()[0].invalidation_target_count(),
        2
    );
}

#[test]
fn bridge_bulk_reduction_artifact_is_stable_across_input_order() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    left_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    right_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("left workload should plan");
    let right = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("right workload should plan");

    assert_eq!(
        left.execution_plan().reduced_artifact().digest(),
        right.execution_plan().reduced_artifact().digest()
    );
    assert_eq!(
        left.execution_plan().reduced_artifact().reduced_publications()[0]
            .publication_identity(),
        right.execution_plan().reduced_artifact().reduced_publications()[0]
            .publication_identity()
    );
    assert_eq!(left.packet_set().digest(), right.packet_set().digest());
}

#[test]
fn bridge_bulk_packet_set_is_stable_across_input_order() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    left_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    right_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("left workload should plan");
    let right = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("right workload should plan");

    assert_eq!(left.packet_set().digest(), right.packet_set().digest());
    assert_eq!(
        left.packet_set()
            .routing_packets()
            .iter()
            .map(|packet| packet.packet_identity())
            .collect::<Vec<_>>(),
        right
            .packet_set()
            .routing_packets()
            .iter()
            .map(|packet| packet.packet_identity())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        left.packet_set()
            .reduction_packets()
            .iter()
            .map(|packet| packet.reduced_target_identity())
            .collect::<Vec<_>>(),
        right
            .packet_set()
            .reduction_packets()
            .iter()
            .map(|packet| packet.reduced_target_identity())
            .collect::<Vec<_>>()
    );
}

#[test]
fn bridge_bulk_packet_reduction_collapses_duplicate_slice_targets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("duplicate workload should plan");

    assert_eq!(planned.packet_set().routing_packets().len(), 2);
    assert_eq!(planned.packet_set().truth_view_packets().len(), 1);
    assert_eq!(planned.packet_set().reduction_packets().len(), 1);
    assert_eq!(
        planned.packet_set().reduction_packets()[0].reduction_family(),
        "publication"
    );
    assert_eq!(
        planned.packet_set().reduction_packets()[0].reduced_target_scope(),
        planned.packet_set().routing_packets()[0].subscription_slice_identity()
    );
    assert_eq!(planned.packet_set().counters().bulk_packet_count(), 3);
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::SharedTruthViewMaterializationTarget
    );
    assert_eq!(
        planned
            .execution_plan()
            .locality_footprint()
            .publication_scope_count(),
        1
    );
}

#[test]
fn bridge_bulk_packet_set_emits_fallback_packets_for_fallback_admitted_slices() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![surface_fallback_registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
            BridgeRouteRequest::for_commit("commit-a"),
        )]))
        .expect("fallback workload should plan");

    assert_eq!(planned.packet_set().routing_packets().len(), 1);
    assert_eq!(planned.packet_set().fallback_packets().len(), 1);
    assert_eq!(
        planned.packet_set().fallback_packets()[0].fallback_class(),
        "surface"
    );
    assert_eq!(planned.packet_set().truth_view_packets().len(), 1);
    assert_eq!(
        planned.packet_set().counters().bulk_packet_count(),
        3
    );
}

#[test]
fn bridge_bulk_packet_set_tracks_truth_view_materialization_packets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("bulk workload should plan");

    assert_eq!(planned.packet_set().truth_view_packets().len(), 2);
    assert_eq!(
        planned
            .packet_set()
            .truth_view_packets()
            .iter()
            .map(|packet| packet.source_snapshot())
            .collect::<Vec<_>>(),
        vec!["snapshot-a", "snapshot-a"]
    );
    assert_eq!(
        planned
            .packet_set()
            .truth_view_packets()
            .iter()
            .map(|packet| packet.snapshot_read_count())
            .sum::<usize>(),
        2
    );
    assert_eq!(planned.packet_set().counters().bulk_packet_count(), 4);
}

#[test]
fn bridge_bulk_reduction_artifact_carries_truth_view_materializations() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
            BridgeRouteRequest::for_commit("commit-a"),
        )]))
        .expect("left bulk workload should plan");
    let right = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
            BridgeRouteRequest::for_commit("commit-a"),
        )]))
        .expect("right bulk workload should plan");

    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_truth_views()
            .len(),
        1
    );
    assert_eq!(
        left.execution_plan().reduced_artifact().reduced_truth_views(),
        right.execution_plan().reduced_artifact().reduced_truth_views()
    );
    assert_eq!(
        left.execution_plan().reduced_artifact().reduced_truth_views()[0].source_snapshot(),
        "snapshot-a"
    );
    assert_eq!(
        left.execution_plan().reduced_artifact().reduced_truth_views()[0].snapshot_read_count(),
        1
    );
}

#[test]
fn bridge_bulk_packet_set_tracks_continuity_remap_packets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );
    let lineage_context = BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
        crate::facade::TruthBranchIdentity::new("main"),
        TruthSnapshotIdentity::new("snapshot-a"),
    ));

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a"))
                .with_mapping_context(
                    BridgeMappingContext::default().with_lineage_context(lineage_context.clone()),
                ),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b"))
                .with_mapping_context(
                    BridgeMappingContext::default().with_lineage_context(lineage_context),
                ),
        ]))
        .expect("continuity-bearing bulk workload should plan");

    assert_eq!(planned.packet_set().continuity_packets().len(), 2);
    assert_eq!(
        planned
            .packet_set()
            .continuity_packets()
            .iter()
            .map(|packet| packet.snapshot_identity())
            .collect::<Vec<_>>(),
        vec!["snapshot-a", "snapshot-a"]
    );
    assert_eq!(
        planned
            .packet_set()
            .continuity_packets()
            .iter()
            .map(|packet| packet.prior_slice_count())
            .sum::<usize>(),
        2
    );
}

#[test]
fn bridge_bulk_reduction_artifact_carries_continuity_remaps() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );
    let lineage_context = BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
        crate::facade::TruthBranchIdentity::new("main"),
        TruthSnapshotIdentity::new("snapshot-a"),
    ));
    let request = BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
        BridgeRouteRequest::for_commit("commit-a"),
    )
    .with_mapping_context(BridgeMappingContext::default().with_lineage_context(lineage_context))]);

    let left = left_runtime
        .plan_bulk_workload(request.clone())
        .expect("left continuity-bearing workload should plan");
    let right = right_runtime
        .plan_bulk_workload(request)
        .expect("right continuity-bearing workload should plan");

    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_continuity_remaps()
            .len(),
        1
    );
    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_continuity_remaps(),
        right
            .execution_plan()
            .reduced_artifact()
            .reduced_continuity_remaps()
    );
    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_continuity_remaps()[0]
            .snapshot_identity(),
        "snapshot-a"
    );
    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_continuity_remaps()[0]
            .prior_slice_count(),
        1
    );
}

#[test]
fn bridge_bulk_execution_plan_rejects_parallel_preparation_for_shared_truth_view_targets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("shared truth-view workload should plan");

    assert_eq!(
        planned.execution_plan().selected_mode(),
        BridgePreparationMode::Serial
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::ParallelPreparationRejected
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::SharedTruthViewMaterializationTarget
    );
    assert_eq!(
        planned.execution_plan().legality_decision().class(),
        BridgeParallelLegalityClass::ParallelPreparationIllegal
    );
    assert_eq!(
        planned.execution_plan().legality_decision().reason(),
        BridgeParallelLegalityReason::SharedTruthViewMaterializationTarget
    );
    assert_eq!(
        planned.execution_plan().profitability_decision().class(),
        BridgeParallelProfitabilityClass::NotApplicable
    );
    assert_eq!(
        planned
            .execution_plan()
            .counters()
            .bulk_parallel_preparation_rejected_count(),
        1
    );
    assert_eq!(
        planned
            .execution_plan()
            .counters()
            .bulk_parallel_fallback_to_serial_count(),
        0
    );
    assert_eq!(planned.execution_plan().planning_failures().len(), 1);
    assert_eq!(
        planned.execution_plan().planning_failures()[0].kind(),
        BridgeBulkPlanningFailureKind::InvalidLegalityBasis
    );
    assert_eq!(planned.packet_set().truth_view_packets().len(), 1);
    assert!(
        planned
            .execution_plan()
            .legality_proof()
            .admitted_partitions()
            .partitions()
            .is_empty()
    );
    assert!(
        planned
            .execution_plan()
            .legality_proof()
            .disjoint_packet_regions()
            .regions()
            .is_empty()
    );
}

#[test]
fn bridge_bulk_execution_plan_rejects_parallel_preparation_for_continuity_remap_workloads() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );
    let lineage_context = BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
        crate::facade::TruthBranchIdentity::new("main"),
        TruthSnapshotIdentity::new("snapshot-a"),
    ));

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a"))
                .with_mapping_context(
                    BridgeMappingContext::default().with_lineage_context(lineage_context.clone()),
                ),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b"))
                .with_mapping_context(
                    BridgeMappingContext::default().with_lineage_context(lineage_context),
                ),
        ]))
        .expect("continuity remap workload should plan");

    assert_eq!(
        planned.execution_plan().selected_mode(),
        BridgePreparationMode::Serial
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::ParallelPreparationRejected
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::ContinuityRemapRequiresSerialPreparation
    );
    assert_eq!(
        planned.execution_plan().legality_decision().class(),
        BridgeParallelLegalityClass::ParallelPreparationIllegal
    );
    assert_eq!(
        planned.execution_plan().legality_decision().reason(),
        BridgeParallelLegalityReason::ContinuityRemapRequiresSerialPreparation
    );
    assert_eq!(
        planned.execution_plan().profitability_decision().class(),
        BridgeParallelProfitabilityClass::NotApplicable
    );
    assert_eq!(
        planned
            .execution_plan()
            .counters()
            .bulk_parallel_preparation_rejected_count(),
        1
    );
    assert_eq!(planned.execution_plan().planning_failures().len(), 1);
    assert_eq!(
        planned.execution_plan().planning_failures()[0].kind(),
        BridgeBulkPlanningFailureKind::InvalidLegalityBasis
    );
    assert!(
        planned
            .execution_plan()
            .legality_proof()
            .admitted_partitions()
            .partitions()
            .is_empty()
    );
    assert!(
        planned
            .execution_plan()
            .legality_proof()
            .disjoint_packet_regions()
            .regions()
            .is_empty()
    );
}

#[test]
fn bridge_continuity_planning_requires_explicit_lineage_context() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(source, sink, vec![registration()]);

    let result = runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("route should plan"),
        )
        .expect("delivery should succeed");
    let route_record = runtime
        .diagnostics()
        .route_record_for_route_identity(result.result_summary().route_identity().as_str())
        .expect("route record should be retained");

    let error = runtime
        .plan_continuity_requests(&route_record)
        .expect_err("continuity planning should reject missing lineage context");

    assert_eq!(
        error.kind(),
        crate::error::BridgeContinuityErrorKind::MissingLineageContext
    );
}

#[test]
fn bridge_historical_lineage_packet_uses_planned_continuity_requests() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let builder = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration());
    let runtime = builder.build().expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(
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

    assert_eq!(requests.requests().len(), 1);
    assert_eq!(requests.authority_basis().branch_identity().as_str(), "main");
    assert_eq!(packet.entries().len(), 1);
    assert_eq!(
        packet.entries()[0]
            .lineage_authority()
            .canonical_resolved_lineage_keys()[0]
            .as_ref(),
        "lineage:test-successor"
    );
    assert_eq!(
        packet.entries()[0].prior_slice().slice_kind(),
        SubscriptionSliceKind::SignalField
    );
    assert_eq!(
        packet.entries()[0].prior_slice().match_status(),
        FineGrainedMatchStatus::Matched
    );
}

#[test]
fn bridge_continuity_planning_rejects_branch_mismatch_against_route_truth() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let builder = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration());
    let runtime = builder.build().expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(
                BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("analysis"),
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

    let error = runtime
        .plan_continuity_requests(&route_record)
        .expect_err("continuity planning should reject branch mismatch");

    assert_eq!(
        error.kind(),
        crate::error::BridgeContinuityErrorKind::LineageAuthorityMismatch
    );
}

#[test]
fn bridge_historical_lineage_packet_rejects_mismatched_returned_authority_basis() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let builder = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestMismatchedAuthorityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration());
    let runtime = builder.build().expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(
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
    let error = runtime
        .plan_historical_lineage_packet(&requests)
        .expect_err("mismatched lineage authority should be rejected");

    assert_eq!(
        error.kind(),
        crate::error::BridgeContinuityErrorKind::LineageAuthorityMismatch
    );
}

#[test]
fn bridge_historical_lineage_packet_preserves_typed_unsupported_class_failure() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let builder = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestUnsupportedContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration());
    let runtime = builder.build().expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(
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
    let error = runtime
        .plan_historical_lineage_packet(&requests)
        .expect_err("unsupported continuity class should stay typed");

    assert_eq!(
        error.kind(),
        crate::error::BridgeContinuityErrorKind::UnsupportedContinuityClass
    );
}

#[test]
fn bridge_resolved_lineage_continuity_lowers_single_successor_artifact() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(
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

    assert_eq!(resolved.continuity_entries().len(), 1);
    assert_eq!(
        resolved.continuity_entries()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor
    );
    assert_eq!(artifact.remapped_slices().len(), 1);
    assert_eq!(
        artifact.remapped_slices().slices()[0].entity_identity(),
        "entity:0:4:2"
    );
    assert_eq!(
        artifact.remapped_slices().slices()[0].aspect_label(),
        "profile"
    );
    assert_eq!(artifact.remapped_slices().slices()[0].surface_label(), "name");
}

#[test]
fn bridge_resolved_lineage_continuity_lowers_split_successor_artifact() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestSplitContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(
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

    assert_eq!(
        resolved.continuity_entries()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors
    );
    assert_eq!(artifact.remapped_slices().len(), 2);
    assert_eq!(
        artifact
            .remapped_slices()
            .slices()
            .iter()
            .map(|slice| slice.entity_identity())
            .collect::<Vec<_>>(),
        vec!["entity:0:4:2", "entity:0:5:2"]
    );
}

#[test]
fn bridge_resolved_lineage_continuity_lowers_merge_like_successor_artifact() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestMergeLikeContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(
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

    assert_eq!(
        resolved.continuity_entries()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
    );
    assert_eq!(artifact.remapped_slices().len(), 1);
    assert_eq!(artifact.remapped_slices().slices()[0].entity_identity(), "entity:0:9:3");
}

#[test]
fn bridge_resolved_lineage_continuity_rejects_ambiguous_successor_sets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestAmbiguousContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(
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

    assert_eq!(
        resolved.continuity_entries()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::RejectedAmbiguousSuccessor
    );
    assert_eq!(artifact.remapped_slices().len(), 0);
}

#[test]
fn bridge_resolved_lineage_continuity_rejects_no_authoritative_successor() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestNoAuthoritativeSuccessorLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(
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

    assert_eq!(
        resolved.continuity_entries()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor
    );
    assert_eq!(artifact.remapped_slices().len(), 0);
}
