use super::progress::PublishedBulkProgressCheckpoint;
use crate::{
    bulk::{
        BudgetAdmittedChunkPlan, DeterministicChunkPlan, FrozenBulkSourceManifest,
        FrozenTransformBasis, FrozenTransformTargetPartition, ProgramChunkWitnessIndex,
    },
    failure::{StoreError, StoreErrorKind},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeBoundaryCandidate {
    program_id: String,
    plan_id: String,
    latest_committed_chunk_ordinal: Option<crate::bulk::ChunkOrdinal>,
    next_chunk_ordinal: crate::bulk::ChunkOrdinal,
    latest_checkpoint_sequence: Option<u64>,
}

impl ResumeBoundaryCandidate {
    pub(crate) fn new(
        program_id: String,
        plan_id: String,
        latest_committed_chunk_ordinal: Option<crate::bulk::ChunkOrdinal>,
        next_chunk_ordinal: crate::bulk::ChunkOrdinal,
        latest_checkpoint_sequence: Option<u64>,
    ) -> Self {
        Self {
            program_id,
            plan_id,
            latest_committed_chunk_ordinal,
            next_chunk_ordinal,
            latest_checkpoint_sequence,
        }
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    pub fn latest_committed_chunk_ordinal(&self) -> Option<crate::bulk::ChunkOrdinal> {
        self.latest_committed_chunk_ordinal
    }

    pub fn next_chunk_ordinal(&self) -> crate::bulk::ChunkOrdinal {
        self.next_chunk_ordinal
    }

    pub fn latest_checkpoint_sequence(&self) -> Option<u64> {
        self.latest_checkpoint_sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeReadyBulkProgram {
    plan: DeterministicChunkPlan,
    witness_index: Option<ProgramChunkWitnessIndex>,
    latest_checkpoint: Option<PublishedBulkProgressCheckpoint>,
    resume_boundary: ResumeBoundaryCandidate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveredBulkChunkResume {
    resumed_chunk_ordinal: crate::bulk::ChunkOrdinal,
    resumed_program: ResumeReadyBulkProgram,
}

impl RecoveredBulkChunkResume {
    pub(crate) fn new(
        resumed_chunk_ordinal: crate::bulk::ChunkOrdinal,
        resumed_program: ResumeReadyBulkProgram,
    ) -> Self {
        Self {
            resumed_chunk_ordinal,
            resumed_program,
        }
    }

    pub fn resumed_chunk_ordinal(&self) -> crate::bulk::ChunkOrdinal {
        self.resumed_chunk_ordinal
    }

    pub fn resumed_program(&self) -> &ResumeReadyBulkProgram {
        &self.resumed_program
    }
}

impl ResumeReadyBulkProgram {
    pub fn admit_ingest(
        manifest: &FrozenBulkSourceManifest,
        plan: DeterministicChunkPlan,
        witness_index: Option<ProgramChunkWitnessIndex>,
        latest_checkpoint: Option<PublishedBulkProgressCheckpoint>,
        resume_boundary: ResumeBoundaryCandidate,
    ) -> Result<Self, StoreError> {
        if plan.program_id() != manifest.program_id()
            || plan.input_digest() != manifest.manifest_digest()
            || plan.target_branch_scope() != manifest.target_branch_scope()
        {
            return Err(StoreError::new(
                StoreErrorKind::BulkResumeBoundaryAmbiguous,
                "bulk ingest resume artifacts did not match the persisted deterministic plan",
            ));
        }
        Self::admit(plan, witness_index, latest_checkpoint, resume_boundary)
    }

    pub fn admit_transform(
        basis: &FrozenTransformBasis,
        partition: &FrozenTransformTargetPartition,
        plan: DeterministicChunkPlan,
        witness_index: Option<ProgramChunkWitnessIndex>,
        latest_checkpoint: Option<PublishedBulkProgressCheckpoint>,
        resume_boundary: ResumeBoundaryCandidate,
    ) -> Result<Self, StoreError> {
        if plan.program_id() != basis.program_id()
            || plan.program_id() != partition.program_id()
            || plan.basis_commit_id() != Some(basis.basis_commit_id())
            || plan.basis_commit_id() != Some(partition.basis_commit_id())
            || plan.input_digest() != partition.partition_digest()
            || basis.target_branch_scope() != partition.target_branch_scope()
            || plan.target_branch_scope() != basis.target_branch_scope()
        {
            return Err(StoreError::new(
                StoreErrorKind::BulkTransformBasisDrift,
                "bulk transform resume artifacts drifted from the locked transform basis",
            ));
        }
        Self::admit(plan, witness_index, latest_checkpoint, resume_boundary)
    }

    fn admit(
        plan: DeterministicChunkPlan,
        witness_index: Option<ProgramChunkWitnessIndex>,
        latest_checkpoint: Option<PublishedBulkProgressCheckpoint>,
        resume_boundary: ResumeBoundaryCandidate,
    ) -> Result<Self, StoreError> {
        if resume_boundary.program_id() != plan.program_id()
            || resume_boundary.plan_id() != plan.plan_id()
        {
            return Err(StoreError::new(
                StoreErrorKind::BulkResumeBoundaryAmbiguous,
                "resume boundary did not match the deterministic bulk plan identity",
            ));
        }
        if let Some(index) = &witness_index {
            if index.program_id() != plan.program_id() || index.plan_id() != plan.plan_id() {
                return Err(StoreError::new(
                    StoreErrorKind::BulkResumeBoundaryAmbiguous,
                    "program witness index did not match the deterministic bulk plan identity",
                ));
            }
        }
        if let Some(checkpoint) = &latest_checkpoint {
            if checkpoint.program_id() != plan.program_id()
                || checkpoint.plan_id() != plan.plan_id()
            {
                return Err(StoreError::new(
                    StoreErrorKind::BulkResumeBoundaryAmbiguous,
                    "latest checkpoint did not match the deterministic bulk plan identity",
                ));
            }
        }
        Ok(Self {
            plan,
            witness_index,
            latest_checkpoint,
            resume_boundary,
        })
    }

    pub fn plan(&self) -> &DeterministicChunkPlan {
        &self.plan
    }

    pub fn witness_index(&self) -> Option<&ProgramChunkWitnessIndex> {
        self.witness_index.as_ref()
    }

    pub fn latest_checkpoint(&self) -> Option<&PublishedBulkProgressCheckpoint> {
        self.latest_checkpoint.as_ref()
    }

    pub fn resume_boundary(&self) -> &ResumeBoundaryCandidate {
        &self.resume_boundary
    }

    pub fn next_chunk_ordinal(&self) -> crate::bulk::ChunkOrdinal {
        self.resume_boundary.next_chunk_ordinal()
    }

    pub fn is_complete(&self) -> bool {
        self.plan
            .chunk_by_ordinal(self.resume_boundary.next_chunk_ordinal())
            .is_none()
    }

    pub fn admit_next_chunk(
        &self,
        admitted_memory_units: u64,
    ) -> Result<Option<BudgetAdmittedChunkPlan>, StoreError> {
        if self.is_complete() {
            return Ok(None);
        }
        let admitted = BudgetAdmittedChunkPlan::admit(
            &self.plan,
            self.resume_boundary.next_chunk_ordinal(),
            admitted_memory_units,
        )?;
        Ok(Some(admitted))
    }
}
