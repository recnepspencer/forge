use worth_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone6CommitCoupledLayoutSeedRecord {
    pub artifact_id: String,
    pub request: crate::AspectLayoutReadRequest,
    pub layout_materialization_artifact_id: String,
    pub authority_basis_commit_id: CommitId,
    pub authority_basis_commit_digest: String,
    pub authority_basis_commit_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone6ScopeSliceMembershipRecord {
    pub artifact_id: String,
    pub branch_id: BranchId,
    pub frontier_commit_id: CommitId,
    pub scope_class: String,
    pub projection_digest: String,
    pub slice_ids: Vec<crate::AspectLayoutSliceId>,
    pub layout_materialization_artifact_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone6ChunkMembershipRecord {
    pub artifact_id: String,
    pub physical_chunk_id: crate::PhysicalChunkId,
    pub chunk_shape_version: crate::ChunkShapeVersion,
    pub determinism_digest: String,
    pub slice_ids: Vec<crate::AspectLayoutSliceId>,
    pub layout_materialization_artifact_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone6StructuralBlockRecord {
    pub artifact_id: String,
    pub structural_block_id: crate::StructuralBlockId,
    pub scope_class: String,
    pub equivalence_contract_version: crate::EquivalenceContractVersion,
    pub slice_ids: Vec<crate::AspectLayoutSliceId>,
    pub supporting_layout_materialization_artifact_ids: Vec<String>,
}
