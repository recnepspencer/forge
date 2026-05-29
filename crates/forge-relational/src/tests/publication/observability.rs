use crate::diagnostics::data::RelationalDiagnosticValue;
use crate::facade::diagnostics::{DiagnosticsDeliveryClass, RelationalDiagnosticsProfile};
use crate::facade::runtime::HarnessAuditMode;
use crate::schema::data::{
    ContractId, EndpointKindContractDeclaration, RelationIntegrityDeclarations,
    SymmetryContractDeclaration, SymmetryMode,
};
use crate::tests::support::*;

use super::harness_summary_projection::{
    harness_diagnostic_entries, harness_diagnostic_field_matches, harness_summary_counter,
    harness_summary_field,
};

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

fn diagnostic_object_field<'a>(
    value: &'a RelationalDiagnosticValue,
    field: &str,
) -> &'a RelationalDiagnosticValue {
    let RelationalDiagnosticValue::Object(fields) = value else {
        panic!("diagnostic value is not an object: {value:?}");
    };
    fields
        .get(field)
        .unwrap_or_else(|| panic!("diagnostic object field '{field}' missing from {value:?}"))
}

fn existing_entity_reference_diagnostic_value(
    entity_id: crate::identity::data::EntityId,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        (
            "reference_kind",
            RelationalDiagnosticValue::string("existing"),
        ),
        ("entity_id", RelationalDiagnosticValue::EntityId(entity_id)),
    ])
}

#[test]
fn diagnostics_and_replay_are_emitted_for_commit() {
    let mut runtime = runtime_with_test_schema();
    let _entity = create_entity(&mut runtime, "first");
    let diagnostics = runtime.publication().diagnostics();

    assert!(diagnostics.artifacts().iter().any(|artifact| {
        artifact.scope == DiagnosticsScope::Transaction
            && artifact.kind == DiagnosticsArtifactKind::MinimalSummary
    }));
    assert!(diagnostics
        .minimal_summaries()
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::EntityCreated));
    assert!(runtime.publication().latest_patch().is_some());
    assert!(runtime.publication().latest_replay().is_some());
    assert_eq!(
        runtime
            .publication()
            .latest_replay()
            .unwrap()
            .schema_authority,
        test_schema_registry().authority_snapshot()
    );
    assert_eq!(
        runtime.publication().diagnostic_artifact_count(),
        diagnostics.artifacts().len()
    );
}

#[test]
fn publication_diagnostics_since_fail_closes_for_stale_cursor() {
    let mut runtime = runtime_with_test_schema();
    let _entity = create_entity(&mut runtime, "first");
    let artifact_count = runtime.publication().diagnostic_artifact_count();

    assert!(runtime
        .publication()
        .diagnostics_since(artifact_count + 100)
        .is_empty());
}

#[test]
fn publication_observation_snapshot_tracks_latest_publication_state() {
    let mut runtime = runtime_with_test_schema();

    let empty = runtime.publication().observation_snapshot();
    assert_eq!(empty.latest_commit_id, None);
    assert_eq!(empty.publication_snapshot_id, None);
    assert_eq!(empty.publication_status, None);
    assert_eq!(empty.latest_patch_position, None);
    assert!(!empty.latest_patch_present);
    assert!(!empty.latest_replay_present);
    assert_eq!(empty.diagnostics_artifact_count, 0);

    let created = create_entity_outcome(&mut runtime, "first");
    let observed = runtime.publication().observation_snapshot();
    let publication = runtime.publication();
    let bundle = publication.latest_bundle().unwrap();

    assert_eq!(observed.latest_commit_id, Some(created.commit.commit_id));
    assert_eq!(
        observed.publication_snapshot_id,
        Some(bundle.snapshot.snapshot_id)
    );
    assert_eq!(observed.publication_status, Some(bundle.status.clone()));
    assert_eq!(observed.latest_patch_position, Some(bundle.patch.position));
    assert_eq!(
        observed.latest_patch_record_count,
        Some(bundle.patch.records.len())
    );
    assert_eq!(
        observed.latest_replay_commit_id,
        Some(bundle.replay.commit_id)
    );
    assert!(observed.latest_patch_present);
    assert!(observed.latest_replay_present);
    assert!(observed.diagnostics_artifact_count > 0);
}

