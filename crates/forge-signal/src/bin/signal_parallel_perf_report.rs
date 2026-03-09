#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;
#[cfg(feature = "parallel")]
use std::time::Instant;

#[cfg(feature = "parallel")]
use forge_signal::facade::{
    mark_dirty, mark_dirty_with_regions, Aspect, AspectVersion, ChangedRegion,
    EvaluationRequestMode, ExecutionReadView, ExecutionReport, NodeEvaluationResult, NodeId,
    ParallelExecutionPolicy, SignalGraph, SignalRuntimePolicy, StageExecutor,
    CORE_STORAGE_PROFILE_ID,
};
#[cfg(feature = "parallel")]
use serde::Serialize;

#[cfg(feature = "parallel")]
const ASPECT_A: Aspect = Aspect::new(0);

#[cfg(feature = "parallel")]
fn version_ab(a: u64, b: u64) -> AspectVersion {
    AspectVersion::from_updates([(ASPECT_A, a), (Aspect::new(1), b)])
}

#[cfg(feature = "parallel")]
#[derive(Debug, Serialize)]
struct PerfRecord {
    workload: &'static str,
    executor_profile: &'static str,
    runtime_policy: &'static str,
    core_storage_profile: &'static str,
    stage_parallel_admission_reasons: Vec<String>,
    plan_task_count: u32,
    plan_stage_count: u32,
    planning_nanos: u64,
    execute_elapsed_nanos: u64,
    snapshot_nanos: u64,
    precompute_nanos: u64,
    apply_nanos: u64,
    semantic_finalize_nanos: u64,
    residual_nanos: u64,
    semantic_segment_count: u32,
    tasks_executed: u32,
}

#[cfg(feature = "parallel")]
fn nanos(value: u128) -> u64 {
    value.min(u64::MAX as u128) as u64
}

#[cfg(feature = "parallel")]
fn executor_profiles() -> [(&'static str, StageExecutor); 4] {
    let policy_2x1 = ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
        .with_worker_count(2)
        .with_chunk_size(1)
        .with_apply_group_min_width(1)
        .with_max_concurrent_apply_groups(2);
    let policy_4x2 = ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
        .with_worker_count(4)
        .with_chunk_size(2)
        .with_apply_group_min_width(1)
        .with_max_concurrent_apply_groups(4);
    [
        ("serial", StageExecutor::Serial),
        (
            "staged-2x1",
            StageExecutor::parallel(1).with_parallel_policy(policy_2x1),
        ),
        (
            "full-2x1",
            StageExecutor::full_parallel(1).with_parallel_policy(policy_2x1),
        ),
        (
            "full-4x2",
            StageExecutor::full_parallel(1).with_parallel_policy(policy_4x2),
        ),
    ]
}

#[cfg(feature = "parallel")]
fn summarize(
    workload: &'static str,
    executor_profile: &'static str,
    runtime_policy: &'static str,
    planning_nanos: u128,
    execute_elapsed_nanos: u128,
    report: &ExecutionReport,
) -> PerfRecord {
    let phase_total = report.execution_snapshot_nanos
        + report.stage_precompute_nanos
        + report.stage_apply_nanos
        + report.semantic_finalize_nanos;
    PerfRecord {
        workload,
        executor_profile,
        runtime_policy,
        core_storage_profile: CORE_STORAGE_PROFILE_ID,
        stage_parallel_admission_reasons: report
            .stages
            .iter()
            .filter_map(|stage| stage.parallel_admission_reason.clone())
            .collect(),
        plan_task_count: report.plan_summary.task_count,
        plan_stage_count: report.plan_summary.stage_count,
        planning_nanos: nanos(planning_nanos),
        execute_elapsed_nanos: nanos(execute_elapsed_nanos),
        snapshot_nanos: nanos(report.execution_snapshot_nanos),
        precompute_nanos: nanos(report.stage_precompute_nanos),
        apply_nanos: nanos(report.stage_apply_nanos),
        semantic_finalize_nanos: nanos(report.semantic_finalize_nanos),
        residual_nanos: nanos(execute_elapsed_nanos.saturating_sub(phase_total)),
        semantic_segment_count: report.semantic_segment_count,
        tasks_executed: report.tasks_executed,
    }
}

#[cfg(feature = "parallel")]
fn runtime_policy_profiles() -> [(&'static str, SignalRuntimePolicy); 3] {
    [
        ("operational", SignalRuntimePolicy::operational()),
        ("development", SignalRuntimePolicy::development()),
        ("forensic", SignalRuntimePolicy::forensic()),
    ]
}

#[cfg(feature = "parallel")]
fn run_deep_chain(
    executor_profile: &'static str,
    runtime_policy_name: &'static str,
    runtime_policy: SignalRuntimePolicy,
    executor: StageExecutor,
) -> PerfRecord {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(runtime_policy);
    let mut chain = Vec::new();
    for _ in 0..512 {
        chain.push(graph.node().build());
    }
    for index in 1..chain.len() {
        graph
            .add_dependency(chain[index], chain[index - 1], ASPECT_A)
            .unwrap();
    }

    let bootstrap = graph
        .build_evaluation_plan(&chain, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &|_node, view| {
            Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
        })
        .unwrap();

    mark_dirty(&mut graph, chain[0], ASPECT_A).unwrap();
    let plan_start = Instant::now();
    let plan = graph
        .build_evaluation_plan(&chain, EvaluationRequestMode::Default)
        .unwrap();
    let planning_nanos = plan_start.elapsed().as_nanos();
    let execute_start = Instant::now();
    let report = graph
        .execute_prepared_plan_with_executor(
            &plan,
            &|_node, view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(2, 0)))),
            executor,
        )
        .unwrap();
    summarize(
        "deep-chain-512",
        executor_profile,
        runtime_policy_name,
        planning_nanos,
        execute_start.elapsed().as_nanos(),
        &report,
    )
}

