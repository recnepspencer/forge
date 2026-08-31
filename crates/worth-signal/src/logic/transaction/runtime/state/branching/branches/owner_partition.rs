use std::collections::{BTreeMap, BTreeSet};

use crate::branch::SignalBranchRetentionRegistry;
use crate::branch::SignalBranchRetirementReceipt;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::authority::BranchState;
use super::catalog::BranchManager;
use super::{SnapshotBranchState, SnapshotStatePacket};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) enum SignalOwnerPartitionDenial {
    ActiveBranchMissing,
    StoredBranchMissing(SignalBranchId),
    UnexpectedStoredBranch(SignalBranchId),
    MissingBranchHead(SignalBranchId),
    UnexpectedBranchHead(SignalBranchId),
    LiveBranchCapacityExhausted { maximum_live_branches: usize },
}

/// Metadata retained after all live per-branch truth moves into owner cells.
pub(crate) struct SignalOwnerMetadataState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    snapshots: BTreeMap<(SignalBranchId, SignalSnapshotId), SnapshotBranchState<D, I, T>>,
    maximum_stored_snapshots: usize,
    children_by_parent: BTreeMap<SignalBranchId, BTreeSet<SignalBranchId>>,
    retirement_receipts: BTreeMap<SignalBranchId, SignalBranchRetirementReceipt>,
    active_merge_participants: BTreeSet<SignalBranchId>,
    reserved_snapshot_count: usize,
}

type SignalOwnerCellSeed<D, I, T> = (
    SignalBranchHandle,
    BranchState<D, I, T>,
    u64,
    Option<SignalSnapshotId>,
);

/// Complete, validated transfer packet consumed exactly once by the owner root.
pub(crate) struct SignalOwnerPartition<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    metadata: SignalOwnerMetadataState<D, I, T>,
    next_branch_id: u64,
    retention: SignalBranchRetentionRegistry,
    cells: Vec<SignalOwnerCellSeed<D, I, T>>,
}

impl<D, I, T> SignalOwnerMetadataState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn from_manager(branches: &mut BranchManager<D, I, T>) -> Self {
        Self {
            snapshots: std::mem::take(&mut branches.snapshots),
            maximum_stored_snapshots: branches.maximum_stored_snapshots,
            children_by_parent: std::mem::take(&mut branches.children_by_parent),
            retirement_receipts: std::mem::take(&mut branches.retirement_receipts),
            active_merge_participants: std::mem::take(&mut branches.active_merge_participants),
            reserved_snapshot_count: 0,
        }
    }

    pub(crate) fn reserve_snapshot_capacity(&mut self) -> Result<(), usize> {
        if self.snapshots.len() + self.reserved_snapshot_count >= self.maximum_stored_snapshots {
            return Err(self.maximum_stored_snapshots);
        }
        self.reserved_snapshot_count += 1;
        Ok(())
    }

    pub(crate) fn release_snapshot_capacity(&mut self) {
        self.reserved_snapshot_count = self
            .reserved_snapshot_count
            .checked_sub(1)
            .expect("owner snapshot reservation releases exactly once");
    }

    pub(crate) fn install_reserved_snapshot(&mut self, packet: SnapshotStatePacket<D, I, T>) {
        self.release_snapshot_capacity();
        let (branch_id, snapshot_id, state) = packet.into_parts();
        self.snapshots.insert((branch_id, snapshot_id), state);
    }

    pub(crate) fn snapshot_state(
        &self,
        branch_id: SignalBranchId,
        snapshot_id: SignalSnapshotId,
    ) -> Option<SnapshotBranchState<D, I, T>> {
        self.snapshots.get(&(branch_id, snapshot_id)).cloned()
    }

    pub(crate) fn record_fork_child(
        &mut self,
        parent_branch_id: SignalBranchId,
        child_branch_id: SignalBranchId,
    ) {
        self.children_by_parent
            .entry(parent_branch_id)
            .or_default()
            .insert(child_branch_id);
    }

    pub(crate) fn remove_fork_child(
        &mut self,
        parent_branch_id: SignalBranchId,
        child_branch_id: SignalBranchId,
    ) {
        let remove_parent = self
            .children_by_parent
            .get_mut(&parent_branch_id)
            .is_some_and(|children| {
                children.remove(&child_branch_id);
                children.is_empty()
            });
        if remove_parent {
            self.children_by_parent.remove(&parent_branch_id);
        }
    }

    pub(crate) fn branch_children(&self, branch_id: SignalBranchId) -> Vec<SignalBranchId> {
        self.children_by_parent
            .get(&branch_id)
            .map(|children| children.iter().copied().collect())
            .unwrap_or_default()
    }

    pub(crate) fn is_merge_participant(&self, branch_id: SignalBranchId) -> bool {
        self.active_merge_participants.contains(&branch_id)
    }

    pub(crate) fn remove_retired_branch(
        &mut self,
        branch_id: SignalBranchId,
        parent_branch_id: Option<SignalBranchId>,
    ) -> u32 {
        let snapshot_keys = self
            .snapshots
            .range((branch_id, SignalSnapshotId(0))..=(branch_id, SignalSnapshotId(u64::MAX)))
            .map(|(key, _)| *key)
            .collect::<Vec<_>>();
        for key in &snapshot_keys {
            self.snapshots.remove(key);
        }
        self.children_by_parent.remove(&branch_id);
        if let Some(parent_branch_id) = parent_branch_id {
            let remove_parent = self
                .children_by_parent
                .get_mut(&parent_branch_id)
                .is_some_and(|children| {
                    children.remove(&branch_id);
                    children.is_empty()
                });
            if remove_parent {
                self.children_by_parent.remove(&parent_branch_id);
            }
        }
        self.active_merge_participants.remove(&branch_id);
        snapshot_keys.len() as u32
    }

    pub(crate) fn retain_retirement_receipt(&mut self, receipt: SignalBranchRetirementReceipt) {
        self.retirement_receipts
            .insert(receipt.retired_branch().id, receipt);
    }

    pub(crate) fn branch_retirement_receipt(
        &self,
        branch_id: SignalBranchId,
    ) -> Option<SignalBranchRetirementReceipt> {
        self.retirement_receipts.get(&branch_id).cloned()
    }
}

