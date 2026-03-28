use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::evaluation::IntoEvaluationOutput;
use crate::logic::planner::{
    build_evaluation_plan_with_policy_resolver, EvaluationPlan, ExecutionReport, StageExecutor,
};

use super::super::state::SignalRuntime;
use super::shared::{
    absorb_execution_report_telemetry, apply_strategy_maintenance,
    execute_plan_with_runtime_config, execute_targets_with_runtime_config, executor_for_strategy,
};

enum ExecutionIntent<'a> {
    Targets {
        targets: &'a [NodeId],
        request_mode: EvaluationRequestMode,
    },
    Dirty,
}

pub struct RuntimeExecutionRequest<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>,
    targets: Vec<NodeId>,
    request_mode: EvaluationRequestMode,
    executor: Option<StageExecutor>,
}

impl<'a, D, I, E, Ctx, T> RuntimeExecutionRequest<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    fn new(
        runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>,
        targets: Vec<NodeId>,
        request_mode: EvaluationRequestMode,
    ) -> Self {
        Self {
            runtime,
            targets,
            request_mode,
            executor: None,
        }
    }

    pub fn on_demand(mut self) -> Self {
        self.request_mode = EvaluationRequestMode::ForceOnDemand;
        self
    }

    pub fn with_mode(mut self, request_mode: EvaluationRequestMode) -> Self {
        self.request_mode = request_mode;
        self
    }

    pub fn with_executor(mut self, executor: StageExecutor) -> Self {
        self.executor = Some(executor);
        self
    }

    pub fn run<F, O>(self, runtime_ctx: &Ctx, evaluator: &F) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let executor = self
            .executor
            .unwrap_or_else(|| executor_for_strategy(self.runtime.derive_evaluation_strategy()));
        self.runtime.execute_evaluation(
            ExecutionIntent::Targets {
                targets: &self.targets,
                request_mode: self.request_mode,
            },
            runtime_ctx,
            evaluator,
            executor,
        )
    }

    pub fn read<F, O>(self, runtime_ctx: &Ctx, evaluator: &F) -> Result<AspectVersion, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let [node] = self.targets.as_slice() else {
            return Err(SignalError::invalid_input(
                "guided read requires exactly one target; use read_many for multiple targets",
            ));
        };
        let executor = self
            .executor
            .unwrap_or_else(|| executor_for_strategy(self.runtime.derive_evaluation_strategy()));
        if !matches!(
            self.runtime.graph.get_state(*node)?,
            crate::data::node::NodeState::Clean
        ) {
            self.runtime.execute_evaluation(
                ExecutionIntent::Targets {
                    targets: &self.targets,
                    request_mode: self.request_mode,
                },
                runtime_ctx,
                evaluator,
                executor,
            )?;
        }
        self.runtime.graph.node_aspect_version(*node)
    }

    pub fn read_many<F, O>(
        self,
        runtime_ctx: &Ctx,
        evaluator: &F,
    ) -> Result<Vec<AspectVersion>, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let pending = self
            .targets
            .iter()
            .copied()
            .filter(|node| {
                !matches!(
                    self.runtime.graph.get_state(*node),
                    Ok(crate::data::node::NodeState::Clean)
                )
            })
            .collect::<Vec<_>>();
        if !pending.is_empty() {
            let executor = self
                .executor
                .unwrap_or_else(|| executor_for_strategy(self.runtime.derive_evaluation_strategy()));
            self.runtime.execute_evaluation(
                ExecutionIntent::Targets {
                    targets: &pending,
                    request_mode: self.request_mode,
                },
                runtime_ctx,
                evaluator,
                executor,
            )?;
        }
        self.targets
            .into_iter()
            .map(|node| self.runtime.graph.node_aspect_version(node))
            .collect()
    }
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    pub fn target(&mut self, node: NodeId) -> RuntimeExecutionRequest<'_, D, I, E, Ctx, T> {
        RuntimeExecutionRequest::new(self, vec![node], EvaluationRequestMode::Default)
    }

    pub fn targets(
        &mut self,
        nodes: impl IntoIterator<Item = NodeId>,
    ) -> RuntimeExecutionRequest<'_, D, I, E, Ctx, T> {
        RuntimeExecutionRequest::new(
            self,
            nodes.into_iter().collect(),
            EvaluationRequestMode::Default,
        )
    }

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
        let report = execute_plan_with_runtime_config(
            &mut self.graph,
            &self.config,
            runtime_ctx,
            plan,
            evaluator,
            executor,
        )?;
        absorb_execution_report_telemetry(&mut self.telemetry, &report);
        Ok(report)
    }

    pub fn evaluate_with_plan<F, O>(
        &mut self,
        node: NodeId,
        runtime_ctx: &Ctx,
        evaluator: &F,
        request_mode: EvaluationRequestMode,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let strategy = self.derive_evaluation_strategy();
        let report = self.evaluate_with_plan_and_executor(
            node,
            runtime_ctx,
            evaluator,
            request_mode,
            executor_for_strategy(strategy),
        )?;
        apply_strategy_maintenance(&mut self.graph, strategy);
        Ok(report)
    }

    pub fn evaluate_with_plan_and_executor<F, O>(
        &mut self,
        node: NodeId,
        runtime_ctx: &Ctx,
        evaluator: &F,
        request_mode: EvaluationRequestMode,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        self.execute_evaluation(
            ExecutionIntent::Targets {
                targets: std::slice::from_ref(&node),
                request_mode,
            },
            runtime_ctx,
            evaluator,
            executor,
        )
    }

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
        if matches!(self.graph.get_state(node)?, crate::data::node::NodeState::Clean) {
            return Ok(self.graph.node_aspect_version(node)?);
        }
        self.evaluate_with_plan_and_executor(
            node,
            runtime_ctx,
            evaluator,
            EvaluationRequestMode::Default,
            executor,
        )?;
        Ok(self.graph.node_aspect_version(node)?)
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
        let versions =
            self.read_many_with_executor(nodes, runtime_ctx, evaluator, executor_for_strategy(strategy))?;
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
        nodes.iter()
            .copied()
            .map(|node| self.graph.node_aspect_version(node))
            .collect()
    }

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

    fn execute_evaluation<F, O>(
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
        let report = execute_targets_with_runtime_config(
            &mut self.graph,
            &self.config,
            runtime_ctx,
            targets,
            request_mode,
            evaluator,
            executor,
        )?;
        absorb_execution_report_telemetry(&mut self.telemetry, &report);
        Ok(report)
    }
}
