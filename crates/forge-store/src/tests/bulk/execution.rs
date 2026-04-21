use super::*;

#[test]
fn bulk_execute_next_resumed_chunk_finalizes_witness_and_checkpoint() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-execute-a");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-execute",
            "source-execute",
            envelope.branch_context.clone(),
            vec![
                BulkSourceMember::new("a", 1),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();
    let resumed = store
        .admit_bulk_ingest_resume(
            "program-execute",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();

    let outcome = store
        .execute_next_resumed_bulk_chunk(
            &resumed,
            3,
            envelope.clone(),
            BulkCheckpointPolicy::Publish,
        )
        .unwrap()
        .expect("not-started program should execute its first chunk");

    assert_eq!(
        outcome.materialization_receipt().chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    assert_eq!(
        outcome.chunk_commit_witness().canonical_commit_id(),
        envelope.commit.commit_id
    );
    assert_eq!(
        outcome
            .published_checkpoint()
            .expect("checkpoint should be published")
            .checkpoint_sequence(),
        1
    );
    let counters = store.counters();
    assert_eq!(counters.bulk_chunk_commit_count, 1);
    assert_eq!(counters.bulk_chunk_witness_write_count, 1);
    assert_eq!(counters.bulk_checkpoint_write_count, 1);
}

#[test]
fn bulk_execute_next_resumed_chunk_advances_checkpoint_sequence_for_started_programs() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-execute-b");
    let first_envelope = latest_envelope(&runtime);
    create_entity(&mut runtime, "bulk-execute-c");
    let second_envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-execute-continued",
            "source-execute-continued",
            first_envelope.branch_context.clone(),
            vec![
                BulkSourceMember::new("a", 1),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
                BulkSourceMember::new("d", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, first_envelope)
        .unwrap();
    store
        .execute_bulk_canonical_chunk(request, BulkCheckpointPolicy::Publish)
        .unwrap();

    let resumed = store
        .admit_bulk_ingest_resume(
            "program-execute-continued",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();
    let outcome = store
        .execute_next_resumed_bulk_chunk(
            &resumed,
            3,
            second_envelope,
            BulkCheckpointPolicy::Publish,
        )
        .unwrap()
        .expect("partially completed program should execute next chunk");

    assert_eq!(
        outcome.materialization_receipt().chunk_ordinal(),
        ChunkOrdinal::new(1)
    );
    assert_eq!(
        outcome
            .published_checkpoint()
            .expect("checkpoint should be published")
            .checkpoint_sequence(),
        2
    );
}

#[test]
fn bulk_execute_canonical_chunk_durably_records_wal_lifecycle_and_acknowledgment() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-durable-a");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-durable",
            "source-durable",
            envelope.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 2)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, envelope.clone())
        .unwrap();

    let durable = store
        .execute_bulk_canonical_chunk_durably(request, BulkCheckpointPolicy::Publish)
        .unwrap();

    assert_eq!(
        durable
            .execution_outcome()
            .chunk_commit_witness()
            .canonical_commit_id(),
        envelope.commit.commit_id
    );
    assert_eq!(
        durable
            .execution_outcome()
            .published_checkpoint()
            .expect("durable execution should publish a checkpoint")
            .checkpoint_sequence(),
        1
    );
    assert_eq!(
        store
            .resolve_durable_retry(durable.durable_mutation_id())
            .unwrap(),
        DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit {
            commit_id: envelope.commit.commit_id
        }
    );
    let counters = store.counters();
    assert_eq!(counters.durable_mutation_admit_count, 1);
    assert_eq!(counters.durable_commit_acknowledged_count, 1);
    assert_eq!(counters.wal_record_append_count, 6);
}

#[test]
fn bulk_execute_next_resumed_chunk_durably_advances_checkpoint_sequence() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-durable-b");
    let first_envelope = latest_envelope(&runtime);
    create_entity(&mut runtime, "bulk-durable-c");
    let second_envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-durable-resume",
            "source-durable-resume",
            first_envelope.branch_context.clone(),
            vec![
                BulkSourceMember::new("a", 1),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
                BulkSourceMember::new("d", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, first_envelope)
        .unwrap();
    store
        .execute_bulk_canonical_chunk(request, BulkCheckpointPolicy::Publish)
        .unwrap();

    let resumed = store
        .admit_bulk_ingest_resume(
            "program-durable-resume",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();
    let durable = store
        .execute_next_resumed_bulk_chunk_durably(
            &resumed,
            3,
            second_envelope.clone(),
            BulkCheckpointPolicy::Publish,
        )
        .unwrap()
        .expect("partially completed program should durably execute next chunk");

    assert_eq!(
        durable
            .execution_outcome()
            .materialization_receipt()
            .chunk_ordinal(),
        ChunkOrdinal::new(1)
    );
    assert_eq!(
        durable
            .execution_outcome()
            .published_checkpoint()
            .expect("durable resumed execution should publish a checkpoint")
            .checkpoint_sequence(),
        2
    );
    assert_eq!(
        store
            .resolve_durable_retry(durable.durable_mutation_id())
            .unwrap(),
        DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit {
            commit_id: second_envelope.commit.commit_id
        }
    );
}

#[test]
fn bulk_canonical_chunk_execution_request_rejects_branch_mismatch() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-branch-mismatch",
            "source-branch-mismatch",
            BranchId("bulk-main".to_string()),
            vec![BulkSourceMember::new("a", 1)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(1))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();

    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-wrong-branch");
    let mut wrong_envelope = latest_envelope(&runtime);
    wrong_envelope.branch_context = BranchId("some-other-branch".to_string());

    let error = store
        .admit_bulk_canonical_chunk_execution(admitted, wrong_envelope)
        .expect_err("branch mismatch must reject before append");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::ConcurrentBulkBoundaryViolation
    );
}
