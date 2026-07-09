use super::*;

#[test]
fn retained_bulk_truth_uses_bulk_identity_in_operator_actions() {
    let path = unique_test_store_path("worth-store-bulk-retained-without-ack");
    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .expect("bulk store should build");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-recovery-beta");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "bulk-program-retained",
            "bulk-source-retained",
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
    let runtime_session_id = request.runtime_session_id();
    let operation_name = request.operation_name();
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
        .record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::CanonicalCommitProduced,
            Some(envelope.commit.commit_id),
        )
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AuthoritativeAppendPublished,
            Some(envelope.commit.commit_id),
        )
        .unwrap();
    drop(store);

    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("bulk recovery should complete");

    let status = recovered.recovery_status_report().unwrap();
    assert_eq!(
        status.operator_disposition(),
        RecoveryOperatorDisposition::RetainedWithoutAcknowledgment
    );
    assert_eq!(status.recommended_actions().len(), 1);
    assert_eq!(
        status.recommended_actions()[0].scope_identity(),
        format!(
            "bulk:ingest:{}:{}:chunk:0",
            "bulk-program-retained",
            plan.plan_id()
        )
    );
    assert_eq!(status.bulk_summary().total_chunks(), 1);
    assert_eq!(status.bulk_summary().already_published(), 1);
    assert_eq!(status.bulk_summary().published_authoritative_truth(), 1);
    assert_eq!(status.bulk_chunks().len(), 1);
    let bulk_chunk = &status.bulk_chunks()[0];
    assert_eq!(bulk_chunk.program_id(), "bulk-program-retained");
    assert_eq!(bulk_chunk.plan_id(), plan.plan_id());
    assert_eq!(bulk_chunk.chunk_ordinal(), 0);
    assert_eq!(
        bulk_chunk.disposition(),
        BulkRecoveryDisposition::AlreadyPublished
    );
    assert_eq!(
        bulk_chunk.decision(),
        Some(RecoveryDecisionClass::RetainPublishedTruth)
    );
    assert_eq!(
        bulk_chunk.source_kind(),
        RecoverySourceKind::PublishedAuthoritativeTruth
    );
    let recovered_resume = recovered
        .admit_recovered_bulk_chunk_resume(&bulk_chunk.admit_resume().unwrap())
        .expect("already-published bulk chunk should still reconstruct program resume state");
    assert_eq!(
        recovered_resume.resumed_chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    assert_eq!(
        recovered_resume.resumed_program().plan().plan_id(),
        plan.plan_id()
    );
    assert_eq!(
        recovered_resume.resumed_program().next_chunk_ordinal(),
        ChunkOrdinal::new(1)
    );
    let witness_index = recovered
        .store()
        .fetch_program_chunk_witness_index("bulk-program-retained", plan.plan_id())
        .expect("published truth recovery should reconstruct the chunk witness");
    assert_eq!(
        witness_index.highest_committed_chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    assert_eq!(witness_index.latest_checkpoint_sequence(), None);
}

#[test]
fn retained_bulk_truth_with_checkpoint_intent_recovers_checkpoint_artifacts() {
    let path = unique_test_store_path("worth-store-bulk-retained-with-checkpoint-intent");
    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .expect("bulk store should build");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-recovery-gamma");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "bulk-program-retained-checkpoint",
            "bulk-source-retained-checkpoint",
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
    let runtime_session_id = request.runtime_session_id();
    let operation_name = request.operation_name();
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
            DurablePublicationPhase::CanonicalCommitProduced,
            Some(envelope.commit.commit_id),
        )
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AuthoritativeAppendPublished,
            Some(envelope.commit.commit_id),
        )
        .unwrap();
    drop(store);

    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("bulk recovery should complete");

    let witness_index = recovered
        .store()
        .fetch_program_chunk_witness_index("bulk-program-retained-checkpoint", plan.plan_id())
        .expect("published truth recovery should reconstruct the chunk witness");
    assert_eq!(
        witness_index.highest_committed_chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    assert_eq!(witness_index.latest_checkpoint_sequence(), Some(1));
    let checkpoint = recovered
        .store()
        .fetch_bulk_progress_checkpoint("bulk-program-retained-checkpoint", plan.plan_id())
        .expect("published truth recovery should reconstruct the requested checkpoint");
    assert_eq!(checkpoint.checkpoint_sequence(), 1);
    let status = recovered.recovery_status_report().unwrap();
    assert_eq!(status.bulk_summary().already_published(), 1);
}
