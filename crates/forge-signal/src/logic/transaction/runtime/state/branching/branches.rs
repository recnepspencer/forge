use std::collections::BTreeMap;

use crate::data::graph::SignalGraph;
use crate::data::node::CheckpointNodeImage;
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::temporal::{
    TemporalWakeOwner, TemporalWakeRetirementBatch, TemporalWakeRetirementReason,
};
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::transaction::runtime::config::SignalRuntimeConfig;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::super::merge::{BranchMergeKind, BranchMergeStrategy, BranchMutationLedger};
use super::super::reconstructability::{AuthorityState, DerivedState};
use super::super::runtime_state::{AuthorityTransferPacket, ExplicitBranchForkPacket};
use super::super::temporal::TemporalRuntimeState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) struct LatestMergeReference {
    source_branch_id: SignalBranchId,
    source_snapshot_id: Option<SignalSnapshotId>,
    target_snapshot_id_before: Option<SignalSnapshotId>,
    target_snapshot_id_after: Option<SignalSnapshotId>,
    merge_kind: BranchMergeKind,
    merge_strategy: BranchMergeStrategy,
}

impl LatestMergeReference {
    pub fn new(
        source_branch_id: SignalBranchId,
        source_snapshot_id: Option<SignalSnapshotId>,
        target_snapshot_id_before: Option<SignalSnapshotId>,
        target_snapshot_id_after: Option<SignalSnapshotId>,
        merge_kind: BranchMergeKind,
        merge_strategy: BranchMergeStrategy,
    ) -> Self {
        Self {
            source_branch_id,
            source_snapshot_id,
            target_snapshot_id_before,
            target_snapshot_id_after,
            merge_kind,
            merge_strategy,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) struct BranchAncestryState {
    branch_id: SignalBranchId,
    parent_branch_id: Option<SignalBranchId>,
    forked_from_snapshot_id: Option<SignalSnapshotId>,
    latest_merge_reference: Option<LatestMergeReference>,
}

impl BranchAncestryState {
    pub fn new(
        branch_id: SignalBranchId,
        parent_branch_id: Option<SignalBranchId>,
        forked_from_snapshot_id: Option<SignalSnapshotId>,
    ) -> Self {
        Self {
            branch_id,
            parent_branch_id,
            forked_from_snapshot_id,
            latest_merge_reference: None,
        }
    }

    pub fn set_latest_merge_reference(
        &mut self,
        latest_merge_reference: Option<LatestMergeReference>,
    ) {
        self.latest_merge_reference = latest_merge_reference;
    }

    pub fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub fn parent_branch_id(&self) -> Option<SignalBranchId> {
        self.parent_branch_id
    }

    pub fn forked_from_snapshot_id(&self) -> Option<SignalSnapshotId> {
        self.forked_from_snapshot_id
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct BranchState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    authority: AuthorityState<T>,
    derived: DerivedState<D, I>,
    ancestry: BranchAncestryState,
    mutation_ledger: BranchMutationLedger,
}

impl<D, I, T> BranchState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn new(
        authority: AuthorityState<T>,
        derived: DerivedState<D, I>,
        ancestry: BranchAncestryState,
        mutation_ledger: BranchMutationLedger,
    ) -> Self {
        Self {
            authority,
            derived,
            ancestry,
            mutation_ledger,
        }
    }

    pub fn graph(&self) -> &SignalGraph {
        &self.authority.graph
    }

    pub fn graph_mut(&mut self) -> &mut SignalGraph {
        &mut self.authority.graph
    }

    pub fn ancestry(&self) -> &BranchAncestryState {
        &self.ancestry
    }

    pub fn ancestry_mut(&mut self) -> &mut BranchAncestryState {
        &mut self.ancestry
    }

    pub fn mutation_ledger(&self) -> &BranchMutationLedger {
        &self.mutation_ledger
    }

    pub fn mutation_ledger_mut(&mut self) -> &mut BranchMutationLedger {
        &mut self.mutation_ledger
    }

    pub fn config(&self) -> &SignalRuntimeConfig<T> {
        &self.authority.config
    }

    pub fn checkpoint(&self) -> &CheckpointRuntime<D, I> {
        &self.derived.checkpoint
    }

    pub fn runtime_telemetry(&self) -> &RuntimeTelemetry {
        &self.derived.telemetry
    }

    pub fn temporal(&self) -> &TemporalRuntimeState {
        &self.derived.temporal
    }

    pub fn branch_id(&self) -> SignalBranchId {
        self.ancestry.branch_id()
    }

    pub fn into_parts(
        self,
    ) -> (
        AuthorityState<T>,
        DerivedState<D, I>,
        BranchAncestryState,
        BranchMutationLedger,
    ) {
        (
            self.authority,
            self.derived,
            self.ancestry,
            self.mutation_ledger,
        )
    }

    pub fn reset_mutation_ledger(&mut self, baseline_snapshot: Option<SignalSnapshotId>) {
        self.mutation_ledger =
            BranchMutationLedger::default().with_baseline_snapshot(baseline_snapshot);
    }

    #[cfg(test)]
    pub fn clear_merge_boundary_proof(&mut self) {
        self.mutation_ledger = BranchMutationLedger::default();
    }

    pub fn clear_branch_mutation_nodes(&mut self) {
        self.authority.graph.clear_branch_mutation_nodes();
    }

    pub fn replace_node_from_checkpoint_image(
        &mut self,
        node: crate::data::handle::NodeId,
        image: CheckpointNodeImage,
    ) -> Result<TemporalWakeRetirementBatch, crate::data::error::SignalError> {
        if !self.authority.graph.is_alive(node) {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot replace checkpoint image for non-live node owner {node}"
            )));
        }

