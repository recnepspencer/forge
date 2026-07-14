use super::*;

pub(super) fn ingest_bundle() -> crate::Milestone9CertificationBundle {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "bulk-cert-ingest-a");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "entity-a-updated", None);
    let second = latest_envelope(&runtime);

    let mut bulk_store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
        .execute_next_resumed_bulk_chunk(&resumed, 1, second.clone(), BulkCheckpointPolicy::Publish)
        .unwrap()
        .expect("second ingest chunk should execute");
    let bulk_export = bulk_store.export_authoritative_records();

    let mut control_store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    control_store
        .append_canonical_commit(first.clone())
        .unwrap();
    control_store
        .append_canonical_commit(second.clone())
        .unwrap();
    let control_export = control_store.export_authoritative_records();

    let rebuilt =
        WORTHStore::restore_from_authoritative_export(bulk_export.clone().admit_restore()).unwrap();
    let rebuilt_export = rebuilt.export_authoritative_records();

    let mut equivalent_plan_store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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

#[test]
fn milestone_9_certification_bundle_proves_ingest_resume_control_and_restore_parity() {
    let bundle = ingest_bundle();
    assert_certification_core(&bundle, BulkPlanKind::Ingest, 2);
    assert_eq!(bundle.counter_snapshot.bulk_chunk_resume_count, 1);
    assert_eq!(bundle.counter_snapshot.bulk_checkpoint_write_count, 2);
    assert_eq!(bundle.counter_snapshot.bulk_chunk_width_units, 2);
    assert_eq!(bundle.counter_snapshot.bulk_peak_in_flight_memory_units, 1);
}
