use crate::{
    bulk::{
        BudgetAdmittedChunkPlan, BulkChunkCommitWitness, BulkChunkExecutionOutcome,
        BulkIngestSourceRequest, BulkTransformRequest, ChunkMaterializationReceipt, ChunkOrdinal,
        ChunkWidthBudget, DeterministicChunkPlan, FrozenBulkSourceManifest, FrozenTransformBasis,
        FrozenTransformTargetPartition, ProgramChunkWitnessIndex, PublishedBulkProgressCheckpoint,
        ResumeBoundaryCandidate, ResumeReadyBulkProgram,
    },
    failure::StoreError,
    BulkCheckpointPolicy, RecoveredBulkChunkResume, ResumeEligibleRecoveredBulkChunk,
};
use worth_relational::facade::history::CommitId;

use super::WORTHStore;

impl WORTHStore {
    pub fn freeze_bulk_ingest_source(
        &mut self,
        request: BulkIngestSourceRequest,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        let manifest = FrozenBulkSourceManifest::freeze(request)?;
        self.backend
            .record_bulk_source_manifest(manifest.ordered_members().len() as u64, 1);
        self.backend.persist_frozen_bulk_manifest(manifest)
    }

    pub fn plan_bulk_ingest(
        &mut self,
        manifest: FrozenBulkSourceManifest,
        chunk_width_budget: ChunkWidthBudget,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        let plan = DeterministicChunkPlan::for_ingest(&manifest, chunk_width_budget)?;
        self.backend
            .record_bulk_chunk_plan(plan.chunk_count() as u64);
        self.backend.persist_bulk_chunk_plan(plan)
    }

    pub fn freeze_bulk_transform_basis(
        &mut self,
        request: BulkTransformRequest,
    ) -> Result<FrozenTransformBasis, StoreError> {
        let basis = FrozenTransformBasis::freeze(&request)?;
        self.backend.persist_frozen_transform_basis(basis)
    }

    pub fn freeze_bulk_transform_target_partition(
        &mut self,
        request: BulkTransformRequest,
        basis: &FrozenTransformBasis,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        let partition = FrozenTransformTargetPartition::freeze(&request, basis)?;
        self.backend.persist_frozen_transform_partition(partition)
    }

    pub fn plan_bulk_transform(
        &mut self,
        basis: &FrozenTransformBasis,
        partition: &FrozenTransformTargetPartition,
        chunk_width_budget: ChunkWidthBudget,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        let plan = DeterministicChunkPlan::for_transform(basis, partition, chunk_width_budget)?;
        self.backend
            .record_bulk_chunk_plan(plan.chunk_count() as u64);
        self.backend.persist_bulk_chunk_plan(plan)
    }

    pub fn fetch_frozen_bulk_manifest(
        &self,
        program_id: &str,
        manifest_digest: &str,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        self.backend
            .fetch_frozen_bulk_manifest(program_id, manifest_digest)
    }

    pub fn fetch_frozen_transform_basis(
        &self,
        program_id: &str,
        basis_digest: &str,
    ) -> Result<FrozenTransformBasis, StoreError> {
        self.backend
            .fetch_frozen_transform_basis(program_id, basis_digest)
    }

    pub fn fetch_frozen_transform_partition(
        &self,
        program_id: &str,
        partition_digest: &str,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        self.backend
            .fetch_frozen_transform_partition(program_id, partition_digest)
    }

    pub fn admit_bulk_ingest_chunk(
        &self,
        plan: &DeterministicChunkPlan,
        ordinal: ChunkOrdinal,
        admitted_memory_units: u64,
    ) -> Result<BudgetAdmittedChunkPlan, StoreError> {
        BudgetAdmittedChunkPlan::admit(plan, ordinal, admitted_memory_units)
    }

    pub fn admit_bulk_transform_chunk(
        &self,
        plan: &DeterministicChunkPlan,
        ordinal: ChunkOrdinal,
        admitted_memory_units: u64,
    ) -> Result<BudgetAdmittedChunkPlan, StoreError> {
        BudgetAdmittedChunkPlan::admit(plan, ordinal, admitted_memory_units)
    }

    pub fn materialize_bulk_ingest_chunk(
        &self,
        admitted: &BudgetAdmittedChunkPlan,
    ) -> Result<ChunkMaterializationReceipt, StoreError> {
        let receipt = ChunkMaterializationReceipt::from_admitted_chunk(admitted);
        self.backend.record_bulk_chunk_execute(
            receipt.admitted_width_units(),
            receipt.memory_units(),
            receipt.materialization_breadth_units(),
            receipt.execution_path(),
        );
        Ok(receipt)
    }

