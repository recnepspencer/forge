use crate::data::error::SignalError;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::IntoEvaluationOutput;
use crate::logic::planner::{EvaluationPlan, ExecutionReport, StageExecutor};

use super::super::super::state::SignalRuntime;
use super::super::shared::{
    absorb_execution_report_telemetry, apply_strategy_maintenance,
    execute_plan_with_runtime_config, executor_for_strategy,
};

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    pub fn execute_prepared_plan<F, O>(
        &mut self,
        plan: &EvaluationPlan,
        runtime_ctx: &Ctx,
        evaluator: &F,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let strategy = self.derive_evaluation_strategy();
        let report = self.execute_prepared_plan_with_executor(
            plan,
            runtime_ctx,
            evaluator,
            executor_for_strategy(strategy),
        )?;
        apply_strategy_maintenance(&mut self.graph, strategy);
        Ok(report)
    }

    pub fn execute_prepared_plan_with_executor<F, O>(
        &mut self,
        plan: &EvaluationPlan,
        runtime_ctx: &Ctx,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.admit_temporal_wakes_for_plan(plan)?;
        self.promote_due_temporal_wakes_ready()?;
        let temporal_lowering = self.temporal_lowering_context_for_plan(plan);
        let report = execute_plan_with_runtime_config(
            &mut self.graph,
            &self.config,
            temporal_lowering,
            runtime_ctx,
            plan,
            evaluator,
            executor,
        )?;
        absorb_execution_report_telemetry(&mut self.telemetry, &report);
        self.retire_consumed_temporal_wakes_from_report(&report)?;
        Ok(report)
    }
}
