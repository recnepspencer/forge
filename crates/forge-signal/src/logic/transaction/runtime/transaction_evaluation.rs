use crate::data::aspect::AspectVersion;
use crate::data::comparator::TierPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::ScratchLeaseKind;
use crate::data::handle::NodeId;
use crate::diagnostics::ExecutionFailurePhase;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::planner::{
    build_evaluation_session_with_policy_resolver, execute_evaluation_session_with_policy,
    execute_prepared_plan_with_policy, EvaluationPlan, ExecutionReport, StageExecutor,
};
use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};

use super::transaction_types::SignalTransaction;

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
        self.stage_evaluate_candidates(node)?;
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let report = match self.graph.with_scratch(ScratchLeaseKind::Evaluation, |graph, scratch| {
            let session = build_evaluation_session_with_policy_resolver(
                graph,
                scratch,
                &[node],
                request_mode,
                &mut resolver,
            )?;
            execute_evaluation_session_with_policy(
                graph,
                &session,
                precompute,
                &mut resolver,
                executor,
            )
        }) {
            Ok(report) => report,
            Err(err) => {
                if let Some(summary) = self.graph.latest_failure_diagnostics().cloned() {
                    self.semantic_delta.failure_summary = Some(summary);
                } else {
                    self.record_failure_from_error(
                        ExecutionFailurePhase::Apply,
                        &err,
                        None,
                    );
                }
                return Err(err);
            }
        };
        self.absorb_execution_report_telemetry(&report);
        Ok(report)
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
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let report = match execute_prepared_plan_with_policy(
            self.graph,
            plan,
            precompute,
            &mut resolver,
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
        self.absorb_execution_report_telemetry(&report);
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
        let targets = self.collect_dirty_targets();
        if targets.is_empty() {
            return Ok(crate::logic::transaction::helpers::empty_execution_report());
        }
        let stage_targets = targets
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
        let mut planning_resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let mut execution_resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let report = match self.graph.with_scratch(ScratchLeaseKind::Evaluation, |graph, scratch| {
            let session = build_evaluation_session_with_policy_resolver(
                graph,
                scratch,
                &targets,
                EvaluationRequestMode::Default,
                &mut planning_resolver,
            )?;
            execute_evaluation_session_with_policy(
                graph,
                &session,
                precompute,
                &mut execution_resolver,
                executor,
            )
        }) {
            Ok(report) => report,
            Err(err) => {
                if let Some(summary) = self.graph.latest_failure_diagnostics().cloned() {
                    self.semantic_delta.failure_summary = Some(summary);
                } else {
                    self.record_failure_from_error(
                        ExecutionFailurePhase::Apply,
                        &err,
                        None,
                    );
                }
                return Err(err);
            }
        };
        self.absorb_execution_report_telemetry(&report);
        Ok(report)
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

    pub(super) fn absorb_execution_report_telemetry(&mut self, report: &ExecutionReport) {
        self.telemetry.plans_built += 1;
        self.telemetry.stages_built += report.stage_count as u64;
        self.telemetry.tasks_scheduled += report.task_count as u64;
        self.telemetry.tasks_pruned_before_execution += report.tasks_pruned as u64;
        self.telemetry.maybe_stale_validation_tasks += report
            .stages
            .iter()
            .flat_map(|stage| &stage.task_records)
            .filter(|record| {
                matches!(
                    record.scheduled_reason,
                    crate::logic::planner::TaskReason::MaybeStaleValidation
                )
            })
            .count() as u64;
        self.telemetry.stage_execution_count += report.stage_count as u64;
        #[cfg(feature = "parallel")]
        let parallel_stages = report
            .stages
            .iter()
            .filter(|stage| {
                matches!(
                    stage.outcome,
                    crate::logic::planner::StageExecutionOutcome::CompletedParallel
                )
            })
            .count() as u64;
        #[cfg(not(feature = "parallel"))]
        let parallel_stages = 0_u64;
        if parallel_stages > 0 {
            self.telemetry.parallel_executor_usage_count += 1;
            self.telemetry.parallel_stage_dispatch_count += parallel_stages;
        } else {
            self.telemetry.serial_executor_usage_count += 1;
        }
        self.telemetry.max_tasks_in_stage = self.telemetry.max_tasks_in_stage.max(
            report
                .stages
                .iter()
                .map(|stage| stage.task_records.len() as u64)
                .max()
                .unwrap_or(0),
        );
    }
}