#[test]
fn publication_artifact_snapshot_tracks_latest_patch_and_replay_with_observation() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "first");

    let snapshot = runtime.publication().artifact_snapshot();
    let publication = runtime.publication();
    let bundle = publication.latest_bundle().unwrap();

    assert_eq!(
        snapshot.observation.latest_commit_id,
        Some(created.commit.commit_id)
    );
    assert_eq!(snapshot.latest_patch, Some(bundle.patch.clone()));
    assert_eq!(snapshot.latest_replay, Some(bundle.replay.clone()));
}

#[test]
fn publication_diagnostics_snapshot_tracks_observation_and_artifacts_together() {
    let mut runtime = runtime_with_test_schema();
    let _created = create_entity_outcome(&mut runtime, "first");

    let snapshot = runtime.publication().diagnostics_snapshot();
    let publication = runtime.publication();

    assert_eq!(snapshot.observation, publication.observation_snapshot());
    assert_eq!(snapshot.diagnostics, publication.diagnostic_artifacts());
}

#[test]
fn invariant_failure_artifact_preserves_specific_code_localization_and_proof_boundary() {
    let mut runtime = RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![SymmetryContractDeclaration {
                contract_id: "paired_twin".into(),
                mode: SymmetryMode::PairedTwinRequired,
            }],
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("one-way").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("one-way"),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(target),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            },
        ))),
    );
    let error = txn.commit().unwrap_err();
    let diagnostics = runtime.publication().diagnostics();
    let artifact = diagnostics
        .by_scope(DiagnosticsScope::Invariant)
        .into_iter()
        .find(|artifact| {
            artifact.kind == DiagnosticsArtifactKind::Failure
                && artifact
                    .entries
                    .iter()
                    .any(|entry| entry.code == DiagnosticCode::RelationSymmetryViolation)
        })
        .expect("invariant failure artifact");
    let entry = artifact
        .entries
        .iter()
        .find(|entry| entry.code == DiagnosticCode::RelationSymmetryViolation)
        .expect("relation symmetry failure entry");

    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationSymmetryViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }
    assert_eq!(
        diagnostic_field(entry, "execution_point"),
        &RelationalDiagnosticValue::string("commit_boundary")
    );
    assert_eq!(
        diagnostic_field(entry, "failure_effect"),
        &RelationalDiagnosticValue::string("block_commit")
    );
    let violation = diagnostic_field(entry, "violation");
    assert_eq!(
        diagnostic_object_field(violation, "violation_kind"),
        &RelationalDiagnosticValue::string("relation_symmetry")
    );
    assert_eq!(
        diagnostic_object_field(violation, "contract_id"),
        &RelationalDiagnosticValue::ContractId(ContractId::new("paired_twin"))
    );
    assert_eq!(
        diagnostic_object_field(violation, "relation_kind_id"),
        &RelationalDiagnosticValue::KindId(KindId(2))
    );
    assert_eq!(
        diagnostic_object_field(violation, "source"),
        &existing_entity_reference_diagnostic_value(source)
    );
    assert_eq!(
        diagnostic_object_field(violation, "target"),
        &existing_entity_reference_diagnostic_value(target)
    );
    assert_eq!(
        diagnostic_object_field(violation, "mode"),
        &RelationalDiagnosticValue::string("paired_twin_required")
    );
    let proof_boundary = diagnostic_field(entry, "proof_boundary");
    assert_eq!(
        diagnostic_object_field(proof_boundary, "scope_class"),
        &RelationalDiagnosticValue::string("partition_scope")
    );
    assert_eq!(
        diagnostic_object_field(proof_boundary, "packet_count"),
        &RelationalDiagnosticValue::Unsigned(1)
    );
}

