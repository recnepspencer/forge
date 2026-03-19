use std::collections::BTreeMap;

use crate::data::graph::SignalGraph;
use crate::data::telemetry::RuntimeTelemetry;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::transaction::runtime::config::SignalRuntimeConfig;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::super::merge::{BranchMergeKind, BranchMergeStrategy, BranchMutationLedger};
use super::super::reconstructability::{AuthorityState, DerivedState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) struct LatestMergeReference {
    pub source_branch_id: SignalBranchId,
    pub source_snapshot_id: Option<SignalSnapshotId>,
    pub target_snapshot_id_before: Option<SignalSnapshotId>,
    pub target_snapshot_id_after: Option<SignalSnapshotId>,
    pub merge_kind: BranchMergeKind,
    pub merge_strategy: BranchMergeStrategy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) struct BranchAncestryState {
    pub branch_id: SignalBranchId,
    pub parent_branch_id: Option<SignalBranchId>,
    pub forked_from_snapshot_id: Option<SignalSnapshotId>,
    pub latest_merge_reference: Option<LatestMergeReference>,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct BranchState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub authority: AuthorityState<T>,
    pub derived: DerivedState<D, I>,
    pub ancestry: BranchAncestryState,
    pub mutation_ledger: BranchMutationLedger,
}

pub(in crate::logic::transaction::runtime) struct BranchManager<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    branches: BTreeMap<SignalBranchId, BranchState<D, I, T>>,
    snapshots: BTreeMap<SignalSnapshotId, BranchState<D, I, T>>,
    next_node_index: u32,
    next_snapshot_id: u64,
    next_branch_id: u64,
    next_lineage_artifact_id: u64,
    next_lineage_sequence: u64,
}

