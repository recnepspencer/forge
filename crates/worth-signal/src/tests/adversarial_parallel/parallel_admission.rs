use std::num::NonZeroUsize;

use crate::facade::{
    mark_dirty, EvaluationRequestMode, ParallelAdmissionReason, ParallelApplyMode,
    ParallelExecutionPolicy, SignalGraph, StageExecutionOutcome, StageExecutor,
};
use crate::tests::support::{version_ab, GraphDependencyBatchExt, ASPECT_A};

use super::executor_policy::aggressive_parallel_runtime_policy;

#[test]
fn many_thin_stages_remain_serial_under_parallel_threshold() {
    let mut graph = SignalGraph::new();
    let mut chain = Vec::new();
    for _ in 0..32 {
        chain.push(graph.node().build());
    }
    for index in 1..chain.len() {
        graph
            .append_dependency(chain[index], chain[index - 1], ASPECT_A)
            .unwrap();
    }

    let bootstrap = graph
        .build_evaluation_plan(&chain, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|ctx| Ok(ctx.finish(version_ab(1, 0))))
        .unwrap();

    mark_dirty(&mut graph, chain[0], ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(&[chain[chain.len() - 1]], EvaluationRequestMode::Default)
        .unwrap();
    let before = graph.telemetry().execution.parallel_stage_dispatch_count;
    let report = graph
        .execute_prepared_plan_with_executor(
            &plan,
            &(),
            &|ctx| Ok(ctx.finish(version_ab(2, 0))),
            StageExecutor::parallel(3),
        )
        .unwrap();

    assert_eq!(
        graph.telemetry().execution.parallel_stage_dispatch_count,
        before
    );
    assert!(report
        .stages
        .iter()
        .all(|stage| { matches!(stage.outcome, StageExecutionOutcome::CompletedSerial) }));
}

#[test]
fn wide_stage_crosses_parallel_threshold() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(aggressive_parallel_runtime_policy());
    let left = graph.node().build();
    let right = graph.node().build();
    let requested = [left, right];

    let bootstrap = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|ctx| Ok(ctx.finish(version_ab(1, 0))))
        .unwrap();

    mark_dirty(&mut graph, left, ASPECT_A).unwrap();
    mark_dirty(&mut graph, right, ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::Default)
        .unwrap();
    assert_eq!(plan.summary.max_stage_width, 2);
    let before = graph.telemetry().execution.parallel_stage_dispatch_count;
    let report = graph
        .execute_prepared_plan_with_executor(
            &plan,
            &(),
            &|ctx| Ok(ctx.finish(version_ab(2, 0))),
            StageExecutor::parallel(2),
        )
        .unwrap();

    assert_eq!(
        graph.telemetry().execution.parallel_stage_dispatch_count,
        before + 1
    );
    assert_eq!(report.stages.len(), 1);
    assert!(report
        .stages
        .iter()
        .any(|stage| { matches!(stage.outcome, StageExecutionOutcome::CompletedParallel) }));
}

#[test]
fn full_parallel_splits_wide_stage_into_deterministic_apply_groups() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(aggressive_parallel_runtime_policy());
    let requested: Vec<_> = (0..4).map(|_| graph.node().build()).collect();

    let bootstrap = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|ctx| Ok(ctx.finish(version_ab(1, 0))))
        .unwrap();

    for &node in &requested {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
    }

    let plan = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::Default)
        .unwrap();
    let policy = ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
        .with_apply_group_min_width(2)
        .with_max_concurrent_apply_groups(2);
    let report = graph
        .execute_prepared_plan_with_executor(
            &plan,
            &(),
            &|ctx| Ok(ctx.finish(version_ab(2, 0))),
            StageExecutor::full_parallel(1).with_parallel_policy(policy),
        )
        .unwrap();

    assert_eq!(report.stages.len(), 1);
    let stage = &report.stages[0];
    assert!(matches!(
        stage.outcome,
        StageExecutionOutcome::CompletedParallel
    ));
    assert_eq!(
        stage.apply_mode,
        Some(ParallelApplyMode::GroupedConcurrentApply)
    );
    assert_eq!(
        stage.parallel_admission_reason,
        Some(ParallelAdmissionReason::AdmittedProofSafeGroupedConcurrent)
    );
    assert_eq!(stage.apply_group_count, 2);
    assert_eq!(stage.serial_fallback_group_count, 0);
    assert_eq!(stage.concurrent_apply_task_count, requested.len() as u32);
}
