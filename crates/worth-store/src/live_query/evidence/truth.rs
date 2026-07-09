use crate::live_query::basis::StableBasisHandle;
use worth_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone8TruthSurface {
    pub branch_id: BranchId,
    pub stable_basis_id: String,
    pub read_scope_fingerprint: String,
    pub final_frontier_commit_id: CommitId,
}

impl Milestone8TruthSurface {
    pub fn from_basis_and_frontier(
        basis: &StableBasisHandle,
        final_frontier_commit_id: CommitId,
    ) -> Self {
        Self {
            branch_id: basis.branch_id().clone(),
            stable_basis_id: basis.stable_basis_id().as_str().to_string(),
            read_scope_fingerprint: basis.read_scope().fingerprint(),
            final_frontier_commit_id,
        }
    }
}
