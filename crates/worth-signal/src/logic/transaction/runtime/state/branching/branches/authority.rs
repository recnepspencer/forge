use crate::data::graph::SignalGraph;
use crate::data::node::CheckpointNodeImage;
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::temporal::{
    TemporalWakeOwner, TemporalWakeRetirementBatch, TemporalWakeRetirementReason,
};
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::transaction::runtime::config::SignalRuntimeConfig;
use crate::state::{SignalBranchId, SignalSnapshotId};

use super::super::super::merge::{BranchMergeKind, BranchMergeStrategy, BranchMutationLedger};
use super::super::super::reconstructability::{AuthorityState, DerivedState};
use super::super::super::temporal::TemporalRuntimeState;

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
    pub(super) derived: DerivedState<D, I>,
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

    pub(in crate::logic::transaction::runtime::state) fn resource(
        &self,
    ) -> &super::super::super::resource::ResourceRuntimeState {
        &self.derived.resource
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
            Some(&mut self.derived.telemetry.temporal),
        )
    }
}
