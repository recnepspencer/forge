mod durability_contracts;
mod index_contracts;
mod lineage_contracts;
mod replay_contracts;

use forge_harness::facade::{
    DiagnosticsHarnessAdapter, ExecutionProfile, ExecutionRequest, HarnessAdapter, MutationBatch,
    ReplayHarnessAdapter, ReplayRequest, ScenarioPlan,
};
use serde_json::json;

use crate::facade::{
    BranchCreateError, BranchId, CommitOutcome, DiagnosticCode, DiagnosticsArtifactKind,
    DiagnosticsScope, EntityKindRegistration, EntityReadRecord, InvariantCatalog, InvariantClass,
    InvariantExecutionPoint, InvariantRule, KindId, PublicationStage, PublicationStatus,
    QueryWorkPacket, ReadTarget, RelationId, RelationKindRegistration, RelationalHarnessAdapter,
    RelationalMutation, RelationalRuntime, RelationalRuntimeApi, RelationalRuntimeProfile,
    RelationalSchemaRegistry, SchemaId, SchemaVersionId, StorageLayoutConfig,
    TransactionCommitError, TransactionIntent, TransactionOptions, WorkerIntentBatch,
};

#[test]
fn runtime_defaults_to_serialized_authority() {
    let runtime = runtime_with_test_schema();

    assert_eq!(
        runtime.config().execution_model,
        crate::facade::RelationalExecutionModel::SerialAuthority
    );
    assert_eq!(
        runtime.config().commit_authority.authority.mode,
        crate::facade::AuthorityMode::SerializedCommit
    );
}

#[test]
fn harness_defaults_require_determinism_and_parity() {
    let expectations = crate::facade::default_harness_expectations();
    assert!(expectations.serial_parallel_parity_required);
}

#[test]
fn entity_slot_reuse_increments_generation() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::AiWorkflow);
    let create_outcome = create_entity_outcome(&mut runtime, "first");
    let entity_a = changed_entities(&create_outcome)[0];
    assert!(runtime.release_snapshot(&create_outcome.snapshot));
    let delete_outcome = delete_entity(&mut runtime, entity_a);
    assert!(runtime.release_snapshot(&delete_outcome.snapshot));
    let retention = runtime.run_retention_pass();
    let entity_b = create_entity(&mut runtime, "second");

    assert!(retention.entity_reclaimed <= 1);
    assert_eq!(runtime.storage_stats().reusable_entity_slots, 0);
    assert_eq!(entity_a.slot, entity_b.slot);
    assert!(entity_b.generation.0 > entity_a.generation.0);
}

#[test]
fn stale_entity_ids_are_rejected() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    delete_entity(&mut runtime, entity);
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("update").push(TransactionIntent::UpdateEntity {
            entity_id: entity,
            payload: json!({"name":"stale"}),
        }),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict) if conflict.code == DiagnosticCode::StaleHandle
    ));
}

#[test]
fn unknown_entity_kind_fails_explicitly() {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("unknown-kind").push(TransactionIntent::CreateEntity(
            crate::data::transaction::EntitySpec {
                kind_id: KindId(999),
                client_key: "bad".to_string(),
                payload: json!({"name":"bad"}),
            },
        )),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict) if conflict.code == DiagnosticCode::InvariantViolation
    ));
}

#[test]
fn duplicate_relation_identity_is_rejected() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    create_relation(&mut runtime, source, target, "r1");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate").push(TransactionIntent::CreateRelation(
            crate::data::transaction::RelationSpec {
                kind_id: KindId(2),
                client_key: "r2".to_string(),
                source,
                target,
                payload: json!({"label":"rel"}),
            },
        )),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict)
            if conflict.code == DiagnosticCode::DuplicateRelationIdentity
    ));
}

#[test]
fn savepoint_rollback_discards_inner_work_only() {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("outer"));
    let savepoint = txn.create_savepoint();
    txn.push_batch(batch_create("inner"));
    let rollback = txn.rollback_to_savepoint(savepoint).unwrap();
    let outcome = txn.commit().unwrap();
    let read = runtime.read_snapshot(&outcome.snapshot).unwrap();

    assert!(!rollback.restored_records.is_empty());
    assert_eq!(read.entities().len(), 1);
}

