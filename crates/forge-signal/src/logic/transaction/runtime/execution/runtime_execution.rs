use crate::data::aspect::AspectVersion;
use crate::data::comparator::TierPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::ScratchLeaseKind;
use crate::data::handle::NodeId;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::planner::{
    build_evaluation_plan_with_policy_resolver, build_evaluation_session_with_policy_resolver,
    execute_evaluation_session_with_policy, execute_prepared_plan_with_policy, EvaluationPlan,
    ExecutionReport, StageExecutor,
};
use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};

use super::super::state::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn build_evaluation_plan(
        &mut self,
        targets: &[NodeId],
        request_mode: EvaluationRequestMode,
    ) -> Result<EvaluationPlan, SignalError> {
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        build_evaluation_plan_with_policy_resolver(
            &mut self.graph,
            targets,
            request_mode,
            &mut resolver,
        )
    }

    pub fn execute_prepared_plan<F>(
        &mut self,
        plan: &EvaluationPlan,
        precompute: &F,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.execute_prepared_plan_with_executor(plan, precompute, StageExecutor::Serial)
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
        let report = execute_prepared_plan_with_policy(
            &mut self.graph,
            plan,
            precompute,
            &mut resolver,
            executor,
        )?;
        self.absorb_execution_report_telemetry(&report);
        Ok(report)
    }

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
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let report = self.graph.with_scratch(ScratchLeaseKind::Evaluation, |graph, scratch| {
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
        })?;
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
        let targets = crate::logic::transaction::helpers::collect_dirty_targets(&self.graph);
        if targets.is_empty() {
            return Ok(crate::logic::transaction::helpers::empty_execution_report());
        }
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let report = self.graph.with_scratch(ScratchLeaseKind::Evaluation, |graph, scratch| {
            let session = build_evaluation_session_with_policy_resolver(
                graph,
                scratch,
                &targets,
                EvaluationRequestMode::Default,
                &mut resolver,
            )?;
            execute_evaluation_session_with_policy(
                graph,
                &session,
                precompute,
                &mut resolver,
                executor,
            )
        })?;
        self.absorb_execution_report_telemetry(&report);
        Ok(report)
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
        self.telemetry.stage_execution_nanos += report
            .stages
            .iter()
            .map(|stage| stage.duration_nanos)
            .sum::<u128>();
        self.telemetry.execution_snapshots_built += report.execution_snapshots_built as u64;
        self.telemetry.prepared_evaluations_produced += report.prepared_evaluations_produced as u64;
        self.telemetry.prepared_evaluations_applied += report.prepared_evaluations_applied as u64;
        self.telemetry.dependency_capture_updates += report.dependency_capture_updates as u64;
        self.telemetry.execution_snapshot_nanos += report.execution_snapshot_nanos;
        self.telemetry.stage_precompute_nanos += report.stage_precompute_nanos;
        self.telemetry.stage_apply_nanos += report.stage_apply_nanos;
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
            self.telemetry.parallel_precompute_task_count += report.task_count as u64;
        } else {
            self.telemetry.serial_executor_usage_count += 1;
            self.telemetry.serial_precompute_task_count += report.task_count as u64;
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
