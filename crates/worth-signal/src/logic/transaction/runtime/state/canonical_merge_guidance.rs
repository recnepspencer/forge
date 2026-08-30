use crate::branch::{AdmittedSignalBranchBasis, SignalBranchMergeOutcome};
use crate::data::aspect::Aspect;
use crate::data::error::SignalError;

use super::guided::{RawPlannedRuntimeMerge, RawRuntimeMerge};
use super::merge::{
    AspectMergePolicyBinding, AspectMergePolicyName, BranchMergePlan, BranchMergeRequest,
    BranchMergeRequestScope, BranchMergeStrategy, ConflictIsolationPolicyName, ConflictPolicyName,
    DeletionPolicyName, IdentityMatcherName, LoweredFoundationalMergeRequest,
    MergeBaseStrategyName, MergeStrategyName, NormalizedBranchMergeRequest,
    SignalSelectedAspectRequestEntry, SourceOnlyPolicyName,
};
use super::runtime_state::SignalRuntime;

pub struct PlannedRuntimeMerge<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    raw: RawPlannedRuntimeMerge<'a, D, I, E, Ctx, T>,
    source: AdmittedSignalBranchBasis,
    target: AdmittedSignalBranchBasis,
}

impl<'a, D, I, E, Ctx, T> PlannedRuntimeMerge<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn plan(&self) -> &BranchMergePlan {
        self.raw.plan()
    }

    pub fn request(&self) -> &BranchMergeRequest {
        self.raw.request()
    }

    pub fn normalized_request(&self) -> &NormalizedBranchMergeRequest {
        self.raw.normalized_request()
    }

    pub fn lowered_request(&self) -> &LoweredFoundationalMergeRequest {
        self.raw.lowered_request()
    }

    pub fn execute(self) -> Result<SignalBranchMergeOutcome, SignalError> {
        self.raw.execute_admitted(&self.source, &self.target)
    }
}

pub struct RuntimeMerge<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>,
    source: Option<AdmittedSignalBranchBasis>,
    target: Option<AdmittedSignalBranchBasis>,
    strategy_name: Option<MergeStrategyName>,
    strategy_hint: Option<BranchMergeStrategy>,
    merge_base_name: Option<MergeBaseStrategyName>,
    conflict_policy_name: Option<ConflictPolicyName>,
    conflict_isolation_policy_name: Option<ConflictIsolationPolicyName>,
    identity_matcher_name: Option<IdentityMatcherName>,
    source_only_policy_name: Option<SourceOnlyPolicyName>,
    deletion_policy_name: Option<DeletionPolicyName>,
    aspect_policy_bindings: Vec<AspectMergePolicyBinding>,
    scope: Option<BranchMergeRequestScope>,
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
            conflict_isolation_policy_name: None,
            identity_matcher_name: None,
            source_only_policy_name: None,
            deletion_policy_name: None,
            aspect_policy_bindings: Vec::new(),
            scope: None,
        }
    }

    pub fn from(mut self, basis: &AdmittedSignalBranchBasis) -> Self {
        self.source = Some(basis.clone());
        self
    }

    pub fn into_branch(mut self, basis: &AdmittedSignalBranchBasis) -> Self {
        self.target = Some(basis.clone());
        self
    }

    pub fn into(self, basis: &AdmittedSignalBranchBasis) -> Self {
        self.into_branch(basis)
    }

    pub fn strategy_hint(mut self, strategy: BranchMergeStrategy) -> Self {
        self.strategy_hint = Some(strategy);
        self
    }

    pub fn strategy_named(mut self, name: impl Into<String>) -> Self {
        self.strategy_name = Some(MergeStrategyName::new(name));
        self
    }

    pub fn conflict_policy_named(mut self, name: impl Into<String>) -> Self {
        self.conflict_policy_name = Some(ConflictPolicyName::new(name));
        self
    }

    pub fn conflict_isolation_policy_named(mut self, name: impl Into<String>) -> Self {
        self.conflict_isolation_policy_name = Some(ConflictIsolationPolicyName::new(name));
        self
    }

    pub fn merge_base_named(mut self, name: impl Into<String>) -> Self {
        self.merge_base_name = Some(MergeBaseStrategyName::new(name));
        self
    }

    pub fn identity_matcher_named(mut self, name: impl Into<String>) -> Self {
        self.identity_matcher_name = Some(IdentityMatcherName::new(name));
        self
    }

    pub fn source_only_policy_named(mut self, name: impl Into<String>) -> Self {
        self.source_only_policy_name = Some(SourceOnlyPolicyName::new(name));
        self
    }

    pub fn deletion_policy_named(mut self, name: impl Into<String>) -> Self {
        self.deletion_policy_name = Some(DeletionPolicyName::new(name));
        self
    }

    pub fn aspect_policy_named(mut self, aspect: Aspect, name: impl Into<String>) -> Self {
        self.aspect_policy_bindings
            .push(AspectMergePolicyBinding::new(
                aspect,
                AspectMergePolicyName::new(name),
            ));
        self
    }

    pub fn full_branch(mut self) -> Self {
        self.scope = Some(BranchMergeRequestScope::full_branch());
        self
    }

    pub fn selected_nodes(
        mut self,
        nodes: impl IntoIterator<Item = crate::data::handle::NodeId>,
    ) -> Self {
        self.scope = Some(BranchMergeRequestScope::selected_nodes(nodes));
        self
    }

    pub fn selected_aspects(
        mut self,
        aspects: impl IntoIterator<Item = SignalSelectedAspectRequestEntry>,
    ) -> Self {
        self.scope = Some(BranchMergeRequestScope::selected_aspects(aspects));
        self
    }

    pub fn plan(self) -> Result<PlannedRuntimeMerge<'a, D, I, E, Ctx, T>, SignalError> {
        let source = self
            .source
            .clone()
            .ok_or_else(|| SignalError::invalid_input("merge source branch basis is required"))?;
        let target = self
            .target
            .clone()
            .ok_or_else(|| SignalError::invalid_input("merge target branch basis is required"))?;
        let (source_handle, target_handle) = self
            .runtime
            .validate_signal_branch_merge_bases(&source, &target)
            .map_err(|denial| {
                SignalError::invalid_input(format!("canonical Signal merge denied: {denial:?}"))
            })?;
        let mut raw = RawRuntimeMerge::new(self.runtime)
            .from(source_handle)
            .into_branch(target_handle);
        raw.strategy_name = self.strategy_name;
        raw.strategy_hint = self.strategy_hint;
        raw.merge_base_name = self.merge_base_name;
        raw.conflict_policy_name = self.conflict_policy_name;
        raw.conflict_isolation_policy_name = self.conflict_isolation_policy_name;
        raw.identity_matcher_name = self.identity_matcher_name;
        raw.source_only_policy_name = self.source_only_policy_name;
        raw.deletion_policy_name = self.deletion_policy_name;
        raw.aspect_policy_bindings = self.aspect_policy_bindings;
        raw.scope = self.scope;
        Ok(PlannedRuntimeMerge {
            raw: raw.plan()?,
            source,
            target,
        })
    }

    pub fn run(self) -> Result<SignalBranchMergeOutcome, SignalError> {
        self.plan()?.execute()
    }

    pub fn execute(self) -> Result<SignalBranchMergeOutcome, SignalError> {
        self.run()
    }
}
