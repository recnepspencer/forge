use std::time::Instant;

use serde::Serialize;
use serde_json::json;

use crate::facade::*;
use crate::presentation::harness::{
    signal_bench, SignalProfileCatalog, SignalScenario,
};
use crate::data::dependency::DependencyEdge;
use crate::tests::domains::fintech::{setup_seeded_world_with, FintechScale, MarketRegime};
use crate::tests::support::{version_ab, ASPECT_A};

#[derive(Serialize)]
struct PerfRecord<'a> {
    suite: &'a str,
    profile: &'a str,
    executor: &'a str,
    elapsed_micros: u128,
    metrics: serde_json::Value,
}

fn emit(record: &PerfRecord<'_>) {
    eprintln!("{}", serde_json::to_string(record).expect("perf record should serialize"));
}

fn eval_metrics_delta(before: RuntimeMetrics, after: RuntimeMetrics) -> serde_json::Value {
    json!({
        "evaluation_calls": after.evaluation.evaluation_calls - before.evaluation.evaluation_calls,
        "evaluation_nanos": after.evaluation.evaluation_nanos - before.evaluation.evaluation_nanos,
        "nodes_evaluated": after.evaluation.nodes_evaluated - before.evaluation.nodes_evaluated,
        "nodes_recomputed": after.evaluation.nodes_recomputed - before.evaluation.nodes_recomputed,
        "skipped_by_comparator": after.evaluation.skipped_by_comparator - before.evaluation.skipped_by_comparator,
        "suppressed_downstream_propagations": after.evaluation.suppressed_downstream_propagations - before.evaluation.suppressed_downstream_propagations,
        "plans_built": after.planner.plans_built - before.planner.plans_built,
        "tasks_scheduled": after.planner.tasks_scheduled - before.planner.tasks_scheduled,
        "tasks_pruned_before_execution": after.planner.tasks_pruned_before_execution - before.planner.tasks_pruned_before_execution,
        "stage_execution_count": after.execution.stage_execution_count - before.execution.stage_execution_count,
        "stage_execution_nanos": after.execution.stage_execution_nanos - before.execution.stage_execution_nanos,
    })
}

