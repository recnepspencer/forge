use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
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
use crate::logic::evaluation::{
    apply_prepared_evaluation_with_policy,
    evaluate_direct_with_policy_and_condition_resolvers_and_metadata, DefaultConditionResolver,
    EvaluationExecutionMetadata, EvaluationRequestMode,
};
use crate::logic::prepared::{ExecutionSnapshot, PreparedEvaluation};

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
    Parallel,
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

    let mut depth_cache = BTreeMap::<NodeId, u32>::new();
    let planned_ids: BTreeSet<NodeId> = planned.keys().copied().collect();
    for &node in planned_ids.iter() {
        compute_depth(graph, node, &planned_ids, &mut depth_cache)?;
    }

    let mut stages_by_depth = BTreeMap::<u32, Vec<EvaluationTask>>::new();
    for (&node, planned_node) in &planned {
        let reason = classify_reason(graph, node, planned_node.direct_request, request_mode)?;
        let task = EvaluationTask {
            node,
            request_mode,
            direct_request: planned_node.direct_request,
            reason,
        };
        let depth = *depth_cache.get(&node).unwrap_or(&0);
        stages_by_depth.entry(depth).or_default().push(task);
    }

    let mut stages = Vec::new();
    for (index, (depth, mut tasks)) in stages_by_depth.into_iter().enumerate() {
        let _ = depth;
        tasks.sort_by_key(|task| node_sort_key(task.node));
        stages.push(ExecutionStage {
            index: index as u32,
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

pub fn execute_plan<F, O>(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    compute: &mut F,
) -> Result<ExecutionReport, SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
{
    let mut comparator = DefaultComparatorResolver;
    let mut resolver = DefaultComparatorPolicyResolver {
        fallback: VersionComparatorPolicy::Exact,
        custom: &mut comparator,
    };
    let mut condition = DefaultConditionResolver;
    execute_plan_with_policy_and_condition(
        graph,
        plan,
        compute,
        &mut resolver,
        &mut condition,
        StageExecutor::Serial,
        None,
    )
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
        StageExecutor::Parallel => {
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
                StageExecutor::Serial => precompute_stage_serial(stage, &snapshot, precompute)?,
                #[cfg(feature = "parallel")]
                StageExecutor::Parallel => precompute_stage_parallel(stage, &snapshot, precompute)?,
            };
            prepared
        };
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
            StageExecutor::Parallel => {
                graph.telemetry_mut().parallel_stage_dispatch_count += 1;
                graph.telemetry_mut().parallel_precompute_task_count += prepared.len() as u64;
            }
        }

        let apply_start = Instant::now();
        let mut stage_record = StageExecutionRecord {
            stage_index: stage.index,
            outcome: {
                #[cfg(feature = "parallel")]
                {
                    match executor {
                        StageExecutor::Serial => StageExecutionOutcome::CompletedSerial,
                        StageExecutor::Parallel => StageExecutionOutcome::CompletedParallel,
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
        report.stage_apply_nanos += stage_record.apply_duration_nanos;
        graph.telemetry_mut().stage_apply_nanos += stage_record.apply_duration_nanos;

        stage_record.duration_nanos = stage_start.elapsed().as_nanos();
        graph.telemetry_mut().stage_execution_count += 1;
        graph.telemetry_mut().stage_execution_nanos += stage_record.duration_nanos;
        report.stages.push(stage_record);
    }

    Ok(report)
}

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
        StageExecutor::Parallel => Err(SignalError::invalid_input(
            "parallel stage execution is not yet supported by the current mutable graph engine",
        )),
    }
}

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
        let mut stage_record = StageExecutionRecord {
            stage_index: stage.index,
            outcome: StageExecutionOutcome::CompletedSerial,
            snapshot_duration_nanos: 0,
            precompute_duration_nanos: 0,
            apply_duration_nanos: 0,
            duration_nanos: 0,
            task_records: Vec::new(),
        };

        for task in &stage.tasks {
            let record_id = ExecutionRecordId(next_record_id);
            next_record_id += 1;
            let before_state = graph.get_state(task.node)?;
            let before_trace = graph.get_entry(task.node)?.get_trace_summary().cloned();
            let result = evaluate_direct_with_policy_and_condition_resolvers_and_metadata(
                graph,
                task.node,
                compute,
                comparator_resolver,
                condition_resolver,
                task.request_mode,
                execution_metadata.filter(|_| task.direct_request),
            );
            result?;
            let after_state = graph.get_state(task.node)?;
            let after_trace = graph.get_entry(task.node)?.get_trace_summary().cloned();
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
        }

        stage_record.duration_nanos = stage_start.elapsed().as_nanos();
        graph.telemetry_mut().stage_execution_count += 1;
        graph.telemetry_mut().stage_execution_nanos += stage_record.duration_nanos;
        report.stages.push(stage_record);
    }

    Ok(report)
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
        && preview_upstream_unchanged(graph, node, resolver)?;
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
    let mut deps = graph.get_entry(node)?.get_dependencies().to_vec();
    deps.sort_by_key(|dep| (node_sort_key(dep.source()), dep.aspect().index()));
    Ok(deps)
}

fn compute_depth(
    graph: &SignalGraph,
    node: NodeId,
    planned_ids: &BTreeSet<NodeId>,
    cache: &mut BTreeMap<NodeId, u32>,
) -> Result<u32, SignalError> {
    if let Some(depth) = cache.get(&node).copied() {
        return Ok(depth);
    }
    let mut max_depth = 0_u32;
    for dep in graph.get_entry(node)?.get_dependencies() {
        if !planned_ids.contains(&dep.source()) {
            continue;
        }
        max_depth = max_depth.max(compute_depth(graph, dep.source(), planned_ids, cache)? + 1);
    }
    cache.insert(node, max_depth);
    Ok(max_depth)
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
    if entry
        .get_dependencies()
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

fn preview_upstream_unchanged(
    graph: &SignalGraph,
    node: NodeId,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<bool, SignalError> {
    let entry = graph.get_entry(node)?;
    let snapshot = entry.get_dep_snapshot();
    let comparator = resolver.policy_for_node(node, entry.get_eval_config().comparator.as_ref());

    for (source, aspect, cached_version, scope) in snapshot.entries() {
        if !graph.is_alive(*source) {
            return Ok(false);
        }
        if !matches!(graph.get_entry(*source)?.get_state(), NodeState::Clean) {
            return Ok(false);
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        if let Some(scope) = scope {
            if current_version == *cached_version {
                continue;
            }
            if partition_scope_untouched(graph.get_entry(*source)?.get_trace_summary(), scope) {
                continue;
            }
            return Ok(false);
        }
        if comparator.has_meaningful_change(*aspect, *cached_version, current_version, resolver)? {
            return Ok(false);
        }
    }

    Ok(true)
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
