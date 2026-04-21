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

#[path = "wal_recovery/bulk_finish_publication.rs"]
mod bulk_finish_publication;
#[path = "wal_recovery/bulk_restart_convergence.rs"]
mod bulk_restart_convergence;
#[path = "wal_recovery/bulk_retained_truth.rs"]
mod bulk_retained_truth;
#[path = "wal_recovery/durable_crash_paths.rs"]
mod durable_crash_paths;
#[path = "wal_recovery/durable_restart_convergence.rs"]
mod durable_restart_convergence;
#[path = "wal_recovery/recovery_source_integrity.rs"]
mod recovery_source_integrity;
