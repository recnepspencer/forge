use crate::{
    bulk::{BudgetAdmittedChunkPlan, ChunkOrdinal},
    failure::{StoreError, StoreErrorKind},
};
use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkChunkCommitWitness {
    program_id: String,
    plan_id: String,
    chunk_ordinal: ChunkOrdinal,
    target_branch_scope: BranchId,
    canonical_commit_id: CommitId,
}

impl BulkChunkCommitWitness {
    pub fn publish(
        admitted: &BudgetAdmittedChunkPlan,
        canonical_commit_id: CommitId,
    ) -> Result<Self, StoreError> {
        if canonical_commit_id.0 == 0 {
            return Err(StoreError::new(
                StoreErrorKind::BulkCanonicalLoweringViolation,
                "bulk chunk witnesses require a non-zero canonical commit id",
            ));
        }
        Ok(Self {
            program_id: admitted.program_id().to_string(),
            plan_id: admitted.plan_id().to_string(),
            chunk_ordinal: admitted.chunk().ordinal(),
            target_branch_scope: admitted.target_branch_scope().clone(),
            canonical_commit_id,
        })
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    pub fn chunk_ordinal(&self) -> ChunkOrdinal {
        self.chunk_ordinal
    }

    pub fn target_branch_scope(&self) -> &BranchId {
        &self.target_branch_scope
    }

    pub fn canonical_commit_id(&self) -> CommitId {
        self.canonical_commit_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgramChunkWitnessIndex {
    program_id: String,
    plan_id: String,
    highest_committed_chunk_ordinal: ChunkOrdinal,
    highest_committed_commit_id: CommitId,
    latest_checkpoint_sequence: Option<u64>,
    witness_count: u64,
}

impl ProgramChunkWitnessIndex {
    pub(crate) fn new(
        program_id: String,
        plan_id: String,
        highest_committed_chunk_ordinal: ChunkOrdinal,
        highest_committed_commit_id: CommitId,
        latest_checkpoint_sequence: Option<u64>,
        witness_count: u64,
    ) -> Self {
        Self {
            program_id,
            plan_id,
            highest_committed_chunk_ordinal,
            highest_committed_commit_id,
            latest_checkpoint_sequence,
            witness_count,
        }
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    pub fn highest_committed_chunk_ordinal(&self) -> ChunkOrdinal {
        self.highest_committed_chunk_ordinal
    }

    pub fn highest_committed_commit_id(&self) -> CommitId {
        self.highest_committed_commit_id
    }

    pub fn latest_checkpoint_sequence(&self) -> Option<u64> {
        self.latest_checkpoint_sequence
    }

    pub fn witness_count(&self) -> u64 {
        self.witness_count
    }
}
