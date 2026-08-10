use std::collections::BTreeMap;

use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::DiagnosticsState;

impl DiagnosticsState {
    pub fn bootstrap_defaults(&mut self) {
        if self.branch_catalog.is_empty() {
            self.branch_catalog.insert(
                SignalBranchId(0),
                SignalBranchHandle {
                    id: SignalBranchId(0),
                    name: "main".to_string(),
                    parent_branch_id: None,
                    head_snapshot_id: None,
                },
            );
        }
    }

    pub fn branch_catalog(&self) -> &BTreeMap<SignalBranchId, SignalBranchHandle> {
        &self.branch_catalog
    }

    pub fn active_branch(&self) -> SignalBranchHandle {
        self.branch_catalog
            .get(&self.active_branch)
            .cloned()
            .unwrap_or_else(|| SignalBranchHandle {
                id: self.active_branch,
                name: "unknown".to_string(),
                parent_branch_id: None,
                head_snapshot_id: None,
            })
    }

    pub fn create_branch_from_basis(
        &mut self,
        name: impl Into<String>,
        parent_branch_id: SignalBranchId,
        parent_head_snapshot_id: Option<SignalSnapshotId>,
    ) -> SignalBranchHandle {
        self.bootstrap_defaults();
        let handle = SignalBranchHandle {
            id: SignalBranchId(self.next_branch_id.max(1)),
            name: name.into(),
            parent_branch_id: Some(parent_branch_id),
            head_snapshot_id: parent_head_snapshot_id,
        };
        self.next_branch_id = handle.id.0 + 1;
        self.branch_catalog.insert(handle.id, handle.clone());
        handle
    }

    pub fn set_active_branch(&mut self, branch_id: SignalBranchId) {
        self.bootstrap_defaults();
        self.active_branch = branch_id;
    }

    pub(crate) fn retire_branch_from_catalog(&mut self, branch_id: SignalBranchId) {
        debug_assert_ne!(self.active_branch, branch_id);
        self.branch_catalog.remove(&branch_id);
    }

    pub fn set_branch_head_snapshot(
        &mut self,
        branch_id: SignalBranchId,
        snapshot_id: SignalSnapshotId,
    ) {
        self.bootstrap_defaults();
        if let Some(branch) = self.branch_catalog.get_mut(&branch_id) {
            branch.head_snapshot_id = Some(snapshot_id);
        }
    }

    pub fn synchronize_branch_catalog(
        &mut self,
        branch_catalog: &BTreeMap<SignalBranchId, SignalBranchHandle>,
        active_branch: SignalBranchId,
    ) {
        self.branch_catalog.clone_from(branch_catalog);
        self.active_branch = active_branch;
    }

    pub fn branch_snapshot_allocator_state(&self) -> (u64, u64) {
        (self.next_snapshot_id, self.next_branch_id)
    }

    pub fn synchronize_branch_snapshot_allocator(
        &mut self,
        next_snapshot_id: u64,
        next_branch_id: u64,
    ) {
        self.next_snapshot_id = self.next_snapshot_id.max(next_snapshot_id);
        self.next_branch_id = self.next_branch_id.max(next_branch_id);
    }
}
