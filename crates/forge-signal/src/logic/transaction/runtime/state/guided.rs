use std::collections::VecDeque;

use crate::data::error::SignalError;
use crate::diagnostics::{LineageEvent, ReplayView, SynthesizedLineageChain};
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotV1};

use super::merge::{BranchMergeResult, BranchMergeStrategy};
use super::runtime_state::SignalRuntime;

pub struct PlannedRuntimeMerge<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>,
    request: crate::logic::transaction::runtime::BranchMergeRequest,
    plan: crate::logic::transaction::runtime::BranchMergePlan,
}

impl<'a, D, I, E, Ctx, T> PlannedRuntimeMerge<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn plan(&self) -> &crate::logic::transaction::runtime::BranchMergePlan {
        &self.plan
    }

    pub fn execute(self) -> Result<BranchMergeResult, SignalError> {
        self.runtime
            .execute_branch_merge_request_plan(&self.request, &self.plan)
    }
}

pub struct RuntimeHistory<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>,
}

impl<'a, D, I, E, Ctx, T> RuntimeHistory<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>) -> Self {
        Self { runtime }
    }

    pub fn current_branch(&self) -> SignalBranchHandle {
        self.runtime.current_branch()
    }

    pub fn branches(&self) -> Vec<SignalBranchHandle> {
        self.runtime.known_branches()
    }

    pub fn branch(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.runtime.branch_handle(branch_id)
    }

    pub fn ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        self.runtime.branch_ancestry(branch_id)
    }

    pub fn snapshot(&mut self) -> SignalSnapshotV1 {
        self.runtime.capture_snapshot()
    }

    pub fn branch_snapshot(
        &mut self,
        branch: SignalBranchHandle,
    ) -> Result<SignalSnapshotV1, SignalError> {
        self.runtime.capture_branch_snapshot(branch)
    }

    pub fn replay_for_branch(&self, branch_id: SignalBranchId) -> ReplayView {
        self.runtime.replay_for_branch(branch_id)
    }

    pub fn replay_for_node(&self, node: crate::data::handle::NodeId) -> ReplayView {
        self.runtime.observe().replay_for_node(node)
    }

    pub fn lineage_for_node(&self, node: crate::data::handle::NodeId) -> SynthesizedLineageChain {
        self.runtime.observe().lineage_chain_for_node(node)
    }

    pub fn latest_lineage(&self) -> &VecDeque<LineageEvent> {
        self.runtime.graph().observe().lineage_records()
    }
}

pub struct RuntimeMerge<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>,
    source: Option<SignalBranchHandle>,
    target: Option<SignalBranchHandle>,
}

impl<'a, D, I, E, Ctx, T> RuntimeMerge<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>) -> Self {
        Self {
            runtime,
            source: None,
            target: None,
        }
    }

    pub fn from(mut self, branch: SignalBranchHandle) -> Self {
        self.source = Some(branch);
        self
    }

    pub fn into_branch(mut self, branch: SignalBranchHandle) -> Self {
        self.target = Some(branch);
        self
    }

    pub fn into(self, branch: SignalBranchHandle) -> Self {
        self.into_branch(branch)
    }

    pub fn strategy_hint(self, _strategy: BranchMergeStrategy) -> Self {
        self
    }

    pub fn plan(self) -> Result<PlannedRuntimeMerge<'a, D, I, E, Ctx, T>, SignalError> {
        let source = self
            .source
            .ok_or_else(|| SignalError::invalid_input("merge source branch is required"))?;
        let target = self
            .target
            .ok_or_else(|| SignalError::invalid_input("merge target branch is required"))?;
        let request = crate::logic::transaction::runtime::BranchMergeRequest {
            source_branch: source,
            target_branch: target,
        };
        let plan = self.runtime.plan_branch_merge_request(&request)?;
        Ok(PlannedRuntimeMerge {
            runtime: self.runtime,
            request,
            plan,
        })
    }

    pub fn run(self) -> Result<BranchMergeResult, SignalError> {
        self.plan()?.execute()
    }

    pub fn execute(self) -> Result<BranchMergeResult, SignalError> {
        self.run()
    }
}
