use forge_harness::facade::{
    certification_matrix, parity_suite, ExecutionProfile, ExecutionRequest, HarnessAdapter,
    HarnessRunner, MutationBatch, ReplayHarnessAdapter, ReplayRequest, ScenarioPlan,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::facade::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeDeliveryErrorKind,
    BridgeMappingContext, BridgeMappingId, BridgeMappingRegistration, BridgeRouteErrorKind,
    CoarseRoutingMode, FineGrainedMatchStatus, MappingSelector, RawCommittedPatchEnvelope,
    RuntimeBridgeBuilder, SignalBridgeSinkError, InvalidationSink, SignalInvalidationScope, SliceFallbackPolicy, SnapshotReadRecord, SnapshotReaderPool,
    SubscriptionSliceKind, TruthBranchIdentity, TruthCommitIdentity, TruthDeltaSurfaceKind,
    TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity, BridgeProducerMetadata,
};

use super::adapter::{BridgeHarnessAdapter, BridgeHarnessMutation};
use super::fixtures::{
    BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink,
    SnapshotFixture,
};

fn registration() -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new("profile-name"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        SignalInvalidationScope::new("signal.profile"),
        CoarseRoutingMode::Direct,
    )
}

fn surface_fallback_registration() -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new("profile-surface-fallback"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::any(),
        ),
        SignalInvalidationScope::new("signal.profile.fallback"),
        CoarseRoutingMode::Direct,
    )
}

fn committed_patch(
    commit: &str,
    patch: &str,
    snapshot: &str,
    surface: &str,
) -> RawCommittedPatchEnvelope {
    RawCommittedPatchEnvelope::new_with_metadata(
        BridgeProducerMetadata::bridge_harness_fixture(),
        TruthCommitIdentity::new(commit),
        TruthPatchIdentity::new(patch),
        TruthSnapshotIdentity::new(snapshot),
        TruthBranchIdentity::new("main"),
        vec![crate::facade::BridgeCommittedPatchItem::new(
            "user", "profile", surface,
        )],
    )
}

fn committed_patch_items(
    commit: &str,
    patch: &str,
    snapshot: &str,
    items: Vec<crate::facade::BridgeCommittedPatchItem>,
) -> RawCommittedPatchEnvelope {
    RawCommittedPatchEnvelope::new_with_metadata(
        BridgeProducerMetadata::bridge_harness_fixture(),
        TruthCommitIdentity::new(commit),
        TruthPatchIdentity::new(patch),
        TruthSnapshotIdentity::new(snapshot),
        TruthBranchIdentity::new("main"),
        items,
    )
}

fn snapshot(snapshot: &str, value: &str) -> SnapshotFixture {
    SnapshotFixture::new(
        TruthSnapshotIdentity::new(snapshot),
        vec![SnapshotReadRecord::new("user:profile", value.as_bytes().to_vec())],
    )
}

fn field_slice_snapshot(snapshot: &str, value: &str) -> SnapshotFixture {
    SnapshotFixture::new(
        TruthSnapshotIdentity::new(snapshot),
        vec![SnapshotReadRecord::new(
            "user:profile:signal-field:name",
            value.as_bytes().to_vec(),
        )],
    )
}

fn build_runtime<S>(
    source: InMemoryRelationalBridgeSource,
    sink: S,
    mappings: Vec<BridgeMappingRegistration>,
) -> crate::facade::RuntimeBridge
where
    S: InvalidationSink,
{
    build_runtime_with_aspects(source, sink, mappings, vec![])
}

fn build_runtime_with_aspects<S>(
    source: InMemoryRelationalBridgeSource,
    sink: S,
    mappings: Vec<BridgeMappingRegistration>,
    aspect_mappings: Vec<BridgeAspectRegistration>,
) -> crate::facade::RuntimeBridge
where
    S: InvalidationSink,
{
    let builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink);
    let mut mappings = mappings.into_iter();
    let first_mapping = mappings
        .next()
        .expect("bridge harness tests require at least one mapping");
    let mut builder = builder.register_mapping(first_mapping);
    for mapping in mappings {
        builder = builder.register_mapping(mapping);
    }
    for aspect_mapping in aspect_mappings {
        builder = builder.register_aspect_mapping(aspect_mapping);
    }
    builder
        .build()
        .expect("bridge runtime should build for harness tests")
}