#[test]
fn invariant_diagnostics_trace_proof_boundary_for_relation_integrity_execution() {
    let mut runtime = RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![SymmetryContractDeclaration {
                contract_id: "paired_twin".into(),
                mode: SymmetryMode::PairedTwinRequired,
            }],
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("paired").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("forward"),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(target),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            },
        ))),
    );
    txn.push_batch(
        WorkerIntentBatch::new("paired-inverse").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("reverse"),
                source: crate::transactions::data::EntityReference::Existing(target),
                target: crate::transactions::data::EntityReference::Existing(source),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            }),
        )),
    );
    txn.commit().unwrap();

    let diagnostics = runtime.publication().diagnostics();
    let entry = diagnostics
        .by_scope(DiagnosticsScope::Invariant)
        .into_iter()
        .filter(|artifact| artifact.kind == DiagnosticsArtifactKind::DetailedTrace)
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| {
            entry.code == DiagnosticCode::InvariantProofBoundaryObserved
                && diagnostic_field_optional(entry, "execution_point")
                    == Some(&RelationalDiagnosticValue::string("commit_boundary"))
                && diagnostic_field_optional(entry, "proof_boundary").is_some_and(|value| {
                    diagnostic_object_field(value, "packet_count")
                        == &RelationalDiagnosticValue::Unsigned(1)
                })
        })
        .expect("proof boundary trace entry");

    assert_eq!(
        diagnostic_field(entry, "execution_point"),
        &RelationalDiagnosticValue::string("commit_boundary")
    );
    let proof_boundary = diagnostic_field(entry, "proof_boundary");
    assert_eq!(
        diagnostic_object_field(proof_boundary, "scope_class"),
        &RelationalDiagnosticValue::string("partition_scope")
    );
    assert_eq!(
        diagnostic_object_field(proof_boundary, "packet_count"),
        &RelationalDiagnosticValue::Unsigned(1)
    );
    assert_eq!(
        diagnostic_object_field(proof_boundary, "touched_partition_count"),
        &RelationalDiagnosticValue::Unsigned(1)
    );
}

#[test]
fn collect_all_invariant_failures_emits_multiple_relation_integrity_entries_for_one_commit() {
    let diagnostics = RelationalDiagnosticsProfile {
        collect_all_invariant_failures: true,
        ..RelationalDiagnosticsProfile::default()
    };
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(
            RelationIntegritySchemaFixture {
                relation_integrity: RelationIntegrityDeclarations::new(
                    vec![EndpointKindContractDeclaration {
                        contract_id: "no_self".into(),
                        allowed_source_kinds: vec![KindId(1)],
                        allowed_target_kinds: vec![KindId(1)],
                        self_edges_allowed: false,
                        cross_context_policy: CrossContextPolicy::AllowExplicit,
                    }],
                    Vec::new(),
                    Vec::new(),
                    vec![SymmetryContractDeclaration {
                        contract_id: "paired_twin".into(),
                        mode: SymmetryMode::InverseProhibited,
                    }],
                    Vec::new(),
                ),
                ..RelationIntegritySchemaFixture::default()
            }
            .build_registry(),
        )
        .diagnostics(diagnostics)
        .build();
    let source = create_entity(&mut runtime, "source");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("self-edge").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("self-edge"),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(source),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            },
        ))),
    );
    let _error = txn.commit().unwrap_err();

    let diagnostics = runtime.publication().diagnostics();
    let failure_artifact = diagnostics
        .by_scope(DiagnosticsScope::Invariant)
        .into_iter()
        .find(|artifact| artifact.kind == DiagnosticsArtifactKind::Failure)
        .expect("collect-all invariant failure artifact");

    assert!(failure_artifact
        .entries
        .iter()
        .any(|entry| entry.code == DiagnosticCode::RelationEndpointKindViolation));
    assert!(failure_artifact
        .entries
        .iter()
        .any(|entry| entry.code == DiagnosticCode::RelationSymmetryViolation));
}

#[test]
fn publication_bundle_is_the_single_visible_commit_surface() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "first");
    let publication = runtime.publication();
    let bundle = publication.latest_bundle().unwrap();

    assert_eq!(outcome.publication_status, PublicationStatus::Published);
    assert_eq!(bundle.snapshot, outcome.snapshot);
    assert_eq!(bundle.commit, outcome.commit);
    assert_eq!(bundle.commit, *runtime.history().latest_commit().unwrap());
    assert_eq!(bundle.patch, *runtime.publication().latest_patch().unwrap());
    assert_eq!(
        bundle.replay,
        *runtime.publication().latest_replay().unwrap()
    );
}

