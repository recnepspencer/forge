#[path = "materialization_validation.rs"]
mod validation;

use crate::layout::{
    AdmittedAspectLayoutReadPlan, ChunkDeterminismWitness, ChunkModelFrozenPhysicalLayout,
    DedupAdmittedBlockReuse, Milestone6LayoutMaterialization,
    Milestone7IndependentLayoutReference, Milestone9PhysicalChunkReference,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Milestone6LayoutMaterializationRecord {
    pub artifact_id: String,
    pub materialization: Milestone6LayoutMaterialization,
}

impl Serialize for Milestone6LayoutMaterializationRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        PersistedMilestone6LayoutMaterializationRecord::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Milestone6LayoutMaterializationRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedMilestone6LayoutMaterializationRecord::deserialize(deserializer)?;
        Self::try_from(persisted).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedMilestone6LayoutMaterializationRecord {
    artifact_id: String,
    materialization: PersistedMilestone6LayoutMaterialization,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedMilestone6LayoutMaterialization {
    artifact_id: String,
    admitted_plan: PersistedAdmittedAspectLayoutReadPlan,
    block_reuse: PersistedDedupAdmittedBlockReuse,
    frozen_layout: PersistedChunkModelFrozenPhysicalLayout,
    milestone_7_reference: PersistedMilestone7IndependentLayoutReference,
    milestone_9_reference: PersistedMilestone9PhysicalChunkReference,
    semantic_truth_digest: String,
    authoritative_commit_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedAdmittedAspectLayoutReadPlan {
    request: crate::AspectLayoutReadRequest,
    slice_ids: Vec<crate::AspectLayoutSliceId>,
    structural_block_id: crate::StructuralBlockId,
    performance: crate::AspectLayoutPerformanceEnvelope,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedDedupAdmittedBlockReuse {
    branch_id: forge_relational::facade::history::BranchId,
    frontier_commit_id: forge_relational::facade::history::CommitId,
    scope_class: String,
    structural_block_id: crate::StructuralBlockId,
    equivalence_contract_version: crate::EquivalenceContractVersion,
    slice_ids: Vec<crate::AspectLayoutSliceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedChunkDeterminismWitness {
    physical_chunk_id: crate::PhysicalChunkId,
    chunk_shape_version: crate::ChunkShapeVersion,
    determinism_digest: String,
    ordered_slice_ids: Vec<crate::AspectLayoutSliceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedChunkModelFrozenPhysicalLayout {
    request: crate::AspectLayoutReadRequest,
    chunk_width: u64,
    witness: PersistedChunkDeterminismWitness,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedMilestone7IndependentLayoutReference {
    branch_id: forge_relational::facade::history::BranchId,
    frontier_commit_id: forge_relational::facade::history::CommitId,
    scope_class: String,
    projection_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedMilestone9PhysicalChunkReference {
    physical_chunk_id: crate::PhysicalChunkId,
    chunk_shape_version: crate::ChunkShapeVersion,
    determinism_digest: String,
    chunk_member_count: usize,
}

impl From<&Milestone6LayoutMaterializationRecord> for PersistedMilestone6LayoutMaterializationRecord {
    fn from(record: &Milestone6LayoutMaterializationRecord) -> Self {
        Self {
            artifact_id: record.artifact_id.clone(),
            materialization: PersistedMilestone6LayoutMaterialization::from(&record.materialization),
        }
    }
}

impl From<&Milestone6LayoutMaterialization> for PersistedMilestone6LayoutMaterialization {
    fn from(materialization: &Milestone6LayoutMaterialization) -> Self {
        Self {
            artifact_id: materialization.artifact_id().to_string(),
            admitted_plan: PersistedAdmittedAspectLayoutReadPlan::from(materialization.admitted_plan()),
            block_reuse: PersistedDedupAdmittedBlockReuse::from(materialization.block_reuse()),
            frozen_layout: PersistedChunkModelFrozenPhysicalLayout::from(materialization.frozen_layout()),
            milestone_7_reference: PersistedMilestone7IndependentLayoutReference::from(materialization.milestone_7_reference()),
            milestone_9_reference: PersistedMilestone9PhysicalChunkReference::from(materialization.milestone_9_reference()),
            semantic_truth_digest: materialization.semantic_truth_digest().to_string(),
            authoritative_commit_count: materialization.authoritative_commit_count(),
        }
    }
}

impl From<&AdmittedAspectLayoutReadPlan> for PersistedAdmittedAspectLayoutReadPlan {
    fn from(plan: &AdmittedAspectLayoutReadPlan) -> Self {
        Self {
            request: plan.request().clone(),
            slice_ids: plan.slice_ids().to_vec(),
            structural_block_id: plan.structural_block_id().clone(),
            performance: plan.performance().clone(),
        }
    }
}

impl From<&DedupAdmittedBlockReuse> for PersistedDedupAdmittedBlockReuse {
    fn from(reuse: &DedupAdmittedBlockReuse) -> Self {
        Self {
            branch_id: reuse.branch_id().clone(),
            frontier_commit_id: reuse.frontier_commit_id(),
            scope_class: reuse.scope_class().to_string(),
            structural_block_id: reuse.structural_block_id().clone(),
            equivalence_contract_version: reuse.equivalence_contract_version(),
            slice_ids: reuse.slice_ids().to_vec(),
        }
    }
}

impl From<&ChunkDeterminismWitness> for PersistedChunkDeterminismWitness {
    fn from(witness: &ChunkDeterminismWitness) -> Self {
        Self {
            physical_chunk_id: witness.physical_chunk_id().clone(),
            chunk_shape_version: witness.chunk_shape_version(),
            determinism_digest: witness.determinism_digest().to_string(),
            ordered_slice_ids: witness.ordered_slice_ids().to_vec(),
        }
    }
}

impl From<&ChunkModelFrozenPhysicalLayout> for PersistedChunkModelFrozenPhysicalLayout {
    fn from(frozen: &ChunkModelFrozenPhysicalLayout) -> Self {
        Self {
            request: frozen.request().clone(),
            chunk_width: frozen.chunk_width(),
            witness: PersistedChunkDeterminismWitness::from(frozen.witness()),
        }
    }
}

impl From<&Milestone7IndependentLayoutReference> for PersistedMilestone7IndependentLayoutReference {
    fn from(reference: &Milestone7IndependentLayoutReference) -> Self {
        Self {
            branch_id: reference.branch_id().clone(),
            frontier_commit_id: reference.frontier_commit_id(),
            scope_class: reference.scope_class().to_string(),
            projection_digest: reference.projection_digest().to_string(),
        }
    }
}

impl From<&Milestone9PhysicalChunkReference> for PersistedMilestone9PhysicalChunkReference {
    fn from(reference: &Milestone9PhysicalChunkReference) -> Self {
        Self {
            physical_chunk_id: reference.physical_chunk_id().clone(),
            chunk_shape_version: reference.chunk_shape_version(),
            determinism_digest: reference.determinism_digest().to_string(),
            chunk_member_count: reference.chunk_member_count(),
        }
    }
}

impl TryFrom<PersistedMilestone6LayoutMaterializationRecord> for Milestone6LayoutMaterializationRecord {
    type Error = String;

    fn try_from(record: PersistedMilestone6LayoutMaterializationRecord) -> Result<Self, Self::Error> {
        validation::validate_persisted_milestone_6_layout_materialization_record(&record)?;
        Ok(Self {
            artifact_id: record.artifact_id,
            materialization: Milestone6LayoutMaterialization::try_from(record.materialization)?,
        })
    }
}

impl TryFrom<PersistedMilestone6LayoutMaterialization> for Milestone6LayoutMaterialization {
    type Error = String;

    fn try_from(materialization: PersistedMilestone6LayoutMaterialization) -> Result<Self, Self::Error> {
        validation::validate_persisted_milestone_6_layout_materialization(&materialization)?;
        let admitted_plan = AdmittedAspectLayoutReadPlan::new(
            materialization.admitted_plan.request,
            materialization.admitted_plan.slice_ids,
            materialization.admitted_plan.structural_block_id,
            materialization.admitted_plan.performance,
        );
        let block_reuse = DedupAdmittedBlockReuse::from_parts(
            materialization.block_reuse.branch_id,
            materialization.block_reuse.frontier_commit_id,
            materialization.block_reuse.scope_class,
            materialization.block_reuse.structural_block_id,
            materialization.block_reuse.equivalence_contract_version,
            materialization.block_reuse.slice_ids,
        );
        let witness = ChunkDeterminismWitness::new(
            materialization.frozen_layout.witness.physical_chunk_id,
            materialization.frozen_layout.witness.chunk_shape_version,
            materialization.frozen_layout.witness.determinism_digest,
            materialization.frozen_layout.witness.ordered_slice_ids,
        );
        let frozen_layout = ChunkModelFrozenPhysicalLayout::new(
            materialization.frozen_layout.request,
            materialization.frozen_layout.chunk_width,
            witness,
        );
        let milestone_7_reference = Milestone7IndependentLayoutReference::new(
            materialization.milestone_7_reference.branch_id,
            materialization.milestone_7_reference.frontier_commit_id,
            materialization.milestone_7_reference.scope_class,
            materialization.milestone_7_reference.projection_digest,
        );
        let milestone_9_reference = Milestone9PhysicalChunkReference::new(
            materialization.milestone_9_reference.physical_chunk_id,
            materialization.milestone_9_reference.chunk_shape_version,
            materialization.milestone_9_reference.determinism_digest,
            materialization.milestone_9_reference.chunk_member_count,
        );
        Ok(Milestone6LayoutMaterialization::new(
            materialization.artifact_id,
            admitted_plan,
            block_reuse,
            frozen_layout,
            milestone_7_reference,
            milestone_9_reference,
            materialization.semantic_truth_digest,
            materialization.authoritative_commit_count,
        ))
    }
}