#[cfg(feature = "parallel")]
fn run_wide_stage(
    executor_profile: &'static str,
    runtime_policy_name: &'static str,
    runtime_policy: SignalRuntimePolicy,
    executor: StageExecutor,
) -> PerfRecord {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(runtime_policy);
    let requested: Vec<_> = (0..256).map(|_| graph.node().build()).collect();
    let bootstrap = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &|_node, view| {
            Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
        })
        .unwrap();

    for &node in &requested {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
    }
    let plan_start = Instant::now();
    let plan = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::Default)
        .unwrap();
    let planning_nanos = plan_start.elapsed().as_nanos();
    let execute_start = Instant::now();
    let report = graph
        .execute_prepared_plan_with_executor(
            &plan,
            &|_node, view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(2, 0)))),
            executor,
        )
        .unwrap();
    summarize(
        "wide-stage-256",
        executor_profile,
        runtime_policy_name,
        planning_nanos,
        execute_start.elapsed().as_nanos(),
        &report,
    )
}

#[cfg(feature = "parallel")]
fn run_partition_tolerance(
    executor_profile: &'static str,
    runtime_policy_name: &'static str,
    runtime_policy: SignalRuntimePolicy,
    executor: StageExecutor,
) -> PerfRecord {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(runtime_policy);
    let source = graph.node().build();
    let branches: Vec<_> = (0..96).map(|_| graph.node().tolerance(1).build()).collect();
    let target = graph.node().output_identity().build();
    for (index, branch) in branches.iter().copied().enumerate() {
        let partition = if index % 2 == 0 { "shell" } else { "core" };
        graph
            .add_partition_dependency(branch, source, ASPECT_A, partition)
            .unwrap();
        graph.add_dependency(target, branch, ASPECT_A).unwrap();
    }

    let bootstrap_targets: Vec<_> = std::iter::once(source)
        .chain(branches.iter().copied())
        .chain(std::iter::once(target))
        .collect();
    let bootstrap = graph
        .build_evaluation_plan(&bootstrap_targets, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    let bootstrap_branches = branches.clone();
    graph
        .execute_prepared_plan(
            &bootstrap,
            &move |node: NodeId, view: &ExecutionReadView<'_>| {
                let result = if node == source {
                    view.finish(
                        NodeEvaluationResult::from_version(version_ab(10, 0))
                            .with_changed_region(ChangedRegion::new("shell"))
                            .with_changed_region(ChangedRegion::new("core")),
                    )
                } else if bootstrap_branches.contains(&node) {
                    let version = view.read_aspect_version(source, ASPECT_A)?;
                    view.finish(NodeEvaluationResult::from_version(version))
                } else {
                    let mut total = 0_u64;
                    for branch in &bootstrap_branches {
                        total += view.read_aspect_version(*branch, ASPECT_A)?.get(ASPECT_A);
                    }
                    view.finish(
                        NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                            ASPECT_A, total,
                        )]))
                        .with_output_identity("partition-aggregate"),
                    )
                };
                Ok(result)
            },
        )
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("core"), ChangedRegion::new("shell")],
    )
    .unwrap();
    let plan_start = Instant::now();
    let plan = graph
        .build_evaluation_plan(&[target], EvaluationRequestMode::Default)
        .unwrap();
    let planning_nanos = plan_start.elapsed().as_nanos();
    let execute_start = Instant::now();
    let execute_branches = branches.clone();
    let report = graph
        .execute_prepared_plan_with_executor(
            &plan,
            &move |node: NodeId, view: &ExecutionReadView<'_>| {
                let result = if node == source {
                    view.finish(
                        NodeEvaluationResult::from_version(version_ab(12, 0))
                            .with_changed_region(ChangedRegion::new("shell"))
                            .with_changed_region(ChangedRegion::new("core")),
                    )
                } else if execute_branches.contains(&node) {
                    let version = view.read_aspect_version(source, ASPECT_A)?;
                    view.finish(NodeEvaluationResult::from_version(version))
                } else {
                    let mut total = 0_u64;
                    for branch in &execute_branches {
                        total += view.read_aspect_version(*branch, ASPECT_A)?.get(ASPECT_A);
                    }
                    view.finish(
                        NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                            ASPECT_A, total,
                        )]))
                        .with_output_identity("partition-aggregate"),
                    )
                };
                Ok(result)
            },
            executor,
        )
        .unwrap();
    summarize(
        "partition-tolerance-96",
        executor_profile,
        runtime_policy_name,
        planning_nanos,
        execute_start.elapsed().as_nanos(),
        &report,
    )
}

#[cfg(feature = "parallel")]
fn main() {
    let mut records = Vec::new();
    for (runtime_policy_name, runtime_policy) in runtime_policy_profiles() {
        for (executor_profile, executor) in executor_profiles() {
            records.push(run_deep_chain(
                executor_profile,
                runtime_policy_name,
                runtime_policy,
                executor,
            ));
            records.push(run_wide_stage(
                executor_profile,
                runtime_policy_name,
                runtime_policy,
                executor,
            ));
            records.push(run_partition_tolerance(
                executor_profile,
                runtime_policy_name,
                runtime_policy,
                executor,
            ));
        }
    }
    println!("{}", serde_json::to_string_pretty(&records).unwrap());
}

#[cfg(not(feature = "parallel"))]
fn main() {
    panic!("signal_parallel_perf_report requires the `parallel` feature");
}
