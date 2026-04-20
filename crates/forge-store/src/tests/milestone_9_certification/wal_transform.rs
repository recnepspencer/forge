use super::*;

pub(super) fn wal_recovered_transform_bundle() -> crate::Milestone9CertificationBundle {
    let path = unique_test_store_path("forge-store-m9-cert-wal-recovered-transform");

    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-cert-wal-transform-alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let request = BulkTransformRequest::new(
        "program-cert-wal-transform",
        "transform-cert-wal",
        envelope.branch_context.clone(),
        envelope.commit.commit_id,
        vec![BulkSourceMember::new("a", 1)],
    );
    let basis = store.freeze_bulk_transform_basis(request.clone()).unwrap();
    let partition = store
        .freeze_bulk_transform_target_partition(request, &basis)
        .unwrap();
    let plan = store
        .plan_bulk_transform(&basis, &partition, ChunkWidthBudget::new(1))
        .unwrap();
    let admitted = store
        .admit_bulk_transform_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, envelope.clone())
        .unwrap();
    let runtime_session_id = request.runtime_session_id().to_string();
    let operation_name = request.operation_name().to_string();
    let durable_mutation_id = store
        .admit_durable_mutation(&runtime_session_id, &operation_name)
        .unwrap();
    store
        .record_hosted_runtime_commit_result(
            &runtime_session_id,
            durable_mutation_id,
            request.canonical_envelope().clone(),
        )
        .unwrap();
    store
        .record_bulk_checkpoint_publication_intent(
            &runtime_session_id,
            durable_mutation_id,
            Some(1),
        )
        .unwrap();
    store
        .record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            crate::DurablePublicationPhase::CanonicalCommitProduced,
            Some(envelope.commit.commit_id),
        )
        .unwrap();
    drop(store);

    let recovered = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();
    let recovered_export = recovered.store().export_authoritative_records();

    let mut control_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    control_store
        .append_canonical_commit(envelope.clone())
        .unwrap();
    let control_export = control_store.export_authoritative_records();

    let rebuilt =
        ForgeStore::restore_from_authoritative_export(recovered_export.clone().admit_restore())
            .unwrap();
    let rebuilt_export = rebuilt.export_authoritative_records();

    let mut equivalent_plan_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let equivalent_request = BulkTransformRequest::new(
        "program-cert-wal-transform",
        "transform-cert-wal",
        envelope.branch_context.clone(),
        envelope.commit.commit_id,
        vec![BulkSourceMember::new("a", 1)],
    );
    let equivalent_basis = equivalent_plan_store
        .freeze_bulk_transform_basis(equivalent_request.clone())
        .unwrap();
    let equivalent_partition = equivalent_plan_store
        .freeze_bulk_transform_target_partition(equivalent_request, &equivalent_basis)
        .unwrap();
    let equivalent_plan = equivalent_plan_store
        .plan_bulk_transform(
            &equivalent_basis,
            &equivalent_partition,
            ChunkWidthBudget::new(1),
        )
        .unwrap();

    Milestone9CertificationBundle::new(
        &recovered_export,
        &control_export,
        &rebuilt_export,
        &plan,
        &equivalent_plan,
        recovered.store().counters(),
    )
}

#[test]
fn milestone_9_certification_bundle_proves_ingest_resume_control_and_restore_parity() {
    let bundle = ingest_bundle();
    assert_certification_core(&bundle, BulkPlanKind::Ingest, 2);
    assert_eq!(bundle.counter_snapshot.bulk_chunk_resume_count, 1);
    assert_eq!(bundle.counter_snapshot.bulk_checkpoint_write_count, 2);
    assert_eq!(bundle.counter_snapshot.bulk_chunk_width_units, 2);
    assert_eq!(bundle.counter_snapshot.bulk_peak_in_flight_memory_units, 1);
}

#[test]
fn milestone_9_certification_bundle_proves_transform_resume_control_and_restore_parity() {
    let bundle = transform_bundle();
    assert_certification_core(&bundle, BulkPlanKind::Transform, 2);
    assert_eq!(bundle.counter_snapshot.bulk_chunk_resume_count, 1);
    assert_eq!(bundle.counter_snapshot.bulk_checkpoint_write_count, 2);
    assert_eq!(bundle.counter_snapshot.bulk_chunk_width_units, 2);
    assert_eq!(bundle.counter_snapshot.bulk_peak_in_flight_memory_units, 1);
}

#[test]
fn milestone_9_certification_bundle_proves_wal_recovered_ingest_control_and_restore_parity() {
    let bundle = wal_recovered_ingest_bundle();
    assert_certification_core(&bundle, BulkPlanKind::Ingest, 1);
    assert_eq!(bundle.counter_snapshot.durable_commit_recovered_count, 1);
    assert_eq!(bundle.counter_snapshot.bulk_checkpoint_write_count, 1);
    assert_eq!(bundle.counter_snapshot.bulk_chunk_witness_write_count, 1);
}

#[test]
fn milestone_9_certification_bundle_proves_wal_recovered_transform_control_and_restore_parity() {
    let bundle = wal_recovered_transform_bundle();
    assert_certification_core(&bundle, BulkPlanKind::Transform, 1);
    assert_eq!(bundle.counter_snapshot.durable_commit_recovered_count, 1);
    assert_eq!(bundle.counter_snapshot.bulk_checkpoint_write_count, 1);
    assert_eq!(bundle.counter_snapshot.bulk_chunk_witness_write_count, 1);
}