#[test]
fn merged_plan_is_stable_across_batch_order() {
    let mut runtime_a = runtime_with_test_schema();
    let mut txn_a = runtime_a.begin_transaction(TransactionOptions::default());
    txn_a.push_batch(batch_create("b"));
    txn_a.push_batch(batch_create("a"));
    let plan_a = txn_a.merged_plan().unwrap().clone();

    let mut runtime_b = runtime_with_test_schema();
    let mut txn_b = runtime_b.begin_transaction(TransactionOptions::default());
    txn_b.push_batch(batch_create("a"));
    txn_b.push_batch(batch_create("b"));
    let plan_b = txn_b.merged_plan().unwrap().clone();

    assert_eq!(plan_a, plan_b);
}

#[test]
fn snapshot_reads_are_immutable_after_later_mutation() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity(&mut runtime, "first");
    let snapshot = runtime.snapshot();
    let _second = create_entity(&mut runtime, "second");
    let read = runtime.read_snapshot(&snapshot).unwrap();

    assert!(read.get_entity(first).is_some());
    assert_eq!(read.entities().len(), 1);
}

#[test]
fn snapshots_resolve_historical_entity_payloads_by_version() {
    let mut runtime = runtime_with_test_schema();
    let create_outcome = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&create_outcome)[0];
    let snapshot = runtime.snapshot();
    let update_outcome = update_entity(&mut runtime, entity, "after");

    let old_read = runtime.read_snapshot(&snapshot).unwrap();
    let current_read = runtime.read_snapshot(&update_outcome.snapshot).unwrap();
    let version_read = runtime.read_version(create_outcome.version_id);

    assert_eq!(
        _read_entity_name(old_read.get_entity(entity).unwrap()),
        Some("before")
    );
    assert_eq!(
        _read_entity_name(current_read.get_entity(entity).unwrap()),
        Some("after")
    );
    assert_eq!(
        _read_entity_name(version_read.get_entity(entity).unwrap()),
        Some("before")
    );
}

#[test]
fn profile_resolution_and_provenance_are_explicit() {
    let runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::GeometryKernel)
        .schema_registry(test_schema_registry())
        .entity_capacity(999)
        .build();

    assert_eq!(
        runtime.config().profile,
        RelationalRuntimeProfile::GeometryKernel
    );
    assert_eq!(runtime.config().initial_entity_capacity, 999);
    assert!(runtime.config().diagnostics.detailed_traces_enabled);
    assert_eq!(runtime.config().storage_layout.entity_chunk_size, 2048);
    assert_eq!(
        runtime
            .config()
            .config_provenance
            .source_for("initial_entity_capacity")
            .unwrap()
            .source,
        crate::facade::ConfigValueSource::BuilderOverride
    );
    assert_eq!(
        runtime
            .config()
            .config_provenance
            .source_for("storage_layout")
            .unwrap()
            .source,
        crate::facade::ConfigValueSource::ProfileDefault
    );
}

#[test]
fn snapshot_pins_block_reclaim_until_release() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::AiWorkflow);
    let create_outcome = create_entity_outcome(&mut runtime, "pinned");
    let entity = changed_entities(&create_outcome)[0];
    let delete_outcome = delete_entity(&mut runtime, entity);
    let first_retention = runtime.run_retention_pass();

    assert_eq!(first_retention.entity_reclaimed, 0);
    assert_eq!(runtime.storage_stats().deleted_entities, 1);
    assert_eq!(first_retention.entity_chunks_scanned, 1);

    assert!(runtime.release_snapshot(&create_outcome.snapshot));
    assert!(runtime.release_snapshot(&delete_outcome.snapshot));
    let second_retention = runtime.run_retention_pass();

    assert!(second_retention.entity_reclaimed <= 1);
    assert_eq!(runtime.storage_stats().reusable_entity_slots, 1);
}

#[test]
fn read_records_expose_visibility_metadata() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "visible");
    let read = runtime.read_snapshot(&outcome.snapshot).unwrap();
    let record = read.entities().first().unwrap();

    assert_eq!(record.created_at_version, outcome.version_id);
    assert_eq!(record.retired_at_version, None);
}

