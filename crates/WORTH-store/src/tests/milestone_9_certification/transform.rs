use super::*;

pub(super) fn transform_bundle() -> crate::Milestone9CertificationBundle {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "bulk-cert-transform-a");
    let first = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "entity-a-updated", None);
    let second = latest_envelope(&runtime);

    let mut bulk_store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let request = BulkTransformRequest::new(
        "program-cert-transform",
        "transform-cert",
        first.branch_context.clone(),
        first.commit.commit_id,
        vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 1)],
    );
    let basis = bulk_store
        .freeze_bulk_transform_basis(request.clone())
        .unwrap();
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
        .execute_next_resumed_bulk_chunk(&resumed, 1, second.clone(), BulkCheckpointPolicy::Publish)
        .unwrap()
        .expect("second transform chunk should execute");
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
        .plan_bulk_transform(
            &equivalent_basis,
            &equivalent_partition,
            ChunkWidthBudget::new(1),
        )
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
fn milestone_9_certification_bundle_proves_transform_resume_control_and_restore_parity() {
    let bundle = transform_bundle();
    assert_certification_core(&bundle, BulkPlanKind::Transform, 2);
    assert_eq!(bundle.counter_snapshot.bulk_chunk_resume_count, 1);
    assert_eq!(bundle.counter_snapshot.bulk_checkpoint_write_count, 2);
    assert_eq!(bundle.counter_snapshot.bulk_chunk_width_units, 2);
    assert_eq!(bundle.counter_snapshot.bulk_peak_in_flight_memory_units, 1);
}
