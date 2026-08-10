use crate::data::graph::SignalGraph;
use crate::state::{SignalBranchId, SignalSnapshotId};

use super::authority::BranchState;
use super::catalog::BranchManager;
use super::{SnapshotBranchState, SnapshotStatePacket};

impl<D, I, T> BranchManager<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn insert_snapshot(&mut self, packet: SnapshotStatePacket<D, I, T>) {
        let (branch_id, snapshot_id, state) = packet.into_parts();
        self.snapshots.insert((branch_id, snapshot_id), state);
    }

    pub fn snapshot_state(
        &self,
        branch_id: SignalBranchId,
        snapshot_id: SignalSnapshotId,
    ) -> Option<&SnapshotBranchState<D, I, T>> {
        self.snapshots.get(&(branch_id, snapshot_id))
    }

    pub fn replay_graph<'a>(
        &'a self,
        branch_id: SignalBranchId,
        active_branch: SignalBranchId,
        active_graph: &'a SignalGraph,
    ) -> Option<&'a SignalGraph> {
        if branch_id == active_branch {
            Some(active_graph)
        } else {
            self.branches.get(&branch_id).map(BranchState::graph)
        }
    }
}
