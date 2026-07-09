use crate::bulk::{
    BulkChunkCommitWitness, BulkExecutionPath, DeterministicChunkPlan, FrozenBulkSourceManifest,
    FrozenTransformBasis, FrozenTransformTargetPartition, ProgramChunkWitnessIndex,
    PublishedBulkProgressCheckpoint, ResumeBoundaryCandidate,
};
use crate::failure::StoreError;

use super::{dispatch_mut, dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn record_bulk_source_manifest(&self, member_count: u64, stream_pass_count: u64) {
        dispatch_ref!(self, |backend| backend
            .record_bulk_source_manifest(member_count, stream_pass_count))
    }
    pub fn record_bulk_chunk_plan(&self, chunk_count: u64) {
        dispatch_ref!(self, |backend| backend.record_bulk_chunk_plan(chunk_count))
    }
    pub(crate) fn record_bulk_checkpoint_publication_intent(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: crate::wal::DurableMutationId,
        checkpoint_sequence: Option<u64>,
    ) -> Result<(), StoreError> {
        dispatch_mut!(self, |backend| backend
            .record_bulk_checkpoint_publication_intent(
                runtime_session_id,
                durable_mutation_id,
                checkpoint_sequence,
            ))
    }
    pub fn record_bulk_chunk_execute(
        &self,
        width_units: u64,
        memory_units: u64,
        fallback_breadth_units: u64,
        execution_path: BulkExecutionPath,
    ) {
        dispatch_ref!(self, |backend| backend.record_bulk_chunk_execute(
            width_units,
            memory_units,
            fallback_breadth_units,
            matches!(execution_path, BulkExecutionPath::ExplicitFallbackPath),
        ))
    }
    pub fn record_bulk_chunk_resume(&self) {
        dispatch_ref!(self, |backend| backend.record_bulk_chunk_resume())
    }
    pub fn record_bulk_chunk_commit(&self) {
        dispatch_ref!(self, |backend| backend.record_bulk_chunk_commit())
    }
    pub fn persist_frozen_bulk_manifest(
        &mut self,
        manifest: FrozenBulkSourceManifest,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        dispatch_mut!(self, |backend| backend
            .persist_frozen_bulk_manifest(manifest))
    }
    pub fn persist_frozen_transform_basis(
        &mut self,
        basis: FrozenTransformBasis,
    ) -> Result<FrozenTransformBasis, StoreError> {
        dispatch_mut!(self, |backend| backend
            .persist_frozen_transform_basis(basis))
    }
    pub fn persist_frozen_transform_partition(
        &mut self,
        partition: FrozenTransformTargetPartition,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        dispatch_mut!(self, |backend| backend
            .persist_frozen_transform_partition(partition))
    }
    pub fn persist_bulk_chunk_plan(
        &mut self,
        plan: DeterministicChunkPlan,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        dispatch_mut!(self, |backend| backend.persist_bulk_chunk_plan(plan))
    }
    pub fn fetch_frozen_bulk_manifest(
        &self,
        program_id: &str,
        manifest_digest: &str,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        dispatch_ref!(self, |backend| backend
            .fetch_frozen_bulk_manifest(program_id, manifest_digest))
    }
    pub fn fetch_frozen_transform_basis(
        &self,
        program_id: &str,
        basis_digest: &str,
    ) -> Result<FrozenTransformBasis, StoreError> {
        dispatch_ref!(self, |backend| backend
            .fetch_frozen_transform_basis(program_id, basis_digest))
    }
    pub fn fetch_frozen_transform_partition(
        &self,
        program_id: &str,
        partition_digest: &str,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        dispatch_ref!(self, |backend| backend
            .fetch_frozen_transform_partition(program_id, partition_digest))
    }
    pub fn find_frozen_transform_basis_for_plan(
        &self,
        program_id: &str,
        target_branch_scope: &worth_relational::facade::history::BranchId,
        basis_commit_id: worth_relational::facade::history::CommitId,
    ) -> Result<FrozenTransformBasis, StoreError> {
        dispatch_ref!(self, |backend| backend
            .find_frozen_transform_basis_for_plan(
                program_id,
                target_branch_scope,
                basis_commit_id,
            ))
    }
    pub fn fetch_bulk_chunk_plan(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        dispatch_ref!(self, |backend| backend
            .fetch_bulk_chunk_plan(program_id, plan_id))
    }
    pub fn publish_bulk_chunk_witness(
        &mut self,
        witness: BulkChunkCommitWitness,
    ) -> Result<BulkChunkCommitWitness, StoreError> {
        dispatch_mut!(self, |backend| backend.publish_bulk_chunk_witness(witness))
    }
    pub fn publish_bulk_progress_checkpoint(
        &mut self,
        witness: BulkChunkCommitWitness,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        dispatch_mut!(self, |backend| backend
            .publish_bulk_progress_checkpoint(witness))
    }
    pub fn fetch_bulk_progress_checkpoint(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        dispatch_ref!(self, |backend| backend
            .fetch_bulk_progress_checkpoint(program_id, plan_id))
    }
    pub fn fetch_program_chunk_witness_index(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ProgramChunkWitnessIndex, StoreError> {
        dispatch_ref!(self, |backend| backend
            .fetch_program_chunk_witness_index(program_id, plan_id))
    }
    pub fn fetch_latest_resume_boundary(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ResumeBoundaryCandidate, StoreError> {
        dispatch_ref!(self, |backend| backend
            .fetch_latest_resume_boundary(program_id, plan_id))
    }
}
