use std::time::Instant;

use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::logic::evaluation::apply_prepared_evaluation_with_policy;
use crate::logic::prepared::{
    ExecutionSnapshot, PreparedEvaluation, PreparedEvaluationOrigin, PreparedEvaluationOutcome,
};

#[cfg(feature = "parallel")]
use super::full_parallel::apply_full_parallel_stage;
#[cfg(feature = "parallel")]
use super::precompute::{build_parallel_stage_patches, precompute_stage_parallel};
use super::precompute::{precompute_stage_serial, StageExecutionData};
use super::reporting::{record_execution_failure, record_successful_execution};
use super::semantic::{
    finalize_stage_batch, reserve_stage_identities, segment_for_single_update, SemanticTaskUpdate,
    StageSemanticBatch,
};
#[cfg(feature = "parallel")]
use super::types::ParallelExecutionKind;
use super::types::{
    EvaluationPlan, ExecutionReport, StageExecutionOutcome, StageExecutionRecord, StageExecutor,
};

#[cfg(feature = "parallel")]
#[derive(Debug, Clone, Copy)]
struct StageParallelAdmission {
    use_parallel: bool,
    reason: &'static str,
    kind: Option<ParallelExecutionKind>,
}

#[cfg(feature = "parallel")]
fn decide_stage_parallel_admission(
    graph: &SignalGraph,
    stage: &super::types::ExecutionStage,
    executor: StageExecutor,
) -> StageParallelAdmission {
    let Some(parallel_policy) = executor.parallel_policy() else {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "serial-executor",
            kind: None,
        };
    };
    let stage_width = stage.tasks.len();
    if stage_width < parallel_policy.min_stage_width.get() {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "below-min-stage-width",
            kind: None,
        };
    }
    let runtime_policy = graph.runtime_policy();
    let min_parallel_tasks = runtime_policy
        .parallel_admission
        .min_parallel_tasks_for(runtime_policy.profile);
    let semantic_cost_multiplier = match runtime_policy.semantic_retention {
        crate::diagnostics::SemanticRetentionPolicy::Minimal => 1,
        crate::diagnostics::SemanticRetentionPolicy::Development => 2,
        crate::diagnostics::SemanticRetentionPolicy::Forensic => 3,
    };
    let effective_parallel_threshold = min_parallel_tasks.saturating_mul(semantic_cost_multiplier);
    if stage_width < effective_parallel_threshold {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "below-policy-work-threshold",
            kind: None,
        };
    }
    let compute_pressure = stage
        .tasks
        .iter()
        .filter(|task| {
            !matches!(
                task.reason,
                super::types::TaskReason::MaybeStaleValidation
                    | super::types::TaskReason::MemoValidation
            )
        })
        .count();
    if compute_pressure.saturating_mul(2) < stage_width
        && stage_width < effective_parallel_threshold.saturating_mul(2)
    {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "validation-heavy-stage",
            kind: None,
        };
    }
    if executor.is_full_parallel()
        && stage_width
            < runtime_policy
                .parallel_admission
                .full_parallel_min_tasks
                .saturating_mul(semantic_cost_multiplier)
    {
        return StageParallelAdmission {
            use_parallel: false,
            reason: "below-full-parallel-threshold",
            kind: None,
        };
    }
    StageParallelAdmission {
        use_parallel: true,
        reason: match runtime_policy.profile {
            crate::diagnostics::profile::DiagnosticsProfile::Operational => "admitted-operational",
            crate::diagnostics::profile::DiagnosticsProfile::Development => "admitted-development",
            crate::diagnostics::profile::DiagnosticsProfile::Forensic => "admitted-forensic",
        },
        kind: executor.parallel_kind(),
    }
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
    let mut comparator = crate::data::comparator::DefaultComparatorResolver;
    let mut resolver = crate::data::comparator::DefaultComparatorPolicyResolver {
        fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
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
        .filter(|task| matches!(task.reason, super::types::TaskReason::MaybeStaleValidation))
        .count() as u64;

    #[cfg(feature = "parallel")]
    match executor {
        StageExecutor::StagedParallelPrecompute { .. } | StageExecutor::FullParallel { .. } => {
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
    let mut next_segment_id = 1_u64;
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
        semantic_finalize_nanos: 0,
        semantic_segment_count: 0,
        stages: Vec::new(),
    };

    for stage in &plan.stages {
        let stage_start = Instant::now();
        #[cfg(feature = "parallel")]
        let stage_parallel = decide_stage_parallel_admission(graph, stage, executor);
        let snapshot_start = Instant::now();
        graph.telemetry_mut().execution_snapshots_built += 1;
        let precompute_start = Instant::now();
        let stage_execution = (|| {
            let snapshot = ExecutionSnapshot::new(&*graph);
            let execution = match executor {
                StageExecutor::Serial => StageExecutionData::Prepared(precompute_stage_serial(
                    stage,
                    &snapshot,
                    precompute,
                    comparator_resolver,
                )?),
                #[cfg(feature = "parallel")]
                _ if stage_parallel.use_parallel => {
                    if executor.is_full_parallel() {
                        StageExecutionData::Patched(build_parallel_stage_patches(
                            stage,
                            &snapshot,
                            precompute,
                            executor
                                .parallel_policy()
                                .expect("parallel policy should exist"),
                            comparator_resolver,
                        )?)
                    } else {
                        StageExecutionData::Prepared(precompute_stage_parallel(
                            stage,
                            &snapshot,
                            precompute,
                            executor
                                .parallel_policy()
                                .expect("parallel policy should exist"),
                            comparator_resolver,
                        )?)
                    }
                }
                #[cfg(feature = "parallel")]
                StageExecutor::StagedParallelPrecompute { .. }
                | StageExecutor::FullParallel { .. } => StageExecutionData::Prepared(
                    precompute_stage_serial(stage, &snapshot, precompute, comparator_resolver)?,
                ),
            };
            Ok::<StageExecutionData, SignalError>(execution)
        })()
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
        graph.telemetry_mut().prepared_evaluations_produced += stage_execution.len() as u64;
        report.prepared_evaluations_produced += stage_execution.len() as u32;
        report.stage_precompute_nanos += precompute_nanos;
        match executor {
            StageExecutor::Serial => {
                graph.telemetry_mut().serial_precompute_task_count += stage_execution.len() as u64;
            }
            #[cfg(feature = "parallel")]
            _ if stage_parallel.use_parallel => {
                graph.telemetry_mut().parallel_stage_dispatch_count += 1;
                graph.telemetry_mut().parallel_precompute_task_count +=
                    stage_execution.len() as u64;
            }
            #[cfg(feature = "parallel")]
            StageExecutor::StagedParallelPrecompute { .. } | StageExecutor::FullParallel { .. } => {
                graph.telemetry_mut().serial_precompute_task_count += stage_execution.len() as u64;
            }
        }

        let apply_start = Instant::now();
        let mut stage_record = StageExecutionRecord {
            stage_index: stage.index,
            outcome: {
                #[cfg(feature = "parallel")]
                {
                    if stage_parallel.use_parallel {
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
            parallel_admission_reason: Some({
                #[cfg(feature = "parallel")]
                {
                    stage_parallel.reason.to_string()
                }
                #[cfg(not(feature = "parallel"))]
                {
                    "serial-executor".to_string()
                }
            }),
            #[cfg(feature = "parallel")]
            parallel_kind: stage_parallel.kind,
            #[cfg(feature = "parallel")]
            apply_mode: None,
            #[cfg(feature = "parallel")]
            apply_group_count: 0,
            #[cfg(feature = "parallel")]
            serial_fallback_group_count: 0,
            #[cfg(feature = "parallel")]
            concurrent_apply_task_count: 0,
            #[cfg(feature = "parallel")]
            serial_apply_task_count: 0,
            snapshot_duration_nanos: snapshot_nanos,
            precompute_duration_nanos: precompute_nanos,
            apply_duration_nanos: 0,
            semantic_finalize_duration_nanos: 0,
            duration_nanos: 0,
            semantic_task_range: None,
            semantic_segment_count: 0,
            task_records: Vec::new(),
        };
        let stage_identities =
            reserve_stage_identities(&mut next_record_id, &mut next_segment_id, stage.tasks.len());

        #[cfg(feature = "parallel")]
        if executor.is_full_parallel() && stage_parallel.use_parallel {
            apply_full_parallel_stage(
                graph,
                stage,
                stage_execution.into_patches(stage),
                comparator_resolver,
                executor,
                plan,
                &stage_identities,
                &mut report,
                &mut stage_record,
            )?;
        } else {
            let mut semantic_batch = StageSemanticBatch::default();
            for patch in stage_execution.into_patches(stage) {
                let identity = stage_identities[patch.task_index];
                let recomputed =
                    matches!(patch.prepared.outcome, PreparedEvaluationOutcome::Evaluate)
                        && !matches!(
                            patch.prepared.origin,
                            PreparedEvaluationOrigin::MemoizedReuse
                        );
                let prepared_outcome = patch.prepared.outcome;
                let prepared_origin = patch.prepared.origin;
                let partition_aware = !patch.prepared.result.changed_regions.is_empty();
                let before_state = graph.get_state(patch.node)?;
                let before_trace = graph.get_entry(patch.node)?.get_trace_summary().cloned();
                let dependency_updates = apply_prepared_evaluation_with_policy(
                    graph,
                    patch.node,
                    patch.prepared,
                    comparator_resolver,
                    None,
                )
                .map_err(|err| {
                    record_execution_failure(
                        graph,
                        ExecutionFailureContext::new(
                            ExecutionFailurePhase::Apply,
                            Some(stage.index),
                            Some(patch.node),
                            Some(executor),
                            Some(identity.record_id),
                            Some(plan.summary.clone()),
                            err.to_string(),
                        ),
                    );
                    err
                })?;
                let after_state = graph.get_state(patch.node)?;
                semantic_batch.push_segment(segment_for_single_update(SemanticTaskUpdate {
                    task_index: patch.task_index,
                    node: patch.node,
                    identity,
                    before_state,
                    before_trace,
                    after_state,
                    dependency_updates,
                    recomputed,
                    partition_aware,
                    prepared_outcome,
                    prepared_origin,
                }));
            }
            let semantic_finalize_start = Instant::now();
            finalize_stage_batch(
                graph,
                &stage.tasks,
                semantic_batch,
                &mut report,
                &mut stage_record,
            )?;
            stage_record.semantic_finalize_duration_nanos =
                semantic_finalize_start.elapsed().as_nanos();
        }
        #[cfg(not(feature = "parallel"))]
        {
            let mut semantic_batch = StageSemanticBatch::default();
            for patch in stage_execution.into_patches(stage) {
                let identity = stage_identities[patch.task_index];
                let recomputed =
                    matches!(patch.prepared.outcome, PreparedEvaluationOutcome::Evaluate)
                        && !matches!(
                            patch.prepared.origin,
                            PreparedEvaluationOrigin::MemoizedReuse
                        );
                let prepared_outcome = patch.prepared.outcome;
                let prepared_origin = patch.prepared.origin;
                let partition_aware = !patch.prepared.result.changed_regions.is_empty();
                let before_state = graph.get_state(patch.node)?;
                let before_trace = graph.get_entry(patch.node)?.get_trace_summary().cloned();
                let dependency_updates = apply_prepared_evaluation_with_policy(
                    graph,
                    patch.node,
                    patch.prepared,
                    comparator_resolver,
                    None,
                )
                .map_err(|err| {
                    record_execution_failure(
                        graph,
                        ExecutionFailureContext::new(
                            ExecutionFailurePhase::Apply,
                            Some(stage.index),
                            Some(patch.node),
                            Some(executor),
                            Some(identity.record_id),
                            Some(plan.summary.clone()),
                            err.to_string(),
                        ),
                    );
                    err
                })?;
                let after_state = graph.get_state(patch.node)?;
                semantic_batch.push_segment(segment_for_single_update(SemanticTaskUpdate {
                    task_index: patch.task_index,
                    node: patch.node,
                    identity,
                    before_state,
                    before_trace,
                    after_state,
                    dependency_updates,
                    recomputed,
                    partition_aware,
                    prepared_outcome,
                    prepared_origin,
                }));
            }
            let semantic_finalize_start = Instant::now();
            finalize_stage_batch(
                graph,
                &stage.tasks,
                semantic_batch,
                &mut report,
                &mut stage_record,
            )?;
            stage_record.semantic_finalize_duration_nanos =
                semantic_finalize_start.elapsed().as_nanos();
        }
        stage_record.apply_duration_nanos = apply_start
            .elapsed()
            .as_nanos()
            .saturating_sub(stage_record.semantic_finalize_duration_nanos);
        report.stage_apply_nanos += stage_record.apply_duration_nanos;
        graph.telemetry_mut().stage_apply_nanos += stage_record.apply_duration_nanos;
        report.semantic_finalize_nanos += stage_record.semantic_finalize_duration_nanos;

        stage_record.duration_nanos = stage_start.elapsed().as_nanos();
        graph.telemetry_mut().stage_execution_count += 1;
        graph.telemetry_mut().stage_execution_nanos += stage_record.duration_nanos;
        report.stages.push(stage_record);
    }

    record_successful_execution(graph, plan, &report);
    Ok(report)
}
