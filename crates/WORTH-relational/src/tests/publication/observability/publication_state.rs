use super::fixtures::*;

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
        Some(bundle.patch.authoritative_record_patches.len())
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
