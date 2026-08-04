mod artifact_projection;
mod aspect_policy;
mod aspect_policy_selection;
mod candidates;
mod conflict_classification;
mod conflict_isolation;
mod conflict_policy;
mod conflict_resolution;
mod correspondence;
mod correspondence_evidence;
mod deletion_policy;
mod execution_application;
mod execution_artifacts;
mod execution_finalization;
mod execution_lifecycle;
mod execution_preparation;
mod execution_summary;
mod identity_matcher;
mod merge_base_strategy;
mod node_plan;
mod plan_compiler;
mod request_boundary;
mod result_projection;
mod source_only_policy;

use crate::data::error::SignalError;
use crate::state::SignalBranchHandle;

use super::super::merge::{
    BranchMergePlan, BranchMergeRequest, BranchMergeRequestDenial, BranchMergeResult,
    LoweredFoundationalMergeRequest,
};

#[cfg(test)]
use super::super::merge::BranchMergeExecutionSummary;
use super::super::runtime_state::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn merge_branch(
        &mut self,
        source: SignalBranchHandle,
        target: SignalBranchHandle,
    ) -> Result<BranchMergeResult, SignalError> {
        let normalized_request = BranchMergeRequest::full_branch(source, target)
            .normalize()
            .map_err(BranchMergeRequestDenial::into_signal_error)?;
        let lowered_request = self.lower_foundational_merge_request(&normalized_request)?;
        let plan = self.plan_branch_merge_request(&lowered_request)?;
        self.execute_branch_merge_request_plan(&lowered_request, &plan)
    }

    pub(crate) fn lower_foundational_merge_request(
        &mut self,
        request: &crate::logic::transaction::runtime::NormalizedBranchMergeRequest,
    ) -> Result<LoweredFoundationalMergeRequest, SignalError> {
        request_boundary::lower_foundational_request(self, request)
    }

    pub(crate) fn plan_branch_merge_request(
        &mut self,
        request: &LoweredFoundationalMergeRequest,
    ) -> Result<BranchMergePlan, SignalError> {
        request_boundary::admit_and_compile(self, request)
    }

    pub(crate) fn execute_branch_merge_request_plan(
        &mut self,
        request: &LoweredFoundationalMergeRequest,
        plan: &BranchMergePlan,
    ) -> Result<BranchMergeResult, SignalError> {
        execution_lifecycle::execute_and_project(self, request, plan)
    }

    #[cfg(test)]
    pub(crate) fn execute_branch_merge_request_plan_summary_for_test(
        &mut self,
        request: &LoweredFoundationalMergeRequest,
        plan: &BranchMergePlan,
    ) -> Result<BranchMergeExecutionSummary, SignalError> {
        execution_lifecycle::execute_summary_for_test(self, request, plan)
    }

    #[cfg(test)]
    pub(crate) fn inspect_branch_merge_plan_for_test(
        &mut self,
        source: SignalBranchHandle,
        target: SignalBranchHandle,
    ) -> Result<BranchMergePlan, SignalError> {
        let normalized_request = BranchMergeRequest::full_branch(source, target)
            .normalize()
            .map_err(BranchMergeRequestDenial::into_signal_error)?;
        let lowered_request = self.lower_foundational_merge_request(&normalized_request)?;
        self.plan_branch_merge_request(&lowered_request)
    }
}