fn graph_metrics_delta(before: GraphMetrics, after: GraphMetrics) -> serde_json::Value {
    json!({
        "nodes_evaluated": after.evaluation.nodes_evaluated - before.evaluation.nodes_evaluated,
        "nodes_recomputed": after.evaluation.nodes_recomputed - before.evaluation.nodes_recomputed,
        "skipped_by_comparator": after.evaluation.skipped_by_comparator - before.evaluation.skipped_by_comparator,
        "suppressed_downstream_propagations": after.evaluation.suppressed_downstream_propagations - before.evaluation.suppressed_downstream_propagations,
        "rewiring_apply_count": after.execution.rewiring_apply_count - before.execution.rewiring_apply_count,
        "dependency_capture_updates": after.execution.dependency_capture_updates - before.execution.dependency_capture_updates,
        "graph_storage_compaction_count": after.storage.graph_storage_compaction_count - before.storage.graph_storage_compaction_count,
        "dependency_segments_rewritten": after.storage.graph_storage_dependency_segments_rewritten - before.storage.graph_storage_dependency_segments_rewritten,
        "subscriber_segments_rewritten": after.storage.graph_storage_subscriber_segments_rewritten - before.storage.graph_storage_subscriber_segments_rewritten,
    })
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_fintech_mixed_fanout_profile_matrix() {
    let profiles = [
        ("operational", SignalRuntimePolicy::operational().with_history_limit(4)),
        ("development", SignalRuntimePolicy::development().with_history_limit(6)),
        ("forensic", SignalRuntimePolicy::forensic().with_history_limit(8)),
    ];

    for (profile_name, policy) in profiles {
        let mut world =
            setup_seeded_world_with(FintechScale::fanout(), MarketRegime::Calm, 7);
        world.set_runtime_policy(policy);

        let before = world.runtime_metrics();
        let start = Instant::now();
        let _ = world.read_top_desk_with_executor(StageExecutor::Serial).unwrap();
        let _ = world.read_top_scenario_with_executor(StageExecutor::Serial).unwrap();
        let _ = world
            .bump_primary_market(7, 4, 2, 1, StageExecutor::Serial)
            .unwrap();
        let _ = world.read_top_desk_with_executor(StageExecutor::Serial).unwrap();
        let _ = world.read_top_scenario_with_executor(StageExecutor::Serial).unwrap();
        let elapsed = start.elapsed();
        let after = world.runtime_metrics();

        let record = PerfRecord {
            suite: "fintech_mixed_fanout",
            profile: profile_name,
            executor: "serial",
            elapsed_micros: elapsed.as_micros(),
            metrics: eval_metrics_delta(before, after),
        };
        emit(&record);

        assert!(record.elapsed_micros > 0);
        assert!(after.evaluation.evaluation_calls >= before.evaluation.evaluation_calls);
    }
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_topology_rewiring_churn_serial() {
    let mut graph = SignalGraph::new();
    let sources = (0..32).map(|_| graph.node().build()).collect::<Vec<_>>();
    let leaves = (0..256).map(|_| graph.node().build()).collect::<Vec<_>>();

    for (index, &leaf) in leaves.iter().enumerate() {
        graph.add_dependency(leaf, sources[index % sources.len()], ASPECT_A)
            .unwrap();
    }

    let before = graph.observe().metrics();
    let start = Instant::now();
    for round in 0..48 {
        for (index, &leaf) in leaves.iter().enumerate() {
            let old = sources[(index + round) % sources.len()];
            let new = sources[(index + round + 1) % sources.len()];
            let _ = graph.remove_dependency(leaf, old, ASPECT_A);
            graph.add_dependency(leaf, new, ASPECT_A).unwrap();
        }
    }
    let elapsed = start.elapsed();
    let after = graph.observe().metrics();

    let record = PerfRecord {
        suite: "topology_rewiring_churn",
        profile: "balanced",
        executor: "serial",
        elapsed_micros: elapsed.as_micros(),
        metrics: graph_metrics_delta(before, after),
    };
    emit(&record);

    graph.assert_bidirectional_consistency().unwrap();
    assert!(record.elapsed_micros > 0);
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_topology_rewiring_rotating_window_serial() {
    let mut graph = SignalGraph::new();
    let sources = (0..64).map(|_| graph.node().build()).collect::<Vec<_>>();
    let leaves = (0..512).map(|_| graph.node().build()).collect::<Vec<_>>();
    let window = 8usize;

    for (index, &leaf) in leaves.iter().enumerate() {
        for offset in 0..window {
            let source = sources[(index + offset) % sources.len()];
            graph.add_dependency(leaf, source, ASPECT_A).unwrap();
        }
    }

    let before = graph.observe().metrics();
    let start = Instant::now();
    for round in 0..24 {
        for (index, &leaf) in leaves.iter().enumerate() {
            for offset in 0..window {
                let old = sources[(index + round + offset) % sources.len()];
                let new = sources[(index + round + offset + 1) % sources.len()];
                let _ = graph.remove_dependency(leaf, old, ASPECT_A);
                graph.add_dependency(leaf, new, ASPECT_A).unwrap();
            }
        }
    }
    let elapsed = start.elapsed();
    let after = graph.observe().metrics();

    let record = PerfRecord {
        suite: "topology_rewiring_rotating_window",
        profile: "balanced",
        executor: "serial",
        elapsed_micros: elapsed.as_micros(),
        metrics: graph_metrics_delta(before, after),
    };
    emit(&record);

    graph.assert_bidirectional_consistency().unwrap();
    assert!(record.elapsed_micros > 0);
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_dependency_reconciliation_rotating_window_serial() {
    let mut graph = SignalGraph::new();
    let sources = (0..64).map(|_| graph.node().build()).collect::<Vec<_>>();
    let leaves = (0..512).map(|_| graph.node().build()).collect::<Vec<_>>();
    let window = 8usize;

    for (index, &leaf) in leaves.iter().enumerate() {
        let mut desired = (0..window)
            .map(|offset| DependencyEdge::new(sources[(index + offset) % sources.len()], ASPECT_A))
            .collect::<Vec<_>>();
        desired.sort_unstable_by_key(|edge| edge.sort_key());
        graph.reconcile_dependencies(leaf, &desired).unwrap();
    }

    let before = graph.observe().metrics();
    let start = Instant::now();
    for round in 0..24 {
        for (index, &leaf) in leaves.iter().enumerate() {
            let mut desired = (0..window)
                .map(|offset| {
                    DependencyEdge::new(
                        sources[(index + round + offset + 1) % sources.len()],
                        ASPECT_A,
                    )
                })
                .collect::<Vec<_>>();
            desired.sort_unstable_by_key(|edge| edge.sort_key());
            graph.reconcile_dependencies(leaf, &desired).unwrap();
        }
    }
    let elapsed = start.elapsed();
    let after = graph.observe().metrics();

    let record = PerfRecord {
        suite: "dependency_reconciliation_rotating_window",
        profile: "balanced",
        executor: "serial",
        elapsed_micros: elapsed.as_micros(),
        metrics: graph_metrics_delta(before, after),
    };
    emit(&record);

    graph.assert_bidirectional_consistency().unwrap();
    assert!(record.elapsed_micros > 0);
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_suppression_wide_fanout_serial() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let source = runtime.graph_mut().node().build();
    let middle = runtime.graph_mut().node().tolerance(2).build();
    let leaves = (0..128)
        .map(|_| runtime.graph_mut().node().tolerance(2).build())
        .collect::<Vec<_>>();

    runtime.graph_mut().add_dependency(middle, source, ASPECT_A).unwrap();
    for &leaf in &leaves {
        runtime.graph_mut().add_dependency(leaf, middle, ASPECT_A).unwrap();
    }

    let evaluator = move |ctx: &mut EvaluationContext<'_, ()>| -> Result<EvaluationOutput, SignalError> {
        let node = ctx.node();
        let result = if node == source {
            let current = ctx.graph().get_entry(source)?.get_aspect_version().get(ASPECT_A);
            let next = if current == 0 { 10 } else { 12 };
            version_ab(next, 0)
        } else if node == middle {
            let source_version = ctx.read_aspect_version(source, ASPECT_A)?.get(ASPECT_A);
            let version = if source_version <= 10 { 100 } else { 102 };
            version_ab(version, 0)
        } else {
            let middle_version = ctx.read_aspect_version(middle, ASPECT_A)?.get(ASPECT_A);
            let version = if middle_version <= 100 { 1_000 } else { 1_002 };
            version_ab(version, 0)
        };
        Ok(EvaluationOutput::from_result(result))
    };

    let _ = runtime.read_with_executor(source, &(), &evaluator, StageExecutor::Serial).unwrap();
    let _ = runtime.read_with_executor(middle, &(), &evaluator, StageExecutor::Serial).unwrap();
    for &leaf in &leaves {
        let _ = runtime.read_with_executor(leaf, &(), &evaluator, StageExecutor::Serial).unwrap();
    }

    let before = runtime.observe().metrics();
    let start = Instant::now();
    runtime.transaction(&mut (), |tx| {
        tx.mark_dirty(source, ASPECT_A)?;
        tx.read(source, &|ctx| {
            Ok(ctx.finish(NodeEvaluationResult::from_version(version_ab(12, 0))))
        })?;
        Ok(())
    }).unwrap();
    for &leaf in &leaves {
        let _ = runtime.read_with_executor(leaf, &(), &evaluator, StageExecutor::Serial).unwrap();
    }
    let elapsed = start.elapsed();
    let after = runtime.observe().metrics();

    let record = PerfRecord {
        suite: "suppression_wide_fanout",
        profile: "balanced",
        executor: "serial",
        elapsed_micros: elapsed.as_micros(),
        metrics: eval_metrics_delta(before, after),
    };
    emit(&record);

    assert!(record.metrics["tasks_pruned_before_execution"].as_u64().unwrap_or(0) > 0);
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_harness_observability_profile_delta() {
    let mut scenario = SignalScenario::new("perf-observability-profile");
    let source = scenario.node("source");
    let dependent = scenario.node("dependent");
    scenario
        .graph_mut()
        .add_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let fixture = scenario
        .input("source")
        .observe("dependent")
        .with_evaluator(move |ctx: &mut EvaluationContext<'_, ()>| {
            let version = if ctx.node() == source { 1 } else { 10 };
            Ok(EvaluationOutput::from_result(version_ab(version, 0)))
        })
        .fixture()
        .unwrap();

    let request =
        forge_harness::facade::ExecutionRequest::target("observe-dependent", "dependent".to_string());

    let profiles = [
        ("development", SignalProfileCatalog::development("development")),
        ("forensic", SignalProfileCatalog::forensic("forensic")),
    ];

    for (profile_name, profile) in profiles {
        let start = Instant::now();
        let bundle = signal_bench(fixture.clone(), request.clone())
            .observe(&profile)
            .unwrap();
        let elapsed = start.elapsed();

        let metrics = json!({
            "explanations": bundle.explanations.len(),
            "provenance": bundle.provenance.len(),
            "has_diagnostics": bundle.diagnostics.is_some(),
            "tasks_executed": bundle.core.run.summary.get("tasks_executed").and_then(|value| value.as_u64()).unwrap_or(0),
            "tasks_pruned": bundle.core.run.summary.get("tasks_pruned").and_then(|value| value.as_u64()).unwrap_or(0),
        });
        let record = PerfRecord {
            suite: "harness_observability_profile",
            profile: profile_name,
            executor: "serial",
            elapsed_micros: elapsed.as_micros(),
            metrics,
        };
        emit(&record);

        assert!(record.elapsed_micros > 0);
    }
}
