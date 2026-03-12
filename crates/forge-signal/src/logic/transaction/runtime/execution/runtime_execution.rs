use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::planner::{
    build_evaluation_plan_with_policy_resolver, EvaluationPlan, ExecutionReport, StageExecutor,
};
use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};

use super::super::state::SignalRuntime;
use super::shared::{
    absorb_execution_report_telemetry, execute_plan_with_runtime_config,
    execute_targets_with_runtime_config,
};

enum ExecutionIntent<'a> {
    Targets {
        targets: &'a [NodeId],
        request_mode: EvaluationRequestMode,
    },
    Dirty,
}

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
        let mut resolver = crate::data::comparator::TierPolicyResolver::new(
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
        let report = execute_plan_with_runtime_config(
            &mut self.graph,
            &self.config,
            plan,
            precompute,
            executor,
        )?;
        absorb_execution_report_telemetry(&mut self.telemetry, &report);
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
        self.execute_evaluation(
            ExecutionIntent::Targets {
                targets: std::slice::from_ref(&node),
                request_mode,
            },
            precompute,
            executor,
        )
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
        self.execute_evaluation(ExecutionIntent::Dirty, precompute, executor)
    }

    fn execute_evaluation<F>(
        &mut self,
        intent: ExecutionIntent<'_>,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        let owned_targets;
        let (targets, request_mode) = match intent {
            ExecutionIntent::Targets {
                targets,
                request_mode,
            } => (targets, request_mode),
            ExecutionIntent::Dirty => {
                owned_targets = crate::logic::transaction::helpers::collect_dirty_targets(&self.graph);
                if owned_targets.is_empty() {
                    return Ok(crate::logic::transaction::helpers::empty_execution_report());
                }
                (&owned_targets[..], EvaluationRequestMode::Default)
            }
        };
        let report = execute_targets_with_runtime_config(
            &mut self.graph,
            &self.config,
            targets,
            request_mode,
            precompute,
            executor,
        )?;
        absorb_execution_report_telemetry(&mut self.telemetry, &report);
        Ok(report)
    }
}
