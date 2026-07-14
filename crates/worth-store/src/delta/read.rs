use crate::{authority::AuthoritativeExportBundle, delta::ComplexityStatus};
use worth_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

use super::BranchDeltaLayerId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaReadRequest {
    pub branch_id: BranchId,
    pub target_commit_id: CommitId,
}

impl BranchDeltaReadRequest {
    pub fn new(branch_id: BranchId, target_commit_id: CommitId) -> Self {
        Self {
            branch_id,
            target_commit_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaReadStrategy {
    EmptyBranchReuse,
    DirectLayerRead,
    AuthorityReplayControl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaReadRegime {
    Sparse,
    Dense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaFallbackClass {
    None,
    RequiresAuthorityReplayControlLane,
    RequiresMergeAwareWidening,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaLocality {
    pub branch_id: BranchId,
    pub base_frontier_commit_id: Option<CommitId>,
    pub target_commit_id: CommitId,
    pub commit_span: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaPerformanceEnvelope {
    pub layers_traversed: usize,
    pub records_decoded: usize,
    pub replay_commit_count: usize,
    pub fallback_class: BranchDeltaFallbackClass,
    pub complexity_status: ComplexityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaReadPlan {
    pub strategy: BranchDeltaReadStrategy,
    pub regime: BranchDeltaReadRegime,
    pub locality: BranchDeltaLocality,
    pub used_layer_ids: Vec<BranchDeltaLayerId>,
    pub commit_ids: Vec<CommitId>,
    pub performance: BranchDeltaPerformanceEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SameBranchDescendantWitness {
    branch_id: BranchId,
    base_frontier_commit_id: Option<CommitId>,
    target_commit_id: CommitId,
    commit_ids: Vec<CommitId>,
}

impl SameBranchDescendantWitness {
    pub(crate) fn new(
        branch_id: BranchId,
        base_frontier_commit_id: Option<CommitId>,
        target_commit_id: CommitId,
        commit_ids: Vec<CommitId>,
    ) -> Self {
        Self {
            branch_id,
            base_frontier_commit_id,
            target_commit_id,
            commit_ids,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn base_frontier_commit_id(&self) -> Option<CommitId> {
        self.base_frontier_commit_id
    }

    pub fn target_commit_id(&self) -> CommitId {
        self.target_commit_id
    }

    pub fn commit_ids(&self) -> &[CommitId] {
        &self.commit_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone7IndependentReference {
    branch_id: BranchId,
    target_commit_id: CommitId,
}

impl Milestone7IndependentReference {
    pub(crate) fn new(branch_id: BranchId, target_commit_id: CommitId) -> Self {
        Self {
            branch_id,
            target_commit_id,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn target_commit_id(&self) -> CommitId {
        self.target_commit_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaReadResult {
    pub plan: BranchDeltaReadPlan,
    authoritative_export: AuthoritativeExportBundle,
}

impl BranchDeltaReadResult {
    pub fn new(plan: BranchDeltaReadPlan, authoritative_export: AuthoritativeExportBundle) -> Self {
        Self {
            plan,
            authoritative_export: authoritative_export.into_canonicalized(),
        }
    }

    pub fn authoritative_export(&self) -> &AuthoritativeExportBundle {
        &self.authoritative_export
    }
}
