use crate::data::graph::SignalGraph;

use super::super::execution::StageSlice;
#[cfg(feature = "parallel")]
use super::super::types::ParallelExecutionKind;
use super::super::types::StageExecutor;

#[cfg(feature = "parallel")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct StageParallelAdmission {
    pub(crate) use_parallel: bool,
    pub(crate) reason: &'static str,
    pub(crate) kind: Option<ParallelExecutionKind>,
}

#[cfg(feature = "parallel")]
pub(crate) fn decide_stage_parallel_admission(
    graph: &SignalGraph,
    stage: &StageSlice<'_>,
    executor: StageExecutor,
) -> StageParallelAdmission {
    let Some(parallel_policy) = executor.parallel_policy() else {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "serial-executor",
            kind: None,
        };
    };
    if executor.is_full_parallel() {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "full-parallel-unsupported-by-mutable-engine",
            kind: None,
        };
    }
    let stage_width = stage.tasks.len();
    if stage_width < parallel_policy.min_stage_width.get() {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "below-min-stage-width",
            kind: None,
        };
    }
    let runtime_policy = graph.runtime_policy();
    let min_parallel_tasks = runtime_policy
        .parallel_admission
        .min_parallel_tasks_for(runtime_policy.tier);
    let semantic_cost_multiplier = match runtime_policy.retention_budget.semantic_detail {
        crate::diagnostics::SemanticRetentionPolicy::Minimal => 1,
        crate::diagnostics::SemanticRetentionPolicy::Development => 2,
        crate::diagnostics::SemanticRetentionPolicy::Forensic => 3,
    };
    let effective_parallel_threshold = min_parallel_tasks.saturating_mul(semantic_cost_multiplier);
    if stage_width < effective_parallel_threshold {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "below-policy-work-threshold",
            kind: None,
        };
    }
    let compute_pressure = stage
        .tasks
        .iter()
        .filter(|task| {
            !matches!(
                task.reason,
                super::super::types::TaskReason::MaybeStaleValidation
                    | super::super::types::TaskReason::MemoValidation
            )
        })
        .count();
    if compute_pressure.saturating_mul(2) < stage_width
        && stage_width < effective_parallel_threshold.saturating_mul(2)
    {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "validation-heavy-stage",
            kind: None,
        };
    }
    if executor.is_full_parallel()
        && stage_width
            < runtime_policy
                .parallel_admission
                .full_parallel_min_tasks
                .saturating_mul(semantic_cost_multiplier)
    {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "below-full-parallel-threshold",
            kind: None,
        };
    }
    StageParallelAdmission {
        use_parallel: true,
        reason: match runtime_policy.tier {
            crate::diagnostics::profile::DiagnosticsTier::Operational => "admitted-operational",
            crate::diagnostics::profile::DiagnosticsTier::Development => "admitted-development",
            crate::diagnostics::profile::DiagnosticsTier::Forensic => "admitted-forensic",
        },
        kind: executor.parallel_kind(),
    }
}


