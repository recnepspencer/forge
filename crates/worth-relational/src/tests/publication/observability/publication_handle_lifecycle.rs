use super::fixtures::*;

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
fn pruned_publication_version_cannot_mint_historical_execution_authority() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .publication(PublicationConfig {
            coherent_publication_required: true,
            max_patch_records_per_commit: 4096,
            max_published_snapshot_handles: 2,
        })
        .build();
    let first = create_entity_outcome(&mut runtime, "first");
    let first_branch = first.snapshot.branch_id.clone();
    let entity = changed_entities(&first)[0];
    update_entity(&mut runtime, entity, "second");
    update_entity(&mut runtime, entity, "third");
    update_entity(&mut runtime, entity, "fourth");

    assert!(runtime
        .read_truth()
        .read_snapshot(&first.snapshot)
        .is_none());
    let denial = match runtime
        .snapshots()
        .admit_execution_basis(&first_branch, first.version_id)
    {
        Ok(_) => panic!("pruned publication version admitted execution authority"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial.kind(),
        crate::facade::runtime::RelationalExecutionBasisDenialKind::VersionUnavailable
    );
    assert_eq!(denial.counters().snapshot_identity_allocation_count(), 0);
    assert_eq!(denial.counters().lease_registry_insert_count(), 0);
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
