use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::super::super::runtime_state::ExplicitBranchForkPacket;

use super::authority::{BranchAncestryState, BranchState};
use super::catalog::BranchManager;

impl<D, I, T> BranchManager<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime::state::branching) fn store_fork_packet(
        &mut self,
        packet: ExplicitBranchForkPacket<D, I, T>,
    ) -> Result<(), crate::data::error::SignalError> {
        let packet_branch_id = packet.branch_id();
        let state_branch_id = packet.state().branch_id();
        if packet_branch_id != state_branch_id {
            return Err(crate::data::error::SignalError::internal(format!(
                "fork packet branch mismatch: packet branch {} does not match state branch {}",
                packet_branch_id.0, state_branch_id.0
            )));
        }
        let expected_parent = packet
            .state()
            .ancestry()
            .parent_branch_id()
            .unwrap_or(packet.source_branch());
        if packet.source_branch() != expected_parent {
            return Err(crate::data::error::SignalError::internal(format!(
                "fork packet ancestry mismatch: source branch {} does not match stored parent {}",
                packet.source_branch().0,
                expected_parent.0
            )));
        }
        self.children_by_parent
            .entry(packet.source_branch())
            .or_default()
            .insert(packet.branch_id());
        self.branch_head_generations.insert(packet.branch_id(), 0);
        self.store_branch_state(packet.into_state());
        Ok(())
    }

    pub fn branch_children(&self, branch_id: SignalBranchId) -> Vec<SignalBranchId> {
        self.children_by_parent
            .get(&branch_id)
            .map(|children| children.iter().copied().collect())
            .unwrap_or_default()
    }

    pub fn is_merge_participant(&self, branch_id: SignalBranchId) -> bool {
        self.active_merge_participants.contains(&branch_id)
    }

    pub fn branch_head_generation(&self, branch_id: SignalBranchId) -> u64 {
        self.branch_head_generations
            .get(&branch_id)
            .copied()
            .unwrap_or(0)
    }

    pub(in crate::logic::transaction::runtime) fn advance_branch_head_generation(
        &mut self,
        branch_id: SignalBranchId,
    ) -> u64 {
        let generation = self.branch_head_generations.entry(branch_id).or_default();
        *generation = generation.saturating_add(1);
        *generation
    }

    pub fn mark_merge_participants(
        &mut self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) {
        self.active_merge_participants.insert(source_branch_id);
        self.active_merge_participants.insert(target_branch_id);
    }

    pub fn clear_merge_participants(
        &mut self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) {
        self.active_merge_participants.remove(&source_branch_id);
        self.active_merge_participants.remove(&target_branch_id);
    }

    pub fn branch_state(&self, branch_id: SignalBranchId) -> Option<&BranchState<D, I, T>> {
        self.branches.get(&branch_id)
    }

    pub fn branch_head_snapshot_id(&self, branch_id: SignalBranchId) -> Option<SignalSnapshotId> {
        self.live_branch_catalog
            .get(&branch_id)
            .and_then(|branch| branch.head_snapshot_id)
    }

    pub fn branch_handle(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.live_branch_catalog.get(&branch_id).cloned()
    }

    pub fn branch_ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        let mut ancestry = Vec::new();
        let mut cursor = self.live_branch_catalog.get(&branch_id);
        while let Some(branch) = cursor {
            ancestry.push(branch.clone());
            cursor = branch
                .parent_branch_id
                .and_then(|parent| self.live_branch_catalog.get(&parent));
        }
        ancestry
    }

    pub fn branch_ancestry_state(&self, branch_id: SignalBranchId) -> Option<&BranchAncestryState> {
        self.branches
            .get(&branch_id)
            .map(BranchState::ancestry)
            .or_else(|| self.branch_meta.get(&branch_id).map(|meta| &meta.ancestry))
    }
}
