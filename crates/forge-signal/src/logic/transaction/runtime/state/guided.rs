use std::collections::VecDeque;

use crate::data::aspect::Aspect;
use crate::data::error::SignalError;
use crate::diagnostics::{LineageEvent, ReplayView, SynthesizedLineageChain};
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotV1};

use super::merge::{
    AspectMergePolicyBinding, AspectMergePolicyName, BranchMergeResult, BranchMergeStrategy,
    ConflictIsolationPolicyName, ConflictPolicyName, DeletionPolicyName, IdentityMatcherName,
    MergeBaseStrategyName, MergeStrategyName, SourceOnlyPolicyName,
};
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
    strategy_name: Option<MergeStrategyName>,
    strategy_hint: Option<BranchMergeStrategy>,
    merge_base_name: Option<MergeBaseStrategyName>,
    conflict_policy_name: Option<ConflictPolicyName>,
    conflict_isolation_policy_name: Option<ConflictIsolationPolicyName>,
    identity_matcher_name: Option<IdentityMatcherName>,
    source_only_policy_name: Option<SourceOnlyPolicyName>,
    deletion_policy_name: Option<DeletionPolicyName>,
    aspect_policy_bindings: Vec<AspectMergePolicyBinding>,
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
            strategy_name: None,
            strategy_hint: None,
            merge_base_name: None,
            conflict_policy_name: None,
            identity_matcher_name: None,
            source_only_policy_name: None,
            conflict_isolation_policy_name: None,
            deletion_policy_name: None,
            aspect_policy_bindings: Vec::new(),
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

    pub fn strategy_hint(mut self, strategy: BranchMergeStrategy) -> Self {
        self.strategy_hint = Some(strategy);
        self
    }

    pub fn strategy_named(mut self, strategy_name: impl Into<String>) -> Self {
        self.strategy_name = Some(MergeStrategyName::new(strategy_name));
        self
    }

    pub fn conflict_policy_named(mut self, policy_name: impl Into<String>) -> Self {
        self.conflict_policy_name = Some(ConflictPolicyName::new(policy_name));
        self
    }

    pub fn conflict_isolation_policy_named(mut self, policy_name: impl Into<String>) -> Self {
        self.conflict_isolation_policy_name = Some(ConflictIsolationPolicyName::new(policy_name));
        self
    }

    pub fn merge_base_named(mut self, strategy_name: impl Into<String>) -> Self {
        self.merge_base_name = Some(MergeBaseStrategyName::new(strategy_name));
        self
    }

    pub fn identity_matcher_named(mut self, matcher_name: impl Into<String>) -> Self {
        self.identity_matcher_name = Some(IdentityMatcherName::new(matcher_name));
        self
    }

    pub fn source_only_policy_named(mut self, policy_name: impl Into<String>) -> Self {
        self.source_only_policy_name = Some(SourceOnlyPolicyName::new(policy_name));
        self
    }

    pub fn deletion_policy_named(mut self, policy_name: impl Into<String>) -> Self {
        self.deletion_policy_name = Some(DeletionPolicyName::new(policy_name));
        self
    }

    pub fn aspect_policy_named(mut self, aspect: Aspect, policy_name: impl Into<String>) -> Self {
        self.aspect_policy_bindings
            .push(AspectMergePolicyBinding::new(
                aspect,
                AspectMergePolicyName::new(policy_name),
            ));
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
            strategy_name: self.strategy_name,
            strategy_hint: self.strategy_hint,
            merge_base_name: self.merge_base_name,
            conflict_policy_name: self.conflict_policy_name,
            identity_matcher_name: self.identity_matcher_name,
            source_only_policy_name: self.source_only_policy_name,
            deletion_policy_name: self.deletion_policy_name,
            conflict_isolation_policy_name: self.conflict_isolation_policy_name,
            aspect_policy_bindings: self.aspect_policy_bindings,
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
