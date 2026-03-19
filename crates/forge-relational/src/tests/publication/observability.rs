use crate::facade::diagnostics::RelationalDiagnosticsProfile;
use crate::facade::runtime::HarnessAuditMode;
use crate::tests::support::*;

#[derive(Clone)]
struct InvariantHarnessAdapter {
    invariant_catalog: InvariantCatalog,
}

impl InvariantHarnessAdapter {
    fn new(invariant_catalog: InvariantCatalog) -> Self {
        Self { invariant_catalog }
    }
}

impl forge_harness::facade::HarnessAdapter for InvariantHarnessAdapter {
    type Runtime = crate::facade::runtime::RelationalRuntime;
    type Fixture = crate::presentation::harness::RelationalFixture;
    type Mutation = crate::facade::transactions::WorkerIntentBatch;
    type TargetId = String;
    type Error = crate::facade::harness::RelationalHarnessError;

    fn adapter_name(&self) -> &'static str {
        RelationalHarnessAdapter.adapter_name()
    }

    fn capabilities(&self) -> forge_harness::facade::HarnessCapabilities {
        RelationalHarnessAdapter.capabilities()
    }

    fn create_runtime(&self) -> Result<Self::Runtime, Self::Error> {
        Ok(RelationalRuntimeApi::builder()
            .schema_registry(test_schema_registry())
            .invariant_catalog(self.invariant_catalog.clone())
            .build())
    }

    fn prepare_runtime(
        &self,
        runtime: &mut Self::Runtime,
        profile: &forge_harness::facade::ExecutionProfile,
    ) -> Result<(), Self::Error> {
        RelationalHarnessAdapter.prepare_runtime(runtime, profile)
    }

    fn load_fixture(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &forge_harness::facade::ScenarioFixture<Self::Fixture>,
    ) -> Result<(), Self::Error> {
        RelationalHarnessAdapter.load_fixture(runtime, fixture)
    }

    fn apply_mutation_batch(
        &self,
        runtime: &mut Self::Runtime,
        batch: &forge_harness::facade::MutationBatch<Self::Mutation>,
    ) -> Result<(), Self::Error> {
        RelationalHarnessAdapter.apply_mutation_batch(runtime, batch)
    }

    fn execute(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &forge_harness::facade::ScenarioFixture<Self::Fixture>,
        request: &forge_harness::facade::ExecutionRequest<Self::TargetId>,
        profile: &forge_harness::facade::ExecutionProfile,
    ) -> Result<forge_harness::facade::RunRecord<Self::TargetId>, Self::Error> {
        RelationalHarnessAdapter.execute(runtime, fixture, request, profile)
    }

    fn capture_snapshot(
        &self,
        runtime: &Self::Runtime,
        fixture: &forge_harness::facade::ScenarioFixture<Self::Fixture>,
        request: &forge_harness::facade::ExecutionRequest<Self::TargetId>,
        profile: &forge_harness::facade::ExecutionProfile,
    ) -> Result<forge_harness::facade::SnapshotRecord<Self::TargetId>, Self::Error> {
        RelationalHarnessAdapter.capture_snapshot(runtime, fixture, request, profile)
    }
}

impl forge_harness::facade::DiagnosticsHarnessAdapter for InvariantHarnessAdapter {
    fn capture_diagnostics(
        &self,
        runtime: &Self::Runtime,
        fixture: &forge_harness::facade::ScenarioFixture<Self::Fixture>,
        profile: &forge_harness::facade::ExecutionProfile,
    ) -> Result<forge_harness::facade::DiagnosticsRecord, Self::Error> {
        RelationalHarnessAdapter.capture_diagnostics(runtime, fixture, profile)
    }
}

#[test]
fn diagnostics_and_replay_are_emitted_for_commit() {
    let mut runtime = runtime_with_test_schema();
    let _entity = create_entity(&mut runtime, "first");
    let diagnostics = runtime.publication_access().diagnostics();

    assert!(diagnostics.artifacts().iter().any(|artifact| {
        artifact.scope == DiagnosticsScope::Transaction
            && artifact.kind == DiagnosticsArtifactKind::MinimalSummary
    }));
    assert!(diagnostics
        .minimal_summaries()
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::EntityCreated));
    assert!(runtime.publication_access().latest_patch().is_some());
    assert!(runtime.publication_access().latest_replay().is_some());
    assert_eq!(
        runtime
            .publication_access()
            .latest_replay()
            .unwrap()
            .schema_registry,
        test_schema_registry()
    );
}

#[test]
fn publication_bundle_is_the_single_visible_commit_surface() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "first");
    let publication = runtime.publication_access();
    let bundle = publication.latest_bundle().unwrap();

    assert_eq!(outcome.publication_status, PublicationStatus::Published);
    assert_eq!(bundle.snapshot, outcome.snapshot);
    assert_eq!(bundle.commit, outcome.commit);
    assert_eq!(
        bundle.commit,
        *runtime.history_access().latest_commit().unwrap()
    );
    assert_eq!(
        bundle.patch,
        *runtime.publication_access().latest_patch().unwrap()
    );
    assert_eq!(
        bundle.replay,
        *runtime.publication_access().latest_replay().unwrap()
    );
}

