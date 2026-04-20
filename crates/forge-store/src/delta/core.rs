use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

pub const BRANCH_DELTA_FAMILY_VERSION: u32 = 1;
pub const MAX_DIRECT_LAYER_READ_DEPTH: usize = 4;
pub const MAX_DIRECT_LAYER_READ_RECORDS: usize = 32;
pub const MAX_REWRITE_LAYER_WIDTH: usize = 3;
pub const RECOMMENDED_REWRITE_LAYER_WIDTH: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BranchDeltaLayerId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedBaseBranchCreationRequest {
    pub new_branch_id: BranchId,
    pub source_branch_id: BranchId,
}

impl SharedBaseBranchCreationRequest {
    pub fn new(new_branch_id: BranchId, source_branch_id: BranchId) -> Self {
        Self {
            new_branch_id,
            source_branch_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedBaseBranchCreationReceipt {
    pub branch_id: BranchId,
    pub source_branch_id: BranchId,
    pub source_frontier_commit_id: Option<CommitId>,
    pub delta_family_version: u32,
    pub authority_basis_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedBaseBranchCreationWitness {
    request: SharedBaseBranchCreationRequest,
    source_frontier_commit_id: Option<CommitId>,
    authority_basis_digest: String,
}

impl SharedBaseBranchCreationWitness {
    pub(crate) fn new(
        request: SharedBaseBranchCreationRequest,
        source_frontier_commit_id: Option<CommitId>,
        authority_basis_digest: String,
    ) -> Self {
        Self {
            request,
            source_frontier_commit_id,
            authority_basis_digest,
        }
    }

    pub fn request(&self) -> &SharedBaseBranchCreationRequest {
        &self.request
    }

    pub fn source_frontier_commit_id(&self) -> Option<CommitId> {
        self.source_frontier_commit_id
    }

    pub fn authority_basis_digest(&self) -> &str {
        &self.authority_basis_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplexityStatus {
    Verified,
    Debt,
}
