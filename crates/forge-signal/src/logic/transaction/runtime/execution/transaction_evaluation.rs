use crate::clock::RuntimeInstant;
use crate::data::aspect::AspectVersion;
use crate::data::comparator::{
    DefaultComparatorPolicyResolver, DefaultComparatorResolver, VersionComparatorPolicy,
};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::proof::DedupedNodeBatch;
use crate::diagnostics::ExecutionFailurePhase;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::{EvaluationRequestMode, IntoEvaluationOutput};
use crate::logic::planner::{
    admit_direct_task_with_policy_resolver, EvaluationPlan, ExecutionReport, StageExecutor,
};

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

pub struct TransactionExecutionRequest<'tx, 'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    tx: &'tx mut SignalTransaction<'a, D, I, E, Ctx, T>,
    targets: Vec<NodeId>,
    request_mode: EvaluationRequestMode,
    executor: Option<StageExecutor>,
}

impl<'tx, 'a, D, I, E, Ctx, T> TransactionExecutionRequest<'tx, 'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    Ctx: Sync,
    T: Copy + Ord,
{
    fn new(
        tx: &'tx mut SignalTransaction<'a, D, I, E, Ctx, T>,
        targets: Vec<NodeId>,
        request_mode: EvaluationRequestMode,
    ) -> Self {
        Self {
            tx,
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

    pub fn run<F, O>(self, evaluator: &F) -> Result<ExecutionReport, SignalError>
    where
        F: for<'ctx> Fn(&mut EvaluationContext<'ctx, Ctx>) -> Result<O, SignalError> + Sync,
        O: IntoEvaluationOutput,
    {
        let executor = self
            .executor
            .unwrap_or_else(|| executor_for_strategy(self.tx.graph.derive_evaluation_strategy()));
        self.tx.execute_evaluation(
            TransactionExecutionIntent::Targets {
                targets: &self.targets,
                request_mode: self.request_mode,
                stage_task_candidates: false,
            },
            evaluator,
            executor,
        )
    }

    pub fn read<F, O>(self, evaluator: &F) -> Result<AspectVersion, SignalError>
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
            .unwrap_or_else(|| executor_for_strategy(self.tx.graph.derive_evaluation_strategy()));
        if !matches!(
            self.tx.graph.get_state(*node)?,
            crate::data::node::NodeState::Clean
        ) {
            self.tx.execute_evaluation(
                TransactionExecutionIntent::Targets {
                    targets: &self.targets,
                    request_mode: self.request_mode,
                    stage_task_candidates: false,
                },
                evaluator,
                executor,
            )?;
        }
        self.tx.graph.node_aspect_version(*node)
    }

    pub fn read_many<F, O>(self, evaluator: &F) -> Result<Vec<AspectVersion>, SignalError>
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
                    self.tx.graph.get_state(*node),
                    Ok(crate::data::node::NodeState::Clean)
                )
            })
            .collect::<Vec<_>>();
        if !pending.is_empty() {
            let executor = self.executor.unwrap_or_else(|| {
                executor_for_strategy(self.tx.graph.derive_evaluation_strategy())
            });
            self.tx.execute_evaluation(
                TransactionExecutionIntent::Targets {
                    targets: &pending,
                    request_mode: self.request_mode,
                    stage_task_candidates: false,
                },
                evaluator,
                executor,
            )?;
        }
        self.targets
            .into_iter()
            .map(|node| self.tx.graph.node_aspect_version(node))
            .collect()
    }
}

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
        for stage in &plan.stages {
            self.stage_task_candidates(&stage.tasks)?;
        }
        let execution_start = RuntimeInstant::now();
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
                        Some(plan.summary),
                    );
                }
                return Err(err);
            }
        };
        self.execution_state
            .record_report(&report, execution_start.elapsed().as_nanos());
        self.lower_observation_classifications_from_report(&report)?;
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
        if matches!(
            self.graph.get_state(node)?,
            crate::data::node::NodeState::Clean
        ) {
            return Ok(self.graph.node_aspect_version(node)?);
        }
        self.evaluate_with_plan_and_executor(
            node,
            evaluator,
            EvaluationRequestMode::Default,
            executor,
        )?;
        Ok(self.graph.node_aspect_version(node)?)
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

    fn collect_dirty_targets(&self) -> Vec<NodeId> {
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
                    let mut comparator = DefaultComparatorResolver;
                    let mut resolver = DefaultComparatorPolicyResolver {
                        fallback: VersionComparatorPolicy::Exact,
                        custom: &mut comparator,
                    };
                    let stage_targets = targets
                        .iter()
                        .copied()
                        .map(|node| {
                            admit_direct_task_with_policy_resolver(
                                &*self.graph,
                                node,
                                request_mode,
                                &mut resolver,
                            )
                        })
                        .collect::<Result<Vec<_>, SignalError>>()?;
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
                let mut comparator = DefaultComparatorResolver;
                let mut resolver = DefaultComparatorPolicyResolver {
                    fallback: VersionComparatorPolicy::Exact,
                    custom: &mut comparator,
                };
                let stage_targets = owned_targets
                    .iter()
                    .copied()
                    .map(|node| {
                        admit_direct_task_with_policy_resolver(
                            &*self.graph,
                            node,
                            EvaluationRequestMode::Default,
                            &mut resolver,
                        )
                    })
                    .collect::<Result<Vec<_>, SignalError>>()?;
                self.stage_task_candidates(&stage_targets)?;
                (&owned_targets[..], EvaluationRequestMode::Default)
            }
        };

        let execution_start = RuntimeInstant::now();
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
        self.lower_observation_classifications_from_report(&report)?;
        absorb_execution_report_telemetry(self.telemetry, &report);
        Ok(report)
    }
}