impl<D, I, T> BranchManager<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn new() -> Self {
        Self {
            branches: BTreeMap::new(),
            snapshots: BTreeMap::new(),
            next_node_index: 0,
            next_snapshot_id: 0,
            next_branch_id: 1,
            next_lineage_artifact_id: 0,
            next_lineage_sequence: 0,
        }
    }

    pub fn capture_active_state(
        &mut self,
        authority: AuthorityState<T>,
        derived: DerivedState<D, I>,
        ancestry: BranchAncestryState,
        mutation_ledger: BranchMutationLedger,
    ) -> BranchState<D, I, T> {
        self.observe_allocator_state(&authority.graph);
        BranchState {
            authority,
            derived,
            ancestry,
            mutation_ledger,
        }
    }

    pub fn restore_active_state(
        &mut self,
        mut state: BranchState<D, I, T>,
        graph: &mut SignalGraph,
        config: &mut SignalRuntimeConfig<T>,
        checkpoint: &mut CheckpointRuntime<D, I>,
        telemetry: &mut RuntimeTelemetry,
    ) {
        self.observe_allocator_state(&state.authority.graph);
        state
            .authority
            .graph
            .synchronize_node_allocator(self.next_node_index);
        state
            .authority
            .graph
            .diagnostics_state_mut()
            .synchronize_branch_snapshot_allocator(self.next_snapshot_id, self.next_branch_id);
        state
            .authority
            .graph
            .diagnostics_state_mut()
            .synchronize_lineage_allocator(
                self.next_lineage_artifact_id,
                self.next_lineage_sequence,
            );
        *graph = state.authority.graph;
        *config = state.authority.config;
        *checkpoint = state.derived.checkpoint;
        *telemetry = state.derived.telemetry;
        self.observe_allocator_state(graph);
    }

    pub fn synchronize_catalogs(
        &mut self,
        branch_catalog: BTreeMap<SignalBranchId, SignalBranchHandle>,
        active_branch: SignalBranchId,
        active_graph: &mut SignalGraph,
    ) {
        active_graph
            .diagnostics_state_mut()
            .synchronize_branch_catalog(branch_catalog.clone(), active_branch);
        for state in self.branches.values_mut() {
            let state_active_branch = state.authority.graph.current_branch().id;
            state
                .authority
                .graph
                .diagnostics_state_mut()
                .synchronize_branch_catalog(branch_catalog.clone(), state_active_branch);
        }
    }

    pub fn insert_branch(&mut self, branch_id: SignalBranchId, state: BranchState<D, I, T>) {
        self.observe_allocator_state(&state.authority.graph);
        self.branches.insert(branch_id, state);
    }

    pub fn branch_state(&self, branch_id: SignalBranchId) -> Option<&BranchState<D, I, T>> {
        self.branches.get(&branch_id)
    }

    pub fn branch_state_mut_with_allocator_sync(
        &mut self,
        branch_id: SignalBranchId,
    ) -> Option<&mut BranchState<D, I, T>> {
        let next_node_index = self.next_node_index;
        let next_snapshot_id = self.next_snapshot_id;
        let next_branch_id = self.next_branch_id;
        let next_lineage_artifact_id = self.next_lineage_artifact_id;
        let next_lineage_sequence = self.next_lineage_sequence;
        let state = self.branches.get_mut(&branch_id)?;
        state
            .authority
            .graph
            .synchronize_node_allocator(next_node_index);
        state
            .authority
            .graph
            .diagnostics_state_mut()
            .synchronize_branch_snapshot_allocator(next_snapshot_id, next_branch_id);
        state
            .authority
            .graph
            .diagnostics_state_mut()
            .synchronize_lineage_allocator(next_lineage_artifact_id, next_lineage_sequence);
        Some(state)
    }

    pub fn cloned_branch_state(&self, branch_id: SignalBranchId) -> Option<BranchState<D, I, T>> {
        let mut state = self.branches.get(&branch_id).cloned()?;
        state
            .authority
            .graph
            .synchronize_node_allocator(self.next_node_index);
        state
            .authority
            .graph
            .diagnostics_state_mut()
            .synchronize_branch_snapshot_allocator(self.next_snapshot_id, self.next_branch_id);
        state
            .authority
            .graph
            .diagnostics_state_mut()
            .synchronize_lineage_allocator(
                self.next_lineage_artifact_id,
                self.next_lineage_sequence,
            );
        Some(state)
    }

    pub fn insert_snapshot(&mut self, snapshot_id: SignalSnapshotId, state: BranchState<D, I, T>) {
        self.observe_allocator_state(&state.authority.graph);
        self.snapshots.insert(snapshot_id, state);
    }

    pub fn snapshot_state(&self, snapshot_id: SignalSnapshotId) -> Option<&BranchState<D, I, T>> {
        self.snapshots.get(&snapshot_id)
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
            self.branches.get(&branch_id).map(|state| &state.authority.graph)
        }
    }

    pub fn branch_head_snapshot_id(&self, branch_id: SignalBranchId) -> Option<SignalSnapshotId> {
        self.branches
            .get(&branch_id)
            .and_then(|state| state.authority.graph.branch_head_snapshot_id(branch_id))
    }

    pub fn branch_handle(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.branches
            .get(&branch_id)
            .and_then(|state| state.authority.graph.branch_handle(branch_id))
    }

    pub fn branch_ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        self.branches
            .get(&branch_id)
            .map(|state| state.authority.graph.branch_ancestry(branch_id))
            .unwrap_or_default()
    }

    pub fn branch_ancestry_state(
        &self,
        branch_id: SignalBranchId,
    ) -> Option<&BranchAncestryState> {
        self.branches.get(&branch_id).map(|state| &state.ancestry)
    }

    fn observe_allocator_state(&mut self, graph: &SignalGraph) {
        self.next_node_index = self.next_node_index.max(graph.node_allocator_state());
        let (next_snapshot_id, next_branch_id) =
            graph.diagnostics_state().branch_snapshot_allocator_state();
        let (next_lineage_artifact_id, next_lineage_sequence) =
            graph.diagnostics_state().lineage_allocator_state();
        self.next_snapshot_id = self.next_snapshot_id.max(next_snapshot_id);
        self.next_branch_id = self.next_branch_id.max(next_branch_id);
        self.next_lineage_artifact_id = self.next_lineage_artifact_id.max(next_lineage_artifact_id);
        self.next_lineage_sequence = self.next_lineage_sequence.max(next_lineage_sequence);
    }
}

impl<D, I, T> Default for BranchManager<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}