        self.authority
            .graph
            .replace_entry_from_checkpoint_image(node, image)?;
        self.derived.temporal.retire_wakes_for_owner(
            TemporalWakeOwner::Node(node),
            TemporalWakeRetirementReason::Superseded,
            &mut self.derived.telemetry.temporal,
        )
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct SnapshotBranchState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    config: SignalRuntimeConfig<T>,
    derived: DerivedState<D, I>,
    ancestry: BranchAncestryState,
    mutation_ledger: BranchMutationLedger,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct SnapshotStatePacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    branch_id: SignalBranchId,
    snapshot_id: SignalSnapshotId,
    state: SnapshotBranchState<D, I, T>,
}

#[derive(Debug, Clone)]
struct BranchRuntimeMeta {
    ancestry: BranchAncestryState,
    mutation_ledger: BranchMutationLedger,
}

impl<D, I, T> SnapshotBranchState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn from_branch_state(state: &BranchState<D, I, T>) -> Self {
        Self {
            config: state.config().clone(),
            derived: state.derived.clone(),
            ancestry: state.ancestry().clone(),
            mutation_ledger: state.mutation_ledger().clone(),
        }
    }

    pub fn into_branch_state(
        self,
        graph: SignalGraph,
        runtime_telemetry: Option<RuntimeTelemetry>,
    ) -> BranchState<D, I, T> {
        BranchState::new(
            AuthorityState {
                graph,
                config: self.config,
            },
            DerivedState {
                checkpoint: self.derived.checkpoint,
                resource: self.derived.resource,
                temporal: self.derived.temporal,
                telemetry: runtime_telemetry.unwrap_or(self.derived.telemetry),
            },
            self.ancestry,
            self.mutation_ledger,
        )
    }

    pub fn packet(self, snapshot_id: SignalSnapshotId) -> SnapshotStatePacket<D, I, T> {
        SnapshotStatePacket {
            branch_id: self.ancestry.branch_id(),
            snapshot_id,
            state: self,
        }
    }
}

impl<D, I, T> SnapshotStatePacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn into_parts(
        self,
    ) -> (
        SignalBranchId,
        SignalSnapshotId,
        SnapshotBranchState<D, I, T>,
    ) {
        (self.branch_id, self.snapshot_id, self.state)
    }
}

