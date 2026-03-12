use crate::tests::support::*;

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
        runtime.publication_access().latest_replay().unwrap().schema_registry,
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
    assert_eq!(bundle.commit, *runtime.history_access().latest_commit().unwrap());
    assert_eq!(bundle.patch, *runtime.publication_access().latest_patch().unwrap());
    assert_eq!(bundle.replay, *runtime.publication_access().latest_replay().unwrap());
}

#[test]
fn publication_snapshot_handle_reads_without_becoming_a_pinned_snapshot() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "first");

    let retention = runtime.retention_access().inspect_plan();
    let read = runtime.visibility_reads().read_snapshot(&outcome.snapshot).unwrap();
    let inspection = runtime.visibility_reads().inspect_snapshot(&outcome.snapshot).unwrap();
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
        .visibility_reads().execute_read_packet(&outcome.snapshot, &packet)
        .is_some());
    assert!(runtime.visibility_authority().release_snapshot(&outcome.snapshot));
    assert!(runtime.visibility_reads().read_snapshot(&outcome.snapshot).is_none());
}

#[test]
fn released_publication_handles_stop_counting_as_readable_runtime_state() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");

    let before = runtime.storage_access().storage_stats();
    assert_eq!(before.published_snapshot_handle_count, 2);

    assert!(runtime.visibility_authority().release_snapshot(&first.snapshot));
    let after_first_release = runtime.storage_access().storage_stats();
    assert_eq!(after_first_release.published_snapshot_handle_count, 1);
    assert!(runtime.visibility_reads().read_snapshot(&first.snapshot).is_none());
    assert!(runtime.visibility_reads().read_snapshot(&second.snapshot).is_some());

    assert!(runtime.visibility_authority().release_snapshot(&second.snapshot));
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
    assert!(runtime.visibility_reads().read_snapshot(&first.snapshot).is_none());
    assert!(runtime.visibility_reads().read_snapshot(&second.snapshot).is_some());
    assert!(runtime.visibility_reads().read_snapshot(&third.snapshot).is_some());
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
        .visibility_reads().execute_read_packet(
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
    let result = runtime.visibility_reads().execute_read_packet(&snapshot, &packet).unwrap();
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
        .harness_audit(crate::facade::HarnessAuditMode::Disabled)
        .into_results();
    let enabled_results = runtime
        .invariant_access()
        .harness_audit(crate::facade::HarnessAuditMode::Full)
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