#[test]
fn publication_snapshot_handle_reads_without_becoming_a_pinned_snapshot() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "first");

    let retention = runtime.retention().inspect_plan();
    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let inspection = runtime
        .read_truth()
        .inspect_snapshot(&outcome.snapshot)
        .unwrap();
    let packet = explicit_query_packet(
        &runtime,
        &outcome.snapshot,
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
        .plan_read_explicit_query_packet(&outcome.snapshot, &packet)
        .is_some());
    assert_eq!(
        execute_explicit_query(
            &runtime,
            &outcome.snapshot,
            "entities",
            vec![RecordRef::Entity(changed_entities(&outcome)[0])],
        )
        .result
        .entities
        .len(),
        1
    );
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&outcome.snapshot));
    assert!(runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .is_none());
}

#[test]
fn publication_snapshot_reads_use_authoritative_published_binding_version() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "first");
    let updated = update_entity(&mut runtime, changed_entities(&created)[0], "second");
    let mut stale_handle = updated.snapshot.clone();
    stale_handle.version_id = created.snapshot.version_id;

    let read = runtime.read_truth().read_snapshot(&stale_handle).unwrap();
    let inspection = runtime
        .read_truth()
        .inspect_snapshot(&stale_handle)
        .unwrap();

    assert_eq!(read.snapshot.version_id, updated.snapshot.version_id);
    assert_eq!(inspection.version_id, updated.snapshot.version_id);
    assert_eq!(read.entities.len(), 1);
    assert_eq!(
        read_entity_field(&read.entities[0], field_key("name")),
        Some("second".into())
    );
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
        .read_truth()
        .read_snapshot(&first.snapshot)
        .is_none());
    assert!(runtime
        .read_truth()
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
        })
        .build();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let third = create_entity_outcome(&mut runtime, "third");

    let stats = runtime.storage_access().storage_stats();

    assert_eq!(stats.published_snapshot_handle_count, 2);
    assert!(runtime
        .read_truth()
        .read_snapshot(&first.snapshot)
        .is_none());
    assert!(runtime
        .read_truth()
        .read_snapshot(&second.snapshot)
        .is_some());
    assert!(runtime
        .read_truth()
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
        })
        .execution_model(crate::facade::runtime::RelationalExecutionModel::SerialAuthority)
        .build();
    let mut parallel = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .publication(PublicationConfig {
            coherent_publication_required: true,
            max_patch_records_per_commit: 4096,
            max_published_snapshot_handles: 2,
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

    let serial_bundle = serial.publication().latest_bundle().unwrap().clone();
    let parallel_bundle = parallel.publication().latest_bundle().unwrap().clone();
    let parallel_stats = parallel.storage_access().storage_stats();
    let diagnostics = parallel.publication().diagnostics();

    assert_eq!(parallel_bundle.commit, serial_bundle.commit);
    assert_eq!(parallel_bundle.patch, serial_bundle.patch);
    assert_eq!(parallel_bundle.replay, serial_bundle.replay);
    assert_eq!(parallel_bundle.snapshot, parallel_third.snapshot);
    assert_eq!(parallel_stats.published_snapshot_handle_count, 2);
    assert!(parallel
        .read_truth()
        .read_snapshot(&parallel_second.snapshot)
        .is_some());
    assert!(parallel
        .read_truth()
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
        3
    );
    assert_eq!(
        parallel
            .performance_access()
            .counters()
            .post_commit_serial_strategy_count,
        3
    );
    assert_eq!(
        parallel
            .performance_access()
            .counters()
            .post_commit_parallel_strategy_count,
        0
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
    let _ = assert_patch_truth_invariants(&serial_outcome);
    let _ = assert_patch_truth_invariants(&staged_outcome);
    let _ = assert_patch_truth_invariants(&post_commit_outcome);
}

fn aspect_relevant_diagnostics(
    diagnostics: &[crate::facade::diagnostics::RelationalDiagnosticArtifact],
) -> Vec<crate::facade::diagnostics::RelationalDiagnosticArtifact> {
    diagnostics
        .iter()
        .filter_map(|artifact| {
            let entries = artifact
                .entries
                .iter()
                .filter(|entry| {
                    matches!(
                        entry.code,
                        DiagnosticCode::AspectEvaluationTraced
                            | DiagnosticCode::AspectEmissionTraced
                            | DiagnosticCode::EntityCreated
                            | DiagnosticCode::CommitPublished
                    )
                })
                .cloned()
                .collect::<Vec<_>>();
            (!entries.is_empty()).then_some(
                crate::facade::diagnostics::RelationalDiagnosticArtifact {
                    scope: artifact.scope.clone(),
                    kind: artifact.kind.clone(),
                    determinism: artifact.determinism.clone(),
                    entries,
                },
            )
        })
        .collect()
}

#[test]
fn geometry_operational_hot_path_policy_suppresses_detailed_traces() {
    let mut runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::GeometryKernel)
        .schema_registry(test_schema_registry())
        .diagnostics(RelationalDiagnosticsProfile::geometry_operational_hot_path())
        .build();

    let _ = create_entity_outcome(&mut runtime, "geometry-hot-policy");
    let diagnostics = runtime.publication().diagnostics();

    assert!(diagnostics.artifacts().iter().any(|artifact| {
        artifact.scope == DiagnosticsScope::Transaction
            && artifact.kind == DiagnosticsArtifactKind::MinimalSummary
    }));
    assert!(!diagnostics
        .artifacts()
        .iter()
        .any(|artifact| { artifact.kind == DiagnosticsArtifactKind::DetailedTrace }));
}

#[test]
fn chip_rich_certification_policy_keeps_detailed_traces_available() {
    let mut runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::ChipSimulation)
        .schema_registry(test_schema_registry())
        .diagnostics(RelationalDiagnosticsProfile::chip_rich_certification())
        .build();

    let _ = create_entity_outcome(&mut runtime, "chip-rich-policy");
    let diagnostics = runtime.publication().diagnostics();

    assert!(diagnostics.artifacts().iter().any(|artifact| {
        artifact.scope == DiagnosticsScope::Transaction
            && artifact.kind == DiagnosticsArtifactKind::MinimalSummary
    }));
    assert!(diagnostics
        .artifacts()
        .iter()
        .any(|artifact| { artifact.kind == DiagnosticsArtifactKind::DetailedTrace }));
}