    pub fn publish_bulk_chunk_witness(
        &mut self,
        admitted: &BudgetAdmittedChunkPlan,
        canonical_commit_id: CommitId,
    ) -> Result<BulkChunkCommitWitness, StoreError> {
        let witness = BulkChunkCommitWitness::publish(admitted, canonical_commit_id)?;
        self.backend.publish_bulk_chunk_witness(witness)
    }

    pub fn publish_bulk_progress_checkpoint(
        &mut self,
        witness: &BulkChunkCommitWitness,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        self.backend
            .publish_bulk_progress_checkpoint(witness.clone())
    }

    pub fn fetch_bulk_progress_checkpoint(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        self.backend
            .fetch_bulk_progress_checkpoint(program_id, plan_id)
    }

    pub fn fetch_bulk_chunk_plan(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        self.backend.fetch_bulk_chunk_plan(program_id, plan_id)
    }

    pub fn fetch_program_chunk_witness_index(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ProgramChunkWitnessIndex, StoreError> {
        self.backend
            .fetch_program_chunk_witness_index(program_id, plan_id)
    }

    pub fn fetch_latest_bulk_resume_boundary(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ResumeBoundaryCandidate, StoreError> {
        self.backend
            .fetch_latest_resume_boundary(program_id, plan_id)
    }

    pub fn admit_bulk_ingest_resume(
        &self,
        program_id: &str,
        plan_id: &str,
        manifest_digest: &str,
    ) -> Result<ResumeReadyBulkProgram, StoreError> {
        let manifest = self.fetch_frozen_bulk_manifest(program_id, manifest_digest)?;
        let (plan, witness_index, latest_checkpoint, resume_boundary) =
            self.load_bulk_resume_artifacts(program_id, plan_id)?;
        self.backend.record_bulk_chunk_resume();
        ResumeReadyBulkProgram::admit_ingest(
            &manifest,
            plan,
            witness_index,
            latest_checkpoint,
            resume_boundary,
        )
    }

    pub fn admit_bulk_transform_resume(
        &self,
        program_id: &str,
        plan_id: &str,
        basis_digest: &str,
        partition_digest: &str,
    ) -> Result<ResumeReadyBulkProgram, StoreError> {
        let basis = self.fetch_frozen_transform_basis(program_id, basis_digest)?;
        let partition = self.fetch_frozen_transform_partition(program_id, partition_digest)?;
        let (plan, witness_index, latest_checkpoint, resume_boundary) =
            self.load_bulk_resume_artifacts(program_id, plan_id)?;
        self.backend.record_bulk_chunk_resume();
        ResumeReadyBulkProgram::admit_transform(
            &basis,
            &partition,
            plan,
            witness_index,
            latest_checkpoint,
            resume_boundary,
        )
    }

    pub fn admit_recovered_bulk_chunk_resume(
        &self,
        recovered: &ResumeEligibleRecoveredBulkChunk,
    ) -> Result<RecoveredBulkChunkResume, StoreError> {
        let recovered = recovered.recovered();
        let (plan, witness_index, latest_checkpoint, resume_boundary) =
            self.load_bulk_resume_artifacts(recovered.program_id(), recovered.plan_id())?;
        self.backend.record_bulk_chunk_resume();
        let resumed_program = self.admit_resume_ready_bulk_program(
            recovered.program_id(),
            recovered.plan_id(),
            plan,
            witness_index,
            latest_checkpoint,
            resume_boundary,
        )?;

        Ok(RecoveredBulkChunkResume::new(
            ChunkOrdinal::new(recovered.chunk_ordinal()),
            resumed_program,
        ))
    }

    pub fn finalize_bulk_chunk_execution(
        &mut self,
        admitted: &BudgetAdmittedChunkPlan,
        canonical_commit_id: CommitId,
        checkpoint_policy: BulkCheckpointPolicy,
    ) -> Result<BulkChunkExecutionOutcome, StoreError> {
        let materialization_receipt = self.materialize_bulk_ingest_chunk(admitted)?;
        let witness = self.publish_bulk_chunk_witness(admitted, canonical_commit_id)?;
        let published_checkpoint = if checkpoint_policy.should_publish() {
            Some(self.publish_bulk_progress_checkpoint(&witness)?)
        } else {
            None
        };
        self.backend.record_bulk_chunk_commit();
        Ok(BulkChunkExecutionOutcome::new(
            materialization_receipt,
            witness,
            published_checkpoint,
        ))
    }
}