#[test]
fn diagnostics_and_replay_are_emitted_for_commit() {
    let mut runtime = runtime_with_test_schema();
    let _entity = create_entity(&mut runtime, "first");
    let diagnostics = runtime.diagnostics();

    assert!(diagnostics.artifacts().iter().any(|artifact| {
        artifact.scope == DiagnosticsScope::Transaction
            && artifact.kind == DiagnosticsArtifactKind::MinimalSummary
    }));
    assert!(diagnostics
        .minimal_summaries()
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::EntityCreated));
    assert!(runtime.latest_patch().is_some());
    assert!(runtime.latest_replay().is_some());
    assert_eq!(
        runtime.latest_replay().unwrap().schema_registry,
        test_schema_registry()
    );
}

#[test]
fn publication_bundle_is_the_single_visible_commit_surface() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "first");
    let bundle = runtime.latest_publication_bundle().unwrap();

    assert_eq!(outcome.publication_status, PublicationStatus::Published);
    assert_eq!(bundle.snapshot, outcome.snapshot);
    assert_eq!(bundle.commit, outcome.commit);
    assert_eq!(bundle.commit, *runtime.latest_commit().unwrap());
    assert_eq!(bundle.patch, *runtime.latest_patch().unwrap());
    assert_eq!(bundle.replay, *runtime.latest_replay().unwrap());
}

#[test]
fn snapshot_audit_failure_blocks_publication() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        snapshot_audit: vec![InvariantRule::MaxSnapshotEntities(0)],
        ..InvariantCatalog::default()
    });
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("blocked"));
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Publication(ref publication)
            if publication.stage == PublicationStage::InvariantCheck
    ));
    assert!(runtime.latest_publication_bundle().is_none());
}

#[test]
fn bulk_packets_are_the_primary_read_surface() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    let snapshot = runtime.snapshot();
    let plan = runtime
        .plan_read_packet(
            &snapshot,
            &QueryWorkPacket::bulk("entities", vec![ReadTarget::Entity(entity)]),
        )
        .unwrap();
    let result = runtime
        .execute_read_packet(
            &snapshot,
            &QueryWorkPacket::bulk("entities", vec![ReadTarget::Entity(entity)]),
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
    let batch =
        MutationBatch::new("mutate").push(RelationalMutation::Batch(batch_create("from-harness")));
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
    let snapshot = runtime.snapshot();
    let packet = QueryWorkPacket::bulk("entities", vec![ReadTarget::Entity(entity)]);
    let result = runtime.execute_read_packet(&snapshot, &packet).unwrap();
    let stats = runtime.storage_stats();

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
    let batch =
        MutationBatch::new("mutate").push(RelationalMutation::Batch(batch_create("stable")));
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

#[test]
fn harness_heavy_invariants_are_opt_in() {
    let runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        harness_heavy: vec![InvariantRule::UniqueEntityPayloadField("name".to_string())],
        ..InvariantCatalog::default()
    });

    let default_results = runtime.run_invariants(InvariantExecutionPoint::HarnessAudit, false);
    let enabled_results = runtime.run_invariants(InvariantExecutionPoint::HarnessAudit, true);

    assert!(default_results.is_empty());
    assert_eq!(enabled_results.len(), 1);
    assert_eq!(enabled_results[0].class, InvariantClass::HarnessHeavy);
}

#[test]
fn cross_order_equivalent_mutations_converge() {
    let runtime_a = apply_batches(vec![batch_create("a"), batch_create("b")]);
    let runtime_b = apply_batches(vec![batch_create("b"), batch_create("a")]);

    assert_eq!(runtime_a.latest_patch(), runtime_b.latest_patch());
    assert_eq!(runtime_a.latest_replay(), runtime_b.latest_replay());
    assert_eq!(runtime_a.diagnostics(), runtime_b.diagnostics());
}

#[test]
fn branch_creation_and_branch_targeted_commits_build_a_version_graph() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let main_second =
        create_entity_outcome_on_branch(&mut runtime, "main-b", BranchId("main".to_string()));
    let graph = runtime.version_graph();

    assert_eq!(
        runtime
            .branch_head(&BranchId("feature".to_string()))
            .unwrap(),
        &feature_outcome.commit
    );
    assert_eq!(
        runtime.branch_head(&BranchId("main".to_string())).unwrap(),
        &main_second.commit
    );
    assert_eq!(
        feature_outcome.commit.parents,
        vec![main_outcome.commit.commit_id]
    );
    assert_eq!(
        main_second.commit.parents,
        vec![main_outcome.commit.commit_id]
    );
    assert_eq!(graph.branches.len(), 2);
    assert_eq!(graph.commits.len(), 3);
}