#[test]
fn publication_snapshot_handle_reads_without_becoming_a_pinned_snapshot() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "first");

    let retention = runtime.retention_access().inspect_plan();
    let read = runtime
        .visibility_reads()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let inspection = runtime
        .visibility_reads()
        .inspect_snapshot(&outcome.snapshot)
        .unwrap();
    let packet = QueryWorkPacket::bulk(
        "entities",
        vec![RecordRef::Entity(changed_entities(&outcome)[0])],
    );

    assert_eq!(retention.active_snapshot_count, 0);
    assert_eq!(retention.snapshot_pinned_entities, 0);
    assert_eq!(retention.snapshot_pinned_relations, 0);
    assert_eq!(read.entities.len(), 1);
    assert_eq!(inspection.pinned_entity_count, 0);
    assert_eq!(inspection.entity_count, 1);
    assert!(runtime
        .storage_access()
        .plan_read_packet(&outcome.snapshot, &packet)
        .is_some());
    assert!(runtime
        .visibility_reads()
        .execute_read_packet(&outcome.snapshot, &packet)
        .is_some());
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&outcome.snapshot));
    assert!(runtime
        .visibility_reads()
        .read_snapshot(&outcome.snapshot)
        .is_none());
}

#[test]
fn released_publication_handles_stop_counting_as_readable_runtime_state() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");

    let before = runtime.storage_access().storage_stats();
    assert_eq!(before.published_snapshot_handle_count, 2);

    assert!(runtime
        .visibility_authority()
        .release_snapshot(&first.snapshot));
    let after_first_release = runtime.storage_access().storage_stats();
    assert_eq!(after_first_release.published_snapshot_handle_count, 1);
    assert!(runtime
        .visibility_reads()
        .read_snapshot(&first.snapshot)
        .is_none());
    assert!(runtime
        .visibility_reads()
        .read_snapshot(&second.snapshot)
        .is_some());

    assert!(runtime
        .visibility_authority()
        .release_snapshot(&second.snapshot));
    let after_second_release = runtime.storage_access().storage_stats();
    assert_eq!(after_second_release.published_snapshot_handle_count, 0);
}

#[test]
fn publication_handle_retention_is_bounded_by_policy() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .publication(PublicationConfig {
            coherent_publication_required: true,
            max_patch_records_per_commit: 4096,
            max_published_snapshot_handles: 2,
            patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
        })
        .build();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let third = create_entity_outcome(&mut runtime, "third");

    let stats = runtime.storage_access().storage_stats();

    assert_eq!(stats.published_snapshot_handle_count, 2);
    assert!(runtime
        .visibility_reads()
        .read_snapshot(&first.snapshot)
        .is_none());
    assert!(runtime
        .visibility_reads()
        .read_snapshot(&second.snapshot)
        .is_some());
    assert!(runtime
        .visibility_reads()
        .read_snapshot(&third.snapshot)
        .is_some());
}

#[test]
fn parallel_post_commit_consumption_preserves_publication_surfaces() {
    let mut serial = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .publication(PublicationConfig {
            coherent_publication_required: true,
            max_patch_records_per_commit: 4096,
            max_published_snapshot_handles: 2,
            patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
        })
        .execution_model(crate::facade::runtime::RelationalExecutionModel::SerialAuthority)
        .build();
    let mut parallel = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .publication(PublicationConfig {
            coherent_publication_required: true,
            max_patch_records_per_commit: 4096,
            max_published_snapshot_handles: 2,
            patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
        })
        .execution_model(
            crate::facade::runtime::RelationalExecutionModel::ParallelPostCommitConsumption,
        )
        .build();

    let _ = create_entity_outcome(&mut serial, "first");
    let _serial_second = create_entity_outcome(&mut serial, "second");
    let _serial_third = create_entity_outcome(&mut serial, "third");

    parallel.performance_access().reset_counters();
    let _ = create_entity_outcome(&mut parallel, "first");
    let parallel_second = create_entity_outcome(&mut parallel, "second");
    let parallel_third = create_entity_outcome(&mut parallel, "third");

    let serial_bundle = serial.publication_access().latest_bundle().unwrap().clone();
    let parallel_bundle = parallel
        .publication_access()
        .latest_bundle()
        .unwrap()
        .clone();
    let parallel_stats = parallel.storage_access().storage_stats();
    let diagnostics = parallel.publication_access().diagnostics();

    assert_eq!(parallel_bundle.commit, serial_bundle.commit);
    assert_eq!(parallel_bundle.patch, serial_bundle.patch);
    assert_eq!(parallel_bundle.replay, serial_bundle.replay);
    assert_eq!(parallel_bundle.snapshot, parallel_third.snapshot);
    assert_eq!(parallel_stats.published_snapshot_handle_count, 2);
    assert!(parallel
        .visibility_reads()
        .read_snapshot(&parallel_second.snapshot)
        .is_some());
    assert!(parallel
        .visibility_reads()
        .read_snapshot(&parallel_third.snapshot)
        .is_some());
    assert!(diagnostics
        .minimal_summaries()
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::CommitPublished));
    assert_eq!(
        parallel
            .performance_access()
            .counters()
            .post_commit_consumer_packet_count,
        6
    );
    assert_eq!(
        parallel
            .performance_access()
            .counters()
            .post_commit_parallel_strategy_count,
        3
    );
}

