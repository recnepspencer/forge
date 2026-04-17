use crate::{
    BulkCheckpointPolicy, BulkIngestSourceRequest, BulkPlanKind, BulkSourceMember,
    BulkTransformRequest, ChunkOrdinal, ChunkWidthBudget, ForgeStore,
    ForgeStoreBuilder, Milestone9CertificationBundle,
};

use super::harness::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};
use super::harness::fixtures::stores::unique_test_store_path;

fn assert_certification_core(
    bundle: &Milestone9CertificationBundle,
    plan_kind: BulkPlanKind,
    chunk_count: usize,
) {
    assert_eq!(bundle.plan_kind, plan_kind);
    assert_eq!(bundle.chunk_count, chunk_count);
    assert!(bundle.certification_summary.truth_matches_control_lane);
    assert!(bundle.certification_summary.history_matches_control_lane);
    assert!(bundle.certification_summary.restore_truth_parity);
    assert!(bundle.certification_summary.restore_history_parity);
    assert!(bundle.certification_summary.deterministic_chunk_plan_observed);
    assert!(!bundle.chunk_plan_digest.is_empty());
}

fn ingest_bundle() -> crate::Milestone9CertificationBundle {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "bulk-cert-ingest-a");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "entity-a-updated", None);
    let second = latest_envelope(&runtime);

    let mut bulk_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let manifest = bulk_store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-cert-ingest",
            "source-cert-ingest",
            first.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 1)],
        ))
        .unwrap();
    let plan = bulk_store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(1))
        .unwrap();
    let admitted_first = bulk_store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();
    let request_first = bulk_store
        .admit_bulk_canonical_chunk_execution(admitted_first, first.clone())
        .unwrap();
    bulk_store
        .execute_bulk_canonical_chunk(request_first, BulkCheckpointPolicy::Publish)
        .unwrap();

    let resumed = bulk_store
        .admit_bulk_ingest_resume(
            "program-cert-ingest",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();
    bulk_store
        .execute_next_resumed_bulk_chunk(
            &resumed,
            1,
            second.clone(),
            BulkCheckpointPolicy::Publish,
        )
        .unwrap()
        .expect("second ingest chunk should execute");
    let bulk_export = bulk_store.export_authoritative_records();

    let mut control_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    control_store
        .append_canonical_commit(first.clone())
        .unwrap();
    control_store
        .append_canonical_commit(second.clone())
        .unwrap();
    let control_export = control_store.export_authoritative_records();

    let rebuilt =
        ForgeStore::restore_from_authoritative_export(bulk_export.clone().admit_restore()).unwrap();
    let rebuilt_export = rebuilt.export_authoritative_records();

    let mut equivalent_plan_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let equivalent_manifest = equivalent_plan_store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-cert-ingest",
            "source-cert-ingest",
            first.branch_context.clone(),
            vec![BulkSourceMember::new("b", 1), BulkSourceMember::new("a", 1)],
        ))
        .unwrap();
    let equivalent_plan = equivalent_plan_store
        .plan_bulk_ingest(equivalent_manifest, ChunkWidthBudget::new(1))
        .unwrap();

    Milestone9CertificationBundle::new(
        &bulk_export,
        &control_export,
        &rebuilt_export,
        &plan,
        &equivalent_plan,
        bulk_store.counters(),
    )
}

