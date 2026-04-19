use crate::{
    modes::SimulatedCrashPoint, BulkIngestSourceRequest, BulkPlanKind, BulkRecoveryDisposition,
    BulkSourceMember, ChunkOrdinal, ChunkWidthBudget, DurableMutationIdentity,
    DurableMutationRequest, DurablePublicationPhase, ForgeStore, ForgeStoreBuilder,
    RecoveryDecisionClass, RecoveryOperatorActionKind, RecoveryOperatorDisposition,
    RecoverySourceKind,
};
use forge_relational::facade::replay::CanonicalCommitEnvelope;

use super::harness::{
    corruption::local_file::{force_branch_head_gap, force_publication_commit_id_conflict},
    fixtures::{
        runtime::{create_entity, create_entity_commit, latest_envelope, runtime_with_demo_schema},
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
};

fn create_alpha_commit(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "alpha"))
}

fn create_beta_commit(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "beta"))
}

fn prepare_pending_bulk_ingest_mutation(
    store: &mut ForgeStore,
    program_id: &str,
    source_identity: &str,
    include_checkpoint_intent: bool,
) -> (
    crate::DeterministicChunkPlan,
    CanonicalCommitEnvelope,
    String,
    crate::wal::DurableMutationId,
) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, &format!("{program_id}-entity"));
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            program_id,
            source_identity,
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
    if include_checkpoint_intent {
        store
            .record_bulk_checkpoint_publication_intent(
                &runtime_session_id,
                durable_mutation_id,
                Some(1),
            )
            .unwrap();
    }
    store
        .record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::CanonicalCommitProduced,
            Some(envelope.commit.commit_id),
        )
        .unwrap();
    (plan, envelope, runtime_session_id, durable_mutation_id)
}

#[test]
fn crash_before_ack_discards_unpublished_intent() {
    let path = unique_test_store_path("forge-store-m3-before-ack");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .expect("durable store should build");

    let durable_mutation_id = durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-alpha", create_alpha_commit),
            SimulatedCrashPoint::AfterIntentRecorded,
        )
        .expect("crash simulation should record durable intent");
    drop(durable);

    let reopened_runtime = runtime_with_demo_schema();
    let recovery_handle = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(reopened_runtime)
        .build_pending()
        .expect("recovery handle should build");
    let plan = recovery_handle.plan();
    assert_eq!(plan.pending_durable_mutation_ids, vec![durable_mutation_id]);
    let recovered = recovery_handle.recover().expect("recovery should complete");

    assert!(recovered
        .store()
        .export_authoritative_records()
        .commit_envelopes
        .is_empty());
    assert_eq!(recovered.last_recovery().decisions.len(), 1);
    assert_eq!(
        recovered.last_recovery().decisions[0].durable_mutation_id,
        durable_mutation_id
    );
    assert_eq!(
        recovered.last_recovery().decisions[0].decision,
        RecoveryDecisionClass::DiscardUnpublished
    );
    assert_eq!(recovered.last_recovery().degraded.len(), 0);
    assert_eq!(recovered.last_recovery().source_reports.len(), 1);

    let counters = recovered.store().counters();
    assert_eq!(counters.durable_commit_unacknowledged_discard_count, 1);
    assert_eq!(counters.durable_commit_recovered_count, 0);
    assert_eq!(counters.recovery_source_precedence_resolution_count, 1);
    assert_eq!(counters.recovery_source_precedence_fallback_count, 1);
    assert_eq!(counters.recovery_non_quiescent_restart_count, 1);
    assert_eq!(counters.recovery_quiescent_restart_count, 0);
}