#[derive(Debug, Clone, Default)]
struct RejectingSignalSink;

impl InvalidationSink for RejectingSignalSink {
    fn deliver_invalidation(
        &self,
        _delivery: crate::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<crate::facade::BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Err(SignalBridgeSinkError::new("forced sink rejection"))
    }
}

#[derive(Clone)]
struct CountingSnapshotReaderPool {
    source: InMemoryRelationalBridgeSource,
    acquire_count: Arc<AtomicUsize>,
    release_count: Arc<AtomicUsize>,
}

impl CountingSnapshotReaderPool {
    fn new(source: InMemoryRelationalBridgeSource) -> Self {
        Self {
            source,
            acquire_count: Arc::new(AtomicUsize::new(0)),
            release_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn acquire_count(&self) -> usize {
        self.acquire_count.load(Ordering::SeqCst)
    }

    fn release_count(&self) -> usize {
        self.release_count.load(Ordering::SeqCst)
    }
}

impl SnapshotReaderPool for CountingSnapshotReaderPool {
    fn acquire(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn crate::facade::TruthSnapshotReader>, crate::facade::RelationalBridgeSourceError> {
        self.acquire_count.fetch_add(1, Ordering::SeqCst);
        crate::facade::SnapshotReadSource::open_snapshot(&self.source, identity)
    }

    fn release(&self, _reader: Box<dyn crate::facade::TruthSnapshotReader>) {
        self.release_count.fetch_add(1, Ordering::SeqCst);
    }
}

fn field_aspect_registration() -> BridgeAspectRegistration {
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::new("profile-name-field"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceFallbackPolicy::Disallow,
    )
}

fn field_aspect_registration_with_kind(
    registration_id: &str,
    surface_kind: TruthDeltaSurfaceKind,
    slice_kind: SubscriptionSliceKind,
) -> BridgeAspectRegistration {
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::new(registration_id),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        surface_kind,
        slice_kind,
        SliceFallbackPolicy::Disallow,
    )
}

#[test]
fn bridge_harness_parity_proves_routing_truth_is_invariant_across_diagnostics_tiers() {
    let fixture = ScenarioPlan::new(
        "bridge-parity",
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([
        ExecutionProfile::operational("operational"),
        ExecutionProfile::forensic("forensic"),
    ])
    .compare()
    .expect("bridge parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
}

#[test]
fn bridge_harness_parity_proves_fine_grained_slice_truth_is_invariant_across_diagnostics_tiers() {
    let fixture = ScenarioPlan::new(
        "bridge-fine-grained-parity",
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_aspect_mapping(field_aspect_registration())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([
        ExecutionProfile::operational("operational"),
        ExecutionProfile::forensic("forensic"),
    ])
    .compare()
    .expect("fine-grained bridge parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
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
                .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
                .expect("one-shot route should plan"),
        )
        .expect("one-shot delivery should succeed");
    let prepared = right_runtime.prepare_delivery(
        right_runtime
            .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
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
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("default planning should succeed");
    let explicit_route = right_runtime
        .plan_committed_patch_with_mapping_context(
            crate::facade::BridgeRouteRequest::for_commit("commit-a"),
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
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("unprefixed field route should plan");
    let right_route = right_runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("prefixed field route should plan");

    assert_eq!(left_route.route_identity(), right_route.route_identity());
    assert_eq!(left_route.read_packet(), right_route.read_packet());
    assert_eq!(
        left_route.lowering_summary().subscription_slice_identity(),
        right_route.lowering_summary().subscription_slice_identity()
    );
}

#[test]
fn bridge_snapshot_delivery_remains_stable_after_newer_truth_arrives() {
    let runner = HarnessRunner::new(BridgeHarnessAdapter);
    let fixture = ScenarioPlan::new(
        "bridge-snapshot-stability",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let mutation = MutationBatch::new("publish-newer-truth")
        .push(BridgeHarnessMutation::PublishCommittedPatch(committed_patch(
            "commit-b",
            "patch-b",
            "snapshot-b",
            "name",
        )))
        .push(BridgeHarnessMutation::PublishSnapshot(snapshot("snapshot-b", "bob")));
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let profile = ExecutionProfile::development("development");

    let bundle = runner
        .execute_core(&fixture, Some(&mutation), &request, &profile)
        .expect("bridge snapshot-stability execution should succeed");

    assert_eq!(bundle.run.summary["snapshot_identity"], "snapshot-a");
}

#[test]
fn bridge_delivery_keeps_preplanned_snapshot_after_newer_truth_arrives_during_delivery() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(
        source.clone(),
        sink.clone(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan from the original committed artifact");

    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));

    let result = runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver the preplanned route against its original snapshot");

    assert_eq!(result.result_summary().snapshot_identity().as_str(), "snapshot-a");
    assert_eq!(result.receipt().snapshot_identity().as_str(), "snapshot-a");
    let delivered = sink
        .last_delivery()
        .expect("bridge sink should record the delivered artifact");
    assert_eq!(delivered.delivery.source_snapshot().as_str(), "snapshot-a");
}

#[test]
fn bridge_prepares_signal_evaluation_with_snapshot_context_without_sink_delivery() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(source, sink.clone(), vec![registration()]);

    let route = runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan before preparing signal evaluation");

    let evaluation = runtime
        .prepare_signal_evaluation(route)
        .expect("bridge should prepare signal evaluation with a snapshot context");

    assert_eq!(evaluation.artifact().source_snapshot().as_str(), "snapshot-a");
    assert_eq!(evaluation.snapshot().snapshot_identity().as_str(), "snapshot-a");
    let reads = evaluation
        .snapshot()
        .read_packet(evaluation.read_packet())
        .expect("bridge evaluation request should carry a usable read packet");
    assert_eq!(reads.snapshot_identity().as_str(), "snapshot-a");
    assert_eq!(reads.records().len(), 1);
    assert!(sink.deliveries().is_empty());
}

#[test]
fn bridge_prepared_signal_evaluation_keeps_preplanned_snapshot_after_newer_truth_arrives() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source.clone(),
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan from the original committed artifact");

    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));

    let evaluation = runtime
        .prepare_signal_evaluation(route)
        .expect("bridge should prepare evaluation against the original snapshot");

    assert_eq!(evaluation.artifact().source_snapshot().as_str(), "snapshot-a");
    assert_eq!(evaluation.snapshot().snapshot_identity().as_str(), "snapshot-a");
}

#[test]
fn bridge_snapshot_identity_mismatch_fails_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-snapshot-mismatch",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(
                snapshot("snapshot-a", "alice")
                    .with_read_result_identity(TruthSnapshotIdentity::new("snapshot-bad")),
            ),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let error = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect_err("bridge execution should fail on snapshot identity mismatch");

    assert!(format!("{error}")
        .to_ascii_lowercase()
        .contains("snapshot"));
    let failure_record = session
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_failure_record()
        .expect("bridge failure record");
    assert!(failure_record.detail().contains("Snapshot read returned"));
    assert_eq!(failure_record.counters().snapshot_identity_mismatch_count(), 1);
}

#[test]
fn bridge_snapshot_contract_rejects_missing_required_reads() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(SnapshotFixture::new(
        TruthSnapshotIdentity::new("snapshot-a"),
        vec![],
    ));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan before validating snapshot reads");

