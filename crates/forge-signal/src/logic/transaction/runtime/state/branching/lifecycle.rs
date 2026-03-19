use crate::data::error::SignalError;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::branches::BranchAncestryState;
use super::super::merge::BranchMutationLedger;
use super::super::runtime_state::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn create_branch(
        &mut self,
        name: impl Into<String>,
    ) -> Result<SignalBranchHandle, SignalError> {
        let current_branch_name = self.graph.current_branch().name;
        let parent_branch_id = self.graph.current_branch().id;
        let handle = self.graph.diagnostics_state_mut().create_branch(name);
        let mut branch_state = self.capture_branch_state();
        branch_state.ancestry = BranchAncestryState {
            branch_id: handle.id,
            parent_branch_id: Some(parent_branch_id),
            forked_from_snapshot_id: self.graph.current_branch().head_snapshot_id,
            latest_merge_reference: None,
        };
        branch_state.mutation_ledger =
            BranchMutationLedger::default().with_baseline_snapshot(handle.head_snapshot_id);
        branch_state.authority.graph.clear_branch_mutation_nodes();
        branch_state
            .authority
            .graph
            .diagnostics_state_mut()
            .set_active_branch(handle.id);
        self.branches.insert_branch(handle.id, branch_state);
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        crate::diagnostics::recorder::record_snapshot_event(
            &mut self.graph,
            crate::diagnostics::replay::ReplayEventKind::BranchCreated,
            None,
            format!("created branch `{}`", handle.name),
        );
        crate::diagnostics::recorder::record_branch_fork_lineage(
            &mut self.graph,
            handle.id,
            parent_branch_id,
            handle.name.clone(),
            current_branch_name.to_string(),
        );
        Ok(handle)
    }

    pub fn switch_branch(&mut self, branch: SignalBranchHandle) -> Result<(), SignalError> {
        let current = self.graph.current_branch();
        let current_state = self.capture_branch_state();
        self.branches.insert_branch(current.id, current_state);
        let Some(state) = self.branches.cloned_branch_state(branch.id) else {
            return Err(SignalError::unknown_branch(Some(branch.id), branch.name));
        };
        self.load_branch_state(state);
        self.graph.diagnostics_state_mut().set_active_branch(branch.id);
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        crate::diagnostics::recorder::record_snapshot_event(
            &mut self.graph,
            crate::diagnostics::replay::ReplayEventKind::BranchSwitched,
            None,
            format!("switched from `{}` to `{}`", current.name, branch.name),
        );
        crate::diagnostics::recorder::record_branch_switch_lineage(
            &mut self.graph,
            current.id,
            branch.id,
            current.name.to_string(),
            branch.name.clone(),
        );
        Ok(())
    }

    pub fn current_branch(&self) -> SignalBranchHandle {
        self.graph.current_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.graph.known_branches()
    }

    pub fn branch_handle(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.graph
            .branch_handle(branch_id)
            .or_else(|| self.branches.branch_handle(branch_id))
    }

    pub fn branch_ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        if self.graph.branch_handle(branch_id).is_some() {
            self.graph.branch_ancestry(branch_id)
        } else {
            self.branches.branch_ancestry(branch_id)
        }
    }

    pub fn branch_head_snapshot_id(&self, branch_id: SignalBranchId) -> Option<SignalSnapshotId> {
        self.graph
            .branch_head_snapshot_id(branch_id)
            .or_else(|| self.branches.branch_head_snapshot_id(branch_id))
    }

    fn replay_graph_for_branch(
        &self,
        branch_id: SignalBranchId,
    ) -> Option<&crate::data::graph::SignalGraph> {
        self.branches
            .replay_graph(branch_id, self.graph.current_branch().id, &self.graph)
    }

    pub fn replay_for_branch(&self, branch_id: SignalBranchId) -> crate::diagnostics::ReplaySlice {
        self.replay_graph_for_branch(branch_id)
            .map(|graph| graph.replay_for_branch(branch_id))
            .unwrap_or_default()
    }
}
