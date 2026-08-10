use crate::state::SignalBranchId;

use super::super::super::runtime_state::AuthorityTransferPacket;

use super::authority::BranchState;
use super::catalog::BranchManager;

impl<D, I, T> BranchManager<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn take_branch_state(&mut self, branch_id: SignalBranchId) -> Option<BranchState<D, I, T>> {
        self.branches.remove(&branch_id)
    }

    pub(in crate::logic::transaction::runtime::state::branching) fn take_stored_branch_transfer(
        &mut self,
        branch_id: SignalBranchId,
    ) -> Option<AuthorityTransferPacket<D, I, T>> {
        self.take_branch_state(branch_id)
            .map(|state| AuthorityTransferPacket::new(branch_id, state))
    }

    pub(in crate::logic::transaction::runtime::state::branching) fn with_stored_branch_state_mut<
        R,
    >(
        &mut self,
        branch_id: SignalBranchId,
        f: impl FnOnce(&mut BranchState<D, I, T>) -> R,
    ) -> Option<R> {
        let next_node_index = self.next_node_index;
        let next_snapshot_id = self.next_snapshot_id;
        let next_branch_id = self.next_branch_id;
        let next_lineage_artifact_id = self.next_lineage_artifact_id;
        let next_lineage_sequence = self.next_lineage_sequence;
        let live_branch_catalog = self.live_branch_catalog.clone();
        let state = self.branches.get_mut(&branch_id)?;
        state
            .graph_mut()
            .diagnostics_state_mut()
            .synchronize_branch_catalog(&live_branch_catalog, branch_id);
        state
            .graph_mut()
            .synchronize_node_allocator(next_node_index);
        state
            .graph_mut()
            .diagnostics_state_mut()
            .synchronize_branch_snapshot_allocator(next_snapshot_id, next_branch_id);
        state
            .graph_mut()
            .diagnostics_state_mut()
            .synchronize_lineage_allocator(next_lineage_artifact_id, next_lineage_sequence);
        let result = f(state);
        let ancestry = state.ancestry().clone();
        let mutation_ledger = state.mutation_ledger().clone();
        let live_handle = state.graph().branch_handle(branch_id);
        let _ = state;
        if let Some(handle) = live_handle {
            self.live_branch_catalog.insert(branch_id, handle);
        }
        self.record_branch_meta(branch_id, ancestry, mutation_ledger);
        Some(result)
    }
}
