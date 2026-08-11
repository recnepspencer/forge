use crate::data::error::SignalError;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::evaluation::IntoEvaluationOutput;
use crate::logic::planner::{ExecutionReport, StageExecutor};

use super::super::super::state::SignalRuntime;
use super::super::shared::{
    absorb_execution_report_telemetry, apply_strategy_maintenance,
    execute_targets_with_runtime_config, executor_for_strategy,
};

use super::request::ExecutionIntent;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    pub fn evaluate_dirty<F, O>(
        &mut self,
        runtime_ctx: &Ctx,
        evaluator: &F,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let strategy = self.derive_evaluation_strategy();
        let report = self.evaluate_dirty_with_executor(
            runtime_ctx,
            evaluator,
            executor_for_strategy(strategy),
        )?;
        apply_strategy_maintenance(&mut self.graph, strategy);
        Ok(report)
    }

    pub fn evaluate_dirty_with_executor<F, O>(
        &mut self,
        runtime_ctx: &Ctx,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.execute_evaluation(ExecutionIntent::Dirty, runtime_ctx, evaluator, executor)
    }

    pub(super) fn execute_evaluation<F, O>(
        &mut self,
        intent: ExecutionIntent<'_>,
        runtime_ctx: &Ctx,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let owned_targets;
        let (targets, request_mode) = match intent {
            ExecutionIntent::Targets {
                targets,
                request_mode,
            } => (targets, request_mode),
            ExecutionIntent::Dirty => {
                owned_targets =
                    crate::logic::transaction::helpers::collect_dirty_targets(&self.graph);
                if owned_targets.is_empty() {
                    return Ok(crate::logic::transaction::helpers::empty_execution_report());
                }
                (&owned_targets[..], EvaluationRequestMode::Default)
            }
        };
        self.admit_temporal_wakes_for_nodes(targets)?;
        self.promote_due_temporal_wakes_ready()?;
        let temporal_lowering = self.temporal_lowering_context_for_nodes(targets);
        let report = execute_targets_with_runtime_config(
            &mut self.graph,
            &self.config,
            temporal_lowering,
            runtime_ctx,
            targets,
            request_mode,
            evaluator,
            executor,
        )?;
        absorb_execution_report_telemetry(&mut self.telemetry, &report);
        self.retire_consumed_temporal_wakes_from_report(&report)?;
        Ok(report)
    }
}