#[test]
fn geometry_operational_hot_path_policy_defers_replay_reconstructable_artifacts() {
    let profile = RelationalDiagnosticsProfile::geometry_operational_hot_path();
    let comparison_policy = profile.artifact_policy(
        DiagnosticsScope::Replay,
        DiagnosticsArtifactKind::Comparison,
    );

    assert_eq!(
        comparison_policy.delivery_class,
        DiagnosticsDeliveryClass::ReconstructableFromReplay
    );
    assert!(!comparison_policy.enabled);
    assert_eq!(comparison_policy.max_entries, 0);

    let summary_policy = profile.artifact_policy(
        DiagnosticsScope::Transaction,
        DiagnosticsArtifactKind::MinimalSummary,
    );
    assert_eq!(
        summary_policy.delivery_class,
        DiagnosticsDeliveryClass::MustBeHot
    );
    assert!(summary_policy.enabled);
    assert!(summary_policy.max_entries > 0);
}

#[test]
fn snapshot_audit_failure_blocks_publication() {
    let mut runtime = runtime_with_declared_aspect_schema_and_invariants(InvariantCatalog {
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
    assert!(runtime.publication().latest_bundle().is_none());
}

#[test]
fn bulk_packets_are_the_primary_read_surface() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    let snapshot = runtime.visibility_authority().snapshot();
    let plan = runtime
        .storage_access()
        .plan_read_explicit_query_packet(
            &snapshot,
            &explicit_query_packet(
                &runtime,
                &snapshot,
                "entities",
                vec![RecordRef::Entity(entity)],
            ),
        )
        .unwrap();
    let result = execute_explicit_query(
        &runtime,
        &snapshot,
        "entities",
        vec![RecordRef::Entity(entity)],
    )
    .result;

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
    assert_eq!(
        snapshot.observations[0].status,
        forge_harness::facade::ObservationStatus::Clean
    );
    let Some(forge_harness::facade::SnapshotPayload::Binary(snapshot_binary)) =
        &snapshot.observations[0].value
    else {
        panic!(
            "expected aspect-native binary snapshot value, got {:?}",
            snapshot.observations[0].value
        );
    };
    assert_eq!(
        snapshot_binary.media_type,
        "application/vnd.forge.relational.harness.aspect-snapshot.v1+octet-stream"
    );
    assert!(snapshot_binary
        .bytes
        .starts_with(b"forge.relational.harness.aspect-snapshot.v1"));
    assert_eq!(
        snapshot_binary.size_bytes,
        Some(snapshot_binary.bytes.len() as u64)
    );
    assert!(snapshot_binary
        .content_hash
        .as_deref()
        .is_some_and(|hash| { hash.starts_with("sha256:") }));
    assert!(diagnostics.summary.is_object());
    assert!(replay.summary.is_object());
}

#[test]
fn harness_snapshot_marks_missing_targets_unknown() {
    let adapter = RelationalHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "fixture",
        crate::presentation::harness::RelationalFixture {
            entities: Vec::new(),
            relations: Vec::new(),
        },
    )
    .compile();
    let profile = ExecutionProfile::forensic("forensic");
    let request = ExecutionRequest::target("missing", "entity:0:999".to_string());
    let runtime = adapter.create_runtime().unwrap();
    let snapshot = adapter
        .capture_snapshot(&runtime, &fixture, &request, &profile)
        .unwrap();

    assert_eq!(snapshot.observations.len(), 1);
    assert_eq!(
        snapshot.observations[0].status,
        forge_harness::facade::ObservationStatus::Unknown
    );
    assert_eq!(
        snapshot.observations[0].detail.as_deref(),
        Some("target not visible at captured snapshot".into())
    );
    assert!(snapshot.observations[0].value.is_none());
}

