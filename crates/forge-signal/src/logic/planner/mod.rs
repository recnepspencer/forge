use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    VersionComparatorPolicy,
};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{EvaluationCondition, NodeState};
use crate::data::output::{IntoNodeEvaluationResult, MemoizedResultOrigin};
use crate::data::trace::TraceSummary;
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::diagnostics::recorder::DiagnosticsRecorder;
use crate::logic::evaluation::{
    apply_prepared_evaluation_with_policy, EvaluationExecutionMetadata, EvaluationRequestMode,
};
use crate::logic::prepared::{ExecutionSnapshot, PreparedDependencyCapture, PreparedEvaluation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskReason {
    Dirty,
    MaybeStaleValidation,
    ConditionForced,
    RequestedTarget,
    DependencyRequired,
    MemoValidation,
    PartitionScopedDependency,
    OutputDiffDependent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageBarrier {
    StageBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionRecordId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationTask {
    pub node: NodeId,
    pub request_mode: EvaluationRequestMode,
    pub direct_request: bool,
    pub reason: TaskReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionStage {
    pub index: u32,
    pub tasks: Vec<EvaluationTask>,
    pub barrier: Option<StageBarrier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanSummary {
    pub requested_target_count: u32,
    pub stage_count: u32,
    pub task_count: u32,
    pub max_stage_width: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationPlan {
    pub request_mode: EvaluationRequestMode,
    pub targets: Vec<NodeId>,
    pub stages: Vec<ExecutionStage>,
    pub summary: PlanSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionPruneReason {
    CleanAtPlanTime,
    CleanAfterValidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskExecutionOutcome {
    Recomputed,
    ValidatedClean,
    ConditionDeferred,
    ConditionRevertedClean,
    MemoizedReuse,
    PropagationSuppressed,
    Pruned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageExecutionOutcome {
    CompletedSerial,
    #[cfg(feature = "parallel")]
    CompletedParallel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskExecutionRecord {
    pub id: ExecutionRecordId,
    pub node: NodeId,
    pub scheduled_reason: TaskReason,
    pub direct_request: bool,
    pub outcome: TaskExecutionOutcome,
    pub prune_reason: Option<ExecutionPruneReason>,
    pub recomputed: bool,
    pub memoized_reuse: bool,
    pub condition_deferred: bool,
    pub condition_reverted_clean: bool,
    pub propagation_suppressed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageExecutionRecord {
    pub stage_index: u32,
    pub outcome: StageExecutionOutcome,
    pub snapshot_duration_nanos: u128,
    pub precompute_duration_nanos: u128,
    pub apply_duration_nanos: u128,
    pub duration_nanos: u128,
    pub task_records: Vec<TaskExecutionRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub plan_summary: PlanSummary,
    pub stage_count: u32,
    pub task_count: u32,
    pub tasks_executed: u32,
    pub tasks_pruned: u32,
    pub tasks_validated_clean: u32,
    pub tasks_deferred_by_condition: u32,
    pub tasks_reverted_clean_by_condition: u32,
    pub tasks_satisfied_by_memoization: u32,
    pub tasks_with_suppressed_propagation: u32,
    pub execution_snapshots_built: u32,
    pub prepared_evaluations_produced: u32,
    pub prepared_evaluations_applied: u32,
    pub dependency_capture_updates: u32,
    pub execution_snapshot_nanos: u128,
    pub stage_precompute_nanos: u128,
    pub stage_apply_nanos: u128,
    pub stages: Vec<StageExecutionRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum StageExecutor {
    #[default]
    Serial,
    #[cfg(feature = "parallel")]
    Parallel { min_stage_width: NonZeroUsize },
}

impl StageExecutor {
    #[cfg(feature = "parallel")]
    pub fn parallel(min_stage_width: usize) -> Self {
        let min_stage_width = match NonZeroUsize::new(min_stage_width.max(1)) {
            Some(width) => width,
            None => unreachable!("parallel min stage width is clamped to at least one"),
        };
        Self::Parallel { min_stage_width }
    }

    #[cfg(feature = "parallel")]
    fn uses_parallel_for_stage(&self, stage: &ExecutionStage) -> bool {
        matches!(
            self,
            Self::Parallel { min_stage_width }
                if stage.tasks.len() >= min_stage_width.get()
        )
    }
}

impl fmt::Display for PlanSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "targets={} stages={} tasks={} max_stage_width={}",
            self.requested_target_count, self.stage_count, self.task_count, self.max_stage_width
        )
    }
}

impl fmt::Display for EvaluationPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "EvaluationPlan {}", self.summary)?;
        for stage in &self.stages {
            writeln!(f, "  stage {} tasks={}", stage.index, stage.tasks.len())?;
            for task in &stage.tasks {
                writeln!(
                    f,
                    "    {} direct={} reason={:?} mode={:?}",
                    task.node, task.direct_request, task.reason, task.request_mode
                )?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for ExecutionReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
        "ExecutionReport stages={} tasks={} executed={} pruned={} validated_clean={} deferred={} memoized={} suppressed={}",
            self.stage_count,
            self.task_count,
            self.tasks_executed,
            self.tasks_pruned,
            self.tasks_validated_clean,
            self.tasks_deferred_by_condition,
            self.tasks_satisfied_by_memoization,
            self.tasks_with_suppressed_propagation
        )?;
        for stage in &self.stages {
            writeln!(
                f,
                "  stage {} outcome={:?} tasks={}",
                stage.stage_index,
                stage.outcome,
                stage.task_records.len()
            )?;
        }
        Ok(())
    }
}

pub fn build_evaluation_plan(
    graph: &SignalGraph,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
) -> Result<EvaluationPlan, SignalError> {
    let mut comparator = DefaultComparatorResolver;
    let mut resolver = DefaultComparatorPolicyResolver {
        fallback: VersionComparatorPolicy::Exact,
        custom: &mut comparator,
    };
    build_evaluation_plan_with_policy_resolver(graph, targets, request_mode, &mut resolver)
}

pub fn build_evaluation_plan_with_policy_resolver(
    graph: &SignalGraph,
    targets: &[NodeId],
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<EvaluationPlan, SignalError> {
    let mut planned = BTreeMap::<NodeId, PlannedNode>::new();
    let mut visiting = BTreeSet::<NodeId>::new();

    let mut deduped_targets = targets.to_vec();
    deduped_targets.sort_by_key(|node| node_sort_key(*node));
    deduped_targets.dedup();

    for &target in &deduped_targets {
        graph.get_entry(target)?;
        visit_node(
            graph,
            target,
            request_mode,
            true,
            TaskReason::RequestedTarget,
            resolver,
            &mut visiting,
            &mut planned,
        )?;
    }

    let depth_cache = compute_depths(graph, &planned)?;
    let max_depth = depth_cache.values().copied().max().unwrap_or(0) as usize;
    let mut stages_by_depth = vec![Vec::<EvaluationTask>::new(); max_depth + 1];
    for (&node, planned_node) in &planned {
        let reason = classify_reason(graph, node, planned_node.direct_request, request_mode)?;
        let task = EvaluationTask {
            node,
            request_mode,
            direct_request: planned_node.direct_request,
            reason,
        };
        let depth = *depth_cache.get(&node).unwrap_or(&0) as usize;
        stages_by_depth[depth].push(task);
    }

    let mut stages = Vec::new();
    for mut tasks in stages_by_depth {
        if tasks.is_empty() {
            continue;
        }
        tasks.sort_by_key(|task| node_sort_key(task.node));
        stages.push(ExecutionStage {
            index: stages.len() as u32,
            tasks,
            barrier: Some(StageBarrier::StageBoundary),
        });
    }

    let summary = PlanSummary {
        requested_target_count: deduped_targets.len() as u32,
        stage_count: stages.len() as u32,
        task_count: stages.iter().map(|stage| stage.tasks.len() as u32).sum(),
        max_stage_width: stages
            .iter()
            .map(|stage| stage.tasks.len() as u32)
            .max()
            .unwrap_or(0),
    };

    Ok(EvaluationPlan {
        request_mode,
        targets: deduped_targets,
        stages,
        summary,
    })
}

pub fn execute_prepared_plan<F>(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    precompute: &F,
) -> Result<ExecutionReport, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let mut comparator = DefaultComparatorResolver;
    let mut resolver = DefaultComparatorPolicyResolver {
        fallback: VersionComparatorPolicy::Exact,
        custom: &mut comparator,
    };
    execute_prepared_plan_with_policy(
        graph,
        plan,
        precompute,
        &mut resolver,
        StageExecutor::Serial,
    )
}

pub fn execute_prepared_plan_with_policy<F>(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    precompute: &F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
) -> Result<ExecutionReport, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    graph.telemetry_mut().plans_built += 1;
    graph.telemetry_mut().stages_built += plan.stages.len() as u64;
    graph.telemetry_mut().tasks_scheduled += plan.summary.task_count as u64;
    graph.telemetry_mut().max_tasks_in_stage = graph
        .telemetry()
        .max_tasks_in_stage
        .max(plan.summary.max_stage_width as u64);
    graph.telemetry_mut().maybe_stale_validation_tasks += plan
        .stages
        .iter()
        .flat_map(|stage| &stage.tasks)
        .filter(|task| matches!(task.reason, TaskReason::MaybeStaleValidation))
        .count() as u64;

    #[cfg(feature = "parallel")]
    match executor {
        StageExecutor::Parallel { .. } => {
            graph.telemetry_mut().parallel_executor_usage_count += 1;
        }
        StageExecutor::Serial => {
            graph.telemetry_mut().serial_executor_usage_count += 1;
        }
    }
    #[cfg(not(feature = "parallel"))]
    {
        let _ = executor;
        graph.telemetry_mut().serial_executor_usage_count += 1;
    }

    let mut next_record_id = 1_u64;
    let mut report = ExecutionReport {
        plan_summary: plan.summary.clone(),
        stage_count: plan.summary.stage_count,
        task_count: plan.summary.task_count,
        tasks_executed: 0,
        tasks_pruned: 0,
        tasks_validated_clean: 0,
        tasks_deferred_by_condition: 0,
        tasks_reverted_clean_by_condition: 0,
        tasks_satisfied_by_memoization: 0,
        tasks_with_suppressed_propagation: 0,
        execution_snapshots_built: 0,
        prepared_evaluations_produced: 0,
        prepared_evaluations_applied: 0,
        dependency_capture_updates: 0,
        execution_snapshot_nanos: 0,
        stage_precompute_nanos: 0,
        stage_apply_nanos: 0,
        stages: Vec::new(),
    };

    for stage in &plan.stages {
        let stage_start = Instant::now();
        let snapshot_start = Instant::now();
        graph.telemetry_mut().execution_snapshots_built += 1;
        let precompute_start = Instant::now();
        let prepared = {
            let snapshot = ExecutionSnapshot::new(&*graph);
            let prepared = match executor {
                StageExecutor::Serial => precompute_stage_serial(stage, &snapshot, precompute),
                #[cfg(feature = "parallel")]
                _ if executor.uses_parallel_for_stage(stage) => {
                    precompute_stage_parallel(stage, &snapshot, precompute)
                }
                #[cfg(feature = "parallel")]
                StageExecutor::Parallel { .. } => {
                    precompute_stage_serial(stage, &snapshot, precompute)
                }
            };
            prepared
        }
        .map_err(|err| {
            record_execution_failure(
                graph,
                ExecutionFailureContext::new(
                    ExecutionFailurePhase::Precompute,
                    Some(stage.index),
                    None,
                    Some(executor),
                    None,
                    Some(plan.summary.clone()),
                    err.to_string(),
                ),
            );
            err
        })?;
        let snapshot_nanos = snapshot_start.elapsed().as_nanos();
        graph.telemetry_mut().execution_snapshot_nanos += snapshot_nanos;
        report.execution_snapshots_built += 1;
        report.execution_snapshot_nanos += snapshot_nanos;
        let precompute_nanos = precompute_start.elapsed().as_nanos();
        graph.telemetry_mut().stage_precompute_nanos += precompute_nanos;
        graph.telemetry_mut().prepared_evaluations_produced += prepared.len() as u64;
        report.prepared_evaluations_produced += prepared.len() as u32;
        report.stage_precompute_nanos += precompute_nanos;
        match executor {
            StageExecutor::Serial => {
                graph.telemetry_mut().serial_precompute_task_count += prepared.len() as u64;
            }
            #[cfg(feature = "parallel")]
            _ if executor.uses_parallel_for_stage(stage) => {
                graph.telemetry_mut().parallel_stage_dispatch_count += 1;
                graph.telemetry_mut().parallel_precompute_task_count += prepared.len() as u64;
            }
            #[cfg(feature = "parallel")]
            StageExecutor::Parallel { .. } => {
                graph.telemetry_mut().serial_precompute_task_count += prepared.len() as u64;
            }
        }

        let apply_start = Instant::now();
        let mut stage_record = StageExecutionRecord {
            stage_index: stage.index,
            outcome: {
                #[cfg(feature = "parallel")]
                {
                    if executor.uses_parallel_for_stage(stage) {
                        StageExecutionOutcome::CompletedParallel
                    } else {
                        StageExecutionOutcome::CompletedSerial
                    }
                }
                #[cfg(not(feature = "parallel"))]
                {
                    StageExecutionOutcome::CompletedSerial
                }
            },
            snapshot_duration_nanos: snapshot_nanos,
            precompute_duration_nanos: precompute_nanos,
            apply_duration_nanos: 0,
            duration_nanos: 0,
            task_records: Vec::new(),
        };

        for (task, prepared) in stage.tasks.iter().zip(prepared.into_iter()) {
            let record_id = ExecutionRecordId(next_record_id);
            next_record_id += 1;
            let before_state = graph.get_state(task.node)?;
            let before_trace = graph.get_entry(task.node)?.get_trace_summary().cloned();
            let dependency_updates = apply_prepared_evaluation_with_policy(
                graph,
                task.node,
                prepared,
                comparator_resolver,
                None,
            )
            .map_err(|err| {
                record_execution_failure(
                    graph,
                    ExecutionFailureContext::new(
                        ExecutionFailurePhase::Apply,
                        Some(stage.index),
                        Some(task.node),
                        Some(executor),
                        Some(record_id),
                        Some(plan.summary.clone()),
                        err.to_string(),
                    ),
                );
                err
            })?;
            if let Some(summary) = graph
                .get_entry_mut(task.node)?
                .get_trace_summary()
                .cloned()
                .as_mut()
            {
                let mut updated = summary.clone();
                updated.execution_record_id = Some(record_id.0);
                graph
                    .get_entry_mut(task.node)?
                    .set_trace_summary(Some(updated));
            }
            let after_state = graph.get_state(task.node)?;
            let after_trace = graph.get_entry(task.node)?.get_trace_summary().cloned();
            let task_record = classify_task_record(
                record_id,
                task,
                before_state,
                after_state,
                before_trace.as_ref(),
                after_trace.as_ref(),
            );
            accumulate_report_counters(&mut report, &task_record);
            stage_record.task_records.push(task_record);
            graph.telemetry_mut().prepared_evaluations_applied += 1;
            graph.telemetry_mut().dependency_capture_updates += dependency_updates as u64;
            report.prepared_evaluations_applied += 1;
            report.dependency_capture_updates += dependency_updates;
        }
        stage_record.apply_duration_nanos = apply_start.elapsed().as_nanos();
        report.stage_apply_nanos += stage_record.apply_duration_nanos;
        graph.telemetry_mut().stage_apply_nanos += stage_record.apply_duration_nanos;

        stage_record.duration_nanos = stage_start.elapsed().as_nanos();
        graph.telemetry_mut().stage_execution_count += 1;
        graph.telemetry_mut().stage_execution_nanos += stage_record.duration_nanos;
        report.stages.push(stage_record);
    }

    record_successful_execution(graph, plan, &report);
    Ok(report)
}

#[cfg(test)]
pub fn execute_plan_with_policy_and_condition<F, O>(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    compute: &mut F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    condition_resolver: &mut impl crate::logic::evaluation::ConditionResolver,
    executor: StageExecutor,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
) -> Result<ExecutionReport, SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
{
    match executor {
        StageExecutor::Serial => execute_plan_serial(
            graph,
            plan,
            compute,
            comparator_resolver,
            condition_resolver,
            execution_metadata,
        ),
        #[cfg(feature = "parallel")]
        StageExecutor::Parallel { .. } => Err(SignalError::invalid_input(
            "parallel stage execution is not yet supported by the current mutable graph engine",
        )),
    }
}

#[cfg(test)]
pub(crate) fn execute_test_prepared_plan_with_resolvers<F>(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    precompute: &F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    condition_resolver: &mut impl crate::logic::evaluation::ConditionResolver,
) -> Result<ExecutionReport, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    graph.telemetry_mut().plans_built += 1;
    graph.telemetry_mut().stages_built += plan.stages.len() as u64;
    graph.telemetry_mut().tasks_scheduled += plan.summary.task_count as u64;
    graph.telemetry_mut().max_tasks_in_stage = graph
        .telemetry()
        .max_tasks_in_stage
        .max(plan.summary.max_stage_width as u64);
    graph.telemetry_mut().serial_executor_usage_count += 1;
    graph.telemetry_mut().maybe_stale_validation_tasks += plan
        .stages
        .iter()
        .flat_map(|stage| &stage.tasks)
        .filter(|task| matches!(task.reason, TaskReason::MaybeStaleValidation))
        .count() as u64;

    let mut next_record_id = 1_u64;
    let mut report = ExecutionReport {
        plan_summary: plan.summary.clone(),
        stage_count: plan.summary.stage_count,
        task_count: plan.summary.task_count,
        tasks_executed: 0,
        tasks_pruned: 0,
        tasks_validated_clean: 0,
        tasks_deferred_by_condition: 0,
        tasks_reverted_clean_by_condition: 0,
        tasks_satisfied_by_memoization: 0,
        tasks_with_suppressed_propagation: 0,
        execution_snapshots_built: 0,
        prepared_evaluations_produced: 0,
        prepared_evaluations_applied: 0,
        dependency_capture_updates: 0,
        execution_snapshot_nanos: 0,
        stage_precompute_nanos: 0,
        stage_apply_nanos: 0,
        stages: Vec::new(),
    };

    for stage in &plan.stages {
        let stage_start = Instant::now();
        let snapshot_start = Instant::now();
        graph.telemetry_mut().execution_snapshots_built += 1;
        let mut prepared_tasks = Vec::with_capacity(stage.tasks.len());
        let mut precompute_telemetry = TestPrecomputeTelemetry::default();
        let precompute_start = Instant::now();
        {
            let snapshot = ExecutionSnapshot::new(&*graph);
            for task in &stage.tasks {
                let prepared = prepare_test_precomputed_task(
                    &snapshot,
                    task.node,
                    precompute,
                    comparator_resolver,
                    condition_resolver,
                    task.request_mode,
                )?;
                precompute_telemetry.accumulate(&prepared.telemetry);
                prepared_tasks.push(prepared.prepared);
            }
        }
        apply_test_precompute_telemetry(graph, &precompute_telemetry);
        let snapshot_nanos = snapshot_start.elapsed().as_nanos();
        let precompute_nanos = precompute_start.elapsed().as_nanos();
        graph.telemetry_mut().execution_snapshot_nanos += snapshot_nanos;
        graph.telemetry_mut().stage_precompute_nanos += precompute_nanos;
        graph.telemetry_mut().prepared_evaluations_produced += prepared_tasks.len() as u64;
        graph.telemetry_mut().serial_precompute_task_count += prepared_tasks.len() as u64;
        report.execution_snapshots_built += 1;
        report.execution_snapshot_nanos += snapshot_nanos;
        report.prepared_evaluations_produced += prepared_tasks.len() as u32;
        report.stage_precompute_nanos += precompute_nanos;

        let apply_start = Instant::now();
        let mut stage_record = StageExecutionRecord {
            stage_index: stage.index,
            outcome: StageExecutionOutcome::CompletedSerial,
            snapshot_duration_nanos: snapshot_nanos,
            precompute_duration_nanos: precompute_nanos,
            apply_duration_nanos: 0,
            duration_nanos: 0,
            task_records: Vec::new(),
        };

        for (task, prepared) in stage.tasks.iter().zip(prepared_tasks.into_iter()) {
            let record_id = ExecutionRecordId(next_record_id);
            next_record_id += 1;
            let before_state = graph.get_state(task.node)?;
            let before_trace = graph.get_entry(task.node)?.get_trace_summary().cloned();
            let dependency_updates = apply_prepared_evaluation_with_policy(
                graph,
                task.node,
                prepared,
                comparator_resolver,
                None,
            )?;
            if let Some(summary) = graph
                .get_entry_mut(task.node)?
                .get_trace_summary()
                .cloned()
                .as_mut()
            {
                let mut updated = summary.clone();
                updated.execution_record_id = Some(record_id.0);
                graph
                    .get_entry_mut(task.node)?
                    .set_trace_summary(Some(updated));
            }
            let after_state = graph.get_state(task.node)?;
            let after_trace = graph.get_entry(task.node)?.get_trace_summary().cloned();
            let task_record = classify_task_record(
                record_id,
                task,
                before_state,
                after_state,
                before_trace.as_ref(),
                after_trace.as_ref(),
            );
            accumulate_report_counters(&mut report, &task_record);
            stage_record.task_records.push(task_record);
            graph.telemetry_mut().prepared_evaluations_applied += 1;
            graph.telemetry_mut().dependency_capture_updates += dependency_updates as u64;
            report.prepared_evaluations_applied += 1;
            report.dependency_capture_updates += dependency_updates;
        }

        stage_record.apply_duration_nanos = apply_start.elapsed().as_nanos();
        stage_record.duration_nanos = stage_start.elapsed().as_nanos();
        report.stage_apply_nanos += stage_record.apply_duration_nanos;
        graph.telemetry_mut().stage_apply_nanos += stage_record.apply_duration_nanos;
        graph.telemetry_mut().stage_execution_count += 1;
        graph.telemetry_mut().stage_execution_nanos += stage_record.duration_nanos;
        report.stages.push(stage_record);
    }

    record_successful_execution(graph, plan, &report);
    Ok(report)
}

#[cfg(test)]
fn execute_plan_serial<F, O>(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    compute: &mut F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    condition_resolver: &mut impl crate::logic::evaluation::ConditionResolver,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
) -> Result<ExecutionReport, SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
{
    graph.telemetry_mut().plans_built += 1;
    graph.telemetry_mut().stages_built += plan.stages.len() as u64;
    graph.telemetry_mut().tasks_scheduled += plan.summary.task_count as u64;
    graph.telemetry_mut().max_tasks_in_stage = graph
        .telemetry()
        .max_tasks_in_stage
        .max(plan.summary.max_stage_width as u64);
    graph.telemetry_mut().serial_executor_usage_count += 1;
    graph.telemetry_mut().evaluation_calls += 1;
    graph.telemetry_mut().evaluation_stack_peak = graph
        .telemetry()
        .evaluation_stack_peak
        .max(plan.summary.task_count as u64);
    graph.telemetry_mut().maybe_stale_validation_tasks += plan
        .stages
        .iter()
        .flat_map(|stage| &stage.tasks)
        .filter(|task| matches!(task.reason, TaskReason::MaybeStaleValidation))
        .count() as u64;

    let mut next_record_id = 1_u64;
    let mut report = ExecutionReport {
        plan_summary: plan.summary.clone(),
        stage_count: plan.summary.stage_count,
        task_count: plan.summary.task_count,
        tasks_executed: 0,
        tasks_pruned: 0,
        tasks_validated_clean: 0,
        tasks_deferred_by_condition: 0,
        tasks_reverted_clean_by_condition: 0,
        tasks_satisfied_by_memoization: 0,
        tasks_with_suppressed_propagation: 0,
        execution_snapshots_built: 0,
        prepared_evaluations_produced: 0,
        prepared_evaluations_applied: 0,
        dependency_capture_updates: 0,
        execution_snapshot_nanos: 0,
        stage_precompute_nanos: 0,
        stage_apply_nanos: 0,
        stages: Vec::new(),
    };

    for stage in &plan.stages {
        let stage_start = Instant::now();
        let snapshot_start = Instant::now();
        graph.telemetry_mut().execution_snapshots_built += 1;
        let mut prepared_tasks = Vec::with_capacity(stage.tasks.len());
        let mut precompute_telemetry = TestPrecomputeTelemetry::default();
        let precompute_start = Instant::now();
        {
            let snapshot = ExecutionSnapshot::new(&*graph);
            for task in &stage.tasks {
                let prepared = prepare_test_task(
                    snapshot.graph(),
                    task.node,
                    compute,
                    comparator_resolver,
                    condition_resolver,
                    task.request_mode,
                )?;
                precompute_telemetry.accumulate(&prepared.telemetry);
                prepared_tasks.push(prepared.prepared);
            }
        }
        apply_test_precompute_telemetry(graph, &precompute_telemetry);
        let snapshot_nanos = snapshot_start.elapsed().as_nanos();
        let precompute_nanos = precompute_start.elapsed().as_nanos();
        graph.telemetry_mut().execution_snapshot_nanos += snapshot_nanos;
        graph.telemetry_mut().stage_precompute_nanos += precompute_nanos;
        graph.telemetry_mut().prepared_evaluations_produced += prepared_tasks.len() as u64;
        graph.telemetry_mut().serial_precompute_task_count += prepared_tasks.len() as u64;
        report.execution_snapshots_built += 1;
        report.execution_snapshot_nanos += snapshot_nanos;
        report.prepared_evaluations_produced += prepared_tasks.len() as u32;
        report.stage_precompute_nanos += precompute_nanos;

        let apply_start = Instant::now();
        let mut stage_record = StageExecutionRecord {
            stage_index: stage.index,
            outcome: StageExecutionOutcome::CompletedSerial,
            snapshot_duration_nanos: snapshot_nanos,
            precompute_duration_nanos: precompute_nanos,
            apply_duration_nanos: 0,
            duration_nanos: 0,
            task_records: Vec::new(),
        };

        for (task, prepared) in stage.tasks.iter().zip(prepared_tasks.into_iter()) {
            let record_id = ExecutionRecordId(next_record_id);
            next_record_id += 1;
            let before_state = graph.get_state(task.node)?;
            let before_trace = graph.get_entry(task.node)?.get_trace_summary().cloned();
            let dependency_updates = apply_prepared_evaluation_with_policy(
                graph,
                task.node,
                prepared,
                comparator_resolver,
                execution_metadata.filter(|_| task.direct_request),
            )?;
            if let Some(summary) = graph
                .get_entry_mut(task.node)?
                .get_trace_summary()
                .cloned()
                .as_mut()
            {
                let mut updated = summary.clone();
                updated.execution_record_id = Some(record_id.0);
                graph
                    .get_entry_mut(task.node)?
                    .set_trace_summary(Some(updated));
            }
            let after_state = graph.get_state(task.node)?;
            let after_trace = graph.get_entry(task.node)?.get_trace_summary().cloned();
            let task_record = classify_task_record(
                record_id,
                task,
                before_state,
                after_state,
                before_trace.as_ref(),
                after_trace.as_ref(),
            );
            accumulate_report_counters(&mut report, &task_record);
            stage_record.task_records.push(task_record);
            graph.telemetry_mut().prepared_evaluations_applied += 1;
            graph.telemetry_mut().dependency_capture_updates += dependency_updates as u64;
            report.prepared_evaluations_applied += 1;
            report.dependency_capture_updates += dependency_updates;
        }

        stage_record.apply_duration_nanos = apply_start.elapsed().as_nanos();
        stage_record.duration_nanos = stage_start.elapsed().as_nanos();
        report.stage_apply_nanos += stage_record.apply_duration_nanos;
        graph.telemetry_mut().stage_apply_nanos += stage_record.apply_duration_nanos;
        graph.telemetry_mut().stage_execution_count += 1;
        graph.telemetry_mut().stage_execution_nanos += stage_record.duration_nanos;
        report.stages.push(stage_record);
    }

    record_successful_execution(graph, plan, &report);
    Ok(report)
}

#[cfg(test)]
fn prepare_test_precomputed_task<F>(
    snapshot: &ExecutionSnapshot<'_>,
    node: NodeId,
    precompute: &F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    condition_resolver: &mut impl crate::logic::evaluation::ConditionResolver,
    request_mode: EvaluationRequestMode,
) -> Result<TestPreparedTask, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let mut telemetry = TestPrecomputeTelemetry::default();
    let graph = snapshot.graph();
    let state = *graph.get_entry(node)?.get_state();
    let dependencies = capture_current_dependencies(graph, node)?;

    if matches!(state, NodeState::MaybeStale) {
        let preview = preview_upstream_state(graph, node, comparator_resolver)?;
        telemetry.partition_scope_revert_clean_count = preview.partition_scope_revert_clean_count;
        if preview.unchanged {
            return Ok(TestPreparedTask {
                prepared: PreparedEvaluation::validated_clean().with_dependencies(dependencies),
                telemetry,
            });
        }
    }

    telemetry.nodes_evaluated += 1;
    match preview_condition_action(graph, node, request_mode, condition_resolver)? {
        TestConditionAction::Evaluate => {
            let view = snapshot.read_view(node);
            let prepared = precompute(node, &view)?;
            Ok(TestPreparedTask {
                prepared,
                telemetry,
            })
        }
        TestConditionAction::RevertClean => {
            telemetry.condition_skip_count += 1;
            Ok(TestPreparedTask {
                prepared: PreparedEvaluation::reverted_clean_by_condition()
                    .with_dependencies(dependencies),
                telemetry,
            })
        }
        TestConditionAction::Defer {
            on_demand,
            debounce,
        } => {
            telemetry.condition_skip_count += 1;
            telemetry.ondemand_deferred_count += u64::from(on_demand);
            telemetry.debounce_deferred_count += u64::from(debounce);
            Ok(TestPreparedTask {
                prepared: PreparedEvaluation::deferred_by_condition()
                    .with_dependencies(dependencies),
                telemetry,
            })
        }
    }
}

#[cfg(test)]
struct TestPreparedTask {
    prepared: PreparedEvaluation,
    telemetry: TestPrecomputeTelemetry,
}

#[cfg(test)]
#[derive(Default)]
struct TestPrecomputeTelemetry {
    nodes_evaluated: u64,
    condition_skip_count: u64,
    ondemand_deferred_count: u64,
    debounce_deferred_count: u64,
    partition_scope_revert_clean_count: u64,
}

#[cfg(test)]
impl TestPrecomputeTelemetry {
    fn accumulate(&mut self, other: &Self) {
        self.nodes_evaluated += other.nodes_evaluated;
        self.condition_skip_count += other.condition_skip_count;
        self.ondemand_deferred_count += other.ondemand_deferred_count;
        self.debounce_deferred_count += other.debounce_deferred_count;
        self.partition_scope_revert_clean_count += other.partition_scope_revert_clean_count;
    }
}

#[cfg(test)]
fn prepare_test_task<F, O>(
    graph: &SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    condition_resolver: &mut impl crate::logic::evaluation::ConditionResolver,
    request_mode: EvaluationRequestMode,
) -> Result<TestPreparedTask, SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
{
    let mut telemetry = TestPrecomputeTelemetry::default();
    let state = *graph.get_entry(node)?.get_state();
    let dependencies = capture_current_dependencies(graph, node)?;

    if matches!(state, NodeState::MaybeStale) {
        let preview = preview_upstream_state(graph, node, comparator_resolver)?;
        telemetry.partition_scope_revert_clean_count = preview.partition_scope_revert_clean_count;
        if preview.unchanged {
            return Ok(TestPreparedTask {
                prepared: PreparedEvaluation::validated_clean().with_dependencies(dependencies),
                telemetry,
            });
        }
    }

    telemetry.nodes_evaluated += 1;
    match preview_condition_action(graph, node, request_mode, condition_resolver)? {
        TestConditionAction::Evaluate => {
            let result = compute(node, graph)?.into_evaluation_result();
            Ok(TestPreparedTask {
                prepared: PreparedEvaluation::from_result(result).with_dependencies(dependencies),
                telemetry,
            })
        }
        TestConditionAction::RevertClean => {
            telemetry.condition_skip_count += 1;
            Ok(TestPreparedTask {
                prepared: PreparedEvaluation::reverted_clean_by_condition()
                    .with_dependencies(dependencies),
                telemetry,
            })
        }
        TestConditionAction::Defer {
            on_demand,
            debounce,
        } => {
            telemetry.condition_skip_count += 1;
            telemetry.ondemand_deferred_count += u64::from(on_demand);
            telemetry.debounce_deferred_count += u64::from(debounce);
            Ok(TestPreparedTask {
                prepared: PreparedEvaluation::deferred_by_condition()
                    .with_dependencies(dependencies),
                telemetry,
            })
        }
    }
}

#[cfg(test)]
fn apply_test_precompute_telemetry(graph: &mut SignalGraph, telemetry: &TestPrecomputeTelemetry) {
    graph.telemetry_mut().nodes_evaluated += telemetry.nodes_evaluated;
    graph.telemetry_mut().condition_skip_count += telemetry.condition_skip_count;
    graph.telemetry_mut().ondemand_deferred_count += telemetry.ondemand_deferred_count;
    graph.telemetry_mut().debounce_deferred_count += telemetry.debounce_deferred_count;
    graph.telemetry_mut().partition_scope_revert_clean_count +=
        telemetry.partition_scope_revert_clean_count;
}

#[cfg(test)]
enum TestConditionAction {
    Evaluate,
    RevertClean,
    Defer { on_demand: bool, debounce: bool },
}

#[cfg(test)]
fn preview_condition_action(
    graph: &SignalGraph,
    node: NodeId,
    request_mode: EvaluationRequestMode,
    resolver: &mut impl crate::logic::evaluation::ConditionResolver,
) -> Result<TestConditionAction, SignalError> {
    let entry = graph.get_entry(node)?;
    let dirty_aspects = entry.get_dirty_aspects();
    let max_dependency_delta = max_dependency_delta(graph, node)?;
    let ctx = crate::logic::evaluation::ConditionEvaluationContext {
        node,
        request_mode,
        dirty_aspects,
        max_dependency_delta,
    };

    match &entry.get_eval_config().condition {
        EvaluationCondition::Always => Ok(TestConditionAction::Evaluate),
        EvaluationCondition::AspectFilter(mask) => {
            if dirty_aspects.is_empty() || dirty_aspects.intersects(*mask) {
                Ok(TestConditionAction::Evaluate)
            } else {
                Ok(TestConditionAction::Defer {
                    on_demand: false,
                    debounce: false,
                })
            }
        }
        EvaluationCondition::OnDemand => match request_mode {
            EvaluationRequestMode::Default => Ok(TestConditionAction::Defer {
                on_demand: true,
                debounce: false,
            }),
            EvaluationRequestMode::ForceOnDemand => Ok(TestConditionAction::Evaluate),
        },
        EvaluationCondition::DeltaThreshold(threshold) => {
            if dirty_aspects.is_empty() || (max_dependency_delta as f64) > *threshold {
                Ok(TestConditionAction::Evaluate)
            } else {
                Ok(TestConditionAction::RevertClean)
            }
        }
        EvaluationCondition::Debounce(quiet_period_ms) => {
            if resolver.debounce_ready(*quiet_period_ms, &ctx)? {
                Ok(TestConditionAction::Evaluate)
            } else {
                Ok(TestConditionAction::Defer {
                    on_demand: false,
                    debounce: true,
                })
            }
        }
        EvaluationCondition::Custom(key) => {
            if resolver.resolve_custom(key, &ctx)? {
                Ok(TestConditionAction::Evaluate)
            } else {
                Ok(TestConditionAction::Defer {
                    on_demand: false,
                    debounce: false,
                })
            }
        }
    }
}

#[cfg(test)]
fn max_dependency_delta(graph: &SignalGraph, node: NodeId) -> Result<u64, SignalError> {
    let mut max_delta = 0;
    for (source, aspect, cached_version, _) in graph.get_dep_snapshot(node)?.entries() {
        if !graph.is_alive(*source) {
            continue;
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        max_delta = max_delta.max(current_version.abs_diff(*cached_version));
    }
    Ok(max_delta)
}

#[cfg(test)]
fn capture_current_dependencies(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<PreparedDependencyCapture, SignalError> {
    let mut capture = PreparedDependencyCapture::new();
    for dependency in graph.dependencies_of(node)? {
        capture.record(
            dependency.source(),
            dependency.aspect(),
            dependency.scope_ref().cloned(),
        );
    }
    Ok(capture.into_sorted_unique())
}

#[cfg(test)]
struct UpstreamPreview {
    unchanged: bool,
    partition_scope_revert_clean_count: u64,
}

#[cfg(test)]
fn preview_upstream_state(
    graph: &SignalGraph,
    node: NodeId,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<UpstreamPreview, SignalError> {
    let entry = graph.get_entry(node)?;
    let snapshot = graph.get_dep_snapshot(node)?;
    let comparator = resolver.policy_for_node(node, entry.get_eval_config().comparator.as_ref());
    let mut partition_scope_revert_clean_count = 0;

    for (source, aspect, cached_version, scope) in snapshot.entries() {
        if !graph.is_alive(*source) {
            return Ok(UpstreamPreview {
                unchanged: false,
                partition_scope_revert_clean_count,
            });
        }
        if !matches!(graph.get_entry(*source)?.get_state(), NodeState::Clean) {
            return Ok(UpstreamPreview {
                unchanged: false,
                partition_scope_revert_clean_count,
            });
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        if let Some(scope) = scope {
            if current_version == *cached_version {
                continue;
            }
            if partition_scope_untouched(graph.get_entry(*source)?.get_trace_summary(), scope) {
                partition_scope_revert_clean_count += 1;
                continue;
            }
            return Ok(UpstreamPreview {
                unchanged: false,
                partition_scope_revert_clean_count,
            });
        }
        if comparator.has_meaningful_change(*aspect, *cached_version, current_version, resolver)? {
            return Ok(UpstreamPreview {
                unchanged: false,
                partition_scope_revert_clean_count,
            });
        }
    }

    Ok(UpstreamPreview {
        unchanged: true,
        partition_scope_revert_clean_count,
    })
}

fn precompute_stage_serial<F>(
    stage: &ExecutionStage,
    snapshot: &ExecutionSnapshot<'_>,
    precompute: &F,
) -> Result<Vec<PreparedEvaluation>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let mut prepared = Vec::with_capacity(stage.tasks.len());
    for task in &stage.tasks {
        let view = snapshot.read_view(task.node);
        prepared.push(precompute(task.node, &view)?);
    }
    Ok(prepared)
}

#[cfg(feature = "parallel")]
fn precompute_stage_parallel<F>(
    stage: &ExecutionStage,
    snapshot: &ExecutionSnapshot<'_>,
    precompute: &F,
) -> Result<Vec<PreparedEvaluation>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(stage.tasks.len());
        for task in &stage.tasks {
            let node = task.node;
            handles.push(scope.spawn(move || {
                let view = snapshot.read_view(node);
                precompute(node, &view)
            }));
        }

        let mut prepared = Vec::with_capacity(stage.tasks.len());
        for handle in handles {
            prepared.push(
                handle
                    .join()
                    .map_err(|_| SignalError::internal("parallel stage worker panicked"))??,
            );
        }
        Ok(prepared)
    })
}

fn classify_task_record(
    id: ExecutionRecordId,
    task: &EvaluationTask,
    before_state: NodeState,
    after_state: NodeState,
    before_trace: Option<&TraceSummary>,
    after_trace: Option<&TraceSummary>,
) -> TaskExecutionRecord {
    let trace_changed = before_trace != after_trace;
    let recomputed = after_trace.map(|trace| trace.recomputed).unwrap_or(false);
    let memoized_reuse = after_trace
        .map(|trace| trace.memoized_origin == MemoizedResultOrigin::MemoizedFromCache)
        .unwrap_or(false);
    let propagation_suppressed = after_trace
        .map(|trace| trace.propagation_suppressed)
        .unwrap_or(false);

    let (outcome, prune_reason, condition_deferred, condition_reverted_clean) =
        match (before_state, after_state) {
            (NodeState::Clean, NodeState::Clean) => (
                TaskExecutionOutcome::Pruned,
                Some(ExecutionPruneReason::CleanAtPlanTime),
                false,
                false,
            ),
            (NodeState::MaybeStale, NodeState::Clean) if !trace_changed => (
                TaskExecutionOutcome::ValidatedClean,
                Some(ExecutionPruneReason::CleanAfterValidation),
                false,
                false,
            ),
            (_, NodeState::MaybeStale) => {
                (TaskExecutionOutcome::ConditionDeferred, None, true, false)
            }
            (_, NodeState::Clean) if memoized_reuse => {
                (TaskExecutionOutcome::MemoizedReuse, None, false, false)
            }
            (_, NodeState::Clean) if propagation_suppressed => (
                TaskExecutionOutcome::PropagationSuppressed,
                None,
                false,
                false,
            ),
            (_, NodeState::Clean) if recomputed => {
                (TaskExecutionOutcome::Recomputed, None, false, false)
            }
            (_, NodeState::Clean) => (
                TaskExecutionOutcome::ConditionRevertedClean,
                None,
                false,
                true,
            ),
            _ => (TaskExecutionOutcome::Recomputed, None, false, false),
        };

    TaskExecutionRecord {
        id,
        node: task.node,
        scheduled_reason: task.reason,
        direct_request: task.direct_request,
        outcome,
        prune_reason,
        recomputed,
        memoized_reuse,
        condition_deferred,
        condition_reverted_clean,
        propagation_suppressed,
    }
}

fn record_successful_execution(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    report: &ExecutionReport,
) {
    DiagnosticsRecorder::new(graph).record_execution_completed(plan, report);
}

fn record_execution_failure(graph: &mut SignalGraph, context: ExecutionFailureContext) {
    DiagnosticsRecorder::new(graph).record_failure(context);
}

fn accumulate_report_counters(report: &mut ExecutionReport, task_record: &TaskExecutionRecord) {
    match task_record.outcome {
        TaskExecutionOutcome::Recomputed | TaskExecutionOutcome::PropagationSuppressed => {
            report.tasks_executed += 1;
        }
        TaskExecutionOutcome::ValidatedClean => {
            report.tasks_validated_clean += 1;
            report.tasks_pruned += 1;
        }
        TaskExecutionOutcome::ConditionDeferred => {
            report.tasks_deferred_by_condition += 1;
        }
        TaskExecutionOutcome::ConditionRevertedClean => {
            report.tasks_reverted_clean_by_condition += 1;
        }
        TaskExecutionOutcome::MemoizedReuse => {
            report.tasks_satisfied_by_memoization += 1;
        }
        TaskExecutionOutcome::Pruned => {
            report.tasks_pruned += 1;
        }
    }
    if task_record.propagation_suppressed {
        report.tasks_with_suppressed_propagation += 1;
    }
}

#[derive(Debug, Clone, Copy)]
struct PlannedNode {
    direct_request: bool,
}

fn visit_node(
    graph: &SignalGraph,
    node: NodeId,
    request_mode: EvaluationRequestMode,
    direct_request: bool,
    _reason: TaskReason,
    resolver: &mut impl ComparatorPolicyResolver,
    visiting: &mut BTreeSet<NodeId>,
    planned: &mut BTreeMap<NodeId, PlannedNode>,
) -> Result<(), SignalError> {
    if !visiting.insert(node) {
        return Err(SignalError::invalid_input(format!(
            "Circular reference detected while planning signal node: {}",
            node
        )));
    }

    let state = *graph.get_entry(node)?.get_state();
    let preview_clean = matches!(state, NodeState::MaybeStale)
        && preview_upstream_state(graph, node, resolver)?.unchanged;
    let validation_only = matches!(state, NodeState::MaybeStale) && direct_request && preview_clean;
    let needs_execution = match state {
        NodeState::Clean => false,
        NodeState::Dirty => true,
        NodeState::MaybeStale => direct_request || !preview_clean,
    };

    if needs_execution {
        if !validation_only {
            let deps = sorted_dependencies(graph, node)?;
            for dep in deps {
                visit_node(
                    graph,
                    dep.source(),
                    request_mode,
                    false,
                    if dep.scope_ref().is_some() {
                        TaskReason::PartitionScopedDependency
                    } else {
                        TaskReason::DependencyRequired
                    },
                    resolver,
                    visiting,
                    planned,
                )?;
            }
        }
        planned
            .entry(node)
            .and_modify(|current| current.direct_request |= direct_request)
            .or_insert(PlannedNode { direct_request });
    } else if direct_request && matches!(state, NodeState::Clean) {
        // requested clean targets intentionally remain omitted from the plan
    }

    visiting.remove(&node);
    let _ = request_mode;
    Ok(())
}

fn sorted_dependencies(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<Vec<crate::data::dependency::DependencyEdge>, SignalError> {
    let mut deps = graph.dependencies_of(node)?.to_vec();
    deps.sort_by_key(|dep| (node_sort_key(dep.source()), dep.aspect().index()));
    Ok(deps)
}

fn compute_depths(
    graph: &SignalGraph,
    planned: &BTreeMap<NodeId, PlannedNode>,
) -> Result<BTreeMap<NodeId, u32>, SignalError> {
    let planned_ids: BTreeSet<NodeId> = planned.keys().copied().collect();
    let mut indegree = BTreeMap::<NodeId, u32>::new();
    let mut downstreams = BTreeMap::<NodeId, Vec<NodeId>>::new();
    let mut depths = BTreeMap::<NodeId, u32>::new();

    for &node in planned.keys() {
        indegree.insert(node, 0);
        depths.insert(node, 0);
    }

    for &node in planned.keys() {
        for dep in graph.dependencies_of(node)? {
            if !planned_ids.contains(&dep.source()) {
                continue;
            }
            *indegree.entry(node).or_insert(0) += 1;
            downstreams.entry(dep.source()).or_default().push(node);
        }
    }

    for nodes in downstreams.values_mut() {
        nodes.sort_by_key(|node| node_sort_key(*node));
    }

    let mut ready = indegree
        .iter()
        .filter_map(|(&node, &degree)| (degree == 0).then_some(node))
        .collect::<Vec<_>>();
    ready.sort_by_key(|node| node_sort_key(*node));

    let mut cursor = 0;
    let mut visited = 0_usize;
    while cursor < ready.len() {
        let node = ready[cursor];
        cursor += 1;
        visited += 1;
        let node_depth = *depths.get(&node).unwrap_or(&0);
        if let Some(children) = downstreams.get(&node) {
            for &child in children {
                let child_depth = depths.entry(child).or_insert(0);
                *child_depth = (*child_depth).max(node_depth + 1);
                let degree = indegree
                    .get_mut(&child)
                    .ok_or_else(|| SignalError::internal("planned node missing indegree"))?;
                *degree = degree.saturating_sub(1);
                if *degree == 0 {
                    ready.push(child);
                }
            }
        }
    }

    if visited != planned.len() {
        return Err(SignalError::internal(
            "planner depth layering encountered a cycle in the planned node set",
        ));
    }

    Ok(depths)
}

fn classify_reason(
    graph: &SignalGraph,
    node: NodeId,
    direct_request: bool,
    request_mode: EvaluationRequestMode,
) -> Result<TaskReason, SignalError> {
    if direct_request {
        let condition = &graph.get_entry(node)?.get_eval_config().condition;
        if matches!(request_mode, EvaluationRequestMode::ForceOnDemand)
            && matches!(condition, EvaluationCondition::OnDemand)
        {
            return Ok(TaskReason::ConditionForced);
        }
        return Ok(TaskReason::RequestedTarget);
    }

    let entry = graph.get_entry(node)?;
    if matches!(entry.get_state(), NodeState::MaybeStale) {
        return Ok(TaskReason::MaybeStaleValidation);
    }
    if graph
        .dependencies_of(node)?
        .iter()
        .any(|dep| dep.scope_ref().is_some())
    {
        return Ok(TaskReason::PartitionScopedDependency);
    }
    if entry
        .get_trace_summary()
        .is_some_and(|trace| trace.output_identity.is_some())
    {
        return Ok(TaskReason::OutputDiffDependent);
    }
    if entry.get_trace_summary().is_some_and(|trace| {
        trace.keyed_family.is_some() || trace.memoized_origin != MemoizedResultOrigin::DirectCompute
    }) {
        return Ok(TaskReason::MemoValidation);
    }
    Ok(TaskReason::DependencyRequired)
}

fn partition_scope_untouched(
    trace_summary: Option<&TraceSummary>,
    scope: &crate::data::output::PartitionSubscription,
) -> bool {
    let Some(trace_summary) = trace_summary else {
        return false;
    };
    if trace_summary.output_change == crate::data::output::OutputChange::Unchanged {
        return true;
    }
    if trace_summary.changed_regions.is_empty() {
        return false;
    }
    !trace_summary.changed_regions.iter().any(|region| {
        region.partition == scope.partition
            && match scope.match_mode {
                crate::data::output::PartitionMatchMode::WholePartition => true,
                crate::data::output::PartitionMatchMode::PartitionAndDetail => {
                    region.detail == scope.detail
                }
            }
    })
}

fn node_sort_key(node: NodeId) -> (u32, u32) {
    (node.index(), node.generation())
}
