use crate::bulk::{
    BulkChunkCommitWitness, BulkPlanKind, DeterministicChunkPlan, FrozenBulkSourceManifest,
    FrozenTransformBasis, FrozenTransformTargetPartition, ProgramChunkWitnessIndex,
    PublishedBulkProgressCheckpoint,
};
use worth_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkProgramIdentityRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub kind: BulkPlanKind,
    pub program_id: String,
    pub source_identity: String,
    pub target_branch_scope: BranchId,
    pub basis_commit_id: Option<CommitId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenBulkManifestRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub manifest: FrozenBulkSourceManifest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenTransformBasisRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub basis: FrozenTransformBasis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenTransformPartitionRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub partition: FrozenTransformTargetPartition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkDeterministicPlanRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub plan: DeterministicChunkPlan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkProgressCheckpointRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub plan_id: String,
    pub checkpoint: PublishedBulkProgressCheckpoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkChunkWitnessRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub plan_id: String,
    pub witness: BulkChunkCommitWitness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgramChunkWitnessIndexRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub plan_id: String,
    pub index: ProgramChunkWitnessIndex,
}
