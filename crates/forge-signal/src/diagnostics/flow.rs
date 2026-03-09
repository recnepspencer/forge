use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::summary::{
    EvaluationPlanSummary, ExecutionReportSummary, ExplanationSummary,
};
use crate::diagnostics::failure::RollbackDiagnostic;
use crate::logic::planner::{EvaluationPlan, ExecutionReport, StageExecutor};

/// Structured summary of one upstream change input to signal execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeInputSummary {
    pub changed_nodes: Vec<NodeId>,
    pub changed_aspects: Vec<u8>,
    pub changed_region_count: u32,
    pub causality_kind: Option<String>,
}

/// Structured summary of invalidation routing for one flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidationSummary {
    pub invalidated_direct_subscribers: u32,
    pub maybe_stale_direct_subscribers: u32,
    pub partition_scoped_checks: u32,
}

/// End-to-end causal summary for one signal execution flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowSummary {
    pub profile: DiagnosticsProfile,
    pub change: ChangeInputSummary,
    pub invalidation: InvalidationSummary,
    pub planning: PlanningSummary,
    pub precompute: PrecomputeSummary,
    pub apply: ApplySummary,
    pub rollback: Option<RollbackSummary>,
    pub explanation: Option<ExplanationSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningSummary {
    pub plan: EvaluationPlanSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecomputeSummary {
    pub executor: Option<StageExecutor>,
    pub stage_count: u32,
    pub task_count: u32,
    pub prepared_evaluations_produced: u32,
    pub tasks_deferred_by_condition: u32,
    pub tasks_satisfied_by_memoization: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplySummary {
    pub report: ExecutionReportSummary,
    pub prepared_evaluations_applied: u32,
    pub dependency_capture_updates: u32,
    pub tasks_validated_clean: u32,
    pub tasks_pruned: u32,
    pub tasks_with_suppressed_propagation: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackSummary {
    pub rolled_back: bool,
    pub staged_node_patch_count: u64,
    pub max_touched_nodes_in_txn: u64,
    pub reason: Option<String>,
}

impl ChangeInputSummary {
    pub fn new(
        mut changed_nodes: Vec<NodeId>,
        mut changed_aspects: Vec<Aspect>,
        changed_region_count: u32,
        causality_kind: Option<String>,
    ) -> Self {
        changed_nodes.sort();
        changed_nodes.dedup();
        changed_aspects.sort_by_key(|aspect| aspect.index());
        changed_aspects.dedup();
        Self {
            changed_nodes,
            changed_aspects: changed_aspects
                .into_iter()
                .map(|aspect| aspect.index() as u8)
                .collect(),
            changed_region_count,
            causality_kind,
        }
    }
}

impl InvalidationSummary {
    pub fn new(
        invalidated_direct_subscribers: u32,
        maybe_stale_direct_subscribers: u32,
        partition_scoped_checks: u32,
    ) -> Self {
        Self {
            invalidated_direct_subscribers,
            maybe_stale_direct_subscribers,
            partition_scoped_checks,
        }
    }
}

impl FlowSummary {
    pub fn new(
        profile: DiagnosticsProfile,
        change: ChangeInputSummary,
        invalidation: InvalidationSummary,
        planning: PlanningSummary,
        precompute: PrecomputeSummary,
        apply: ApplySummary,
        rollback: Option<RollbackSummary>,
        explanation: Option<ExplanationSummary>,
    ) -> Self {
        Self {
            profile,
            change,
            invalidation,
            planning,
            precompute,
            apply,
            rollback,
            explanation,
        }
    }
}

impl PlanningSummary {
    pub fn from_plan(plan: &EvaluationPlan, profile: DiagnosticsProfile) -> Self {
        Self {
            plan: EvaluationPlanSummary::from_plan(plan, profile),
        }
    }
}

impl PrecomputeSummary {
    pub fn from_report(report: &ExecutionReport, _profile: DiagnosticsProfile) -> Self {
        let executor = report.stages.first().map(|stage| match stage.outcome {
            crate::logic::planner::StageExecutionOutcome::CompletedSerial => StageExecutor::Serial,
            #[cfg(feature = "parallel")]
            crate::logic::planner::StageExecutionOutcome::CompletedParallel => StageExecutor::Parallel,
        });
        Self {
            executor,
            stage_count: report.stage_count,
            task_count: report.task_count,
            prepared_evaluations_produced: report.prepared_evaluations_produced,
            tasks_deferred_by_condition: report.tasks_deferred_by_condition,
            tasks_satisfied_by_memoization: report.tasks_satisfied_by_memoization,
        }
    }
}

impl ApplySummary {
    pub fn from_report(report: &ExecutionReport, profile: DiagnosticsProfile) -> Self {
        Self {
            report: ExecutionReportSummary::from_report(report, profile),
            prepared_evaluations_applied: report.prepared_evaluations_applied,
            dependency_capture_updates: report.dependency_capture_updates,
            tasks_validated_clean: report.tasks_validated_clean,
            tasks_pruned: report.tasks_pruned,
            tasks_with_suppressed_propagation: report.tasks_with_suppressed_propagation,
        }
    }
}

impl RollbackSummary {
    pub fn from_rollback(rollback: &RollbackDiagnostic) -> Self {
        Self {
            rolled_back: rollback.rolled_back,
            staged_node_patch_count: rollback.staged_node_patch_count,
            max_touched_nodes_in_txn: rollback.max_touched_nodes_in_txn,
            reason: rollback.reason.clone(),
        }
    }
}