#[test]
fn crash_after_authoritative_publication_retains_truth_and_resolves_retry() {
    let path = unique_test_store_path("forge-store-m3-after-publish");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .expect("durable store should build");

    let durable_mutation_id = durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-alpha", create_alpha_commit),
            SimulatedCrashPoint::AfterAuthoritativeAppendPublished,
        )
        .expect("crash simulation should publish authoritative truth before crash");
    drop(durable);

    let reopened_runtime = runtime_with_demo_schema();
    let recovered = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(reopened_runtime)
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("recovery should complete");

    let export = recovered.store().export_authoritative_records();
    assert_eq!(export.commit_envelopes.len(), 1);
    assert_eq!(recovered.last_recovery().decisions.len(), 1);
    assert_eq!(
        recovered.last_recovery().decisions[0].decision,
        RecoveryDecisionClass::RetainPublishedTruth
    );
    assert_eq!(recovered.last_recovery().degraded.len(), 1);
    assert!(matches!(
        recovered.resolve_retry(durable_mutation_id),
        Ok(crate::DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit { .. })
    ));
    let counters = recovered.store().counters();
    assert_eq!(counters.recovery_source_precedence_resolution_count, 1);
    assert_eq!(counters.recovery_source_precedence_fallback_count, 0);
    assert_eq!(counters.recovery_non_quiescent_restart_count, 1);
    assert_eq!(counters.recovery_quiescent_restart_count, 0);
    let degraded_report = recovered.last_recovery().degraded_state_report();
    assert_eq!(degraded_report.retained_without_acknowledgment().len(), 1);
    assert!(degraded_report.quarantines().is_empty());
    let status = recovered.recovery_status_report().unwrap();
    assert_eq!(
        status.operator_disposition(),
        RecoveryOperatorDisposition::RetainedWithoutAcknowledgment
    );
    assert_eq!(status.source_summary().published_authoritative_truth(), 1);
    assert_eq!(status.source_summary().requires_quarantine(), 0);
    assert_eq!(status.recommended_actions().len(), 1);
    assert_eq!(
        status.recommended_actions()[0].kind(),
        RecoveryOperatorActionKind::InspectRetainedWithoutAcknowledgment
    );
}

#[test]
fn crash_after_acknowledgment_retains_truth_exactly_once() {
    let path = unique_test_store_path("forge-store-m3-after-ack");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .expect("durable store should build");

    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("durable mutation should acknowledge before crash");
    drop(durable);

    let recovered = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("recovery should complete");

    let export = recovered.store().export_authoritative_records();
    assert_eq!(export.commit_envelopes.len(), 1);
    assert!(recovered.last_recovery().decisions.iter().any(|decision| {
        decision.durable_mutation_id == acknowledged.durable_mutation_id()
            && decision.decision == RecoveryDecisionClass::SuppressDuplicateReplay
    }));
    assert!(recovered.last_recovery().degraded.is_empty());

    let counters = recovered.store().counters();
    assert_eq!(counters.durable_commit_duplicate_suppression_count, 1);
    assert_eq!(counters.durable_commit_recovered_count, 0);
    assert_eq!(counters.durable_commit_unacknowledged_discard_count, 0);
    assert_eq!(counters.recovery_source_precedence_resolution_count, 1);
    assert_eq!(counters.recovery_source_precedence_fallback_count, 0);
    assert_eq!(counters.recovery_non_quiescent_restart_count, 1);
    assert_eq!(counters.recovery_quiescent_restart_count, 0);
}

#[test]
fn repeated_crash_restart_loops_converge_to_same_truth_as_rebuild() {
    let path = unique_test_sqlite_path("forge-store-m3-restart-loop");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .expect("durable store should build");

    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("first durable commit should acknowledge");
    let crashed_mutation_id = durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-alpha-again", create_beta_commit),
            SimulatedCrashPoint::AfterCanonicalResultRecorded,
        )
        .expect("second durable mutation should crash after recording canonical result");
    drop(durable);

    let recovered_once = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("first recovery should complete");
    assert_eq!(
        recovered_once
            .store()
            .export_authoritative_records()
            .commit_envelopes
            .len(),
        2
    );
    assert!(recovered_once
        .last_recovery()
        .decisions
        .iter()
        .any(
            |decision| decision.durable_mutation_id == crashed_mutation_id
                && decision.decision == RecoveryDecisionClass::FinishPublicationFromCanonicalResult
        ));
    assert!(recovered_once.last_recovery().degraded.is_empty());
    drop(recovered_once);

    let recovered_twice = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("second recovery handle should build")
        .recover()
        .expect("second recovery should complete");
    assert!(recovered_twice.last_recovery().decisions.is_empty());

    let recovered_export = recovered_twice.store().export_authoritative_records();
    let rebuilt =
        ForgeStore::restore_from_authoritative_export(recovered_export.clone().admit_restore())
            .expect("rebuild from authoritative export should succeed");
    let rebuilt_export = rebuilt.export_authoritative_records();

    assert_eq!(
        recovered_export.canonical_json(),
        rebuilt_export.canonical_json()
    );
    let counters = recovered_twice.store().counters();
    assert!(counters.wal_record_scan_count >= 1);
    assert_eq!(counters.durable_commit_duplicate_suppression_count, 0);
    assert_eq!(counters.durable_commit_recovered_count, 0);
    assert_eq!(counters.recovery_source_precedence_resolution_count, 0);
    assert_eq!(counters.recovery_source_precedence_fallback_count, 0);
    assert_eq!(counters.recovery_non_quiescent_restart_count, 0);
    assert_eq!(counters.recovery_quiescent_restart_count, 1);
    assert_eq!(
        recovered_twice
            .resolve_retry(acknowledged.durable_mutation_id())
            .expect("retry resolution should stay stable"),
        crate::DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit {
            commit_id: acknowledged.persisted().envelope().commit.commit_id,
        }
    );
    let status = recovered_twice.recovery_status_report().unwrap();
    assert!(status.quiescent_restart());
    assert_eq!(
        status.operator_disposition(),
        RecoveryOperatorDisposition::Clean
    );
    assert_eq!(status.planned_mutation_count(), 0);
}

