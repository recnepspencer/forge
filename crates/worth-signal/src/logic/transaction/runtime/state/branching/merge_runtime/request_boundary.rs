use crate::data::error::SignalError;
use crate::logic::transaction::runtime::{
    BranchMergeFailureKind, BranchMergePlan, FoundationalScopeLoweringDenial,
    LoweredFoundationalMergeRequest, NormalizedBranchMergeRequest,
};

use super::super::super::merge::{
    signal_scope_family_matches_foundational_family, BranchMergeRequest,
};
use super::super::super::runtime_state::SignalRuntime;
use super::plan_compiler;

pub(super) fn lower_foundational_request<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    request: &NormalizedBranchMergeRequest,
) -> Result<LoweredFoundationalMergeRequest, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime
        .telemetry
        .transaction
        .foundational_scope_lowering_count += 1;
    match request.lower_to_foundational_scope() {
        Ok(lowered) => {
            match lowered.foundational_scope().family() {
                worth_foundational::facade::FoundationalMergeScopeFamily::FullBranch => {
                    runtime
                        .telemetry
                        .transaction
                        .foundational_full_branch_lowering_count += 1;
                }
                worth_foundational::facade::FoundationalMergeScopeFamily::SelectedNodes => {
                    runtime
                        .telemetry
                        .transaction
                        .foundational_selected_node_lowering_count += 1;
                }
                worth_foundational::facade::FoundationalMergeScopeFamily::SelectedAspects => {
                    runtime
                        .telemetry
                        .transaction
                        .foundational_selected_aspect_lowering_count += 1;
                }
            }
            Ok(lowered)
        }
        Err(denial) => {
            runtime
                .telemetry
                .transaction
                .foundational_scope_lowering_denial_count += 1;
            Err(FoundationalScopeLoweringDenial::into_signal_error(denial))
        }
    }
}

pub(super) fn admit_and_compile<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    request: &LoweredFoundationalMergeRequest,
) -> Result<BranchMergePlan, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let raw_request = request.normalized_request().request();
    if raw_request.source_branch.id == raw_request.target_branch.id {
        let error = SignalError::branch_merge_failed(
            BranchMergeFailureKind::SelfMergeRejected,
            "branch merge cannot target itself",
        );
        record_failure(runtime, &error, raw_request);
        return Err(error);
    }
    if !signal_scope_family_matches_foundational_family(
        request.normalized_request().normalized_scope().family(),
        request.foundational_scope().family(),
    ) {
        let error = SignalError::invalid_input(
            "foundational merge scope lowering changed the scoped merge family",
        );
        record_failure(runtime, &error, raw_request);
        return Err(error);
    }

    match plan_compiler::compile(runtime, request) {
        Ok(plan) => Ok(plan),
        Err(error) => {
            record_failure(runtime, &error, raw_request);
            Err(error)
        }
    }
}

fn record_failure<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    error: &SignalError,
    request: &BranchMergeRequest,
) where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    crate::diagnostics::recorder::record_branch_merge_failure(
        &mut runtime.graph,
        error,
        Some(request.source_branch.clone()),
        Some(request.target_branch.clone()),
    );
}
