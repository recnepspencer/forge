use forge_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

use crate::live_query::basis::{StableBasisId, StableBasisReadScope};

use crate::live_query::continuation::ContinuationStrategy;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CaughtUpContinuationBatch {
    stable_basis_id: StableBasisId,
    branch_id: BranchId,
    frontier_commit_id: CommitId,
    resolved_scope: StableBasisReadScope,
    resolved_strategy: ContinuationStrategy,
}

impl CaughtUpContinuationBatch {
    pub(crate) fn new(
        stable_basis_id: StableBasisId,
        branch_id: BranchId,
        frontier_commit_id: CommitId,
        resolved_scope: StableBasisReadScope,
        resolved_strategy: ContinuationStrategy,
    ) -> Self {
        Self {
            stable_basis_id,
            branch_id,
            frontier_commit_id,
            resolved_scope,
            resolved_strategy,
        }
    }

    pub fn frontier_commit_id(&self) -> CommitId { self.frontier_commit_id }
    pub fn resolved_scope(&self) -> &StableBasisReadScope { &self.resolved_scope }
    pub fn resolved_strategy(&self) -> ContinuationStrategy { self.resolved_strategy }
}