    let error = runtime
        .deliver_invalidation(route)
        .expect_err("bridge should reject incomplete snapshot read results");

    assert_eq!(error.kind(), BridgeDeliveryErrorKind::SnapshotReadContractViolation);
    assert!(error.to_string().contains("returned 0 records"));
}

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
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
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
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect_err("bridge planning should reject unmapped committed patch surfaces");

    assert_eq!(error.kind(), BridgeRouteErrorKind::MissingMappingRegistration);
    assert!(error.to_string().contains("No bridge mapping registration matched"));
}

#[test]
fn bridge_certification_matrix_reports_diagnostics_for_candidate_profiles() {
    let fixture = ScenarioPlan::new(
        "bridge-certification",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("bridge certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn bridge_replay_capture_exposes_last_route_record() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-replay",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("bridge harness execute");
    let replay = adapter
        .capture_replay(
            &session,
            &fixture,
            &ReplayRequest {
                name: "replay".to_string(),
                source_run: run,
                request: request.clone(),
                profile: profile.clone(),
            },
        )
        .expect("bridge replay capture should succeed");

    assert_eq!(replay.summary["source_commit"], "commit-a");
    assert_eq!(replay.summary["source_snapshot"], "snapshot-a");
}

#[test]
fn bridge_replay_accepts_versioned_canonical_route_record() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan route before canonical replay capture");
    runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver route before canonical replay capture");
    let canonical_record = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("bridge should expose a versioned canonical route record");

    let replay = runtime
        .replay_canonical_record(&canonical_record)
        .expect("bridge should replay a supported canonical route record");

    assert_eq!(replay.source_commit().as_str(), "commit-a");
    assert_eq!(
        canonical_record.schema_version(),
        crate::facade::BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V2
    );
}

#[test]
fn bridge_replay_preserves_canonical_route_outcome_for_delivered_patch() {
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
                .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
                .expect("route should plan before replay parity certification"),
        )
        .expect("route should deliver before replay parity certification");
    let canonical_record = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("bridge should retain a canonical route record for replay parity certification");

    let replay = runtime
        .replay_canonical_record(&canonical_record)
        .expect("bridge replay should preserve the canonical route outcome");

    assert_eq!(
        replay.route_identity(),
        result.result_summary().route_identity()
    );
    assert_eq!(
        replay.invalidation_identity(),
        result.result_summary().invalidation_identity()
    );
    assert_eq!(
        replay.subscription_slice_identity(),
        result.result_summary().subscription_slice_identity()
    );
    assert_eq!(replay.source_commit(), result.result_summary().source_commit());
    assert_eq!(replay.source_patch(), result.result_summary().source_patch());
    assert_eq!(
        replay.source_snapshot(),
        result.result_summary().snapshot_identity()
    );
}

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
            .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit(commit))
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
            .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit(commit))
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
fn bridge_replay_rejects_subscription_slice_drift() {
    let original_source = InMemoryRelationalBridgeSource::default();
    original_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    original_source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let original_runtime = build_runtime_with_aspects(
        original_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let route = original_runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("original route should plan before replay certification");
    original_runtime
        .deliver_invalidation(route)
        .expect("original route should deliver before replay certification");
    let canonical_record = original_runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("original runtime should expose a canonical route record");

    let restarted_source = InMemoryRelationalBridgeSource::default();
    restarted_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    restarted_source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let restarted_runtime = build_runtime_with_aspects(
        restarted_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration_with_kind(
            "profile-name-region",
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalRegion,
        )],
    );

    let error = restarted_runtime
        .replay_canonical_record(&canonical_record)
        .expect_err("replay should reject subscription slice identity drift");

    assert_eq!(
        error.kind(),
        crate::facade::BridgeReplayErrorKind::SubscriptionSliceMismatch
    );
    assert!(error.to_string().contains("subscription slices"));
}

