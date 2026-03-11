use crate::data::error::SignalError;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::planner::{
    build_evaluation_plan, execute_prepared_plan, execute_prepared_plan_with_policy,
    EvaluationPlan, ExecutionReport, StageExecutor,
};
use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};

use super::graph::SignalGraph;

impl SignalGraph {
    pub fn build_evaluation_plan(
        &mut self,
        targets: &[crate::data::handle::NodeId],
        request_mode: EvaluationRequestMode,
    ) -> Result<EvaluationPlan, SignalError> {
        build_evaluation_plan(self, targets, request_mode)
    }

    pub fn execute_prepared_plan<F>(
        &mut self,
        plan: &EvaluationPlan,
        precompute: &F,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(
                crate::data::handle::NodeId,
                &ExecutionReadView<'_>,
            ) -> Result<PreparedEvaluation, SignalError>
            + Sync,
    {
        execute_prepared_plan(self, plan, precompute)
    }

    pub fn execute_prepared_plan_with_executor<F>(
        &mut self,
        plan: &EvaluationPlan,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(
                crate::data::handle::NodeId,
                &ExecutionReadView<'_>,
            ) -> Result<PreparedEvaluation, SignalError>
            + Sync,
    {
        let mut comparator = crate::data::comparator::DefaultComparatorResolver;
        let mut resolver = crate::data::comparator::DefaultComparatorPolicyResolver {
            fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
            custom: &mut comparator,
        };
        execute_prepared_plan_with_policy(self, plan, precompute, &mut resolver, executor)
    }

    pub(crate) fn partition_interner_mut(&mut self) -> &mut crate::data::output::PartitionInterner {
        &mut self.partition_interner
    }
}
