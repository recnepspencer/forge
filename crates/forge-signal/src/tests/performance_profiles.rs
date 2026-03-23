use std::time::Instant;

use serde_json::{json, Value};

use super::performance_support::{capture_perf_samples, PerfMeasurement};
use crate::data::dependency::DependencyEdge;
use crate::facade::*;
use crate::logic::prepared::{PreparedDependencyCapture, PreparedEvaluation};
use crate::presentation::harness::{signal_bench, SignalProfileCatalog, SignalScenario};
use crate::tests::domains::fintech::{setup_seeded_world_with, FintechScale, MarketRegime};
use crate::tests::support::{version_ab, DependencyBatchBuilder, ASPECT_A};

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
        "dependency_reconcile_nanos": after.execution.dependency_reconcile_nanos - before.execution.dependency_reconcile_nanos,
        "dependency_input_build_nanos": after.execution.dependency_input_build_nanos - before.execution.dependency_input_build_nanos,
        "deferred_snapshot_packet_nanos": after.execution.deferred_snapshot_packet_nanos - before.execution.deferred_snapshot_packet_nanos,
        "graph_storage_compaction_count": after.storage.graph_storage_compaction_count - before.storage.graph_storage_compaction_count,
        "dependency_segments_rewritten": after.storage.graph_storage_dependency_segments_rewritten - before.storage.graph_storage_dependency_segments_rewritten,
        "subscriber_segments_rewritten": after.storage.graph_storage_subscriber_segments_rewritten - before.storage.graph_storage_subscriber_segments_rewritten,
        "snapshot_batch_commit_nanos": after.storage.snapshot_batch_commit_nanos - before.storage.snapshot_batch_commit_nanos,
    })
}