pub(in crate::logic::transaction::runtime) struct BranchManager<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    branches: BTreeMap<SignalBranchId, BranchState<D, I, T>>,
    branch_meta: BTreeMap<SignalBranchId, BranchRuntimeMeta>,
    snapshots: BTreeMap<(SignalBranchId, SignalSnapshotId), SnapshotBranchState<D, I, T>>,
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
            branch_meta: BTreeMap::new(),
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
        BranchState::new(authority, derived, ancestry, mutation_ledger)
    }

    pub fn restore_active_state(
        &mut self,
        mut state: BranchState<D, I, T>,
        graph: &mut SignalGraph,
        config: &mut SignalRuntimeConfig<T>,
        checkpoint: &mut CheckpointRuntime<D, I>,
        resource: &mut super::super::resource::ResourceRuntimeState,
        temporal: &mut TemporalRuntimeState,
        telemetry: &mut RuntimeTelemetry,
        count_temporal_restore: bool,
    ) {
        self.observe_allocator_state(state.graph());
        state
            .graph_mut()
            .synchronize_node_allocator(self.next_node_index);
        state
            .graph_mut()
            .diagnostics_state_mut()
            .synchronize_branch_snapshot_allocator(self.next_snapshot_id, self.next_branch_id);
        state
            .graph_mut()
            .diagnostics_state_mut()
            .synchronize_lineage_allocator(
                self.next_lineage_artifact_id,
                self.next_lineage_sequence,
            );
        self.record_branch_meta(
            state.branch_id(),
            state.ancestry().clone(),
            state.mutation_ledger().clone(),
        );
        let branch_id = state.branch_id();
        let (authority, derived, _, _) = state.into_parts();
        *graph = authority.graph;
        *config = authority.config;
        *checkpoint = derived.checkpoint;
        *resource = derived.resource;
        *temporal = derived.temporal;
        let mut restored_telemetry = derived.telemetry;
        if count_temporal_restore {
            resource.bump_restore_epoch(branch_id, &mut restored_telemetry.resource);
            temporal.bump_previous_value_capability_epoch();
        }
        if count_temporal_restore {
            restored_telemetry
                .temporal
                .branch_local_temporal_restore_count += 1;
            restored_telemetry
                .temporal
                .branch_restore_temporal_rebuild_denial_count += 1;
        }
        *telemetry = restored_telemetry;
        self.observe_allocator_state(graph);
    }

    pub fn synchronize_catalogs(
        &mut self,
        branch_catalog: &BTreeMap<SignalBranchId, SignalBranchHandle>,
        active_branch: SignalBranchId,
        active_graph: &mut SignalGraph,
    ) {
        active_graph
            .diagnostics_state_mut()
            .synchronize_branch_catalog(branch_catalog, active_branch);
        for state in self.branches.values_mut() {
            let state_active_branch = state.graph().current_branch().id;
            state
                .graph_mut()
                .diagnostics_state_mut()
                .synchronize_branch_catalog(branch_catalog, state_active_branch);
        }
    }

    pub(in crate::logic::transaction::runtime::state::branching) fn store_branch_state(
        &mut self,
        state: BranchState<D, I, T>,
    ) {
        self.observe_allocator_state(state.graph());
        self.record_branch_meta(
            state.branch_id(),
            state.ancestry().clone(),
            state.mutation_ledger().clone(),
        );
        self.branches.insert(state.branch_id(), state);
    }

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
        self.store_branch_state(packet.into_state());
        Ok(())
    }

    pub fn branch_state(&self, branch_id: SignalBranchId) -> Option<&BranchState<D, I, T>> {
        self.branches.get(&branch_id)
    }

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
        let state = self.branches.get_mut(&branch_id)?;
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
        let _ = state;
        self.record_branch_meta(branch_id, ancestry, mutation_ledger);
        Some(result)
    }

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

    pub fn branch_head_snapshot_id(&self, branch_id: SignalBranchId) -> Option<SignalSnapshotId> {
        self.branches
            .get(&branch_id)
            .and_then(|state| state.graph().branch_head_snapshot_id(branch_id))
    }

    pub fn branch_handle(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.branches
            .get(&branch_id)
            .and_then(|state| state.graph().branch_handle(branch_id))
    }

    pub fn branch_ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        self.branches
            .get(&branch_id)
            .map(|state| state.graph().branch_ancestry(branch_id))
            .unwrap_or_default()
    }

    pub fn branch_ancestry_state(&self, branch_id: SignalBranchId) -> Option<&BranchAncestryState> {
        self.branches
            .get(&branch_id)
            .map(BranchState::ancestry)
            .or_else(|| self.branch_meta.get(&branch_id).map(|meta| &meta.ancestry))
    }

    pub fn branch_mutation_ledger(
        &self,
        branch_id: SignalBranchId,
    ) -> Option<&BranchMutationLedger> {
        self.branches
            .get(&branch_id)
            .map(BranchState::mutation_ledger)
            .or_else(|| {
                self.branch_meta
                    .get(&branch_id)
                    .map(|meta| &meta.mutation_ledger)
            })
    }

    pub(in crate::logic::transaction::runtime) fn branch_mutation_ledger_mut(
        &mut self,
        branch_id: SignalBranchId,
        baseline_snapshot_id: Option<crate::state::SignalSnapshotId>,
    ) -> &mut BranchMutationLedger {
        &mut self
            .branch_meta
            .entry(branch_id)
            .or_insert_with(|| BranchRuntimeMeta {
                ancestry: BranchAncestryState::new(branch_id, None, baseline_snapshot_id),
                mutation_ledger: BranchMutationLedger::default()
                    .with_baseline_snapshot(baseline_snapshot_id),
            })
            .mutation_ledger
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

    fn record_branch_meta(
        &mut self,
        branch_id: SignalBranchId,
        ancestry: BranchAncestryState,
        mutation_ledger: BranchMutationLedger,
    ) {
        self.branch_meta.insert(
            branch_id,
            BranchRuntimeMeta {
                ancestry,
                mutation_ledger,
            },
        );
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
