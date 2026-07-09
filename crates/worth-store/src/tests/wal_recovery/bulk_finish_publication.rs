use super::*;

#[test]
fn bulk_finish_publication_recovery_reports_typed_bulk_identity() {
    let path = unique_test_store_path("worth-store-bulk-finish-publication");
    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .expect("bulk store should build");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-recovery-alpha");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "bulk-program-recover",
            "bulk-source-recover",
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
    drop(store);

    let recovered = WORTHStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("bulk recovery should complete");

    let report = recovered
        .last_recovery()
        .source_reports
        .iter()
        .find(|report| report.durable_mutation_id() == durable_mutation_id)
        .expect("bulk mutation source report should be present");
    assert_eq!(
        report.source_kind(),
        crate::RecoverySourceKind::HostedRuntimeCanonicalResult
    );
    assert_eq!(
        report.mutation_identity(),
        &DurableMutationIdentity::BulkChunk {
            plan_kind: BulkPlanKind::Ingest,
            program_id: "bulk-program-recover".to_string(),
            plan_id: plan.plan_id().to_string(),
            chunk_ordinal: 0,
        }
    );
    let status = recovered.recovery_status_report().unwrap();
    assert_eq!(status.bulk_summary().total_chunks(), 1);
    assert_eq!(status.bulk_summary().ingest_chunks(), 1);
    assert_eq!(status.bulk_summary().resume_ready(), 1);
    assert_eq!(status.bulk_summary().hosted_runtime_canonical_result(), 1);
    assert_eq!(status.bulk_chunks().len(), 1);
    let bulk_chunk = &status.bulk_chunks()[0];
    assert_eq!(bulk_chunk.durable_mutation_id(), durable_mutation_id);
    assert_eq!(bulk_chunk.plan_kind(), BulkPlanKind::Ingest);
    assert_eq!(bulk_chunk.program_id(), "bulk-program-recover");
    assert_eq!(bulk_chunk.plan_id(), plan.plan_id());
    assert_eq!(bulk_chunk.chunk_ordinal(), 0);
    assert_eq!(
        bulk_chunk.disposition(),
        BulkRecoveryDisposition::ResumeReady
    );
    assert_eq!(
        bulk_chunk.decision(),
        Some(RecoveryDecisionClass::FinishPublicationFromCanonicalResult)
    );
    assert_eq!(bulk_chunk.commit_id(), Some(envelope.commit.commit_id));
    assert_eq!(
        bulk_chunk.source_kind(),
        RecoverySourceKind::HostedRuntimeCanonicalResult
    );
    let recovered_resume = recovered
        .admit_recovered_bulk_chunk_resume(&bulk_chunk.admit_resume().unwrap())
        .expect("bulk recovery should reconstruct a resume-ready program");
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
        .fetch_program_chunk_witness_index("bulk-program-recover", plan.plan_id())
        .expect("bulk recovery should publish a witness index");
    assert_eq!(
        witness_index.highest_committed_chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    assert_eq!(witness_index.latest_checkpoint_sequence(), Some(1));
    let checkpoint = recovered
        .store()
        .fetch_bulk_progress_checkpoint("bulk-program-recover", plan.plan_id())
        .expect("bulk recovery should publish the requested checkpoint");
    assert_eq!(checkpoint.checkpoint_sequence(), 1);
    assert_eq!(
        recovered.resolve_retry(durable_mutation_id),
        Ok(
            crate::DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit {
                commit_id: envelope.commit.commit_id
            }
        )
    );
    let counters = recovered.store().counters();
    assert_eq!(counters.bulk_chunk_witness_write_count, 1);
    assert_eq!(counters.bulk_checkpoint_write_count, 1);
    assert_eq!(counters.bulk_chunk_commit_count, 1);
    assert_eq!(counters.durable_commit_acknowledged_count, 1);
}