#[test]
fn bridge_replay_rejects_incompatible_canonical_route_record_version() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan route before canonical replay failure test");
    runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver route before canonical replay failure test");
    let canonical_record = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("bridge should expose a canonical route record")
        .with_schema_version_for_test("forge-runtime-bridge.route-record.v999");

    let error = runtime
        .replay_canonical_record(&canonical_record)
        .expect_err("bridge should reject unsupported canonical route record versions");

    assert_eq!(
        error.kind(),
        crate::facade::BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure
    );
    assert!(error.to_string().contains("not supported"));
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
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
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
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
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
                .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
                .expect("left route should plan"),
        )
        .expect("left route should deliver");
    let right_result = right_runtime
        .deliver_invalidation(
            right_runtime
                .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
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
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
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

#[test]
fn bridge_delivery_and_result_surfaces_expose_planning_and_lowering_proof_contracts() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime_with_aspects(
        source,
        sink.clone(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let route = runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan route with proof metadata");
    let result = runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver route with proof metadata");
    let delivery = sink.last_delivery().expect("recorded sink delivery");

    assert_eq!(
        result.result_summary().producer_metadata().authority_kind(),
        crate::facade::BridgeProducerAuthorityKind::BridgeHarnessFixture
    );
    assert!(result
        .result_summary()
        .planning_provenance_digest()
        .starts_with("planning-provenance:sha256:"));
    assert!(result
        .result_summary()
        .planning_summary_digest()
        .starts_with("planning-summary:sha256:"));
    assert!(result
        .result_summary()
        .lowering_provenance_digest()
        .starts_with("lowering-provenance:sha256:"));
    assert!(result
        .result_summary()
        .lowering_summary_digest()
        .starts_with("lowering-summary:sha256:"));
    assert_eq!(
        result.result_summary().planning_provenance_digest(),
        delivery.delivery.planning_provenance_digest()
    );
    assert_eq!(
        result.result_summary().lowering_summary_digest(),
        delivery.delivery.lowering_summary_digest()
    );
    assert_eq!(
        result.result_summary().mapping_context_digest(),
        delivery.delivery.mapping_context_digest()
    );
    assert_eq!(
        delivery.delivery.producer_metadata().authority_kind(),
        crate::facade::BridgeProducerAuthorityKind::BridgeHarnessFixture
    );
}

#[test]
fn bridge_snapshot_reader_pool_is_used_when_configured() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let pool = CountingSnapshotReaderPool::new(source.clone());
    let runtime = RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_snapshot_reader_pool(pool.clone())
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .register_mapping(registration())
        .build()
        .expect("bridge runtime should build with a snapshot reader pool");

    let route = runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("route should plan when the pool is configured");
    runtime
        .deliver_invalidation(route)
        .expect("delivery should succeed through the pooled snapshot reader");

    assert_eq!(pool.acquire_count(), 1);
    assert_eq!(pool.release_count(), 1);
}

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
                .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
                .expect("route should plan before digest budget capture"),
        )
        .expect("delivery should succeed before digest budget capture");

    assert!(result.counters().digest_computation_count() >= 8);
    assert!(result.counters().digest_input_bytes() > 0);
}

