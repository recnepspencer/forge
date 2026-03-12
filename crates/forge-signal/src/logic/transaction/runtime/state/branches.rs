use std::collections::BTreeMap;

use crate::data::graph::SignalGraph;
use crate::data::telemetry::RuntimeTelemetry;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::transaction::runtime::config::SignalRuntimeConfig;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct BranchState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub graph: SignalGraph,
    pub config: SignalRuntimeConfig<T>,
    pub checkpoint: CheckpointRuntime<D, I>,
    pub telemetry: RuntimeTelemetry,
}

pub(in crate::logic::transaction::runtime) struct BranchManager<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    branches: BTreeMap<SignalBranchId, BranchState<D, I, T>>,
    snapshots: BTreeMap<SignalSnapshotId, BranchState<D, I, T>>,
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
        }
    }

    pub fn capture_active_state(
        &self,
        graph: &SignalGraph,
        config: &SignalRuntimeConfig<T>,
        checkpoint: &CheckpointRuntime<D, I>,
        telemetry: &RuntimeTelemetry,
    ) -> BranchState<D, I, T> {
        BranchState {
            graph: graph.clone_stateful(),
            config: config.clone(),
            checkpoint: checkpoint.clone(),
            telemetry: telemetry.clone(),
        }
    }

    pub fn restore_active_state(
        &self,
        state: BranchState<D, I, T>,
        graph: &mut SignalGraph,
        config: &mut SignalRuntimeConfig<T>,
        checkpoint: &mut CheckpointRuntime<D, I>,
        telemetry: &mut RuntimeTelemetry,
    ) {
        *graph = state.graph;
        *config = state.config;
        *checkpoint = state.checkpoint;
        *telemetry = state.telemetry;
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
            let state_active_branch = state.graph.current_branch().id;
            state
                .graph
                .diagnostics_state_mut()
                .synchronize_branch_catalog(branch_catalog.clone(), state_active_branch);
        }
    }

    pub fn insert_branch(&mut self, branch_id: SignalBranchId, state: BranchState<D, I, T>) {
        self.branches.insert(branch_id, state);
    }

    pub fn branch_state(&self, branch_id: SignalBranchId) -> Option<&BranchState<D, I, T>> {
        self.branches.get(&branch_id)
    }

    pub fn branch_state_mut(
        &mut self,
        branch_id: SignalBranchId,
    ) -> Option<&mut BranchState<D, I, T>> {
        self.branches.get_mut(&branch_id)
    }

    pub fn cloned_branch_state(
        &self,
        branch_id: SignalBranchId,
    ) -> Option<BranchState<D, I, T>> {
        self.branches.get(&branch_id).cloned()
    }

    pub fn insert_snapshot(&mut self, snapshot_id: SignalSnapshotId, state: BranchState<D, I, T>) {
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
            self.branches.get(&branch_id).map(|state| &state.graph)
        }
    }

    pub fn branch_head_snapshot_id(&self, branch_id: SignalBranchId) -> Option<SignalSnapshotId> {
        self.branches
            .get(&branch_id)
            .and_then(|state| state.graph.branch_head_snapshot_id(branch_id))
    }

    pub fn branch_handle(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.branches
            .get(&branch_id)
            .and_then(|state| state.graph.branch_handle(branch_id))
    }

    pub fn branch_ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        self.branches
            .get(&branch_id)
            .map(|state| state.graph.branch_ancestry(branch_id))
            .unwrap_or_default()
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
