use crate::state::SignalBranchHandle;

use super::GraphObserver;

impl<'a> GraphObserver<'a> {
    pub fn current_branch(&self) -> SignalBranchHandle {
        self.graph.observation.diagnostics.active_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.graph
            .observation
            .diagnostics
            .branch_catalog()
            .values()
            .cloned()
            .collect()
    }

    pub fn branch_handle(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Option<SignalBranchHandle> {
        self.graph
            .observation
            .diagnostics
            .branch_catalog()
            .get(&branch_id)
            .cloned()
    }

    pub fn branch_head_snapshot_id(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Option<crate::state::SignalSnapshotId> {
        self.branch_handle(branch_id)
            .and_then(|branch| branch.head_snapshot_id)
    }

    pub fn branch_ancestry(
        &self,
        branch_id: crate::state::SignalBranchId,
    ) -> Vec<SignalBranchHandle> {
        let mut lineage = Vec::new();
        let mut current = self.branch_handle(branch_id);
        while let Some(branch) = current {
            current = branch
                .parent_branch_id
                .and_then(|parent_id| self.branch_handle(parent_id));
            lineage.push(branch);
        }
        lineage.reverse();
        lineage
    }
}
