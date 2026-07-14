use crate::live_query::basis::{StableBasisHandle, StableBasisLayoutPosture};
use crate::live_query::retention_descriptor::ContinuationRetentionStatus;
use worth_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LiveQueryComplexityStatus {
    Verified,
    Debt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LiveQueryBasisEvidence {
    pub stable_basis_id: String,
    pub branch_id: BranchId,
    pub frontier_commit_id: CommitId,
    pub read_scope_fingerprint: String,
    pub schema_boundary_artifact_id: String,
    pub support_context_digest: String,
    pub layout_posture: StableBasisLayoutPosture,
    pub retention_status: ContinuationRetentionStatus,
    pub fallback_class: Option<String>,
    pub complexity_status: LiveQueryComplexityStatus,
}

impl LiveQueryBasisEvidence {
    pub fn from_handle(handle: &StableBasisHandle) -> Self {
        Self {
            stable_basis_id: handle.stable_basis_id().as_str().to_string(),
            branch_id: handle.branch_id().clone(),
            frontier_commit_id: handle.frontier_commit_id(),
            read_scope_fingerprint: handle.read_scope().fingerprint(),
            schema_boundary_artifact_id: handle.schema_boundary_artifact_id().to_string(),
            support_context_digest: handle.support_context_digest().to_string(),
            layout_posture: handle.layout_posture(),
            retention_status: handle.retention_status().clone(),
            fallback_class: handle.fallback_class().map(str::to_string),
            complexity_status: handle.complexity_status(),
        }
    }
}
