use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::evaluation::IntoEvaluationOutput;
use crate::logic::planner::StageExecutor;

use super::super::super::state::SignalRuntime;
use super::super::shared::{apply_strategy_maintenance, executor_for_strategy};

use super::request::ExecutionIntent;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    pub fn read<F, O>(
        &mut self,
        node: NodeId,
        runtime_ctx: &Ctx,
        evaluator: &F,
    ) -> Result<AspectVersion, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let strategy = self.derive_evaluation_strategy();
        let version = self.read_with_executor(
            node,
            runtime_ctx,
            evaluator,
            executor_for_strategy(strategy),
        )?;
        apply_strategy_maintenance(&mut self.graph, strategy);
        Ok(version)
    }

    pub fn get<F, O>(
        &mut self,
        node: NodeId,
        runtime_ctx: &Ctx,
        evaluator: &F,
    ) -> Result<AspectVersion, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.read(node, runtime_ctx, evaluator)
    }

    pub fn read_with_executor<F, O>(
        &mut self,
        node: NodeId,
        runtime_ctx: &Ctx,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<AspectVersion, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        if matches!(
            self.graph.get_state(node)?,
            crate::data::node::NodeState::Clean
        ) {
            return self.graph.node_aspect_version(node);
        }
        self.evaluate_with_plan_and_executor(
            node,
            runtime_ctx,
            evaluator,
            EvaluationRequestMode::Default,
            executor,
        )?;
        self.graph.node_aspect_version(node)
    }

    pub fn read_many<F, O>(
        &mut self,
        nodes: &[NodeId],
        runtime_ctx: &Ctx,
        evaluator: &F,
    ) -> Result<Vec<AspectVersion>, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let strategy = self.derive_evaluation_strategy();
        let versions = self.read_many_with_executor(
            nodes,
            runtime_ctx,
            evaluator,
            executor_for_strategy(strategy),
        )?;
        apply_strategy_maintenance(&mut self.graph, strategy);
        Ok(versions)
    }

    pub fn read_many_with_executor<F, O>(
        &mut self,
        nodes: &[NodeId],
        runtime_ctx: &Ctx,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<Vec<AspectVersion>, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let pending = nodes
            .iter()
            .copied()
            .filter(|node| {
                !matches!(
                    self.graph.get_state(*node),
                    Ok(crate::data::node::NodeState::Clean)
                )
            })
            .collect::<Vec<_>>();
        if !pending.is_empty() {
            self.execute_evaluation(
                ExecutionIntent::Targets {
                    targets: &pending,
                    request_mode: EvaluationRequestMode::Default,
                },
                runtime_ctx,
                evaluator,
                executor,
            )?;
        }
        nodes
            .iter()
            .copied()
            .map(|node| self.graph.node_aspect_version(node))
            .collect()
    }
}