#[test]
fn merge_commit_uses_deterministic_parent_order_and_advances_target_branch() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .create_branch(BranchId("feature".to_string()), &BranchId("main".to_string()))
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let merge_outcome = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );

    assert_eq!(
        merge_outcome.commit.parents,
        vec![main_outcome.commit.commit_id, feature_outcome.commit.commit_id]
    );
    assert_eq!(
        runtime.branch_head(&BranchId("main".to_string())),
        Some(&merge_outcome.commit)
    );
    assert_eq!(
        runtime.branch_head(&BranchId("feature".to_string())),
        Some(&feature_outcome.commit)
    );
    let envelope = runtime
        .canonical_commit_envelope(merge_outcome.commit.commit_id)
        .unwrap();
    assert_eq!(envelope.merge_parent_branches, vec![BranchId("feature".to_string())]);
    assert_eq!(envelope.merge_base_commits, vec![main_outcome.commit.commit_id]);
    assert!(runtime
        .diagnostics()
        .by_scope(DiagnosticsScope::PatchPublication)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeCommitPublished));
    assert!(runtime
        .diagnostics()
        .by_scope(DiagnosticsScope::PatchPublication)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeBaseResolved));
}

#[test]
fn merge_commit_requires_existing_parent_branch_heads() {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    let txn = runtime.begin_transaction(
        TransactionOptions::default()
            .merge_from_branches(vec![BranchId("missing".to_string())]),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict)
            if conflict.code == DiagnosticCode::InvalidMergeParent
    ));
}

#[test]
fn branch_history_helpers_expose_ancestor_and_merge_base_reasoning() {
    let mut runtime = runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main");
    runtime
        .create_branch(BranchId("feature".to_string()), &BranchId("main".to_string()))
        .unwrap();
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let chain = runtime.ancestor_chain(feature.commit.commit_id);
    let merge_base = runtime.latest_common_ancestor_between_branches(
        &BranchId("main".to_string()),
        &BranchId("feature".to_string()),
    );

    assert_eq!(chain, vec![main.commit.commit_id, feature.commit.commit_id]);
    assert_eq!(merge_base, Some(main.commit.commit_id));
    assert!(runtime.can_merge_branch_into(
        &BranchId("feature".to_string()),
        &BranchId("main".to_string())
    ));
}

#[test]
fn duplicate_branch_creation_is_rejected() {
    let mut runtime = runtime_with_test_schema();
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let error = runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap_err();

    assert_eq!(error, BranchCreateError::BranchAlreadyExists);
}

#[test]
fn chunked_storage_summary_tracks_visibility_boundaries() {
    let mut runtime = runtime_with_test_schema_and_chunks(2, 2);
    let first = create_entity_outcome(&mut runtime, "e0");
    let entity_a = changed_entities(&first)[0];
    let _second = create_entity_outcome(&mut runtime, "e1");
    let snapshot = runtime.snapshot();
    let _third = create_entity_outcome(&mut runtime, "e2");
    let _update = update_entity(&mut runtime, entity_a, "e0-updated");

    let summary_before_update = runtime.chunked_storage_summary(snapshot.version_id);
    let summary_current =
        runtime.chunked_storage_summary(runtime.latest_commit().unwrap().version_id);

    assert_eq!(summary_before_update.entity_chunks.len(), 2);
    assert_eq!(summary_before_update.entity_chunks[0].visible_records, 2);
    assert_eq!(summary_before_update.entity_chunks[1].visible_records, 0);
    assert_eq!(summary_current.entity_chunks[1].visible_records, 1);
    assert_eq!(summary_current.entity_chunks[0].slot_len, 2);
}

#[test]
fn chunk_diagnostics_and_packet_plans_are_public_and_stable() {
    let mut runtime = runtime_with_test_schema_and_chunks(2, 2);
    let first = create_entity_outcome(&mut runtime, "e0");
    let second = create_entity_outcome(&mut runtime, "e1");
    let entity_a = changed_entities(&first)[0];
    let entity_b = changed_entities(&second)[0];
    let snapshot = runtime.snapshot();
    let packet = QueryWorkPacket::bulk(
        "pair",
        vec![ReadTarget::Entity(entity_a), ReadTarget::Entity(entity_b)],
    );

    let plan = runtime.plan_read_packet(&snapshot, &packet).unwrap();
    let diagnostics = runtime.chunk_diagnostics(snapshot.version_id);

    assert_eq!(plan.target_count, 2);
    assert_eq!(plan.entity_chunk_indexes, vec![0]);
    assert_eq!(diagnostics.entity_chunks_total, 1);
    assert_eq!(diagnostics.entity_chunks_with_visible_records, 1);
}