#[test]
fn aspect_traces_and_diagnostics_are_stable_across_supported_execution_models() {
    let serial_diagnostics = RelationalDiagnosticsProfile {
        detailed_traces_enabled: true,
        ..RelationalDiagnosticsProfile::default()
    };
    let mut serial = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(declared_aspect_schema_registry(
            CascadeDeletePolicy::CascadeDeleteRelations,
        ))
        .diagnostics(serial_diagnostics.clone())
        .execution_model(crate::facade::runtime::RelationalExecutionModel::SerialAuthority)
        .build();
    let mut staged = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(declared_aspect_schema_registry(
            CascadeDeletePolicy::CascadeDeleteRelations,
        ))
        .diagnostics(serial_diagnostics.clone())
        .execution_model(
            crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
        )
        .build();
    let mut post_commit = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(declared_aspect_schema_registry(
            CascadeDeletePolicy::CascadeDeleteRelations,
        ))
        .diagnostics(serial_diagnostics)
        .execution_model(
            crate::facade::runtime::RelationalExecutionModel::ParallelPostCommitConsumption,
        )
        .build();

    let serial_outcome = create_entity_outcome(&mut serial, "trace-stable");
    let staged_outcome = create_entity_outcome(&mut staged, "trace-stable");
    let post_commit_outcome = create_entity_outcome(&mut post_commit, "trace-stable");

    assert_eq!(
        serial_outcome.aspect_evaluation_traces(),
        staged_outcome.aspect_evaluation_traces()
    );
    assert_eq!(
        serial_outcome.aspect_evaluation_traces(),
        post_commit_outcome.aspect_evaluation_traces()
    );
    assert_eq!(
        serial_outcome.aspect_emission_traces(),
        staged_outcome.aspect_emission_traces()
    );
    assert_eq!(
        serial_outcome.aspect_emission_traces(),
        post_commit_outcome.aspect_emission_traces()
    );
    assert_eq!(serial_outcome.patch(), staged_outcome.patch());
    assert_eq!(serial_outcome.patch(), post_commit_outcome.patch());
    assert_eq!(
        aspect_relevant_diagnostics(serial_outcome.diagnostics()),
        aspect_relevant_diagnostics(staged_outcome.diagnostics())
    );
    assert_eq!(
        aspect_relevant_diagnostics(serial_outcome.diagnostics()),
        aspect_relevant_diagnostics(post_commit_outcome.diagnostics())
    );
    assert_patch_truth_invariants(&serial_outcome);
    assert_patch_truth_invariants(&staged_outcome);
    assert_patch_truth_invariants(&post_commit_outcome);
}

fn aspect_relevant_diagnostics(
    diagnostics: &[crate::facade::diagnostics::RelationalDiagnosticArtifact],
) -> Vec<crate::facade::diagnostics::RelationalDiagnosticArtifact> {
    diagnostics
        .iter()
        .filter(|artifact| {
            artifact.entries.iter().any(|entry| {
                matches!(
                    entry.code,
                    DiagnosticCode::AspectEvaluationTraced
                        | DiagnosticCode::AspectEmissionTraced
                        | DiagnosticCode::EntityCreated
                        | DiagnosticCode::CommitPublished
                )
            })
        })
        .cloned()
        .collect()
}

#[test]
fn snapshot_audit_failure_blocks_publication() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::snapshot_publication_blocking(
            InvariantRule::MaxSnapshotEntities(0),
        )],
        ..InvariantCatalog::default()
    });
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("blocked"));
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Publication { error: ref publication, .. }
            if publication.stage == PublicationStage::InvariantCheck
    ));
    assert!(runtime.publication_access().latest_bundle().is_none());
}

#[test]
fn bulk_packets_are_the_primary_read_surface() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    let snapshot = runtime.visibility_authority().snapshot();
    let plan = runtime
        .storage_access()
        .plan_read_packet(
            &snapshot,
            &QueryWorkPacket::bulk("entities", vec![RecordRef::Entity(entity)]),
        )
        .unwrap();
    let result = runtime
        .visibility_reads()
        .execute_read_packet(
            &snapshot,
            &QueryWorkPacket::bulk("entities", vec![RecordRef::Entity(entity)]),
        )
        .unwrap();

    assert_eq!(plan.entity_chunk_indexes, vec![0]);
    assert_eq!(result.entities.len(), 1);
}

#[test]
fn harness_runner_captures_snapshot_diagnostics_and_replay() {
    let adapter = RelationalHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "fixture",
        crate::presentation::harness::RelationalFixture {
            entities: Vec::new(),
            relations: Vec::new(),
        },
    )
    .compile();
    let batch = MutationBatch::new("mutate").push(batch_create("from-harness"));
    let request = ExecutionRequest::target("inspect", "entity:0:1".to_string());
    let profile = ExecutionProfile::forensic("forensic");
    let mut runtime = adapter.create_runtime().unwrap();
    adapter.load_fixture(&mut runtime, &fixture).unwrap();
    adapter.apply_mutation_batch(&mut runtime, &batch).unwrap();
    let run = adapter
        .execute(&mut runtime, &fixture, &request, &profile)
        .unwrap();
    let snapshot = adapter
        .capture_snapshot(&runtime, &fixture, &request, &profile)
        .unwrap();
    let diagnostics = adapter
        .capture_diagnostics(&runtime, &fixture, &profile)
        .unwrap();
    let replay_request = ReplayRequest {
        name: "replay".to_string(),
        source_run: run.clone(),
        request: request.clone(),
        profile: profile.clone(),
    };
    let replay = adapter
        .capture_replay(&runtime, &fixture, &replay_request)
        .unwrap();

    assert_eq!(snapshot.observations.len(), 1);
    assert!(diagnostics.summary.is_object());
    assert!(replay.summary.is_object());
}