fn with_perf_topology_asserts_disabled<T>(run: impl FnOnce() -> T) -> T {
    let previous = std::env::var_os("FORGE_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS");
    std::env::set_var("FORGE_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS", "1");
    let result = run();
    match previous {
        Some(value) => std::env::set_var("FORGE_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS", value),
        None => std::env::remove_var("FORGE_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS"),
    }
    result
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_fintech_mixed_fanout_profile_matrix() {
    for profile_name in ["operational", "development", "forensic"] {
        let samples = capture_perf_samples("fintech_mixed_fanout", profile_name, "serial", || {
            let mut world = setup_seeded_world_with(FintechScale::fanout(), MarketRegime::Calm, 7);
            world.set_runtime_policy(policy_for(profile_name));

            let before = world.runtime_metrics();
            let start = Instant::now();
            let _ = world
                .read_top_desk_with_executor(StageExecutor::Serial)
                .unwrap();
            let _ = world
                .read_top_scenario_with_executor(StageExecutor::Serial)
                .unwrap();
            let _ = world
                .bump_primary_market(7, 4, 2, 1, StageExecutor::Serial)
                .unwrap();
            let _ = world
                .read_top_desk_with_executor(StageExecutor::Serial)
                .unwrap();
            let _ = world
                .read_top_scenario_with_executor(StageExecutor::Serial)
                .unwrap();
            let elapsed = start.elapsed();
            let after = world.runtime_metrics();

            assert!(after.evaluation.evaluation_calls >= before.evaluation.evaluation_calls);
            PerfMeasurement::new(elapsed.as_micros(), eval_metrics_delta(before, after))
        });

        assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
    }
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_topology_rewiring_churn_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        capture_perf_samples("topology_rewiring_churn", "balanced", "serial", || {
            let mut graph = SignalGraph::new();
            let sources = (0..32).map(|_| graph.node().build()).collect::<Vec<_>>();
            let leaves = (0..256).map(|_| graph.node().build()).collect::<Vec<_>>();

        for (index, &leaf) in leaves.iter().enumerate() {
            graph
                .append_dependency(leaf, sources[index % sources.len()], ASPECT_A)
                .unwrap();
        }

        let before = graph.observe().metrics();
        let start = Instant::now();
        for round in 0..48 {
            for (index, &leaf) in leaves.iter().enumerate() {
                let old = sources[(index + round) % sources.len()];
                let new = sources[(index + round + 1) % sources.len()];
                let _ = graph.drop_dependency(leaf, old, ASPECT_A);
                graph.append_dependency(leaf, new, ASPECT_A).unwrap();
            }
        }
        let elapsed = start.elapsed();
        let after = graph.observe().metrics();

        graph.assert_bidirectional_consistency().unwrap();
            PerfMeasurement::new(elapsed.as_micros(), graph_metrics_delta(before, after))
        })
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_topology_rewiring_rotating_window_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        capture_perf_samples(
            "topology_rewiring_rotating_window",
            "balanced",
            "serial",
            || {
                let mut graph = SignalGraph::new();
                let sources = (0..64).map(|_| graph.node().build()).collect::<Vec<_>>();
                let leaves = (0..512).map(|_| graph.node().build()).collect::<Vec<_>>();
            let window = 8usize;

            for (index, &leaf) in leaves.iter().enumerate() {
                for offset in 0..window {
                    let source = sources[(index + offset) % sources.len()];
                    graph.append_dependency(leaf, source, ASPECT_A).unwrap();
                }
            }

            let before = graph.observe().metrics();
            let start = Instant::now();
            for round in 0..24 {
                for (index, &leaf) in leaves.iter().enumerate() {
                    for offset in 0..window {
                        let old = sources[(index + round + offset) % sources.len()];
                        let new = sources[(index + round + offset + 1) % sources.len()];
                        let _ = graph.drop_dependency(leaf, old, ASPECT_A);
                        graph.append_dependency(leaf, new, ASPECT_A).unwrap();
                    }
                }
            }
            let elapsed = start.elapsed();
            let after = graph.observe().metrics();

            graph.assert_bidirectional_consistency().unwrap();
                PerfMeasurement::new(elapsed.as_micros(), graph_metrics_delta(before, after))
            },
        )
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_dependency_reconciliation_rotating_window_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        capture_perf_samples(
            "dependency_reconciliation_rotating_window",
            "balanced",
            "serial",
            || {
                let mut graph = SignalGraph::new();
                let sources = (0..64).map(|_| graph.node().build()).collect::<Vec<_>>();
                let leaves = (0..512).map(|_| graph.node().build()).collect::<Vec<_>>();
            let window = 8usize;

            for (index, &leaf) in leaves.iter().enumerate() {
                let mut desired = (0..window)
                    .map(|offset| {
                        DependencyEdge::new(sources[(index + offset) % sources.len()], ASPECT_A)
                    })
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

            graph.assert_bidirectional_consistency().unwrap();
                PerfMeasurement::new(elapsed.as_micros(), graph_metrics_delta(before, after))
            },
        )
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_dependency_reconciliation_rotating_window_staged_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        capture_perf_samples(
            "dependency_reconciliation_rotating_window_staged",
            "balanced",
            "serial",
            || {
                let mut graph = SignalGraph::new();
                graph.set_runtime_policy(SignalRuntimePolicy::development());
            let sources = (0..64).map(|_| graph.node().build()).collect::<Vec<_>>();
            let leaves = (0..512).map(|_| graph.node().build()).collect::<Vec<_>>();
            let window = 8usize;
            let max_index = leaves
                .iter()
                .chain(sources.iter())
                .map(|node| node.index() as usize)
                .max()
                .unwrap_or(0);
            let mut leaf_positions = vec![usize::MAX; max_index + 1];
            for (index, leaf) in leaves.iter().enumerate() {
                leaf_positions[leaf.index() as usize] = index;
            }

            let bootstrap = graph
                .build_evaluation_plan(&leaves, EvaluationRequestMode::Default)
                .unwrap();
            graph
                .execute_prepared_plan_with_precompute(&bootstrap, &|node, _view| {
                    let leaf_index = leaf_positions[node.index() as usize];
                    if leaf_index == usize::MAX {
                        return Ok(PreparedEvaluation::from_result(
                            NodeEvaluationResult::from_version(version_ab(1, 0)),
                        ));
                    }
                    let mut capture = PreparedDependencyCapture::new();
                    for offset in 0..window {
                        capture.record(
                            sources[(leaf_index + offset) % sources.len()],
                            ASPECT_A,
                            None,
                        );
                    }
                    Ok(
                        PreparedEvaluation::from_result(NodeEvaluationResult::from_version(
                            version_ab(1, 0),
                        ))
                        .with_dependencies(capture),
                    )
                })
                .unwrap();

            let before = graph.observe().metrics();
            let start = Instant::now();
            let mut planning_nanos = 0_u128;
            let mut report_precompute_nanos = 0_u128;
            let mut report_apply_nanos = 0_u128;
            let mut report_semantic_finalize_nanos = 0_u128;
            for round in 0..24 {
                for &leaf in &leaves {
                    mark_dirty(&mut graph, leaf, ASPECT_A).unwrap();
                }
                let planning_start = Instant::now();
                let plan = graph
                    .build_evaluation_plan(&leaves, EvaluationRequestMode::Default)
                    .unwrap();
                planning_nanos += planning_start.elapsed().as_nanos();
                let report = graph
                    .execute_prepared_plan_with_precompute(&plan, &|node, _view| {
                        let leaf_index = leaf_positions[node.index() as usize];
                        if leaf_index == usize::MAX {
                            return Ok(PreparedEvaluation::from_result(
                                NodeEvaluationResult::from_version(version_ab(
                                    (round + 2) as u64,
                                    0,
                                )),
                            ));
                        }
                        let mut capture = PreparedDependencyCapture::new();
                        for offset in 0..window {
                            capture.record(
                                sources[(leaf_index + round + offset + 1) % sources.len()],
                                ASPECT_A,
                                None,
                            );
                        }
                        Ok(
                            PreparedEvaluation::from_result(NodeEvaluationResult::from_version(
                                version_ab((round + 2) as u64, 0),
                            ))
                            .with_dependencies(capture),
                        )
                    })
                    .unwrap();
                report_precompute_nanos += report.stage_precompute_nanos;
                report_apply_nanos += report.stage_apply_nanos;
                report_semantic_finalize_nanos += report.semantic_finalize_nanos;
            }
            let elapsed = start.elapsed();
            let after = graph.observe().metrics();

            graph.assert_bidirectional_consistency().unwrap();
            let mut metrics = graph_metrics_delta(before, after);
            if let Value::Object(ref mut map) = metrics {
                map.insert("planning_nanos".into(), json!(planning_nanos));
                map.insert(
                    "report_stage_precompute_nanos".into(),
                    json!(report_precompute_nanos),
                );
                map.insert("report_stage_apply_nanos".into(), json!(report_apply_nanos));
                map.insert(
                    "report_semantic_finalize_nanos".into(),
                    json!(report_semantic_finalize_nanos),
                );
            }
            PerfMeasurement::new(elapsed.as_micros(), metrics)
            },
        )
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_suppression_wide_fanout_serial() {
    let samples =
        capture_perf_samples("suppression_wide_fanout", "balanced", "serial", || {
            let mut runtime = SignalRuntime::builder(SignalGraph::new())
                .with_kernel_defaults()
                .build();

            let source = runtime.graph_mut().node().build();
            let middle = runtime.graph_mut().node().tolerance(2).build();
            let leaves = (0..128)
                .map(|_| runtime.graph_mut().node().tolerance(2).build())
                .collect::<Vec<_>>();

            let mut dependencies = DependencyBatchBuilder::new(runtime.graph_mut());
            dependencies
                .append_dependency(middle, source, ASPECT_A)
                .unwrap();
            for &leaf in &leaves {
                dependencies
                    .append_dependency(leaf, middle, ASPECT_A)
                    .unwrap();
            }
            dependencies.commit().unwrap();

            let evaluator =
            move |ctx: &mut EvaluationContext<'_, ()>| -> Result<EvaluationOutput, SignalError> {
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

            let _ = runtime
                .read_with_executor(source, &(), &evaluator, StageExecutor::Serial)
                .unwrap();
            let _ = runtime
                .read_with_executor(middle, &(), &evaluator, StageExecutor::Serial)
                .unwrap();
            for &leaf in &leaves {
                let _ = runtime
                    .read_with_executor(leaf, &(), &evaluator, StageExecutor::Serial)
                    .unwrap();
            }

            let before = runtime.observe().metrics();
            let start = Instant::now();
            runtime
                .transaction(&mut (), |tx| {
                    tx.mark_dirty(source, ASPECT_A)?;
                    tx.read(source, &|ctx| {
                        Ok(ctx.finish(NodeEvaluationResult::from_version(version_ab(12, 0))))
                    })?;
                    Ok(())
                })
                .unwrap();
            for &leaf in &leaves {
                let _ = runtime
                    .read_with_executor(leaf, &(), &evaluator, StageExecutor::Serial)
                    .unwrap();
            }
            let elapsed = start.elapsed();
            let after = runtime.observe().metrics();

            PerfMeasurement::new(elapsed.as_micros(), eval_metrics_delta(before, after))
        });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
    assert!(samples.iter().all(|sample| {
        sample.metrics["tasks_pruned_before_execution"]
            .as_u64()
            .unwrap_or(0)
            > 0
            || sample.metrics["skipped_by_comparator"].as_u64().unwrap_or(0) > 0
            || sample.metrics["suppressed_downstream_propagations"]
                .as_u64()
                .unwrap_or(0)
                > 0
    }));
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_harness_observability_profile_delta() {
    for profile_name in ["development", "forensic"] {
        let samples = capture_perf_samples(
            "harness_observability_profile",
            profile_name,
            "serial",
            || {
                let mut scenario = SignalScenario::new("perf-observability-profile");
                let source = scenario.node("source");
                let dependent = scenario.node("dependent");
                scenario
                    .graph_mut()
                    .append_dependency(dependent, source, ASPECT_A)
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

                let request = forge_harness::facade::ExecutionRequest::target(
                    "observe-dependent",
                    "dependent".to_string(),
                );
                let profile = match profile_name {
                    "development" => SignalProfileCatalog::development("development"),
                    "forensic" => SignalProfileCatalog::forensic("forensic"),
                    other => panic!("unexpected profile for perf test: {other}"),
                };

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

                PerfMeasurement::new(elapsed.as_micros(), metrics)
            },
        );

        assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
    }
}

fn policy_for(profile_name: &str) -> SignalRuntimePolicy {
    match profile_name {
        "operational" => SignalRuntimePolicy::operational().with_history_limit(4),
        "development" => SignalRuntimePolicy::development().with_history_limit(6),
        "forensic" => SignalRuntimePolicy::forensic().with_history_limit(8),
        other => panic!("unexpected profile for perf test: {other}"),
    }
}
