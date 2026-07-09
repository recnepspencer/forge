use crate::{
    AdmittedAspectLayoutReadPlan, Milestone7IndependentLayoutReference,
    Milestone9PhysicalChunkReference,
};
use worth_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6LayoutReadReport {
    pub strategy: crate::AspectReadRegime,
    pub scope_class: String,
    pub complexity_status: crate::ComplexityStatus,
    pub fallback_class: crate::AspectLayoutFallbackClass,
    pub layout_slices_read: usize,
    pub blocks_decoded: usize,
    pub control_replay_breadth: usize,
    pub chunk_count: usize,
}

impl From<&AdmittedAspectLayoutReadPlan> for Milestone6LayoutReadReport {
    fn from(plan: &AdmittedAspectLayoutReadPlan) -> Self {
        Self {
            strategy: plan.performance().strategy,
            scope_class: plan.performance().scope_class.clone(),
            complexity_status: plan.performance().complexity_status,
            fallback_class: plan.performance().fallback_class,
            layout_slices_read: plan.performance().layout_slices_read,
            blocks_decoded: plan.performance().blocks_decoded,
            control_replay_breadth: plan.performance().control_replay_breadth,
            chunk_count: plan.performance().chunk_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6PhysicalLayoutReport {
    pub branch_id: BranchId,
    pub frontier_commit_id: CommitId,
    pub scope_class: String,
    pub projection_digest: String,
    pub slice_ids: Vec<String>,
    pub structural_block_id: String,
    pub equivalence_contract_version: u32,
    pub physical_chunk_id: String,
    pub chunk_shape_version: u32,
    pub chunk_width: u64,
    pub determinism_digest: String,
    pub milestone_9_chunk_member_count: usize,
}

impl Milestone6PhysicalLayoutReport {
    pub(crate) fn from_references(
        reuse: &crate::DedupAdmittedBlockReuse,
        frozen: &crate::ChunkModelFrozenPhysicalLayout,
        milestone_7: &Milestone7IndependentLayoutReference,
        milestone_9: &Milestone9PhysicalChunkReference,
    ) -> Self {
        Self {
            branch_id: milestone_7.branch_id().clone(),
            frontier_commit_id: milestone_7.frontier_commit_id(),
            scope_class: milestone_7.scope_class().to_string(),
            projection_digest: milestone_7.projection_digest().to_string(),
            slice_ids: reuse
                .slice_ids()
                .iter()
                .map(|slice_id| slice_id.as_str().to_string())
                .collect(),
            structural_block_id: reuse.structural_block_id().as_str().to_string(),
            equivalence_contract_version: reuse.equivalence_contract_version().value(),
            physical_chunk_id: milestone_9.physical_chunk_id().as_str().to_string(),
            chunk_shape_version: milestone_9.chunk_shape_version().value(),
            chunk_width: frozen.chunk_width(),
            determinism_digest: milestone_9.determinism_digest().to_string(),
            milestone_9_chunk_member_count: milestone_9.chunk_member_count(),
        }
    }
}