#[test]
fn runtime_packet_execution_and_storage_stats_are_readable() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    let snapshot = runtime.visibility_authority().snapshot();
    let packet = QueryWorkPacket::bulk("entities", vec![RecordRef::Entity(entity)]);
    let result = runtime
        .visibility_reads()
        .execute_read_packet(&snapshot, &packet)
        .unwrap();
    let stats = runtime.storage_access().storage_stats();

    assert_eq!(result.entities.len(), 1);
    assert_eq!(stats.live_entities, 1);
    assert!(stats.snapshot_count >= 1);
    assert!(stats.entity_chunks >= 1);
}

#[test]
fn repeated_serial_runs_are_harness_comparable() {
    let adapter = RelationalHarnessAdapter;
    let runner = forge_harness::facade::HarnessRunner::new(adapter);
    let fixture = ScenarioPlan::new(
        "fixture",
        crate::presentation::harness::RelationalFixture {
            entities: Vec::new(),
            relations: Vec::new(),
        },
    )
    .compile();
    let batch = MutationBatch::new("mutate").push(batch_create("stable"));
    let request = ExecutionRequest::target("inspect", "entity:0:1".to_string());
    let profile = ExecutionProfile::forensic("forensic");
    let run_a = runner
        .execute_core(&fixture, Some(&batch), &request, &profile)
        .unwrap();
    let run_b = runner
        .execute_core(&fixture, Some(&batch), &request, &profile)
        .unwrap();
    let comparison = runner
        .compare_runs(
            &run_a.run,
            &run_b.run,
            &forge_harness::facade::ComparisonProfile::default(),
        )
        .unwrap();

    assert!(comparison.mismatches.is_empty());
}

fn harness_phase8_fixture_batch_request() -> (
    forge_harness::facade::ScenarioFixture<crate::presentation::harness::RelationalFixture>,
    MutationBatch<crate::facade::transactions::WorkerIntentBatch>,
    forge_harness::facade::ExecutionRequest<String>,
) {
    let fixture = ScenarioPlan::new(
        "fixture",
        crate::presentation::harness::RelationalFixture {
            entities: Vec::new(),
            relations: Vec::new(),
        },
    )
    .compile();
    let batch = MutationBatch::new("mutate")
        .push(batch_create("alpha"))
        .push(batch_create("beta"));
    let request = ExecutionRequest::target("inspect", "entity:0:1".to_string());
    (fixture, batch, request)
}

fn harness_diagnostic_entries<'a>(
    summary: &'a serde_json::Value,
    code: &str,
) -> Vec<&'a serde_json::Value> {
    summary["artifacts"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|artifact| artifact["entries"].as_array().into_iter().flatten())
        .filter(|entry| entry["code"] == json!(code))
        .collect()
}

fn certification_case<'a>(
    report: &'a forge_harness::facade::CertificationMatrixReport,
    candidate_profile: &str,
) -> &'a forge_harness::facade::CertificationMatrixCase {
    report
        .cases
        .iter()
        .find(|case| case.candidate_profile == candidate_profile)
        .unwrap_or_else(|| panic!("missing certification case for profile {candidate_profile}"))
}

fn profitable_commit_boundary_adapter() -> InvariantHarnessAdapter {
    InvariantHarnessAdapter::new(InvariantCatalog {
        registrations: vec![
            InvariantRegistration::commit_boundary_blocking(InvariantRule::MaxMergedIntents(16)),
            InvariantRegistration::commit_boundary_blocking(
                InvariantRule::UniqueEntityPayloadField("name".to_string()),
            ),
        ],
        ..InvariantCatalog::default()
    })
}

#[test]
fn harness_parity_suite_matches_serial_and_staged_parallel_runs() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = forge_harness::facade::parity_suite(
        RelationalHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidate(ExecutionProfile::staged_parallel("staged"))
    .compare()
    .unwrap();

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);
    assert!(report.results[0].comparison.mismatches.is_empty());
}

#[test]
fn harness_parity_suite_matches_serial_and_post_commit_parallel_runs() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = forge_harness::facade::parity_suite(
        RelationalHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidate(ExecutionProfile::full_parallel("post-commit"))
    .compare()
    .unwrap();

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);
    assert!(report.results[0].comparison.mismatches.is_empty());
}