#[test]
fn conflicting_publication_commit_ids_are_rejected_during_recovery() {
    let path = unique_test_store_path("forge-store-m3-recovery-source-conflict");
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .expect("durable store should build");

    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("durable mutation should acknowledge");
    drop(durable);

    force_publication_commit_id_conflict(
        &path,
        forge_relational::facade::history::CommitId(
            acknowledged.persisted().envelope().commit.commit_id.0 + 999,
        ),
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .unwrap_err();

    assert_eq!(error.kind(), &crate::StoreErrorKind::RecoverySourceConflict);
}

#[test]
fn publication_gap_is_classified_as_quarantine_before_recovery_bluffs_truth() {
    let path = unique_test_store_path("forge-store-m3-publication-gap");
    let mut durable = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .expect("durable store should build");
    let _acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("durable mutation should acknowledge");
    drop(durable);

    force_branch_head_gap(&path);

    let recovered = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("recovery should emit typed degraded state instead of generic error");

    assert_eq!(
        recovered.last_recovery().decisions[0].decision,
        RecoveryDecisionClass::RequiresQuarantine
    );
    assert_eq!(recovered.last_recovery().degraded.len(), 1);
    assert_eq!(
        recovered.last_recovery().degraded[0].kind,
        crate::DurableRecoveryDegradedKind::QuarantineRequired
    );
    let degraded_report = recovered.last_recovery().degraded_state_report();
    assert_eq!(degraded_report.quarantines().len(), 1);
    assert!(degraded_report.rebuilds().is_empty());
    let status = recovered.recovery_status_report().unwrap();
    assert_eq!(
        status.operator_disposition(),
        RecoveryOperatorDisposition::QuarantineRequired
    );
    assert_eq!(status.source_summary().requires_quarantine(), 1);
    assert_eq!(status.maintenance().entries().len(), 3);
    assert_eq!(status.recommended_actions().len(), 1);
    assert_eq!(
        status.recommended_actions()[0].kind(),
        RecoveryOperatorActionKind::QuarantineScope
    );
}

#[test]
fn bulk_finish_publication_recovery_reports_typed_bulk_identity() {
    let path = unique_test_store_path("forge-store-bulk-finish-publication");
    let mut store = ForgeStoreBuilder::new()
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

    let recovered = ForgeStoreBuilder::new()
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

#[test]
fn retained_bulk_truth_uses_bulk_identity_in_operator_actions() {
    let path = unique_test_store_path("forge-store-bulk-retained-without-ack");
    let mut store = ForgeStoreBuilder::new()
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

    let recovered = ForgeStoreBuilder::new()
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
    let path = unique_test_store_path("forge-store-bulk-retained-with-checkpoint-intent");
    let mut store = ForgeStoreBuilder::new()
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

    let recovered = ForgeStoreBuilder::new()
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

#[test]
fn repeated_bulk_recovery_loops_converge_after_hosted_result_with_checkpoint_intent() {
    let path = unique_test_store_path("forge-store-bulk-repeat-hosted-checkpoint");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .expect("bulk store should build");
    let (plan, envelope, _runtime_session_id, durable_mutation_id) =
        prepare_pending_bulk_ingest_mutation(
            &mut store,
            "bulk-program-repeat-hosted",
            "bulk-source-repeat-hosted",
            true,
        );
    drop(store);

    let recovered_once = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("first recovery should complete");
    assert!(recovered_once
        .last_recovery()
        .decisions
        .iter()
        .any(|decision| {
            decision.durable_mutation_id == durable_mutation_id
                && decision.decision == RecoveryDecisionClass::FinishPublicationFromCanonicalResult
        }));
    assert_eq!(
        recovered_once
            .store()
            .fetch_program_chunk_witness_index("bulk-program-repeat-hosted", plan.plan_id())
            .expect("witness index should be reconstructed")
            .latest_checkpoint_sequence(),
        Some(1)
    );
    assert_eq!(
        recovered_once.resolve_retry(durable_mutation_id),
        Ok(
            crate::DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit {
                commit_id: envelope.commit.commit_id
            }
        )
    );
    let export_once = recovered_once.store().export_authoritative_records();
    drop(recovered_once);

    let recovered_twice = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("second recovery handle should build")
        .recover()
        .expect("second recovery should complete");
    assert!(recovered_twice.last_recovery().decisions.is_empty());
    assert_eq!(
        export_once.canonical_json(),
        recovered_twice
            .store()
            .export_authoritative_records()
            .canonical_json()
    );
}

#[test]
fn repeated_bulk_recovery_loops_converge_after_published_truth_with_existing_witness_only() {
    let path = unique_test_store_path("forge-store-bulk-repeat-published-witness");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .expect("bulk store should build");
    let (plan, envelope, runtime_session_id, durable_mutation_id) =
        prepare_pending_bulk_ingest_mutation(
            &mut store,
            "bulk-program-repeat-published-witness",
            "bulk-source-repeat-published-witness",
            true,
        );
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AuthoritativeAppendPublished,
            Some(envelope.commit.commit_id),
        )
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();
    store
        .publish_bulk_chunk_witness(&admitted, envelope.commit.commit_id)
        .unwrap();
    drop(store);

    let recovered_once = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("first recovery should complete");
    assert!(recovered_once
        .last_recovery()
        .decisions
        .iter()
        .any(|decision| {
            decision.durable_mutation_id == durable_mutation_id
                && decision.decision == RecoveryDecisionClass::RetainPublishedTruth
        }));
    let witness_index = recovered_once
        .store()
        .fetch_program_chunk_witness_index("bulk-program-repeat-published-witness", plan.plan_id())
        .expect("witness index should exist");
    assert_eq!(
        witness_index.highest_committed_chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    assert_eq!(witness_index.latest_checkpoint_sequence(), Some(1));
    let export_once = recovered_once.store().export_authoritative_records();
    drop(recovered_once);

    let recovered_twice = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("second recovery handle should build")
        .recover()
        .expect("second recovery should complete");
    assert!(recovered_twice.last_recovery().decisions.is_empty());
    assert_eq!(
        export_once.canonical_json(),
        recovered_twice
            .store()
            .export_authoritative_records()
            .canonical_json()
    );
    assert_eq!(
        recovered_twice
            .store()
            .fetch_bulk_progress_checkpoint("bulk-program-repeat-published-witness", plan.plan_id())
            .expect("checkpoint should be reconstructed once")
            .checkpoint_sequence(),
        1
    );
}

#[test]
fn repeated_bulk_recovery_loops_converge_after_published_truth_with_existing_witness_and_checkpoint(
) {
    let path = unique_test_store_path("forge-store-bulk-repeat-published-checkpoint");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .expect("bulk store should build");
    let (plan, envelope, runtime_session_id, durable_mutation_id) =
        prepare_pending_bulk_ingest_mutation(
            &mut store,
            "bulk-program-repeat-published-checkpoint",
            "bulk-source-repeat-published-checkpoint",
            true,
        );
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AuthoritativeAppendPublished,
            Some(envelope.commit.commit_id),
        )
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();
    let witness = store
        .publish_bulk_chunk_witness(&admitted, envelope.commit.commit_id)
        .unwrap();
    let checkpoint = store.publish_bulk_progress_checkpoint(&witness).unwrap();
    assert_eq!(checkpoint.checkpoint_sequence(), 1);
    drop(store);

    let recovered_once = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("first recovery should complete");
    assert!(recovered_once
        .last_recovery()
        .decisions
        .iter()
        .any(|decision| {
            decision.durable_mutation_id == durable_mutation_id
                && decision.decision == RecoveryDecisionClass::RetainPublishedTruth
        }));
    assert_eq!(
        recovered_once
            .store()
            .fetch_program_chunk_witness_index(
                "bulk-program-repeat-published-checkpoint",
                plan.plan_id()
            )
            .expect("witness index should exist")
            .latest_checkpoint_sequence(),
        Some(1)
    );
    let export_once = recovered_once.store().export_authoritative_records();
    drop(recovered_once);

    let recovered_twice = ForgeStoreBuilder::new()
        .local_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("second recovery handle should build")
        .recover()
        .expect("second recovery should complete");
    assert!(recovered_twice.last_recovery().decisions.is_empty());
    assert_eq!(
        export_once.canonical_json(),
        recovered_twice
            .store()
            .export_authoritative_records()
            .canonical_json()
    );
    assert_eq!(
        recovered_twice
            .store()
            .fetch_bulk_progress_checkpoint(
                "bulk-program-repeat-published-checkpoint",
                plan.plan_id()
            )
            .expect("checkpoint should still be singular and present")
            .checkpoint_sequence(),
        1
    );
}
