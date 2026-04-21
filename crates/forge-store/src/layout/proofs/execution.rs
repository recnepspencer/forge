use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

use super::{
    core::{
        Milestone6LayoutSupportLane, Milestone6LayoutSupportPublicationDisposition,
        Milestone6ResolvedLayoutSupportLane, PhysicalChunkId,
    },
    lookup::StructuralBlockLookupResult,
    physical::{
        ChunkModelFrozenPhysicalLayout, Milestone7IndependentLayoutReference,
        Milestone9PhysicalChunkReference,
    },
    planning::{
        AdmittedAspectLayoutReadPlan, DedupAdmittedBlockReuse, ExplicitBroadFallbackPlan,
        RejectedAspectLayoutReadPlan,
    },
};
use crate::ForegroundIsolationOutcome;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6LayoutMaterialization {
    artifact_id: String,
    admitted_plan: AdmittedAspectLayoutReadPlan,
    block_reuse: DedupAdmittedBlockReuse,
    frozen_layout: ChunkModelFrozenPhysicalLayout,
    milestone_7_reference: Milestone7IndependentLayoutReference,
    milestone_9_reference: Milestone9PhysicalChunkReference,
    semantic_truth_digest: String,
    authoritative_commit_count: usize,
}
impl Milestone6LayoutMaterialization {
    pub(crate) fn new(
        artifact_id: String,
        admitted_plan: AdmittedAspectLayoutReadPlan,
        block_reuse: DedupAdmittedBlockReuse,
        frozen_layout: ChunkModelFrozenPhysicalLayout,
        milestone_7_reference: Milestone7IndependentLayoutReference,
        milestone_9_reference: Milestone9PhysicalChunkReference,
        semantic_truth_digest: String,
        authoritative_commit_count: usize,
    ) -> Self {
        Self {
            artifact_id,
            admitted_plan,
            block_reuse,
            frozen_layout,
            milestone_7_reference,
            milestone_9_reference,
            semantic_truth_digest,
            authoritative_commit_count,
        }
    }
    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }
    pub fn admitted_plan(&self) -> &AdmittedAspectLayoutReadPlan {
        &self.admitted_plan
    }
    pub fn block_reuse(&self) -> &DedupAdmittedBlockReuse {
        &self.block_reuse
    }
    pub fn frozen_layout(&self) -> &ChunkModelFrozenPhysicalLayout {
        &self.frozen_layout
    }
    pub fn milestone_7_reference(&self) -> &Milestone7IndependentLayoutReference {
        &self.milestone_7_reference
    }
    pub fn milestone_9_reference(&self) -> &Milestone9PhysicalChunkReference {
        &self.milestone_9_reference
    }
    pub fn semantic_truth_digest(&self) -> &str {
        &self.semantic_truth_digest
    }
    pub fn authoritative_commit_count(&self) -> usize {
        self.authoritative_commit_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AspectLayoutReadExecutionResult {
    plan: AdmittedAspectLayoutReadPlan,
    requested_layout_support_lane: Milestone6LayoutSupportLane,
    resolved_layout_support_lane: Milestone6ResolvedLayoutSupportLane,
    layout_support_publication_disposition: Milestone6LayoutSupportPublicationDisposition,
    scope_membership_artifact_id: Option<String>,
    structural_block_artifact_id: String,
    chunk_membership_artifact_id: Option<String>,
    layout_materialization_artifact_id: Option<String>,
    semantic_truth_digest: String,
    authoritative_commit_count: usize,
    foreground_isolation: ForegroundIsolationOutcome,
}
impl AspectLayoutReadExecutionResult {
    pub(crate) fn new(
        plan: AdmittedAspectLayoutReadPlan,
        requested_layout_support_lane: Milestone6LayoutSupportLane,
        resolved_layout_support_lane: Milestone6ResolvedLayoutSupportLane,
        layout_support_publication_disposition: Milestone6LayoutSupportPublicationDisposition,
        scope_membership_artifact_id: Option<String>,
        structural_block_artifact_id: String,
        chunk_membership_artifact_id: Option<String>,
        layout_materialization_artifact_id: Option<String>,
        semantic_truth_digest: String,
        authoritative_commit_count: usize,
    ) -> Self {
        Self {
            plan,
            requested_layout_support_lane,
            resolved_layout_support_lane,
            layout_support_publication_disposition,
            scope_membership_artifact_id,
            structural_block_artifact_id,
            chunk_membership_artifact_id,
            layout_materialization_artifact_id,
            semantic_truth_digest,
            authoritative_commit_count,
            foreground_isolation: ForegroundIsolationOutcome::stayed_isolated(
                crate::ForegroundReservationClass::Read,
            ),
        }
    }
    pub fn plan(&self) -> &AdmittedAspectLayoutReadPlan {
        &self.plan
    }
    pub fn requested_layout_support_lane(&self) -> Milestone6LayoutSupportLane {
        self.requested_layout_support_lane
    }
    pub fn resolved_layout_support_lane(&self) -> Milestone6ResolvedLayoutSupportLane {
        self.resolved_layout_support_lane
    }
    pub fn layout_support_publication_disposition(
        &self,
    ) -> Milestone6LayoutSupportPublicationDisposition {
        self.layout_support_publication_disposition
    }
    pub fn scope_membership_artifact_id(&self) -> Option<&str> {
        self.scope_membership_artifact_id.as_deref()
    }
    pub fn structural_block_artifact_id(&self) -> &str {
        &self.structural_block_artifact_id
    }
    pub fn chunk_membership_artifact_id(&self) -> Option<&str> {
        self.chunk_membership_artifact_id.as_deref()
    }
    pub fn layout_materialization_artifact_id(&self) -> Option<&str> {
        self.layout_materialization_artifact_id.as_deref()
    }
    pub fn semantic_truth_digest(&self) -> &str {
        &self.semantic_truth_digest
    }
    pub fn authoritative_commit_count(&self) -> usize {
        self.authoritative_commit_count
    }
    pub fn foreground_isolation(&self) -> &ForegroundIsolationOutcome {
        &self.foreground_isolation
    }
    pub(crate) fn with_foreground_isolation(
        mut self,
        foreground_isolation: ForegroundIsolationOutcome,
    ) -> Self {
        self.foreground_isolation = foreground_isolation;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AspectLayoutControlTruth {
    branch_id: BranchId,
    frontier_commit_id: CommitId,
    scope_class: String,
    projection_digest: String,
    authoritative_truth_digest: String,
    authoritative_commit_count: usize,
}
impl AspectLayoutControlTruth {
    pub(crate) fn new(
        branch_id: BranchId,
        frontier_commit_id: CommitId,
        scope_class: String,
        projection_digest: String,
        authoritative_truth_digest: String,
        authoritative_commit_count: usize,
    ) -> Self {
        Self {
            branch_id,
            frontier_commit_id,
            scope_class,
            projection_digest,
            authoritative_truth_digest,
            authoritative_commit_count,
        }
    }
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub fn frontier_commit_id(&self) -> CommitId {
        self.frontier_commit_id
    }
    pub fn scope_class(&self) -> &str {
        &self.scope_class
    }
    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }
    pub fn authoritative_truth_digest(&self) -> &str {
        &self.authoritative_truth_digest
    }
    pub fn authoritative_commit_count(&self) -> usize {
        self.authoritative_commit_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum AspectLayoutReadExecutionDecision {
    Admitted(AspectLayoutReadExecutionResult),
    Fallback(ExplicitBroadFallbackPlan),
    Rejected(RejectedAspectLayoutReadPlan),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DedupBackedReadResult {
    read: AspectLayoutReadExecutionResult,
    structural_block_lookup: StructuralBlockLookupResult,
}
impl DedupBackedReadResult {
    pub(crate) fn new(
        read: AspectLayoutReadExecutionResult,
        structural_block_lookup: StructuralBlockLookupResult,
    ) -> Self {
        Self {
            read,
            structural_block_lookup,
        }
    }
    pub fn read(&self) -> &AspectLayoutReadExecutionResult {
        &self.read
    }
    pub fn structural_block_lookup(&self) -> &StructuralBlockLookupResult {
        &self.structural_block_lookup
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone6DerivedArtifactRebuildReport {
    layout_materialization_count: usize,
    scope_membership_count: usize,
    structural_block_count: usize,
    chunk_membership_count: usize,
}
impl Milestone6DerivedArtifactRebuildReport {
    pub(crate) fn new(
        layout_materialization_count: usize,
        scope_membership_count: usize,
        structural_block_count: usize,
        chunk_membership_count: usize,
    ) -> Self {
        Self {
            layout_materialization_count,
            scope_membership_count,
            structural_block_count,
            chunk_membership_count,
        }
    }
    pub fn layout_materialization_count(&self) -> usize {
        self.layout_materialization_count
    }
    pub fn scope_membership_count(&self) -> usize {
        self.scope_membership_count
    }
    pub fn structural_block_count(&self) -> usize {
        self.structural_block_count
    }
    pub fn chunk_membership_count(&self) -> usize {
        self.chunk_membership_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6ChunkModelExport {
    requested_layout_support_lane: Milestone6LayoutSupportLane,
    resolved_layout_support_lane: Milestone6ResolvedLayoutSupportLane,
    layout_support_publication_disposition: Milestone6LayoutSupportPublicationDisposition,
    physical_chunk_id: PhysicalChunkId,
    chunk_membership_artifact_id: Option<String>,
    determinism_digest: String,
    chunk_member_count: usize,
    layout_materialization_artifact_id: Option<String>,
}
impl Milestone6ChunkModelExport {
    pub(crate) fn new(
        requested_layout_support_lane: Milestone6LayoutSupportLane,
        resolved_layout_support_lane: Milestone6ResolvedLayoutSupportLane,
        layout_support_publication_disposition: Milestone6LayoutSupportPublicationDisposition,
        physical_chunk_id: PhysicalChunkId,
        chunk_membership_artifact_id: Option<String>,
        determinism_digest: String,
        chunk_member_count: usize,
        layout_materialization_artifact_id: Option<String>,
    ) -> Self {
        Self {
            requested_layout_support_lane,
            resolved_layout_support_lane,
            layout_support_publication_disposition,
            physical_chunk_id,
            chunk_membership_artifact_id,
            determinism_digest,
            chunk_member_count,
            layout_materialization_artifact_id,
        }
    }
    pub fn physical_chunk_id(&self) -> &PhysicalChunkId {
        &self.physical_chunk_id
    }
    pub fn requested_layout_support_lane(&self) -> Milestone6LayoutSupportLane {
        self.requested_layout_support_lane
    }
    pub fn resolved_layout_support_lane(&self) -> Milestone6ResolvedLayoutSupportLane {
        self.resolved_layout_support_lane
    }
    pub fn layout_support_publication_disposition(
        &self,
    ) -> Milestone6LayoutSupportPublicationDisposition {
        self.layout_support_publication_disposition
    }
    pub fn chunk_membership_artifact_id(&self) -> Option<&str> {
        self.chunk_membership_artifact_id.as_deref()
    }
    pub fn determinism_digest(&self) -> &str {
        &self.determinism_digest
    }
    pub fn chunk_member_count(&self) -> usize {
        self.chunk_member_count
    }
    pub fn layout_materialization_artifact_id(&self) -> Option<&str> {
        self.layout_materialization_artifact_id.as_deref()
    }
}
