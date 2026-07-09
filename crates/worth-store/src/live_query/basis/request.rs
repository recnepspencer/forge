use worth_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

use crate::live_query::retention_descriptor::ContinuationRetentionStatus;
use crate::Milestone6ResolvedLayoutSupportLane;

use super::StableBasisReadScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StableBasisLayoutPosture {
    ProofOnly,
    OnDemandMaterialized,
    PolicyEagerMaterializedPublished,
    PolicyEagerMaterializedReuseExisting,
}

impl StableBasisLayoutPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProofOnly => "proof_only",
            Self::OnDemandMaterialized => "on_demand_materialized",
            Self::PolicyEagerMaterializedPublished => "policy_eager_materialized_published",
            Self::PolicyEagerMaterializedReuseExisting => {
                "policy_eager_materialized_reuse_existing"
            }
        }
    }
}

impl From<Milestone6ResolvedLayoutSupportLane> for StableBasisLayoutPosture {
    fn from(value: Milestone6ResolvedLayoutSupportLane) -> Self {
        match value {
            Milestone6ResolvedLayoutSupportLane::ProofOnly => Self::ProofOnly,
            Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized => Self::OnDemandMaterialized,
            Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedPublished => {
                Self::PolicyEagerMaterializedPublished
            }
            Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedReuseExisting => {
                Self::PolicyEagerMaterializedReuseExisting
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableBasisReadRequest {
    branch_id: BranchId,
    frontier_commit_id: CommitId,
    read_scope: StableBasisReadScope,
    support_context_digest: String,
    schema_boundary_artifact_id: String,
    layout_posture: StableBasisLayoutPosture,
    authority_basis_digest: String,
    retention_status: ContinuationRetentionStatus,
}

impl StableBasisReadRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        branch_id: BranchId,
        frontier_commit_id: CommitId,
        read_scope: StableBasisReadScope,
        support_context_digest: impl Into<String>,
        schema_boundary_artifact_id: impl Into<String>,
        layout_posture: StableBasisLayoutPosture,
        authority_basis_digest: impl Into<String>,
        retention_status: ContinuationRetentionStatus,
    ) -> Self {
        Self {
            branch_id,
            frontier_commit_id,
            read_scope,
            support_context_digest: support_context_digest.into(),
            schema_boundary_artifact_id: schema_boundary_artifact_id.into(),
            layout_posture,
            authority_basis_digest: authority_basis_digest.into(),
            retention_status,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn frontier_commit_id(&self) -> CommitId {
        self.frontier_commit_id
    }

    pub fn read_scope(&self) -> &StableBasisReadScope {
        &self.read_scope
    }

    pub fn support_context_digest(&self) -> &str {
        &self.support_context_digest
    }

    pub fn schema_boundary_artifact_id(&self) -> &str {
        &self.schema_boundary_artifact_id
    }

    pub fn layout_posture(&self) -> StableBasisLayoutPosture {
        self.layout_posture
    }

    pub fn authority_basis_digest(&self) -> &str {
        &self.authority_basis_digest
    }

    pub fn retention_status(&self) -> &ContinuationRetentionStatus {
        &self.retention_status
    }

    pub(crate) fn set_retention_status(&mut self, retention_status: ContinuationRetentionStatus) {
        self.retention_status = retention_status;
    }
}