#[test]
fn harness_fixture_loads_declared_aspect_field_patches() {
    let adapter = RelationalHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "fixture",
        crate::presentation::harness::RelationalFixture {
            entities: vec![crate::presentation::harness::FixtureEntity {
                kind_id: KindId(1),
                client_key: "from-fixture".to_string(),
                fields: name_field_patch("from-fixture"),
            }],
            relations: Vec::new(),
        },
    )
    .compile();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .build();

    adapter.load_fixture(&mut runtime, &fixture).unwrap();

    let snapshot = runtime.visibility_authority().snapshot();
    let read = runtime.read_truth().read_version(snapshot.version_id);
    let entity_id = crate::facade::identity::EntityId::new(PartitionId::main(), 0, 1);
    let entity = read.get_entity(entity_id).expect("fixture entity visible");

    assert_eq!(read_entity_name(entity), Some("from-fixture".into()));
}

#[test]
fn runtime_packet_execution_and_storage_stats_are_readable() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    let snapshot = runtime.visibility_authority().snapshot();
    let result = execute_explicit_query(
        &runtime,
        &snapshot,
        "entities",
        vec![RecordRef::Entity(entity)],
    )
    .result;
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
                InvariantRule::unique_entity_aspect_field(aspect_key("name"), field_key("name")),
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
fn harness_phase8_fault_injection_soak_does_not_corrupt_following_certification_runs() {
    for _ in 0..3 {
        let (fixture, batch, request) = harness_phase8_fixture_batch_request();
        let fragment = crate::authority::commit::with_test_diff_preparation_fault(
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
        assert!(harness_diagnostic_entries(
            certification_case(&fragment, "staged")
                .diagnostics_summary
                .as_ref()
                .unwrap(),
            "PreparationFailure"
        )
        .iter()
        .any(|entry| {
            harness_diagnostic_field_matches(
                entry,
                "failure_class",
                "fragment_canonicalization_failure",
            )
        }));

        let (fixture, batch, request) = harness_phase8_fixture_batch_request();
        let overlap = crate::authority::commit::with_test_diff_preparation_fault(
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
        assert!(harness_diagnostic_entries(
            certification_case(&overlap, "staged")
                .diagnostics_summary
                .as_ref()
                .unwrap(),
            "PreparationFailure"
        )
        .iter()
        .any(|entry| harness_diagnostic_field_matches(
            entry,
            "failure_class",
            "packet_overlap_detected"
        )));

        let (fixture, batch, request) = harness_phase8_fixture_batch_request();
        let clean = forge_harness::facade::parity_suite(
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

        assert!(clean.matched);
        assert!(clean
            .results
            .iter()
            .all(|result| result.comparison.mismatches.is_empty()));
    }
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
        harness_summary_field(
            report.baseline_diagnostics_summary.as_ref().unwrap(),
            "runtime_execution_model"
        ),
        Some("SerialAuthority")
    );
    assert_eq!(report.cases.len(), 2);
    assert_eq!(report.cases[0].candidate_profile, "staged");
    assert_eq!(report.cases[1].candidate_profile, "post-commit");
    assert_eq!(
        harness_summary_field(
            report.cases[0].diagnostics_summary.as_ref().unwrap(),
            "runtime_execution_model"
        ),
        Some("StagedParallelPreparation")
    );
    assert_eq!(
        harness_summary_field(
            report.cases[1].diagnostics_summary.as_ref().unwrap(),
            "runtime_execution_model"
        ),
        Some("ParallelPostCommitConsumption")
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
        harness_summary_field(
            report.baseline_diagnostics_summary.as_ref().unwrap(),
            "execution_mode"
        ),
        Some("Serial")
    );
    assert_eq!(
        harness_summary_field(
            report.baseline_diagnostics_summary.as_ref().unwrap(),
            "runtime_execution_model"
        ),
        Some("SerialAuthority")
    );

    let staged = certification_case(&report, "staged");
    let post_commit = certification_case(&report, "post-commit");

    assert!(staged.comparison.matched);
    assert!(post_commit.comparison.matched);
    assert_eq!(
        harness_summary_field(
            staged.diagnostics_summary.as_ref().unwrap(),
            "runtime_execution_model"
        ),
        Some("StagedParallelPreparation")
    );
    assert_eq!(
        harness_summary_field(
            post_commit.diagnostics_summary.as_ref().unwrap(),
            "runtime_execution_model"
        ),
        Some("ParallelPostCommitConsumption")
    );
    assert!(harness_summary_counter(
        staged.diagnostics_summary.as_ref().unwrap(),
        "preparation_packet_count"
    )
    .is_some());
    assert!(harness_summary_counter(
        post_commit.diagnostics_summary.as_ref().unwrap(),
        "post_commit_consumer_packet_count"
    )
    .is_some());
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

    assert_eq!(
        harness_summary_field(serial_summary, "execution_mode"),
        Some("Serial")
    );
    assert_eq!(
        harness_summary_field(serial_summary, "runtime_execution_model"),
        Some("SerialAuthority")
    );
    assert_eq!(
        harness_summary_field(staged_summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(staged_summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert_eq!(
        harness_summary_field(post_commit_summary, "execution_mode"),
        Some("FullParallel")
    );
    assert_eq!(
        harness_summary_field(post_commit_summary, "runtime_execution_model"),
        Some("ParallelPostCommitConsumption")
    );

    assert!(harness_summary_counter(serial_summary, "preparation_packet_count").is_some());
    assert!(harness_summary_counter(serial_summary, "preparation_packet_item_count").is_some());
    assert!(harness_summary_counter(serial_summary, "preparation_scope_unit_count").is_some());
    assert!(harness_summary_counter(staged_summary, "preparation_packet_count").is_some());
    assert!(
        harness_summary_counter(staged_summary, "preparation_packet_peak_width_total").is_some()
    );
    assert!(
        harness_summary_counter(post_commit_summary, "post_commit_consumer_packet_count").is_some()
    );
    assert!(
        harness_summary_counter(post_commit_summary, "post_commit_consumer_peak_width_total")
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
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert!(harness_summary_counter(&summary, "preparation_packet_count").is_some());
    assert!(harness_summary_counter(&summary, "preparation_packet_item_count").is_some());
    assert!(harness_summary_counter(&summary, "preparation_packet_peak_width_total").is_some());
    assert!(harness_summary_counter(&summary, "preparation_scope_unit_count").is_some());
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

    assert!(fallback_entries.iter().any(|entry| {
        harness_diagnostic_field_matches(entry, "reason", "insufficient_packet_breadth")
    }));
    assert!(failure_entries.iter().any(|entry| {
        harness_diagnostic_field_matches(entry, "failure_class", "fallback_to_serial")
    }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert!(
        harness_summary_counter(&summary, "preparation_serial_strategy_count")
            .is_some_and(|count| count >= 1)
    );
    assert_eq!(
        harness_summary_counter(&summary, "preparation_staged_parallel_strategy_count"),
        Some(0)
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

    assert!(failure_entries.iter().any(|entry| {
        harness_diagnostic_field_matches(entry, "failure_class", "planning_proof_insufficient")
    }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert!(
        harness_summary_counter(&summary, "preparation_staged_parallel_strategy_count")
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
        harness_diagnostic_field_matches(entry, "failure_class", "publication_isolation_violation")
    }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert!(
        harness_summary_counter(&summary, "preparation_staged_parallel_strategy_count")
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

    assert!(failure_entries.iter().any(|entry| {
        harness_diagnostic_field_matches(entry, "failure_class", "reduction_identity_conflict")
    }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert!(
        harness_summary_counter(&summary, "preparation_staged_parallel_strategy_count")
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

    assert!(failure_entries.iter().any(|entry| {
        harness_diagnostic_field_matches(entry, "failure_class", "worker_evaluation_failure")
    }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert!(
        harness_summary_counter(&summary, "preparation_staged_parallel_strategy_count")
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
            harness_diagnostic_field_matches(
                entry,
                "failure_class",
                "consumer_failure_non_authoritative",
            )
        }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("FullParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("ParallelPostCommitConsumption")
    );
    assert!(
        harness_summary_counter(&summary, "post_commit_serial_strategy_count")
            .is_some_and(|count| count >= 1)
    );
    assert_eq!(
        harness_summary_counter(&summary, "post_commit_parallel_strategy_count"),
        Some(0)
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
            harness_diagnostic_field_matches(
                entry,
                "failure_class",
                "fragment_canonicalization_failure",
            )
        }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
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
        .any(|entry| harness_diagnostic_field_matches(
            entry,
            "failure_class",
            "packet_overlap_detected"
        )));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
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
            .any(|entry| harness_diagnostic_field_matches(
                entry,
                "reason",
                "insufficient_packet_breadth"
            ))
    );
    assert!(
        harness_diagnostic_entries(fallback_summary, "PreparationFailure")
            .iter()
            .any(|entry| harness_diagnostic_field_matches(
                entry,
                "failure_class",
                "fallback_to_serial"
            ))
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
            .any(|entry| {
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "planning_proof_insufficient",
                )
            })
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
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "publication_isolation_violation",
                )
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
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "reduction_identity_conflict",
                )
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
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "worker_evaluation_failure",
                )
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
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "consumer_failure_non_authoritative",
                )
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
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "fragment_canonicalization_failure",
                )
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
            .any(|entry| {
                harness_diagnostic_field_matches(entry, "failure_class", "packet_overlap_detected")
            })
    );
}

#[test]
fn harness_heavy_invariants_are_opt_in() {
    let mut runtime = runtime_with_declared_aspect_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::harness_audit_only(
            InvariantRule::unique_entity_aspect_field(aspect_key("name"), field_key("name")),
        )],
        ..InvariantCatalog::default()
    });
    let _ = create_entity(&mut runtime, "duplicate");
    let _ = create_entity(&mut runtime, "duplicate");

    let default_results = runtime
        .validation()
        .harness_audit(HarnessAuditMode::Disabled)
        .into_results();
    let enabled_results = runtime
        .validation()
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
        runtime_a.publication().latest_patch(),
        runtime_b.publication().latest_patch()
    );
    assert_eq!(
        runtime_a.publication().latest_replay(),
        runtime_b.publication().latest_replay()
    );
    assert_eq!(
        runtime_a.publication().diagnostics(),
        runtime_b.publication().diagnostics()
    );
}
