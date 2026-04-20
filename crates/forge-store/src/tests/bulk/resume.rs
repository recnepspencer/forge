use super::*;

#[test]
fn bulk_checkpoint_publication_rejects_non_advancing_duplicate_witness() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-checkpoint-duplicate");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-checkpoint-duplicate",
            "source-checkpoint-duplicate",
            envelope.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 1)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(2))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 2)
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let witness = store
        .publish_bulk_chunk_witness(&admitted, envelope.commit.commit_id)
        .unwrap();

    let first_checkpoint = store.publish_bulk_progress_checkpoint(&witness).unwrap();
    let error = store
        .publish_bulk_progress_checkpoint(&witness)
        .expect_err("duplicate checkpoint publication for the same witness should fail");

    assert_eq!(first_checkpoint.checkpoint_sequence(), 1);
    assert_eq!(error.kind(), &StoreErrorKind::BulkCheckpointPublicationGap);
}

#[test]
fn bulk_ingest_resume_admission_handles_not_started_programs_explicitly() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-resume-ingest",
            "source-resume-ingest",
            BranchId("bulk-main".to_string()),
            vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 2)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();

    let resumed = store
        .admit_bulk_ingest_resume(
            "program-resume-ingest",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();

    assert_eq!(resumed.plan().plan_id(), plan.plan_id());
    assert!(resumed.witness_index().is_none());
    assert!(resumed.latest_checkpoint().is_none());
    assert_eq!(
        resumed.resume_boundary().latest_committed_chunk_ordinal(),
        None
    );
    assert_eq!(
        resumed.resume_boundary().next_chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    let admitted = resumed
        .admit_next_chunk(3)
        .expect("next chunk admission should succeed")
        .expect("not-started program should admit first chunk");
    assert_eq!(admitted.chunk().ordinal(), ChunkOrdinal::new(0));
    assert_eq!(store.counters().bulk_chunk_resume_count, 1);
}

#[test]
fn bulk_transform_resume_admission_requires_locked_basis_and_resume_state() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-transform-resume-authority");
    let envelope = latest_envelope(&runtime);
    let request = BulkTransformRequest::new(
        "program-resume-transform",
        "transform-resume",
        envelope.branch_context.clone(),
        CommitId(44),
        vec![
            BulkSourceMember::new("alpha", 1),
            BulkSourceMember::new("beta", 2),
            BulkSourceMember::new("gamma", 1),
        ],
    );
    let basis = store.freeze_bulk_transform_basis(request.clone()).unwrap();
    let partition = store
        .freeze_bulk_transform_target_partition(request, &basis)
        .unwrap();
    let plan = store
        .plan_bulk_transform(&basis, &partition, ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_transform_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let witness = store
        .publish_bulk_chunk_witness(&admitted, envelope.commit.commit_id)
        .unwrap();
    let checkpoint = store.publish_bulk_progress_checkpoint(&witness).unwrap();

    let resumed = store
        .admit_bulk_transform_resume(
            "program-resume-transform",
            plan.plan_id(),
            basis.basis_digest(),
            partition.partition_digest(),
        )
        .unwrap();

    assert_eq!(resumed.plan().plan_id(), plan.plan_id());
    assert_eq!(
        resumed
            .witness_index()
            .expect("started program must expose witness index")
            .highest_committed_chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    assert_eq!(
        resumed
            .latest_checkpoint()
            .expect("started program must expose latest checkpoint")
            .checkpoint_sequence(),
        checkpoint.checkpoint_sequence()
    );
    assert_eq!(
        resumed.resume_boundary().latest_committed_chunk_ordinal(),
        Some(ChunkOrdinal::new(0))
    );
    assert_eq!(
        resumed.resume_boundary().next_chunk_ordinal(),
        ChunkOrdinal::new(1)
    );
    let admitted = resumed
        .admit_next_chunk(3)
        .expect("next chunk admission should succeed")
        .expect("partially completed transform should admit next chunk");
    assert_eq!(admitted.chunk().ordinal(), ChunkOrdinal::new(1));

    let error = store
        .admit_bulk_transform_resume(
            "program-resume-transform",
            plan.plan_id(),
            "wrong-basis",
            partition.partition_digest(),
        )
        .expect_err("resume admission must reject mismatched locked basis");
    assert_eq!(error.kind(), &StoreErrorKind::BulkTransformBasisDrift);
}

#[test]
fn bulk_resume_ready_program_reports_completion_without_admitting_more_chunks() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-complete");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-complete",
            "source-complete",
            envelope.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 1)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(2))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 2)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, envelope)
        .unwrap();
    store
        .execute_bulk_canonical_chunk(request, BulkCheckpointPolicy::Publish)
        .unwrap();

    let resumed = store
        .admit_bulk_ingest_resume(
            "program-complete",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();

    assert!(resumed.is_complete());
    assert_eq!(resumed.next_chunk_ordinal(), ChunkOrdinal::new(1));
    assert!(resumed
        .admit_next_chunk(2)
        .expect("completion check should succeed")
        .is_none());
}

