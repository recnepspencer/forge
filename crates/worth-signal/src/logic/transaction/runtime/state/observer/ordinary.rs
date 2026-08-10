use crate::data::graph::{EvaluationStrategy, GraphObserver};
use crate::data::handle::NodeId;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::super::runtime_observation::{MatchingObserverSet, ObservationRegistrySummary};
use super::RuntimeObserver;

impl<'a, D, I, E, Ctx, T> RuntimeObserver<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn graph(&self) -> GraphObserver<'a> {
        self.runtime.graph.observe()
    }

    pub fn runtime_policy(&self) -> crate::diagnostics::policy::SignalRuntimePolicy {
        self.graph().runtime_policy()
    }

    pub fn evaluation_strategy(&self) -> EvaluationStrategy {
        self.graph().evaluation_strategy()
    }

    pub fn observation_summary(&self) -> ObservationRegistrySummary {
        self.runtime.observations.summary()
    }

    pub fn matching_observers_for_node(&self, node: NodeId) -> MatchingObserverSet {
        self.runtime.observations.matching_observers_for_node(node)
    }

    pub fn current_branch(&self) -> SignalBranchHandle {
        self.graph().current_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.graph().known_branches()
    }

    pub fn branch_handle(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.graph()
            .branch_handle(branch_id)
            .or_else(|| self.runtime.branches.branch_handle(branch_id))
    }

    pub fn branch_ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        if self.graph().branch_handle(branch_id).is_some() {
            self.graph().branch_ancestry(branch_id)
        } else {
            self.runtime.branches.branch_ancestry(branch_id)
        }
    }

    pub fn branch_head_snapshot_id(&self, branch_id: SignalBranchId) -> Option<SignalSnapshotId> {
        self.graph()
            .branch_head_snapshot_id(branch_id)
            .or_else(|| self.runtime.branches.branch_head_snapshot_id(branch_id))
    }
}
