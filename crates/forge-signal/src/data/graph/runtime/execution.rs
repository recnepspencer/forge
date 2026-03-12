use crate::data::error::SignalError;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::IntoEvaluationOutput;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::planner::{
    build_evaluation_plan, execute_prepared_plan, execute_prepared_plan_with_policy,
    execute_prepared_plan_with_precompute, EvaluationPlan, ExecutionReport, StageExecutor,
};
use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};

use super::graph::SignalGraph;

impl SignalGraph {
    pub(crate) fn execute_prepared_plan_with_precompute<F>(
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
        let mut comparator = crate::data::comparator::DefaultComparatorResolver;
        let mut resolver = crate::data::comparator::DefaultComparatorPolicyResolver {
            fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
            custom: &mut comparator,
        };
        execute_prepared_plan_with_precompute(
            self,
            plan,
            precompute,
            &mut resolver,
            StageExecutor::Serial,
        )
    }

    pub fn build_evaluation_plan(
        &mut self,
        targets: &[crate::data::handle::NodeId],
        request_mode: EvaluationRequestMode,
    ) -> Result<EvaluationPlan, SignalError> {
        build_evaluation_plan(self, targets, request_mode)
    }

    pub fn execute_prepared_plan<Ctx, F, O>(
        &mut self,
        plan: &EvaluationPlan,
        domain_ctx: &Ctx,
        evaluator: &F,
    ) -> Result<ExecutionReport, SignalError>
    where
        Ctx: Sync,
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        execute_prepared_plan(self, plan, domain_ctx, evaluator)
    }

    pub fn execute_prepared_plan_with_executor<Ctx, F, O>(
        &mut self,
        plan: &EvaluationPlan,
        domain_ctx: &Ctx,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        Ctx: Sync,
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let mut comparator = crate::data::comparator::DefaultComparatorResolver;
        let mut resolver = crate::data::comparator::DefaultComparatorPolicyResolver {
            fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
            custom: &mut comparator,
        };
        execute_prepared_plan_with_policy(
            self,
            plan,
            domain_ctx,
            evaluator,
            &mut resolver,
            executor,
        )
    }
}