fn transform_bundle() -> crate::Milestone9CertificationBundle {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "bulk-cert-transform-a");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "entity-a-updated", None);
    let second = latest_envelope(&runtime);

    let mut bulk_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let request = BulkTransformRequest::new(
        "program-cert-transform",
        "transform-cert",
        first.branch_context.clone(),
        first.commit.commit_id,
        vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 1)],
    );
    let basis = bulk_store.freeze_bulk_transform_basis(request.clone()).unwrap();
    let partition = bulk_store
        .freeze_bulk_transform_target_partition(request, &basis)
        .unwrap();
    let plan = bulk_store
        .plan_bulk_transform(&basis, &partition, ChunkWidthBudget::new(1))
        .unwrap();
    let admitted_first = bulk_store
        .admit_bulk_transform_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();
    let request_first = bulk_store
        .admit_bulk_canonical_chunk_execution(admitted_first, first.clone())
        .unwrap();
    bulk_store
        .execute_bulk_canonical_chunk(request_first, BulkCheckpointPolicy::Publish)
        .unwrap();

    let resumed = bulk_store
        .admit_bulk_transform_resume(
            "program-cert-transform",
            plan.plan_id(),
            basis.basis_digest(),
            partition.partition_digest(),
        )
        .unwrap();
    bulk_store
        .execute_next_resumed_bulk_chunk(
            &resumed,
            1,
            second.clone(),
            BulkCheckpointPolicy::Publish,
        )
        .unwrap()
        .expect("second transform chunk should execute");
    let bulk_export = bulk_store.export_authoritative_records();

    let mut control_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    control_store
        .append_canonical_commit(first.clone())
        .unwrap();
    control_store
        .append_canonical_commit(second.clone())
        .unwrap();
    let control_export = control_store.export_authoritative_records();

    let rebuilt =
        ForgeStore::restore_from_authoritative_export(bulk_export.clone().admit_restore()).unwrap();
    let rebuilt_export = rebuilt.export_authoritative_records();

    let mut equivalent_plan_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let equivalent_request = BulkTransformRequest::new(
        "program-cert-transform",
        "transform-cert",
        first.branch_context.clone(),
        first.commit.commit_id,
        vec![BulkSourceMember::new("b", 1), BulkSourceMember::new("a", 1)],
    );
    let equivalent_basis = equivalent_plan_store
        .freeze_bulk_transform_basis(equivalent_request.clone())
        .unwrap();
    let equivalent_partition = equivalent_plan_store
        .freeze_bulk_transform_target_partition(equivalent_request, &equivalent_basis)
        .unwrap();
    let equivalent_plan = equivalent_plan_store
        .plan_bulk_transform(&equivalent_basis, &equivalent_partition, ChunkWidthBudget::new(1))
        .unwrap();

    Milestone9CertificationBundle::new(
        &bulk_export,
        &control_export,
        &rebuilt_export,
        &plan,
        &equivalent_plan,
        bulk_store.counters(),
    )
}

fn wal_recovered_ingest_bundle() -> crate::Milestone9CertificationBundle {
    let path = unique_test_store_path("forge-store-m9-cert-wal-recovered");

    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-cert-wal-alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().local_file(path.clone()).build().unwrap();
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-cert-wal-ingest",
            "source-cert-wal-ingest",
            envelope.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(1))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 1)
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
        .record_bulk_checkpoint_publication_intent(&runtime_session_id, durable_mutation_id, Some(1))
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
    control_store.append_canonical_commit(envelope.clone()).unwrap();
    let control_export = control_store.export_authoritative_records();

    let rebuilt = ForgeStore::restore_from_authoritative_export(
        recovered_export.clone().admit_restore(),
    )
    .unwrap();
    let rebuilt_export = rebuilt.export_authoritative_records();

    let mut equivalent_plan_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let equivalent_manifest = equivalent_plan_store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-cert-wal-ingest",
            "source-cert-wal-ingest",
            envelope.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1)],
        ))
        .unwrap();
    let equivalent_plan = equivalent_plan_store
        .plan_bulk_ingest(equivalent_manifest, ChunkWidthBudget::new(1))
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

fn wal_recovered_transform_bundle() -> crate::Milestone9CertificationBundle {
    let path = unique_test_store_path("forge-store-m9-cert-wal-recovered-transform");

    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-cert-wal-transform-alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().local_file(path.clone()).build().unwrap();
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
        .record_bulk_checkpoint_publication_intent(&runtime_session_id, durable_mutation_id, Some(1))
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
    control_store.append_canonical_commit(envelope.clone()).unwrap();
    let control_export = control_store.export_authoritative_records();

    let rebuilt = ForgeStore::restore_from_authoritative_export(
        recovered_export.clone().admit_restore(),
    )
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
        .plan_bulk_transform(&equivalent_basis, &equivalent_partition, ChunkWidthBudget::new(1))
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
