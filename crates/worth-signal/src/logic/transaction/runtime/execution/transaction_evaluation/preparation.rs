use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::{EvaluationRequestMode, IntoEvaluationOutput};
use crate::logic::planner::{ExecutionReport, StageExecutor};

use super::super::super::transaction::SignalTransaction;
use super::super::shared::executor_for_strategy;

use super::request::{TransactionExecutionIntent, TransactionExecutionRequest};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    pub fn target(&mut self, node: NodeId) -> TransactionExecutionRequest<'_, 'a, D, I, E, Ctx, T> {
        TransactionExecutionRequest::new(self, vec![node], EvaluationRequestMode::Default)
    }

    pub fn targets(
        &mut self,
        nodes: impl IntoIterator<Item = NodeId>,
    ) -> TransactionExecutionRequest<'_, 'a, D, I, E, Ctx, T> {
        TransactionExecutionRequest::new(
            self,
            nodes.into_iter().collect(),
            EvaluationRequestMode::Default,
        )
    }

    pub fn evaluate_with_plan<F, O>(
        &mut self,
        node: NodeId,
        evaluator: &F,
        request_mode: EvaluationRequestMode,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.evaluate_with_plan_and_executor(
            node,
            evaluator,
            request_mode,
            executor_for_strategy(self.graph.derive_evaluation_strategy()),
        )
    }

    pub fn evaluate_with_plan_and_executor<F, O>(
        &mut self,
        node: NodeId,
        evaluator: &F,
        request_mode: EvaluationRequestMode,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.execute_evaluation(
            TransactionExecutionIntent::Targets {
                targets: std::slice::from_ref(&node),
                request_mode,
                stage_task_candidates: false,
            },
            evaluator,
            executor,
        )
    }
}
