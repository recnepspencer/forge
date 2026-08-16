use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::proof::DedupedNodeBatch;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::{EvaluationRequestMode, IntoEvaluationOutput};
use crate::logic::planner::{ExecutionReport, StageExecutor};

use super::super::super::transaction::SignalTransaction;
use super::super::request_order::requested_dependency_order;
use super::super::shared::executor_for_strategy;

use super::request::TransactionExecutionIntent;

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    pub fn read<F, O>(&mut self, node: NodeId, evaluator: &F) -> Result<AspectVersion, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.read_with_executor(
            node,
            evaluator,
            executor_for_strategy(self.graph.derive_evaluation_strategy()),
        )
    }

    pub fn get<F, O>(&mut self, node: NodeId, evaluator: &F) -> Result<AspectVersion, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.read(node, evaluator)
    }

    pub fn read_with_executor<F, O>(
        &mut self,
        node: NodeId,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<AspectVersion, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let max_passes = self.graph.active_node_count().saturating_add(1);
        for _ in 0..max_passes {
            let mut scheduled = 0_u32;
            let mut executed = 0_u32;
            for target in requested_dependency_order(&self.graph, node)? {
                let report = self.evaluate_with_plan_and_executor(
                    target,
                    evaluator,
                    EvaluationRequestMode::Default,
                    executor,
                )?;
                scheduled = scheduled.saturating_add(report.task_count);
                executed = executed.saturating_add(report.tasks_executed);
            }
            if scheduled == 0 || executed == 0 {
                return self.graph.node_aspect_version(node);
            }
        }
        Err(SignalError::internal(
            "transaction dependency settlement did not converge",
        ))
    }

    pub fn read_many<F, O>(
        &mut self,
        nodes: &[NodeId],
        evaluator: &F,
    ) -> Result<Vec<AspectVersion>, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.read_many_with_executor(
            nodes,
            evaluator,
            executor_for_strategy(self.graph.derive_evaluation_strategy()),
        )
    }

    pub fn read_many_with_executor<F, O>(
        &mut self,
        nodes: &[NodeId],
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
                TransactionExecutionIntent::Targets {
                    targets: &pending,
                    request_mode: EvaluationRequestMode::Default,
                    stage_task_candidates: false,
                },
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

    pub fn evaluate_dirty<F, O>(&mut self, evaluator: &F) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.evaluate_dirty_with_executor(
            evaluator,
            executor_for_strategy(self.graph.derive_evaluation_strategy()),
        )
    }

    pub fn evaluate_dirty_with_executor<F, O>(
        &mut self,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.execute_evaluation(TransactionExecutionIntent::Dirty, evaluator, executor)
    }

    pub(super) fn collect_dirty_targets(&self) -> Vec<NodeId> {
        let targets = DedupedNodeBatch::canonicalize_unordered(
            self.scratch
                .dirty_targets
                .marked_indices()
                .into_iter()
                .filter_map(|index| self.graph.live_node_id_at(index))
                .filter(|node| {
                    self.graph
                        .get_state(*node)
                        .map(|state| !matches!(state, crate::data::node::NodeState::Clean))
                        .unwrap_or(false)
                }),
        )
        .into_vec();
        if targets.is_empty() {
            crate::logic::transaction::helpers::collect_dirty_targets(self.graph)
        } else {
            targets
        }
    }
}