#[test]
fn harness_phase8_parity_suite_certifies_all_supported_parallel_lanes() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = forge_harness::facade::parity_suite(
        RelationalHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidates([
        ExecutionProfile::staged_parallel("staged"),
        ExecutionProfile::full_parallel("post-commit"),
    ])
    .compare()
    .unwrap();

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
    assert_eq!(report.results[0].baseline_profile, "serial");
    assert_eq!(report.results[0].candidate_profile, "staged");
    assert_eq!(report.results[1].baseline_profile, "serial");
    assert_eq!(report.results[1].candidate_profile, "post-commit");
    assert!(report
        .results
        .iter()
        .all(|result| result.comparison.mismatches.is_empty()));
}

#[test]
fn harness_phase8_certification_matrix_reports_parallel_lane_diagnostics() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = forge_harness::facade::certification_matrix(
        RelationalHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidates([
        ExecutionProfile::staged_parallel("staged"),
        ExecutionProfile::full_parallel("post-commit"),
    ])
    .certify()
    .unwrap();

    assert!(report.matched);
    assert_eq!(report.baseline_profile, "serial");
    assert_eq!(
        report.baseline_diagnostics_summary.as_ref().unwrap()["runtime_execution_model"],
        json!("SerialAuthority")
    );
    assert_eq!(report.cases.len(), 2);
    assert_eq!(report.cases[0].candidate_profile, "staged");
    assert_eq!(report.cases[1].candidate_profile, "post-commit");
    assert_eq!(
        report.cases[0].diagnostics_summary.as_ref().unwrap()["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
    assert_eq!(
        report.cases[1].diagnostics_summary.as_ref().unwrap()["runtime_execution_model"],
        json!("ParallelPostCommitConsumption")
    );
    assert!(report
        .cases
        .iter()
        .all(|case| case.comparison.mismatches.is_empty()));
}

#[test]
fn harness_phase8_certification_matrix_closes_out_supported_runtime_lanes() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = forge_harness::facade::certification_matrix(
        RelationalHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidates([
        ExecutionProfile::staged_parallel("staged"),
        ExecutionProfile::full_parallel("post-commit"),
    ])
    .certify()
    .unwrap();

    assert!(report.matched);
    assert_eq!(report.baseline_profile, "serial");
    assert_eq!(
        report.baseline_diagnostics_summary.as_ref().unwrap()["execution_mode"],
        json!("Serial")
    );
    assert_eq!(
        report.baseline_diagnostics_summary.as_ref().unwrap()["runtime_execution_model"],
        json!("SerialAuthority")
    );

    let staged = certification_case(&report, "staged");
    let post_commit = certification_case(&report, "post-commit");

    assert!(staged.comparison.matched);
    assert!(post_commit.comparison.matched);
    assert_eq!(
        staged.diagnostics_summary.as_ref().unwrap()["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
    assert_eq!(
        post_commit.diagnostics_summary.as_ref().unwrap()["runtime_execution_model"],
        json!("ParallelPostCommitConsumption")
    );
    assert!(
        staged.diagnostics_summary.as_ref().unwrap()["performance_counters"]
            ["preparation_packet_count"]
            .as_u64()
            .is_some()
    );
    assert!(
        post_commit.diagnostics_summary.as_ref().unwrap()["performance_counters"]
            ["post_commit_consumer_packet_count"]
            .as_u64()
            .is_some()
    );
}

#[test]
fn harness_phase8_observed_matrix_exposes_mode_specific_metadata() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let bundles = forge_harness::facade::run_matrix(RelationalHarnessAdapter, fixture, request)
        .mutate(batch)
        .profiles([
            ExecutionProfile::serial("serial"),
            ExecutionProfile::staged_parallel("staged"),
            ExecutionProfile::full_parallel("post-commit"),
        ])
        .diagnose()
        .unwrap();

    assert_eq!(bundles.len(), 3);

    let serial = &bundles[0];
    let staged = &bundles[1];
    let post_commit = &bundles[2];

    assert_eq!(serial.core.run.profile_name, "serial");
    assert_eq!(staged.core.run.profile_name, "staged");
    assert_eq!(post_commit.core.run.profile_name, "post-commit");

    let serial_summary = &serial.diagnostics.as_ref().unwrap().summary;
    let staged_summary = &staged.diagnostics.as_ref().unwrap().summary;
    let post_commit_summary = &post_commit.diagnostics.as_ref().unwrap().summary;

    assert_eq!(serial_summary["execution_mode"], json!("Serial"));
    assert_eq!(
        serial_summary["runtime_execution_model"],
        json!("SerialAuthority")
    );
    assert_eq!(staged_summary["execution_mode"], json!("StagedParallel"));
    assert_eq!(
        staged_summary["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
    assert_eq!(post_commit_summary["execution_mode"], json!("FullParallel"));
    assert_eq!(
        post_commit_summary["runtime_execution_model"],
        json!("ParallelPostCommitConsumption")
    );

    assert!(
        serial_summary["performance_counters"]["preparation_packet_count"]
            .as_u64()
            .is_some()
    );
    assert!(
        serial_summary["performance_counters"]["preparation_packet_item_count"]
            .as_u64()
            .is_some()
    );
    assert!(
        serial_summary["performance_counters"]["preparation_scope_unit_count"]
            .as_u64()
            .is_some()
    );
    assert!(
        staged_summary["performance_counters"]["preparation_packet_count"]
            .as_u64()
            .is_some()
    );
    assert!(
        staged_summary["performance_counters"]["preparation_packet_peak_width_total"]
            .as_u64()
            .is_some()
    );
    assert!(
        post_commit_summary["performance_counters"]["post_commit_consumer_packet_count"]
            .as_u64()
            .is_some()
    );
    assert!(
        post_commit_summary["performance_counters"]["post_commit_consumer_peak_width_total"]
            .as_u64()
            .is_some()
    );
}

#[test]
fn harness_diagnostics_expose_execution_mode_and_performance_counters() {
    let adapter = RelationalHarnessAdapter;
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();
    let profile = ExecutionProfile::staged_parallel("staged");
    let mut runtime = adapter.create_runtime().unwrap();
    adapter.prepare_runtime(&mut runtime, &profile).unwrap();
    adapter.load_fixture(&mut runtime, &fixture).unwrap();
    adapter.apply_mutation_batch(&mut runtime, &batch).unwrap();
    let _ = adapter
        .execute(&mut runtime, &fixture, &request, &profile)
        .unwrap();
    let diagnostics = adapter
        .capture_diagnostics(&runtime, &fixture, &profile)
        .unwrap();

    let summary = diagnostics.summary;
    assert_eq!(summary["execution_mode"], json!("StagedParallel"));
    assert_eq!(
        summary["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
    assert!(summary["performance_counters"]["preparation_packet_count"]
        .as_u64()
        .is_some());
    assert!(
        summary["performance_counters"]["preparation_packet_item_count"]
            .as_u64()
            .is_some()
    );
    assert!(
        summary["performance_counters"]["preparation_packet_peak_width_total"]
            .as_u64()
            .is_some()
    );
    assert!(
        summary["performance_counters"]["preparation_scope_unit_count"]
            .as_u64()
            .is_some()
    );
}

#[test]
fn harness_phase8_fallbacks_are_harness_visible_and_still_parity_safe() {
    let adapter = InvariantHarnessAdapter::new(InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::MaxMergedIntents(16),
        )],
        ..InvariantCatalog::default()
    });
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = forge_harness::facade::parity_suite(
        adapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidate(ExecutionProfile::staged_parallel("staged"))
    .compare()
    .unwrap();

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);
    assert!(report.results[0].comparison.mismatches.is_empty());

    let adapter = InvariantHarnessAdapter::new(InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::MaxMergedIntents(16),
        )],
        ..InvariantCatalog::default()
    });
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();
    let bundles = forge_harness::facade::run_matrix(adapter, fixture, request)
        .mutate(batch)
        .profile(ExecutionProfile::staged_parallel("staged"))
        .diagnose()
        .unwrap();
    let summary = bundles[0].diagnostics.as_ref().unwrap().summary.clone();

    let fallback_entries = harness_diagnostic_entries(&summary, "PreparationFallback");
    let failure_entries = harness_diagnostic_entries(&summary, "PreparationFailure");

    assert!(fallback_entries
        .iter()
        .any(|entry| { entry["fields"]["reason"] == json!("InsufficientPacketBreadth") }));
    assert!(failure_entries
        .iter()
        .any(|entry| { entry["fields"]["failure_class"] == json!("fallback_to_serial") }));
    assert_eq!(summary["execution_mode"], json!("StagedParallel"));
    assert_eq!(
        summary["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
    assert!(
        summary["performance_counters"]["preparation_serial_strategy_count"]
            .as_u64()
            .is_some_and(|count| count >= 1)
    );
    assert_eq!(
        summary["performance_counters"]["preparation_staged_parallel_strategy_count"],
        json!(0)
    );
}

#[test]
fn harness_phase8_planning_proof_failures_are_harness_visible() {
    let adapter = profitable_commit_boundary_adapter();
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let bundles = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::PlanningProofInsufficient,
        || {
            forge_harness::facade::run_matrix(adapter, fixture, request)
                .mutate(batch)
                .profile(ExecutionProfile::staged_parallel("staged"))
                .diagnose()
                .unwrap()
        },
    );
    let summary = bundles[0].diagnostics.as_ref().unwrap().summary.clone();

    let failure_entries = harness_diagnostic_entries(&summary, "PreparationFailure");

    assert!(failure_entries
        .iter()
        .any(|entry| { entry["fields"]["failure_class"] == json!("planning_proof_insufficient") }));
    assert_eq!(summary["execution_mode"], json!("StagedParallel"));
    assert_eq!(
        summary["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
    assert!(
        summary["performance_counters"]["preparation_staged_parallel_strategy_count"]
            .as_u64()
            .is_some_and(|count| count >= 1)
    );
}

#[test]
fn harness_phase8_publication_isolation_failures_are_harness_visible() {
    let adapter = profitable_commit_boundary_adapter();
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let bundles = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::PublicationIsolationViolation,
        || {
            forge_harness::facade::run_matrix(adapter, fixture, request)
                .mutate(batch)
                .profile(ExecutionProfile::staged_parallel("staged"))
                .diagnose()
                .unwrap()
        },
    );
    let summary = bundles[0].diagnostics.as_ref().unwrap().summary.clone();

    let failure_entries = harness_diagnostic_entries(&summary, "PreparationFailure");

    assert!(failure_entries.iter().any(|entry| {
        entry["fields"]["failure_class"] == json!("publication_isolation_violation")
    }));
    assert_eq!(summary["execution_mode"], json!("StagedParallel"));
    assert_eq!(
        summary["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
    assert!(
        summary["performance_counters"]["preparation_staged_parallel_strategy_count"]
            .as_u64()
            .is_some_and(|count| count >= 1)
    );
}

#[test]
fn harness_phase8_reducer_conflicts_are_harness_visible() {
    let adapter = profitable_commit_boundary_adapter();
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let bundles = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::ReductionIdentityConflict,
        || {
            forge_harness::facade::run_matrix(adapter, fixture, request)
                .mutate(batch)
                .profile(ExecutionProfile::staged_parallel("staged"))
                .diagnose()
                .unwrap()
        },
    );
    let summary = bundles[0].diagnostics.as_ref().unwrap().summary.clone();
    let failure_entries = harness_diagnostic_entries(&summary, "PreparationFailure");

    assert!(failure_entries
        .iter()
        .any(|entry| { entry["fields"]["failure_class"] == json!("reduction_identity_conflict") }));
    assert_eq!(summary["execution_mode"], json!("StagedParallel"));
    assert_eq!(
        summary["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
    assert!(
        summary["performance_counters"]["preparation_staged_parallel_strategy_count"]
            .as_u64()
            .is_some_and(|count| count >= 1)
    );
}

#[test]
fn harness_phase8_worker_evaluation_failures_are_harness_visible() {
    let adapter = profitable_commit_boundary_adapter();
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let bundles = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::WorkerEvaluationFailure,
        || {
            forge_harness::facade::run_matrix(adapter, fixture, request)
                .mutate(batch)
                .profile(ExecutionProfile::staged_parallel("staged"))
                .diagnose()
                .unwrap()
        },
    );
    let summary = bundles[0].diagnostics.as_ref().unwrap().summary.clone();
    let failure_entries = harness_diagnostic_entries(&summary, "PreparationFailure");

    assert!(failure_entries
        .iter()
        .any(|entry| { entry["fields"]["failure_class"] == json!("worker_evaluation_failure") }));
    assert_eq!(summary["execution_mode"], json!("StagedParallel"));
    assert_eq!(
        summary["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
    assert!(
        summary["performance_counters"]["preparation_staged_parallel_strategy_count"]
            .as_u64()
            .is_some_and(|count| count >= 1)
    );
}

#[test]
fn harness_phase8_post_commit_consumer_failures_are_harness_visible() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = crate::publication::logic::with_test_post_commit_fault(
        crate::publication::logic::TestPostCommitFault::ConsumerFailureNonAuthoritative,
        || {
            forge_harness::facade::certification_matrix(
                RelationalHarnessAdapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::full_parallel("post-commit"))
            .certify()
            .unwrap()
        },
    );
    let summary = certification_case(&report, "post-commit")
        .diagnostics_summary
        .as_ref()
        .unwrap();

    assert!(harness_diagnostic_entries(summary, "PreparationFailure")
        .iter()
        .any(|entry| {
            entry["fields"]["failure_class"] == json!("consumer_failure_non_authoritative")
        }));
    assert_eq!(summary["execution_mode"], json!("FullParallel"));
    assert_eq!(
        summary["runtime_execution_model"],
        json!("ParallelPostCommitConsumption")
    );
    assert!(
        summary["performance_counters"]["post_commit_parallel_strategy_count"]
            .as_u64()
            .is_some_and(|count| count >= 1)
    );
}

#[test]
fn harness_phase8_fragment_canonicalization_failures_are_harness_visible() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = crate::authority::commit::with_test_diff_preparation_fault(
        crate::authority::commit::TestDiffPreparationFault::FragmentCanonicalizationFailure,
        || {
            forge_harness::facade::certification_matrix(
                RelationalHarnessAdapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::staged_parallel("staged"))
            .certify()
            .unwrap()
        },
    );
    let summary = certification_case(&report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();

    assert!(harness_diagnostic_entries(summary, "PreparationFailure")
        .iter()
        .any(|entry| {
            entry["fields"]["failure_class"] == json!("fragment_canonicalization_failure")
        }));
    assert_eq!(summary["execution_mode"], json!("StagedParallel"));
    assert_eq!(
        summary["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
}

#[test]
fn harness_phase8_packet_overlap_failures_are_harness_visible() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = crate::authority::commit::with_test_diff_preparation_fault(
        crate::authority::commit::TestDiffPreparationFault::PacketOverlapDetected,
        || {
            forge_harness::facade::certification_matrix(
                RelationalHarnessAdapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::staged_parallel("staged"))
            .certify()
            .unwrap()
        },
    );
    let summary = certification_case(&report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();

    assert!(harness_diagnostic_entries(summary, "PreparationFailure")
        .iter()
        .any(|entry| entry["fields"]["failure_class"] == json!("packet_overlap_detected")));
    assert_eq!(summary["execution_mode"], json!("StagedParallel"));
    assert_eq!(
        summary["runtime_execution_model"],
        json!("StagedParallelPreparation")
    );
}

#[test]
fn harness_phase8_certification_matrix_closes_out_preparation_failures() {
    let fallback_adapter = InvariantHarnessAdapter::new(InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::MaxMergedIntents(16),
        )],
        ..InvariantCatalog::default()
    });
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();
    let fallback_report = forge_harness::facade::certification_matrix(
        fallback_adapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidate(ExecutionProfile::staged_parallel("staged"))
    .certify()
    .unwrap();
    let fallback_summary = certification_case(&fallback_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(fallback_summary, "PreparationFallback")
            .iter()
            .any(|entry| entry["fields"]["reason"] == json!("InsufficientPacketBreadth"))
    );
    assert!(
        harness_diagnostic_entries(fallback_summary, "PreparationFailure")
            .iter()
            .any(|entry| entry["fields"]["failure_class"] == json!("fallback_to_serial"))
    );

    let proof_report = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::PlanningProofInsufficient,
        || {
            let adapter = profitable_commit_boundary_adapter();
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                adapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::staged_parallel("staged"))
            .certify()
            .unwrap()
        },
    );
    let proof_summary = certification_case(&proof_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(proof_summary, "PreparationFailure")
            .iter()
            .any(|entry| entry["fields"]["failure_class"] == json!("planning_proof_insufficient"))
    );

    let isolation_report = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::PublicationIsolationViolation,
        || {
            let adapter = profitable_commit_boundary_adapter();
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                adapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::staged_parallel("staged"))
            .certify()
            .unwrap()
        },
    );
    let isolation_summary = certification_case(&isolation_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(isolation_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                entry["fields"]["failure_class"] == json!("publication_isolation_violation")
            })
    );

    let reducer_report = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::ReductionIdentityConflict,
        || {
            let adapter = profitable_commit_boundary_adapter();
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                adapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::staged_parallel("staged"))
            .certify()
            .unwrap()
        },
    );
    let reducer_summary = certification_case(&reducer_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(reducer_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                entry["fields"]["failure_class"] == json!("reduction_identity_conflict")
            })
    );

    let worker_report = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::WorkerEvaluationFailure,
        || {
            let adapter = profitable_commit_boundary_adapter();
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                adapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::staged_parallel("staged"))
            .certify()
            .unwrap()
        },
    );
    let worker_summary = certification_case(&worker_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(worker_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                entry["fields"]["failure_class"] == json!("worker_evaluation_failure")
            })
    );

    let consumer_report = crate::publication::logic::with_test_post_commit_fault(
        crate::publication::logic::TestPostCommitFault::ConsumerFailureNonAuthoritative,
        || {
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                RelationalHarnessAdapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::full_parallel("post-commit"))
            .certify()
            .unwrap()
        },
    );
    let consumer_summary = certification_case(&consumer_report, "post-commit")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(consumer_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                entry["fields"]["failure_class"] == json!("consumer_failure_non_authoritative")
            })
    );

    let fragment_report = crate::authority::commit::with_test_diff_preparation_fault(
        crate::authority::commit::TestDiffPreparationFault::FragmentCanonicalizationFailure,
        || {
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                RelationalHarnessAdapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::staged_parallel("staged"))
            .certify()
            .unwrap()
        },
    );
    let fragment_summary = certification_case(&fragment_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(fragment_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                entry["fields"]["failure_class"] == json!("fragment_canonicalization_failure")
            })
    );

    let overlap_report = crate::authority::commit::with_test_diff_preparation_fault(
        crate::authority::commit::TestDiffPreparationFault::PacketOverlapDetected,
        || {
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                RelationalHarnessAdapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::staged_parallel("staged"))
            .certify()
            .unwrap()
        },
    );
    let overlap_summary = certification_case(&overlap_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(overlap_summary, "PreparationFailure")
            .iter()
            .any(|entry| entry["fields"]["failure_class"] == json!("packet_overlap_detected"))
    );
}

