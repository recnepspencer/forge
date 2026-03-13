use std::time::Instant;

use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::diagnostics::ExecutionFailurePhase;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::{EvaluationRequestMode, IntoEvaluationOutput};
use crate::logic::planner::{EvaluationPlan, ExecutionReport, StageExecutor};

use super::super::transaction::SignalTransaction;
use super::shared::{
    absorb_execution_report_telemetry, execute_plan_with_runtime_config,
    execute_targets_with_runtime_config_detailed, executor_for_strategy,
};

enum TransactionExecutionIntent<'a> {
    Targets {
        targets: &'a [NodeId],
        request_mode: EvaluationRequestMode,
        stage_task_candidates: bool,
    },
    Dirty,
}

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
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

    pub fn execute_prepared_plan_with_executor<F, O>(
        &mut self,
        plan: &EvaluationPlan,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let execution_start = Instant::now();
        let report = match execute_plan_with_runtime_config(
            self.graph,
            self.config,
            &*self.runtime_ctx,
            plan,
            evaluator,
            executor,
        ) {
            Ok(report) => report,
            Err(err) => {
                if let Some(summary) = self.graph.observe().latest_failure_diagnostics().cloned() {
                    self.scratch.semantic_delta.failure_summary = Some(summary);
                } else {
                    self.record_failure_from_error(
                        ExecutionFailurePhase::Apply,
                        &err,
                        Some(plan.summary.clone()),
                    );
                }
                return Err(err);
            }
        };
        self.execution_state
            .record_report(&report, execution_start.elapsed().as_nanos());
        absorb_execution_report_telemetry(self.telemetry, &report);
        Ok(report)
    }

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
        self.evaluate_with_plan_and_executor(
            node,
            evaluator,
            EvaluationRequestMode::Default,
            executor,
        )?;
        Ok(self.graph.get_entry(node)?.get_aspect_version())
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

    fn collect_dirty_targets(&self) -> Vec<NodeId> {
        let mut targets = self
            .scratch
            .dirty_targets
            .marked_indices()
            .into_iter()
            .filter_map(|index| self.graph.live_node_id_at(index))
            .filter(|node| {
                self.graph
                    .get_entry(*node)
                    .map(|entry| !matches!(entry.get_state(), crate::data::node::NodeState::Clean))
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();
        targets.sort_by_key(|node: &NodeId| (node.index(), node.generation()));
        targets.dedup();
        if targets.is_empty() {
            crate::logic::transaction::helpers::collect_dirty_targets(self.graph)
        } else {
            targets
        }
    }

    fn execute_evaluation<F, O>(
        &mut self,
        intent: TransactionExecutionIntent<'_>,
        evaluator: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let owned_targets;
        let (targets, request_mode) = match intent {
            TransactionExecutionIntent::Targets {
                targets,
                request_mode,
                stage_task_candidates,
            } => {
                if stage_task_candidates {
                    let stage_targets = targets
                        .iter()
                        .copied()
                        .map(|node| crate::logic::planner::EvaluationTask {
                            node,
                            request_mode,
                            direct_request: true,
                            reason: crate::logic::planner::TaskReason::RequestedTarget,
                        })
                        .collect::<Vec<_>>();
                    self.stage_task_candidates(&stage_targets)?;
                } else if let [node] = targets {
                    self.stage_evaluate_candidates(*node)?;
                }
                (targets, request_mode)
            }
            TransactionExecutionIntent::Dirty => {
                owned_targets = self.collect_dirty_targets();
                if owned_targets.is_empty() {
                    return Ok(crate::logic::transaction::helpers::empty_execution_report());
                }
                let stage_targets = owned_targets
                    .iter()
                    .copied()
                    .map(|node| crate::logic::planner::EvaluationTask {
                        node,
                        request_mode: EvaluationRequestMode::Default,
                        direct_request: true,
                        reason: crate::logic::planner::TaskReason::RequestedTarget,
                    })
                    .collect::<Vec<_>>();
                self.stage_task_candidates(&stage_targets)?;
                (&owned_targets[..], EvaluationRequestMode::Default)
            }
        };

        let execution_start = Instant::now();
        let report = match execute_targets_with_runtime_config_detailed(
            self.graph,
            self.config,
            &*self.runtime_ctx,
            targets,
            request_mode,
            evaluator,
            executor,
        ) {
            Ok(report) => report,
            Err(failure) => {
                let err = failure.error;
                if let Some(summary) = self.graph.observe().latest_failure_diagnostics().cloned() {
                    self.scratch.semantic_delta.failure_summary = Some(summary);
                } else {
                    self.record_failure_from_error(
                        ExecutionFailurePhase::Apply,
                        &err,
                        Some(failure.plan_summary),
                    );
                }
                return Err(err);
            }
        };
        self.execution_state
            .record_report(&report, execution_start.elapsed().as_nanos());
        absorb_execution_report_telemetry(self.telemetry, &report);
        Ok(report)
    }
}