pub(super) fn test_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
            })
        })
        .unwrap()
}

pub(super) fn runtime_with_test_schema() -> RelationalRuntime {
    runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore)
}

pub(super) fn runtime_with_test_schema_profile(
    profile: RelationalRuntimeProfile,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(profile)
        .schema_registry(test_schema_registry())
        .build()
}

pub(super) fn runtime_with_test_schema_and_chunks(
    entity_chunk_size: usize,
    relation_chunk_size: usize,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(test_schema_registry())
        .storage_layout(StorageLayoutConfig {
            entity_chunk_size,
            relation_chunk_size,
            scan_packet_size: 64,
        })
        .build()
}

pub(super) fn runtime_with_test_schema_and_invariants(
    invariant_catalog: InvariantCatalog,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .invariant_catalog(invariant_catalog)
        .build()
}

pub(super) fn batch_create(name: &str) -> WorkerIntentBatch {
    WorkerIntentBatch::new(format!("batch-{name}")).push(TransactionIntent::CreateEntity(
        crate::data::transaction::EntitySpec {
            kind_id: KindId(1),
            client_key: name.to_string(),
            payload: json!({ "name": name }),
        },
    ))
}

pub(super) fn create_entity(
    runtime: &mut RelationalRuntime,
    name: &str,
) -> crate::facade::EntityId {
    changed_entities(&create_entity_outcome(runtime, name))[0]
}

pub(super) fn create_entity_outcome(runtime: &mut RelationalRuntime, name: &str) -> CommitOutcome {
    create_entity_outcome_on_branch(runtime, name, BranchId("main".to_string()))
}

pub(super) fn create_entity_outcome_on_branch(
    runtime: &mut RelationalRuntime,
    name: &str,
    branch_id: BranchId,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(batch_create(name));
    txn.commit().unwrap()
}

pub(super) fn delete_entity(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::EntityId,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete").push(TransactionIntent::DeleteEntity { entity_id }),
    );
    txn.commit().unwrap()
}

pub(super) fn update_entity(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::EntityId,
    name: &str,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("update").push(TransactionIntent::UpdateEntity {
            entity_id,
            payload: json!({ "name": name }),
        }),
    );
    txn.commit().unwrap()
}

pub(super) fn create_relation(
    runtime: &mut RelationalRuntime,
    source: crate::facade::EntityId,
    target: crate::facade::EntityId,
    client_key: &str,
) -> RelationId {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("relation").push(TransactionIntent::CreateRelation(
            crate::data::transaction::RelationSpec {
                kind_id: KindId(2),
                client_key: client_key.to_string(),
                source,
                target,
                payload: json!({"label":"rel"}),
            },
        )),
    );
    let outcome = txn.commit().unwrap();
    changed_relations(&outcome)[0]
}

pub(super) fn changed_entities(outcome: &CommitOutcome) -> Vec<crate::facade::EntityId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            crate::facade::RecordRef::Entity(entity_id) => Some(*entity_id),
            crate::facade::RecordRef::Relation(_) => None,
        })
        .collect()
}

pub(super) fn changed_relations(outcome: &CommitOutcome) -> Vec<RelationId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            crate::facade::RecordRef::Relation(relation_id) => Some(*relation_id),
            crate::facade::RecordRef::Entity(_) => None,
        })
        .collect()
}

pub(super) fn apply_batches(batches: Vec<WorkerIntentBatch>) -> RelationalRuntime {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    for batch in batches {
        txn.push_batch(batch);
    }
    txn.commit().unwrap();
    runtime
}

pub(super) fn merge_commit_from_branches(
    runtime: &mut RelationalRuntime,
    target_branch: BranchId,
    merge_parent_branches: Vec<BranchId>,
) -> CommitOutcome {
    let txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(target_branch),
        merge_parent_branches,
        ..TransactionOptions::default()
    });
    txn.commit().unwrap()
}

#[allow(dead_code)]
fn _read_entity_name(record: &EntityReadRecord) -> Option<&str> {
    record.payload.get("name").and_then(|value| value.as_str())
}
