use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::diagnostics::ExecutionFailurePhase;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::planner::{
    EvaluationPlan, ExecutionReport, StageExecutor,
};
use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};
use std::time::Instant;

use super::super::transaction::SignalTransaction;
use super::shared::{
    absorb_execution_report_telemetry, execute_plan_with_runtime_config,
    execute_targets_with_runtime_config_detailed,
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
    T: Copy + Ord,
{
    pub fn evaluate_with_plan<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
        request_mode: EvaluationRequestMode,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_with_plan_and_executor(node, precompute, request_mode, StageExecutor::Serial)
    }

    pub fn evaluate_with_plan_and_executor<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
        request_mode: EvaluationRequestMode,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.execute_evaluation(
            TransactionExecutionIntent::Targets {
                targets: std::slice::from_ref(&node),
                request_mode,
                stage_task_candidates: false,
            },
            precompute,
            executor,
        )
    }

    pub fn execute_prepared_plan_with_executor<F>(
        &mut self,
        plan: &EvaluationPlan,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        let execution_start = Instant::now();
        let report = match execute_plan_with_runtime_config(
            self.graph,
            self.config,
            plan,
            precompute,
            executor,
        ) {
            Ok(report) => report,
            Err(err) => {
                if let Some(summary) = self.graph.latest_failure_diagnostics().cloned() {
                    self.semantic_delta.failure_summary = Some(summary);
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

    pub fn read<F>(&mut self, node: NodeId, precompute: &F) -> Result<AspectVersion, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.read_with_executor(node, precompute, StageExecutor::Serial)
    }

    pub fn get<F>(&mut self, node: NodeId, precompute: &F) -> Result<AspectVersion, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.read(node, precompute)
    }

    pub fn read_with_executor<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<AspectVersion, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_with_plan_and_executor(
            node,
            precompute,
            EvaluationRequestMode::Default,
            executor,
        )?;
        Ok(self.graph.get_entry(node)?.get_aspect_version())
    }

    pub fn evaluate_dirty<F>(&mut self, precompute: &F) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_dirty_with_executor(precompute, StageExecutor::Serial)
    }

    pub fn evaluate_dirty_with_executor<F>(
        &mut self,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.execute_evaluation(TransactionExecutionIntent::Dirty, precompute, executor)
    }

    fn collect_dirty_targets(&self) -> Vec<NodeId> {
        let mut targets = self
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
        targets.sort_by_key(|node| (node.index(), node.generation()));
        targets.dedup();
        if targets.is_empty() {
            crate::logic::transaction::helpers::collect_dirty_targets(self.graph)
        } else {
            targets
        }
    }

    fn execute_evaluation<F>(
        &mut self,
        intent: TransactionExecutionIntent<'_>,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
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
            targets,
            request_mode,
            precompute,
            executor,
        ) {
            Ok(report) => report,
            Err(failure) => {
                let err = failure.error;
                if let Some(summary) = self.graph.latest_failure_diagnostics().cloned() {
                    self.semantic_delta.failure_summary = Some(summary);
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