#[test]
fn bridge_sink_rejection_records_failure_diagnostics_with_slice_identity() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = build_runtime_with_aspects(
        source,
        RejectingSignalSink,
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let route = runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("route should plan before sink rejection");
    let expected_slice_identity = route
        .lowering_summary()
        .subscription_slice_identity()
        .clone();

    let error = runtime
        .deliver_invalidation(route)
        .expect_err("delivery should surface the sink rejection");

    assert_eq!(error.kind(), BridgeDeliveryErrorKind::SignalSinkRejection);
    let failure = runtime
        .diagnostics()
        .last_failure_record()
        .expect("sink rejection should be recorded in diagnostics");
    assert_eq!(
        failure.subscription_slice_identity().map(|id| id.as_str()),
        Some(expected_slice_identity.as_str())
    );
    assert!(failure.invalidation_identity().is_some());
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
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan canonical route identity");
    let right_route = right_runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
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

#[test]
fn bridge_replay_detects_route_drift_after_restart_shaped_truth_change() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-replay-restart-drift",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let profile = ExecutionProfile::development("development");

    let mut original = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut original, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut original, &fixture)
        .expect("bridge harness load fixture");
    adapter
        .execute(&mut original, &fixture, &request, &profile)
        .expect("bridge harness execute");
    let original_record = original
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_canonical_route_record()
        .expect("original canonical bridge route record");

    let drifted_fixture = ScenarioPlan::new(
        "bridge-replay-restart-drift-rehydrated",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_items(
                "commit-a",
                "patch-a",
                "snapshot-a",
                vec![
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "avatar"),
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                ],
            ))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let mut restarted = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut restarted, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut restarted, &drifted_fixture)
        .expect("bridge harness load fixture");
    let error = restarted
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .replay_canonical_record(&original_record)
        .expect_err("bridge replay should reject route drift after restart");

    assert!(!error.to_string().is_empty());
    let failure_record = restarted
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_failure_record()
        .expect("bridge replay failure record");
    assert_eq!(failure_record.counters().route_replay_mismatch_count(), 1);
}

#[test]
fn bridge_delivery_fails_when_newer_truth_arrives_without_required_snapshot() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    let runtime = build_runtime(
        source.clone(),
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan before delivery");

    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));

    let error = runtime
        .deliver_invalidation(route)
        .expect_err("delivery should still require the original planned snapshot");

    assert_eq!(error.kind(), BridgeDeliveryErrorKind::SnapshotAcquisitionFailure);
    assert!(error.to_string().contains("snapshot-a"));
}
