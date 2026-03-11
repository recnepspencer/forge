use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::prepared::PreparedEvaluation;

use super::execution_diagnostics::{record_successful_execution, RecordedPlan};
use super::execution_reporting::begin_execution_report;
use super::types::{ExecutionReport, PlanSummary, StageExecutor};

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
    pub(crate) recorded_plan: RecordedPlan,
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
        recorded_plan: RecordedPlan,
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
            recorded_plan,
            report,
        }
    }

    pub(crate) fn finish(self) -> ExecutionReport {
        record_successful_execution(self.graph, self.recorded_plan, &self.report);
        self.report
    }
}
