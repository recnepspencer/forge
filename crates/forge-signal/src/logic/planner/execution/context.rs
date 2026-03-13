use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::summary::EvaluationPlanSummary;
use crate::logic::prepared::PreparedEvaluation;

use super::super::types::{ExecutionReport, PlanSummary, StageExecutor};
use super::diagnostics::record_successful_execution;
use super::reporting::begin_execution_report;

pub(crate) struct ExecutionContext<'a, F, R>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
    R: ComparatorPolicyResolver,
{
    pub(crate) graph: &'a mut SignalGraph,
    pub(crate) summary: &'a PlanSummary,
    pub(crate) precompute: &'a F,
    pub(crate) comparator_resolver: &'a mut R,
    pub(crate) executor: StageExecutor,
    pub(crate) next_record_id: u64,
    pub(crate) next_segment_id: u64,
    pub(crate) plan_summary: EvaluationPlanSummary,
    pub(crate) first_target: Option<NodeId>,
    pub(crate) report: ExecutionReport,
}

impl<'a, F, R> ExecutionContext<'a, F, R>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
    R: ComparatorPolicyResolver,
{
    pub(crate) fn new(
        graph: &'a mut SignalGraph,
        summary: &'a PlanSummary,
        stage_count: usize,
        maybe_stale_validation_tasks: u64,
        plan_summary: EvaluationPlanSummary,
        first_target: Option<NodeId>,
        precompute: &'a F,
        comparator_resolver: &'a mut R,
        executor: StageExecutor,
    ) -> Self {
        let report = begin_execution_report(
            graph,
            summary,
            stage_count,
            maybe_stale_validation_tasks,
            executor,
        );
        Self {
            graph,
            summary,
            precompute,
            comparator_resolver,
            executor,
            next_record_id: 1,
            next_segment_id: 1,
            plan_summary,
            first_target,
            report,
        }
    }

    pub(crate) fn finish(self) -> ExecutionReport {
        record_successful_execution(
            self.graph,
            self.plan_summary,
            self.first_target,
            &self.report,
        );
        self.report
    }
}
