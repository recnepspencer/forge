use crate::clock::RuntimeInstant;
use crate::data::aspect::AspectVersion;
use crate::data::comparator::{
    DefaultComparatorPolicyResolver, DefaultComparatorResolver, VersionComparatorPolicy,
};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::EvaluationCondition;
use crate::data::proof::DedupedNodeBatch;
use crate::data::temporal::{
    ClockTick, LoweredTemporalEligibility, ScheduledTemporalWake, TemporalCondition,
    TemporalPreviousValueAccess, TemporalPreviousValueReference, TemporalWakeId, TemporalWakeOwner,
    TemporalWakeRetirementReason,
};
use crate::diagnostics::ExecutionFailurePhase;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::{EvaluationRequestMode, IntoEvaluationOutput};
use crate::logic::planner::{
    admit_direct_task_with_policy_resolver, EvaluationPlan, ExecutionReport, StageExecutor,
    TemporalLoweringContext,
};
use std::collections::BTreeSet;

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
        self.admit_temporal_wakes_for_plan(plan)?;
        self.promote_due_temporal_wakes_ready()?;
        let temporal_lowering = self.temporal_lowering_context_for_plan(plan);
        let execution_start = RuntimeInstant::now();
        let report = match execute_plan_with_runtime_config(
            self.graph,
            self.config,
            temporal_lowering,
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
        self.scratch.temporal.absorb_report(&report);
        self.lower_observation_classifications_from_report(&report)?;
        absorb_execution_report_telemetry(self.telemetry, &report);
        self.retire_consumed_temporal_wakes_from_report(&report)?;
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

    pub fn grant_temporal_previous_value_access(
        &mut self,
        wake_id: TemporalWakeId,
    ) -> Result<TemporalPreviousValueAccess, SignalError> {
        let Some(owner) = self.temporal.active_wake_owner(wake_id) else {
            return Err(SignalError::invalid_input(format!(
                "cannot grant previous-value access from inactive temporal wake {}",
                wake_id.get()
            )));
        };
        if let TemporalWakeOwner::Node(node) = owner {
            if !self.graph.is_alive(node) {
                return Err(SignalError::invalid_input(format!(
                    "cannot grant previous-value access from wake {} owned by non-live node {}",
                    wake_id.get(),
                    node
                )));
            }
        }
        self.temporal
            .grant_previous_value_access(self.graph.current_branch().id, wake_id)
    }

    pub fn previous_temporal_value(
        &mut self,
        access: &TemporalPreviousValueAccess,
        node: NodeId,
    ) -> Result<TemporalPreviousValueReference, SignalError> {
        let current_branch = self.graph.current_branch();
        if access.branch_id() != current_branch.id {
            return Err(SignalError::invalid_input(format!(
                "temporal previous-value access belongs to branch {} but current branch is {}",
                access.branch_id().0,
                current_branch.id.0
            )));
        }
        let aspect_version = self.graph.node_aspect_version(node)?;
        let output_identity = self
            .graph
            .observe()
            .runtime_artifact_warm(node)?
            .and_then(|warm| warm.output_identity.clone());
        let reference = self.temporal.capture_previous_value_reference(
            access,
            node,
            aspect_version,
            output_identity,
            &mut self.telemetry.temporal,
        )?;
        self.scratch
            .temporal
            .record_previous_value_reference(reference.clone());
        Ok(reference)
    }

    fn due_tick_for_node_temporal_condition(
        &self,
        condition: &TemporalCondition,
    ) -> Result<Option<ClockTick>, SignalError> {
        let current = self.temporal.clock_basis().current_tick();
        let due = match condition {
            TemporalCondition::AtOrAfter(condition) => {
                if current >= condition.tick() {
                    return Ok(None);
                }
                condition.tick()
            }
            TemporalCondition::After(condition) => {
                ClockTick::new(current.get().saturating_add(condition.delay_ms()))
            }
            TemporalCondition::Debounce(condition) => {
                ClockTick::new(current.get().saturating_add(condition.quiet_period_ms()))
            }
            TemporalCondition::Throttle(condition) => {
                ClockTick::new(current.get().saturating_add(condition.window_ms()))
            }
            TemporalCondition::StaleAfter(condition) => {
                ClockTick::new(current.get().saturating_add(condition.stale_after_ms()))
            }
            TemporalCondition::Interval(interval) => match interval.anchor() {
                crate::data::temporal::IntervalAnchor::ExplicitTick(tick) if *tick > current => {
                    *tick
                }
                crate::data::temporal::IntervalAnchor::ExplicitTick(_)
                | crate::data::temporal::IntervalAnchor::Registration
                | crate::data::temporal::IntervalAnchor::FirstEvaluation => {
                    ClockTick::new(current.get().saturating_add(interval.period_ms()))
                }
            },
        };
        Ok(Some(due))
    }

    fn admit_node_temporal_wake(
        &mut self,
        node: NodeId,
    ) -> Result<Option<ScheduledTemporalWake>, SignalError> {
        if !self.graph.is_alive(node) {
            return Err(SignalError::invalid_input(format!(
                "cannot admit temporal wake for non-live node owner {node}"
            )));
        }
        let EvaluationCondition::Temporal(condition) =
            self.graph.node_eval_config(node)?.condition.clone()
        else {
            return Ok(None);
        };
        let owner = TemporalWakeOwner::Node(node);
        let Some(due_tick) = self.due_tick_for_node_temporal_condition(&condition)? else {
            return Ok(None);
        };
        if let Some(active_wake_id) = self.temporal.active_wake_for_owner(owner) {
            let Some(active) = self.temporal.scheduled_wake(active_wake_id) else {
                if let Some(ready) = self.temporal.ready_wake_for_owner(owner) {
                    if ready.condition() != &condition {
                        let supersession = self.temporal.supersede_wake_with_condition(
                            active_wake_id,
                            condition,
                            due_tick,
                            &mut self.telemetry.temporal,
                        )?;
                        self.scratch
                            .temporal
                            .record_rescheduled_wake(supersession.clone());
                        self.scratch
                            .temporal
                            .record_retired_wake(supersession.retired().clone());
                        self.scratch
                            .temporal
                            .record_scheduled_wake(supersession.scheduled().clone());
                        return Ok(Some(supersession.scheduled().clone()));
                    }
                }
                return Ok(None);
            };
            if active.condition() != &condition {
                let supersession = self.temporal.supersede_wake_with_condition(
                    active_wake_id,
                    condition,
                    due_tick,
                    &mut self.telemetry.temporal,
                )?;
                self.scratch
                    .temporal
                    .record_rescheduled_wake(supersession.clone());
                self.scratch
                    .temporal
                    .record_retired_wake(supersession.retired().clone());
                self.scratch
                    .temporal
                    .record_scheduled_wake(supersession.scheduled().clone());
                return Ok(Some(supersession.scheduled().clone()));
            }
            if matches!(condition, TemporalCondition::Debounce(_))
                && active.due_tick() > self.temporal.clock_basis().current_tick()
                && due_tick > active.due_tick()
            {
                let reschedule = self.temporal.reschedule_wake(
                    active_wake_id,
                    due_tick,
                    &mut self.telemetry.temporal,
                )?;
                self.scratch
                    .temporal
                    .record_rescheduled_wake(reschedule.clone());
                self.scratch
                    .temporal
                    .record_retired_wake(reschedule.retired().clone());
                self.scratch
                    .temporal
                    .record_scheduled_wake(reschedule.scheduled().clone());
            } else {
                let reuse = crate::data::temporal::TemporalWakeReuse::from_scheduled(
                    active,
                    due_tick,
                    self.temporal.clock_basis().current_tick(),
                );
                self.scratch.temporal.record_reused_wake(reuse);
                self.telemetry.temporal.wake_reuse_count += 1;
            }
            return Ok(None);
        }
        let wake = self.temporal.schedule_owned_wake(
            owner,
            condition,
            due_tick,
            &mut self.telemetry.temporal,
        )?;
        self.scratch.temporal.record_scheduled_wake(wake.clone());
        Ok(Some(wake))
    }

    fn admit_temporal_wakes_for_plan(
        &mut self,
        plan: &EvaluationPlan,
    ) -> Result<Vec<ScheduledTemporalWake>, SignalError> {
        let mut scheduled = Vec::new();
        for stage in &plan.stages {
            for task in &stage.tasks {
                if let Some(wake) = self.admit_node_temporal_wake(task.node)? {
                    scheduled.push(wake);
                }
            }
        }
        Ok(scheduled)
    }

    pub(super) fn admit_temporal_wakes_for_nodes(
        &mut self,
        nodes: &[NodeId],
    ) -> Result<Vec<ScheduledTemporalWake>, SignalError> {
        let mut scheduled = Vec::new();
        for node in nodes {
            if let Some(wake) = self.admit_node_temporal_wake(*node)? {
                scheduled.push(wake);
            }
        }
        Ok(scheduled)
    }

    pub(super) fn promote_due_temporal_wakes_ready(&mut self) -> Result<(), SignalError> {
        self.telemetry.temporal.temporal_broad_scan_denial_count += 1;
        loop {
            let frontier = self.temporal.frontier_snapshot();
            let Some(next_due_tick) = frontier.next_due_tick() else {
                break;
            };
            if next_due_tick > self.temporal.clock_basis().current_tick() {
                break;
            }
            let Some(wake_id) = frontier.next_due_wake_id() else {
                return Err(SignalError::internal(
                    "temporal frontier reported due tick without a due wake id",
                ));
            };
            let ready = self
                .temporal
                .promote_wake_ready(wake_id, &mut self.telemetry.temporal)?;
            self.scratch.temporal.record_ready_wake(ready);
        }
        Ok(())
    }

    fn temporal_lowering_context_for_plan(&self, plan: &EvaluationPlan) -> TemporalLoweringContext {
        let mut context = TemporalLoweringContext::runtime_clock_basis(self.temporal.clock_basis());
        for stage in &plan.stages {
            for task in &stage.tasks {
                if let Some(wake) = self
                    .temporal
                    .ready_wake_for_owner(TemporalWakeOwner::Node(task.node))
                    .filter(|wake| {
                        self.graph
                            .node_eval_config(task.node)
                            .is_ok_and(|config| {
                                matches!(&config.condition, EvaluationCondition::Temporal(condition) if wake.condition() == condition)
                            })
                    })
                {
                    context = context.with_ready_wake(wake);
                }
            }
        }
        context
    }

    pub(super) fn temporal_lowering_context_for_nodes(
        &self,
        nodes: &[NodeId],
    ) -> TemporalLoweringContext {
        let mut context = TemporalLoweringContext::runtime_clock_basis(self.temporal.clock_basis());
        for node in nodes {
            if let Some(wake) = self
                .temporal
                .ready_wake_for_owner(TemporalWakeOwner::Node(*node))
                .filter(|wake| {
                    self.graph.node_eval_config(*node).is_ok_and(|config| {
                        matches!(&config.condition, EvaluationCondition::Temporal(condition) if wake.condition() == condition)
                    })
                })
            {
                context = context.with_ready_wake(wake);
            }
        }
        context
    }

    pub(super) fn retire_consumed_temporal_wakes_from_report(
        &mut self,
        report: &ExecutionReport,
    ) -> Result<(), SignalError> {
        let mut consumed = BTreeSet::new();
        for stage in &report.stages {
            for task in &stage.task_records {
                let Some(LoweredTemporalEligibility::Ready(ready)) =
                    task.temporal_eligibility.as_ref()
                else {
                    continue;
                };
                if let Some(wake_id) = ready.wake_id() {
                    consumed.insert(wake_id);
                }
            }
        }
        for wake_id in consumed {
            let Some(owner) = self.temporal.active_wake_owner(wake_id) else {
                continue;
            };
            match self.temporal.ready_wake_for_owner(owner) {
                Some(ready) if matches!(ready.condition(), TemporalCondition::Interval(_)) => {
                    let regeneration = self
                        .temporal
                        .regenerate_interval_wake(wake_id, &mut self.telemetry.temporal)?;
                    self.scratch
                        .temporal
                        .record_interval_regeneration(regeneration.clone());
                    self.scratch
                        .temporal
                        .record_retired_wake(regeneration.retired().clone());
                    self.scratch
                        .temporal
                        .record_scheduled_wake(regeneration.scheduled().clone());
                }
                Some(_) => {
                    let retired = self.temporal.retire_wake(
                        wake_id,
                        TemporalWakeRetirementReason::Consumed,
                        &mut self.telemetry.temporal,
                    )?;
                    self.scratch.temporal.record_retired_wake(retired);
                }
                None => {}
            }
        }
        Ok(())
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

        self.admit_temporal_wakes_for_nodes(targets)?;
        self.promote_due_temporal_wakes_ready()?;
        let temporal_lowering = self.temporal_lowering_context_for_nodes(targets);
        let execution_start = RuntimeInstant::now();
        let report = match execute_targets_with_runtime_config_detailed(
            self.graph,
            self.config,
            temporal_lowering,
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
        self.scratch.temporal.absorb_report(&report);
        self.lower_observation_classifications_from_report(&report)?;
        absorb_execution_report_telemetry(self.telemetry, &report);
        self.retire_consumed_temporal_wakes_from_report(&report)?;
        Ok(report)
    }
}