impl<D, I, T> SignalOwnerPartition<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn new(
        metadata: SignalOwnerMetadataState<D, I, T>,
        next_branch_id: u64,
        retention: SignalBranchRetentionRegistry,
        cells: Vec<SignalOwnerCellSeed<D, I, T>>,
    ) -> Self {
        Self {
            metadata,
            next_branch_id,
            retention,
            cells,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SignalOwnerMetadataState<D, I, T>,
        u64,
        SignalBranchRetentionRegistry,
        Vec<SignalOwnerCellSeed<D, I, T>>,
    ) {
        (
            self.metadata,
            self.next_branch_id,
            self.retention,
            self.cells,
        )
    }
}

impl<D, I, T> BranchManager<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) fn validate_owner_partition(
        &self,
        active_branch_id: SignalBranchId,
        maximum_live_branches: usize,
    ) -> Result<(), SignalOwnerPartitionDenial> {
        if self.live_branch_catalog.len() > maximum_live_branches {
            return Err(SignalOwnerPartitionDenial::LiveBranchCapacityExhausted {
                maximum_live_branches,
            });
        }
        if !self.live_branch_catalog.contains_key(&active_branch_id) {
            return Err(SignalOwnerPartitionDenial::ActiveBranchMissing);
        }
        for branch_id in self.live_branch_catalog.keys().copied() {
            if branch_id != active_branch_id && !self.branches.contains_key(&branch_id) {
                return Err(SignalOwnerPartitionDenial::StoredBranchMissing(branch_id));
            }
        }
        for branch_id in self.branches.keys().copied() {
            if branch_id == active_branch_id || !self.live_branch_catalog.contains_key(&branch_id) {
                return Err(SignalOwnerPartitionDenial::UnexpectedStoredBranch(
                    branch_id,
                ));
            }
        }
        for branch_id in self.live_branch_catalog.keys().copied() {
            if !self.branch_head_generations.contains_key(&branch_id)
                || !self.branch_restore_snapshot_ids.contains_key(&branch_id)
            {
                return Err(SignalOwnerPartitionDenial::MissingBranchHead(branch_id));
            }
        }
        if let Some(branch_id) = self
            .branch_head_generations
            .keys()
            .chain(self.branch_restore_snapshot_ids.keys())
            .copied()
            .find(|branch_id| !self.live_branch_catalog.contains_key(branch_id))
        {
            return Err(SignalOwnerPartitionDenial::UnexpectedBranchHead(branch_id));
        }
        Ok(())
    }

    pub(in crate::logic::transaction::runtime) fn into_owner_partition(
        mut self,
        active_state: BranchState<D, I, T>,
    ) -> SignalOwnerPartition<D, I, T> {
        let active_branch_id = active_state.branch_id();
        let mut active_state = Some(active_state);
        let live_catalog = std::mem::take(&mut self.live_branch_catalog);
        let mut cells = Vec::with_capacity(live_catalog.len());
        let next_node_index = self.next_node_index;
        let next_snapshot_id = self.next_snapshot_id;
        let next_branch_id = self.next_branch_id;
        let next_lineage_artifact_id = self.next_lineage_artifact_id;
        let next_lineage_sequence = self.next_lineage_sequence;
        for (branch_id, handle) in live_catalog {
            let mut state = if branch_id == active_branch_id {
                active_state
                    .take()
                    .expect("the validated active branch appears once")
            } else {
                self.branches
                    .remove(&branch_id)
                    .expect("validated stored branch state remains present")
            };
            let head_generation = self
                .branch_head_generations
                .remove(&branch_id)
                .expect("validated live branch retains its generation");
            let restore_snapshot_id = self
                .branch_restore_snapshot_ids
                .remove(&branch_id)
                .expect("validated live branch retains its restore posture");
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
            cells.push((handle, state, head_generation, restore_snapshot_id));
        }
        debug_assert!(active_state.is_none());
        debug_assert!(self.branches.is_empty());
        self.branch_meta.clear();
        let next_branch_id = self.next_branch_id.max(1);
        self.next_branch_id = 0;
        let retention = self
            .retention
            .take()
            .expect("an unsealed branch manager owns one retention registry");
        let metadata = SignalOwnerMetadataState::from_manager(&mut self);
        debug_assert!(self.owner_partition_membership_is_drained());
        SignalOwnerPartition::new(metadata, next_branch_id, retention, cells)
    }

    fn owner_partition_membership_is_drained(&self) -> bool {
        self.branches.is_empty()
            && self.live_branch_catalog.is_empty()
            && self.branch_meta.is_empty()
            && self.branch_head_generations.is_empty()
            && self.branch_restore_snapshot_ids.is_empty()
            && self.snapshots.is_empty()
            && self.children_by_parent.is_empty()
            && self.retirement_receipts.is_empty()
            && self.active_merge_participants.is_empty()
            && self.retention.is_none()
    }
}