#[test]
fn harness_heavy_invariants_are_opt_in() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::harness_audit_only(
            InvariantRule::UniqueEntityPayloadField("name".to_string()),
        )],
        ..InvariantCatalog::default()
    });
    let _ = create_entity(&mut runtime, "duplicate");
    let _ = create_entity(&mut runtime, "duplicate");

    let default_results = runtime
        .invariant_access()
        .harness_audit(HarnessAuditMode::Disabled)
        .into_results();
    let enabled_results = runtime
        .invariant_access()
        .harness_audit(HarnessAuditMode::Full)
        .into_results();

    assert!(default_results.is_empty());
    assert_eq!(enabled_results.len(), 1);
    assert_eq!(enabled_results[0].class(), InvariantClass::HarnessHeavy);
    assert!(matches!(
        enabled_results[0].verdict,
        crate::validation::data::InvariantVerdict::Advisory { .. }
    ));
}

#[test]
fn cross_order_equivalent_mutations_converge() {
    let runtime_a = apply_batches(vec![batch_create("a"), batch_create("b")]);
    let runtime_b = apply_batches(vec![batch_create("b"), batch_create("a")]);

    assert_eq!(
        runtime_a.publication_access().latest_patch(),
        runtime_b.publication_access().latest_patch()
    );
    assert_eq!(
        runtime_a.publication_access().latest_replay(),
        runtime_b.publication_access().latest_replay()
    );
    assert_eq!(
        runtime_a.publication_access().diagnostics(),
        runtime_b.publication_access().diagnostics()
    );
}
