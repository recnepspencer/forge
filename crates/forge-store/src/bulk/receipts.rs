use crate::bulk::{
    BudgetAdmittedChunkPlan, BulkChunkCommitWitness, ChunkOrdinal, PublishedBulkProgressCheckpoint,
};
use forge_relational::facade::history::{BranchId, CommitId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulkExecutionPath {
    VerifiedFastPath,
    ExplicitFallbackPath,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkMaterializationReceipt {
    program_id: String,
    plan_id: String,
    chunk_ordinal: ChunkOrdinal,
    target_branch_scope: BranchId,
    basis_commit_id: Option<CommitId>,
    admitted_width_units: u64,
    materialized_member_count: u64,
    materialization_breadth_units: u64,
    memory_units: u64,
    execution_path: BulkExecutionPath,
}

impl ChunkMaterializationReceipt {
    pub fn from_admitted_chunk(admitted: &BudgetAdmittedChunkPlan) -> Self {
        let breadth_units = admitted.chunk().member_ids().len() as u64;
        Self {
            program_id: admitted.program_id().to_string(),
            plan_id: admitted.plan_id().to_string(),
            chunk_ordinal: admitted.chunk().ordinal(),
            target_branch_scope: admitted.target_branch_scope().clone(),
            basis_commit_id: admitted.basis_commit_id(),
            admitted_width_units: admitted.chunk().width_units(),
            materialized_member_count: breadth_units,
            materialization_breadth_units: breadth_units,
            memory_units: admitted.admitted_memory_units(),
            execution_path: BulkExecutionPath::VerifiedFastPath,
        }
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

    pub fn basis_commit_id(&self) -> Option<CommitId> {
        self.basis_commit_id
    }

    pub fn admitted_width_units(&self) -> u64 {
        self.admitted_width_units
    }

    pub fn materialized_member_count(&self) -> u64 {
        self.materialized_member_count
    }

    pub fn materialization_breadth_units(&self) -> u64 {
        self.materialization_breadth_units
    }

    pub fn memory_units(&self) -> u64 {
        self.memory_units
    }

    pub fn execution_path(&self) -> BulkExecutionPath {
        self.execution_path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulkChunkExecutionOutcome {
    materialization_receipt: ChunkMaterializationReceipt,
    chunk_commit_witness: BulkChunkCommitWitness,
    published_checkpoint: Option<PublishedBulkProgressCheckpoint>,
}

impl BulkChunkExecutionOutcome {
    pub fn new(
        materialization_receipt: ChunkMaterializationReceipt,
        chunk_commit_witness: BulkChunkCommitWitness,
        published_checkpoint: Option<PublishedBulkProgressCheckpoint>,
    ) -> Self {
        Self {
            materialization_receipt,
            chunk_commit_witness,
            published_checkpoint,
        }
    }

    pub fn materialization_receipt(&self) -> &ChunkMaterializationReceipt {
        &self.materialization_receipt
    }

    pub fn chunk_commit_witness(&self) -> &BulkChunkCommitWitness {
        &self.chunk_commit_witness
    }

    pub fn published_checkpoint(&self) -> Option<&PublishedBulkProgressCheckpoint> {
        self.published_checkpoint.as_ref()
    }
}
