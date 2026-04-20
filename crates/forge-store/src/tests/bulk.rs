use super::harness::{
    corruption::{
        local_file::{
            force_bulk_checkpoint_completed_chunk_regression, force_bulk_checkpoint_gap,
            force_bulk_plan_payload_chunk_width_drift,
            force_bulk_witness_index_highest_ordinal_regression,
            force_bulk_witness_index_witness_count_drift, force_bulk_witness_missing_commit,
            force_frozen_transform_basis_payload_scope_drift,
        },
        sqlite::{
            delete_sqlite_bulk_checkpoint, drift_sqlite_bulk_witness_index_witness_count,
            drift_sqlite_frozen_transform_partition_payload_member_width,
            regress_sqlite_bulk_checkpoint_completed_chunk,
            regress_sqlite_bulk_witness_index_highest_ordinal,
        },
    },
    fixtures::{
        runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
};
use crate::{
    BulkCheckpointPolicy, BulkIngestSourceRequest, BulkSourceMember, BulkTransformRequest,
    ChunkOrdinal, ChunkWidthBudget, DurableRetryResolution, ForgeStoreBuilder, StoreErrorKind,
};
use forge_relational::facade::history::{BranchId, CommitId};

fn persist_two_checkpoint_bulk_family(
    store: &mut crate::ForgeStore,
    program_id: &str,
    source_identity: &str,
) -> (
    crate::FrozenBulkSourceManifest,
    crate::DeterministicChunkPlan,
) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, &format!("{program_id}-entity-a"));
    let first_envelope = latest_envelope(&runtime);
    create_entity(&mut runtime, &format!("{program_id}-entity-b"));
    let second_envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            program_id,
            source_identity,
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
        .admit_bulk_ingest_resume(program_id, plan.plan_id(), manifest.manifest_digest())
        .unwrap();
    store
        .execute_next_resumed_bulk_chunk(
            &resumed,
            3,
            second_envelope,
            BulkCheckpointPolicy::Publish,
        )
        .unwrap()
        .expect("second chunk should execute");

    (manifest, plan)
}

fn persist_transform_artifacts(
    store: &mut crate::ForgeStore,
    program_id: &str,
    transform_identity: &str,
) -> (
    crate::FrozenTransformBasis,
    crate::FrozenTransformTargetPartition,
    crate::DeterministicChunkPlan,
) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, &format!("{program_id}-transform-authority"));
    let envelope = latest_envelope(&runtime);
    let request = BulkTransformRequest::new(
        program_id,
        transform_identity,
        envelope.branch_context,
        envelope.commit.commit_id,
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
    (basis, partition, plan)
}


#[path = "bulk/freeze_and_plan.rs"]
mod freeze_and_plan;
#[path = "bulk/sqlite_reopen.rs"]
mod sqlite_reopen;
#[path = "bulk/local_file_reopen.rs"]
mod local_file_reopen;
#[path = "bulk/resume.rs"]
mod resume;
#[path = "bulk/execution.rs"]
mod execution;
