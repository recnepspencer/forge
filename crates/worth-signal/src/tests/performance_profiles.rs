use std::sync::Once;
use std::time::Instant;

use serde_json::{json, Value};

use super::performance_support::{
    capture_and_certify_perf_samples, PerfCaseContract, PerfMeasurement, PerfTimingPolicy,
};
use crate::data::dependency::DependencyEdge;
use crate::data::proof::DependencyBatchEdit;
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
        "dependency_input_shape_handle_lookup_nanos": after.execution.dependency_input_shape_handle_lookup_nanos - before.execution.dependency_input_shape_handle_lookup_nanos,
        "dependency_input_previous_snapshot_fetch_nanos": after.execution.dependency_input_previous_snapshot_fetch_nanos - before.execution.dependency_input_previous_snapshot_fetch_nanos,
        "dependency_input_version_scan_nanos": after.execution.dependency_input_version_scan_nanos - before.execution.dependency_input_version_scan_nanos,
        "dependency_input_stable_proof_nanos": after.execution.dependency_input_stable_proof_nanos - before.execution.dependency_input_stable_proof_nanos,
        "dependency_input_version_delta_nanos": after.execution.dependency_input_version_delta_nanos - before.execution.dependency_input_version_delta_nanos,
        "dependency_input_replacement_build_nanos": after.execution.dependency_input_replacement_build_nanos - before.execution.dependency_input_replacement_build_nanos,
        "dependency_input_stable_shape_count": after.execution.dependency_input_stable_shape_count - before.execution.dependency_input_stable_shape_count,
        "dependency_input_replacement_count": after.execution.dependency_input_replacement_count - before.execution.dependency_input_replacement_count,
        "deferred_snapshot_packet_nanos": after.execution.deferred_snapshot_packet_nanos - before.execution.deferred_snapshot_packet_nanos,
        "graph_storage_compaction_count": after.storage.graph_storage_compaction_count - before.storage.graph_storage_compaction_count,
        "dependency_segments_rewritten": after.storage.graph_storage_dependency_segments_rewritten - before.storage.graph_storage_dependency_segments_rewritten,
        "subscriber_segments_rewritten": after.storage.graph_storage_subscriber_segments_rewritten - before.storage.graph_storage_subscriber_segments_rewritten,
        "snapshot_batch_commit_nanos": after.storage.snapshot_batch_commit_nanos - before.storage.snapshot_batch_commit_nanos,
    })
}

fn with_perf_topology_asserts_disabled<T>(run: impl FnOnce() -> T) -> T {
    let previous = std::env::var_os("WORTH_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS");
    std::env::set_var("WORTH_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS", "1");
    let result = run();
    match previous {
        Some(value) => std::env::set_var("WORTH_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS", value),
        None => std::env::remove_var("WORTH_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS"),
    }
    result
}

fn perf_contract<'a>(
    suite: &'a str,
    profile: &'a str,
    timing_policy: PerfTimingPolicy,
    phase_metrics: &'a [&'a str],
) -> PerfCaseContract<'a> {
    PerfCaseContract::new(suite, profile, "serial", timing_policy, phase_metrics, &[])
}

fn hot_family_contract<'a>(
    suite: &'a str,
    profile: &'a str,
    timing_policy: PerfTimingPolicy,
    phase_metrics: &'a [&'a str],
    access_counter_maxima: &'a [(&'a str, u128)],
) -> PerfCaseContract<'a> {
    PerfCaseContract::new(
        suite,
        profile,
        "serial",
        timing_policy,
        phase_metrics,
        access_counter_maxima,
    )
}

const ZERO_BROAD_ENTRY_ACCESS: &[(&str, u128)] = &[
    ("materialized_entry_reads", 0),
    ("materialized_entry_writes", 0),
];

const ZERO_BROAD_AND_ARTIFACT_ACCESS: &[(&str, u128)] = &[
    ("materialized_entry_reads", 0),
    ("materialized_entry_writes", 0),
    ("runtime_artifact_state_reads", 0),
    ("runtime_artifact_warm_reads", 0),
];

fn build_chain_graph(count: usize) -> (SignalGraph, Vec<NodeId>) {
    let mut graph = SignalGraph::new();
    graph.reserve_node_capacity(count);
    let mut chain = Vec::with_capacity(count);
    let mut dependency_edits = Vec::with_capacity(count.saturating_sub(1));

    let first = graph.create_node();
    chain.push(first);

    for index in 1..count {
        let node = graph.create_node();
        dependency_edits.push((
            node,
            vec![DependencyEdge::new(
                chain[index - 1],
                crate::tests::support::ASPECT_B,
            )],
        ));
        chain.push(node);
    }

    graph
        .apply_dependency_batch_edit(&DependencyBatchEdit::from_pairs(dependency_edits))
        .unwrap();

    (graph, chain)
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_fintech_mixed_fanout_profile_matrix() {
    for profile_name in ["operational", "development", "forensic"] {
        match profile_name {
            "operational" => {
                static OPERATIONAL_WARMUP: Once = Once::new();
                OPERATIONAL_WARMUP.call_once(|| {
                    let mut world =
                        setup_seeded_world_with(FintechScale::fanout(), MarketRegime::Calm, 7);
                    world.set_runtime_policy(policy_for("operational"));
                    let _ = world.read_top_desk_with_executor(StageExecutor::Serial);
                    let _ = world.read_top_scenario_with_executor(StageExecutor::Serial);
                    let _ = world.bump_primary_market(7, 4, 2, 1, StageExecutor::Serial);
                    let _ = world.read_top_desk_with_executor(StageExecutor::Serial);
                    let _ = world.read_top_scenario_with_executor(StageExecutor::Serial);
                });
            }
            "development" => {
                static DEVELOPMENT_WARMUP: Once = Once::new();
                DEVELOPMENT_WARMUP.call_once(|| {
                    let mut world =
                        setup_seeded_world_with(FintechScale::fanout(), MarketRegime::Calm, 7);
                    world.set_runtime_policy(policy_for("development"));
                    let _ = world.read_top_desk_with_executor(StageExecutor::Serial);
                    let _ = world.read_top_scenario_with_executor(StageExecutor::Serial);
                    let _ = world.bump_primary_market(7, 4, 2, 1, StageExecutor::Serial);
                    let _ = world.read_top_desk_with_executor(StageExecutor::Serial);
                    let _ = world.read_top_scenario_with_executor(StageExecutor::Serial);
                });
            }
            "forensic" => {
                static FORENSIC_WARMUP: Once = Once::new();
                FORENSIC_WARMUP.call_once(|| {
                    let mut world =
                        setup_seeded_world_with(FintechScale::fanout(), MarketRegime::Calm, 7);
                    world.set_runtime_policy(policy_for("forensic"));
                    let _ = world.read_top_desk_with_executor(StageExecutor::Serial);
                    let _ = world.read_top_scenario_with_executor(StageExecutor::Serial);
                    let _ = world.bump_primary_market(7, 4, 2, 1, StageExecutor::Serial);
                    let _ = world.read_top_desk_with_executor(StageExecutor::Serial);
                    let _ = world.read_top_scenario_with_executor(StageExecutor::Serial);
                });
            }
            other => panic!("unexpected profile for perf test: {other}"),
        }

        let samples = capture_and_certify_perf_samples(
            perf_contract(
                "fintech_mixed_fanout",
                profile_name,
                match profile_name {
                    "operational" => PerfTimingPolicy::StrictHeavy,
                    "development" | "forensic" => PerfTimingPolicy::MedianOnly,
                    other => panic!("unexpected profile for perf test: {other}"),
                },
                &["read_before_nanos", "mutation_nanos", "read_after_nanos"],
            ),
            || {
                let mut world =
                    setup_seeded_world_with(FintechScale::fanout(), MarketRegime::Calm, 7);
                world.set_runtime_policy(policy_for(profile_name));

                let warmup_start = Instant::now();
                let _ = world
                    .read_top_desk_with_executor(StageExecutor::Serial)
                    .unwrap();
                let _ = world
                    .read_top_scenario_with_executor(StageExecutor::Serial)
                    .unwrap();
                let warmup_nanos = warmup_start.elapsed().as_nanos();

                let before = world.runtime_metrics();
                let read_before_start = Instant::now();
                let _ = world
                    .read_top_desk_with_executor(StageExecutor::Serial)
                    .unwrap();
                let _ = world
                    .read_top_scenario_with_executor(StageExecutor::Serial)
                    .unwrap();
                let read_before_nanos = read_before_start.elapsed().as_nanos();

                let mutation_start = Instant::now();
                let _ = world
                    .bump_primary_market(7, 4, 2, 1, StageExecutor::Serial)
                    .unwrap();
                let mutation_nanos = mutation_start.elapsed().as_nanos();

                let read_after_start = Instant::now();
                let _ = world
                    .read_top_desk_with_executor(StageExecutor::Serial)
                    .unwrap();
                let _ = world
                    .read_top_scenario_with_executor(StageExecutor::Serial)
                    .unwrap();
                let read_after_nanos = read_after_start.elapsed().as_nanos();
                let after = world.runtime_metrics();

                assert!(after.evaluation.evaluation_calls >= before.evaluation.evaluation_calls);
                let mut metrics = eval_metrics_delta(before, after);
                if let Value::Object(ref mut map) = metrics {
                    map.insert("warmup_nanos".into(), json!(warmup_nanos));
                    map.insert("read_before_nanos".into(), json!(read_before_nanos));
                    map.insert("mutation_nanos".into(), json!(mutation_nanos));
                    map.insert("read_after_nanos".into(), json!(read_after_nanos));
                }
                PerfMeasurement::new(
                    (read_before_nanos + mutation_nanos + read_after_nanos) as u128 / 1_000,
                    metrics,
                )
            },
        );

        assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
    }
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_topology_rewiring_churn_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        static TOPOLOGY_REWIRE_CHURN_WARMUP: Once = Once::new();
        TOPOLOGY_REWIRE_CHURN_WARMUP.call_once(|| {
            let mut graph = SignalGraph::new();
            let sources = (0..32).map(|_| graph.node().build()).collect::<Vec<_>>();
            let leaves = (0..256).map(|_| graph.node().build()).collect::<Vec<_>>();

            for (index, &leaf) in leaves.iter().enumerate() {
                graph
                    .append_dependency(leaf, sources[index % sources.len()], ASPECT_A)
                    .unwrap();
            }

            for round in 0..48 {
                for (index, &leaf) in leaves.iter().enumerate() {
                    let old = sources[(index + round) % sources.len()];
                    let new = sources[(index + round + 1) % sources.len()];
                    graph.rewire_dependency(leaf, old, new, ASPECT_A).unwrap();
                }
            }
        });

        capture_and_certify_perf_samples(
            hot_family_contract(
                "topology_rewiring_churn",
                "balanced",
                PerfTimingPolicy::MedianOnly,
                &["rewire_loop_nanos"],
                ZERO_BROAD_AND_ARTIFACT_ACCESS,
            ),
            || {
                let mut graph = SignalGraph::new();
                let sources = (0..32).map(|_| graph.node().build()).collect::<Vec<_>>();
                let leaves = (0..256).map(|_| graph.node().build()).collect::<Vec<_>>();

                for (index, &leaf) in leaves.iter().enumerate() {
                    graph
                        .append_dependency(leaf, sources[index % sources.len()], ASPECT_A)
                        .unwrap();
                }

                let before = graph.observe().metrics();
                let rewire_start = Instant::now();
                for round in 0..48 {
                    for (index, &leaf) in leaves.iter().enumerate() {
                        let old = sources[(index + round) % sources.len()];
                        let new = sources[(index + round + 1) % sources.len()];
                        graph.rewire_dependency(leaf, old, new, ASPECT_A).unwrap();
                    }
                }
                let rewire_loop_nanos = rewire_start.elapsed().as_nanos();
                let after = graph.observe().metrics();

                let mut metrics = graph_metrics_delta(before, after);
                if let Value::Object(ref mut map) = metrics {
                    map.insert("rewire_loop_nanos".into(), json!(rewire_loop_nanos));
                }
                PerfMeasurement::new(rewire_loop_nanos as u128 / 1_000, metrics)
            },
        )
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_topology_rewiring_rotating_window_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        static TOPOLOGY_REWIRE_WINDOW_WARMUP: Once = Once::new();
        TOPOLOGY_REWIRE_WINDOW_WARMUP.call_once(|| {
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

            for round in 0..24 {
                for (index, &leaf) in leaves.iter().enumerate() {
                    for offset in 0..window {
                        let old = sources[(index + round + offset) % sources.len()];
                        let new = sources[(index + round + offset + 1) % sources.len()];
                        graph.rewire_dependency(leaf, old, new, ASPECT_A).unwrap();
                    }
                }
            }
        });

        capture_and_certify_perf_samples(
            hot_family_contract(
                "topology_rewiring_rotating_window",
                "balanced",
                PerfTimingPolicy::MedianOnly,
                &["rewire_loop_nanos"],
                ZERO_BROAD_AND_ARTIFACT_ACCESS,
            ),
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
                let rewire_start = Instant::now();
                for round in 0..24 {
                    for (index, &leaf) in leaves.iter().enumerate() {
                        for offset in 0..window {
                            let old = sources[(index + round + offset) % sources.len()];
                            let new = sources[(index + round + offset + 1) % sources.len()];
                            graph.rewire_dependency(leaf, old, new, ASPECT_A).unwrap();
                        }
                    }
                }
                let rewire_loop_nanos = rewire_start.elapsed().as_nanos();
                let after = graph.observe().metrics();

                let mut metrics = graph_metrics_delta(before, after);
                if let Value::Object(ref mut map) = metrics {
                    map.insert("rewire_loop_nanos".into(), json!(rewire_loop_nanos));
                }
                PerfMeasurement::new(rewire_loop_nanos as u128 / 1_000, metrics)
            },
        )
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_chain_10k_bootstrap_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        static CHAIN_10K_WARMUP: Once = Once::new();
        CHAIN_10K_WARMUP.call_once(|| {
            let (mut warm_graph, warm_chain) = build_chain_graph(10_000);
            let warm_plan = warm_graph
                .build_evaluation_plan(&warm_chain, EvaluationRequestMode::Default)
                .unwrap();
            warm_graph
                .execute_prepared_plan(&warm_plan, &(), &|_ctx| Ok(version_ab(0, 1)))
                .unwrap();
        });

        capture_and_certify_perf_samples(
            hot_family_contract(
                "chain_10k_bootstrap",
                "balanced",
                PerfTimingPolicy::MedianOnly,
                &[
                    "build_nanos",
                    "bootstrap_plan_nanos",
                    "bootstrap_execute_nanos",
                ],
                ZERO_BROAD_ENTRY_ACCESS,
            ),
            || {
                let build_start = Instant::now();
                let (mut graph, chain) = build_chain_graph(10_000);
                let build_nanos = build_start.elapsed().as_nanos();

                graph.reset_telemetry();
                let before = graph.observe().metrics();
                let plan_start = Instant::now();
                let plan = graph
                    .build_evaluation_plan(&chain, EvaluationRequestMode::Default)
                    .unwrap();
                let bootstrap_plan_nanos = plan_start.elapsed().as_nanos();

                let execute_start = Instant::now();
                graph
                    .execute_prepared_plan(&plan, &(), &|_ctx| Ok(version_ab(0, 1)))
                    .unwrap();
                let bootstrap_execute_nanos = execute_start.elapsed().as_nanos();

                let push_start = Instant::now();
                mark_dirty(&mut graph, chain[0], crate::tests::support::ASPECT_B).unwrap();
                let push_nanos = push_start.elapsed().as_nanos();
                let after = graph.observe().metrics();

                let mut metrics = graph_metrics_delta(before, after);
                if let Value::Object(ref mut map) = metrics {
                    map.insert("build_nanos".into(), json!(build_nanos));
                    map.insert("bootstrap_plan_nanos".into(), json!(bootstrap_plan_nanos));
                    map.insert(
                        "bootstrap_execute_nanos".into(),
                        json!(bootstrap_execute_nanos),
                    );
                    map.insert("push_nanos".into(), json!(push_nanos));
                    map.insert(
                        "plans_built".into(),
                        json!(graph.telemetry().planner.plans_built),
                    );
                    map.insert(
                        "tasks_scheduled".into(),
                        json!(graph.telemetry().planner.tasks_scheduled),
                    );
                    map.insert(
                        "stage_execution_nanos".into(),
                        json!(graph.telemetry().execution.stage_execution_nanos),
                    );
                }

                PerfMeasurement::new(
                    (build_nanos + bootstrap_plan_nanos + bootstrap_execute_nanos + push_nanos)
                        as u128
                        / 1_000,
                    metrics,
                )
            },
        )
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_dependency_reconciliation_rotating_window_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        capture_and_certify_perf_samples(
            perf_contract(
                "dependency_reconciliation_rotating_window",
                "balanced",
                PerfTimingPolicy::StructuralOnly,
                &[
                    "reconcile_loop_nanos",
                    "dependency_reconcile_nanos",
                    "snapshot_batch_commit_nanos",
                ],
            ),
            || {
                let mut graph = SignalGraph::new();
                let sources = (0..64).map(|_| graph.node().build()).collect::<Vec<_>>();
                let leaves = (0..512).map(|_| graph.node().build()).collect::<Vec<_>>();
                let window = 8usize;
                let initial_desired = leaves
                    .iter()
                    .enumerate()
                    .map(|(index, _)| {
                        let mut desired = (0..window)
                            .map(|offset| {
                                DependencyEdge::new(
                                    sources[(index + offset) % sources.len()],
                                    ASPECT_A,
                                )
                            })
                            .collect::<Vec<_>>();
                        desired.sort_unstable_by_key(|edge| edge.sort_key());
                        desired
                    })
                    .collect::<Vec<_>>();
                let desired_by_round = (0..24)
                    .map(|round| {
                        leaves
                            .iter()
                            .enumerate()
                            .map(|(index, _)| {
                                let mut desired = (0..window)
                                    .map(|offset| {
                                        DependencyEdge::new(
                                            sources[(index + round + offset + 1) % sources.len()],
                                            ASPECT_A,
                                        )
                                    })
                                    .collect::<Vec<_>>();
                                desired.sort_unstable_by_key(|edge| edge.sort_key());
                                desired
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                for (leaf, desired) in leaves.iter().copied().zip(initial_desired.iter()) {
                    graph.reconcile_dependencies(leaf, desired).unwrap();
                }

                let before = graph.observe().metrics();
                let reconcile_start = Instant::now();
                for desired_round in &desired_by_round {
                    for (leaf, desired) in leaves.iter().copied().zip(desired_round.iter()) {
                        graph.reconcile_dependencies(leaf, desired).unwrap();
                    }
                }
                let reconcile_loop_nanos = reconcile_start.elapsed().as_nanos();
                let after = graph.observe().metrics();

                let mut metrics = graph_metrics_delta(before, after);
                if let Value::Object(ref mut map) = metrics {
                    map.insert("reconcile_loop_nanos".into(), json!(reconcile_loop_nanos));
                }
                PerfMeasurement::new(reconcile_loop_nanos as u128 / 1_000, metrics)
            },
        )
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_dependency_reconciliation_rotating_window_staged_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        capture_and_certify_perf_samples(
            perf_contract(
                "dependency_reconciliation_rotating_window_staged",
                "balanced",
                PerfTimingPolicy::StructuralOnly,
                &[
                    "planning_nanos",
                    "report_stage_precompute_nanos",
                    "report_stage_apply_nanos",
                    "report_semantic_finalize_nanos",
                    "dependency_reconcile_nanos",
                    "snapshot_batch_commit_nanos",
                ],
            ),
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
                let access_before_loop = crate::data::access_counters::snapshot();
                let mut planning_materialized_entry_reads = 0_u64;
                let mut planning_runtime_artifact_state_reads = 0_u64;
                let mut planning_runtime_artifact_warm_reads = 0_u64;
                let mut execute_materialized_entry_reads = 0_u64;
                let mut execute_runtime_artifact_state_reads = 0_u64;
                let mut execute_runtime_artifact_warm_reads = 0_u64;
                let mut execute_retained_artifact_reads = 0_u64;
                let leaf_dirty_batch =
                    DirtyBatch::from_sources(leaves.iter().copied().map(|leaf| (leaf, ASPECT_A)));
                for round in 0..24 {
                    mark_dirty_batch(&mut graph, &leaf_dirty_batch).unwrap();
                    let access_before_planning = crate::data::access_counters::snapshot();
                    let planning_start = Instant::now();
                    let plan = graph
                        .build_evaluation_plan(&leaves, EvaluationRequestMode::Default)
                        .unwrap();
                    planning_nanos += planning_start.elapsed().as_nanos();
                    let access_after_planning = crate::data::access_counters::snapshot();
                    let planning_delta = access_after_planning.delta_since(access_before_planning);
                    planning_materialized_entry_reads += planning_delta.materialized_entry_reads;
                    planning_runtime_artifact_state_reads +=
                        planning_delta.runtime_artifact_state_reads;
                    planning_runtime_artifact_warm_reads +=
                        planning_delta.runtime_artifact_warm_reads;
                    let access_before_execute = crate::data::access_counters::snapshot();
                    let report =
                        graph
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
                                Ok(PreparedEvaluation::from_result(
                                    NodeEvaluationResult::from_version(version_ab(
                                        (round + 2) as u64,
                                        0,
                                    )),
                                )
                                .with_dependencies(capture))
                            })
                            .unwrap();
                    let access_after_execute = crate::data::access_counters::snapshot();
                    let execute_delta = access_after_execute.delta_since(access_before_execute);
                    execute_materialized_entry_reads += execute_delta.materialized_entry_reads;
                    execute_runtime_artifact_state_reads +=
                        execute_delta.runtime_artifact_state_reads;
                    execute_runtime_artifact_warm_reads +=
                        execute_delta.runtime_artifact_warm_reads;
                    execute_retained_artifact_reads += execute_delta.retained_artifact_reads;
                    report_precompute_nanos += report.stage_precompute_nanos;
                    report_apply_nanos += report.stage_apply_nanos;
                    report_semantic_finalize_nanos += report.semantic_finalize_nanos;
                }
                let elapsed = start.elapsed();
                let after = graph.observe().metrics();
                let loop_access_delta =
                    crate::data::access_counters::snapshot().delta_since(access_before_loop);

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
                    map.insert(
                        "planning_materialized_entry_reads".into(),
                        json!(planning_materialized_entry_reads),
                    );
                    map.insert(
                        "planning_runtime_artifact_state_reads".into(),
                        json!(planning_runtime_artifact_state_reads),
                    );
                    map.insert(
                        "planning_runtime_artifact_warm_reads".into(),
                        json!(planning_runtime_artifact_warm_reads),
                    );
                    map.insert(
                        "execute_materialized_entry_reads".into(),
                        json!(execute_materialized_entry_reads),
                    );
                    map.insert(
                        "execute_runtime_artifact_state_reads".into(),
                        json!(execute_runtime_artifact_state_reads),
                    );
                    map.insert(
                        "execute_runtime_artifact_warm_reads".into(),
                        json!(execute_runtime_artifact_warm_reads),
                    );
                    map.insert(
                        "execute_retained_artifact_reads".into(),
                        json!(execute_retained_artifact_reads),
                    );
                    map.insert(
                        "loop_materialized_entry_reads".into(),
                        json!(loop_access_delta.materialized_entry_reads),
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
fn perf_dependency_reconciliation_stable_shape_staged_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        capture_and_certify_perf_samples(
            perf_contract(
                "dependency_reconciliation_stable_shape_staged",
                "balanced",
                PerfTimingPolicy::StructuralOnly,
                &[
                    "planning_nanos",
                    "report_stage_precompute_nanos",
                    "report_stage_apply_nanos",
                    "report_semantic_finalize_nanos",
                    "dependency_reconcile_nanos",
                    "snapshot_batch_commit_nanos",
                ],
            ),
            || {
                let mut graph = SignalGraph::new();
                graph.set_runtime_policy(SignalRuntimePolicy::development());
                let sources = (0..64).map(|_| graph.node().build()).collect::<Vec<_>>();
                let leaves = (0..512).map(|_| graph.node().build()).collect::<Vec<_>>();
                let window = 8usize;
                let all_nodes = sources
                    .iter()
                    .copied()
                    .chain(leaves.iter().copied())
                    .collect::<Vec<_>>();
                let max_index = all_nodes
                    .iter()
                    .map(|node| node.index() as usize)
                    .max()
                    .unwrap_or(0);
                let mut leaf_positions = vec![usize::MAX; max_index + 1];
                let mut source_positions = vec![usize::MAX; max_index + 1];
                for (index, leaf) in leaves.iter().enumerate() {
                    leaf_positions[leaf.index() as usize] = index;
                }
                for (index, source) in sources.iter().enumerate() {
                    source_positions[source.index() as usize] = index;
                }

                let bootstrap = graph
                    .build_evaluation_plan(&leaves, EvaluationRequestMode::Default)
                    .unwrap();
                graph
                    .execute_prepared_plan_with_precompute(&bootstrap, &|node, _view| {
                        let source_index = source_positions[node.index() as usize];
                        if source_index != usize::MAX {
                            return Ok(PreparedEvaluation::from_result(
                                NodeEvaluationResult::from_version(version_ab(
                                    (source_index + 1) as u64,
                                    0,
                                )),
                            ));
                        }
                        let leaf_index = leaf_positions[node.index() as usize];
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
                                version_ab((leaf_index + 1) as u64, 0),
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
                let access_before_loop = crate::data::access_counters::snapshot();
                let mut planning_materialized_entry_reads = 0_u64;
                let mut planning_runtime_artifact_state_reads = 0_u64;
                let mut planning_runtime_artifact_warm_reads = 0_u64;
                let mut execute_materialized_entry_reads = 0_u64;
                let mut execute_runtime_artifact_state_reads = 0_u64;
                let mut execute_runtime_artifact_warm_reads = 0_u64;
                let mut execute_retained_artifact_reads = 0_u64;
                for round in 0..24 {
                    for &source in &sources {
                        mark_dirty(&mut graph, source, ASPECT_A).unwrap();
                    }
                    let access_before_planning = crate::data::access_counters::snapshot();
                    let planning_start = Instant::now();
                    let plan = graph
                        .build_evaluation_plan(&leaves, EvaluationRequestMode::Default)
                        .unwrap();
                    planning_nanos += planning_start.elapsed().as_nanos();
                    let access_after_planning = crate::data::access_counters::snapshot();
                    let planning_delta = access_after_planning.delta_since(access_before_planning);
                    planning_materialized_entry_reads += planning_delta.materialized_entry_reads;
                    planning_runtime_artifact_state_reads +=
                        planning_delta.runtime_artifact_state_reads;
                    planning_runtime_artifact_warm_reads +=
                        planning_delta.runtime_artifact_warm_reads;
                    let access_before_execute = crate::data::access_counters::snapshot();
                    let report =
                        graph
                            .execute_prepared_plan_with_precompute(&plan, &|node, _view| {
                                let source_index = source_positions[node.index() as usize];
                                if source_index != usize::MAX {
                                    return Ok(PreparedEvaluation::from_result(
                                        NodeEvaluationResult::from_version(version_ab(
                                            (round + 2) as u64,
                                            source_index as u64,
                                        )),
                                    ));
                                }
                                let leaf_index = leaf_positions[node.index() as usize];
                                let mut capture = PreparedDependencyCapture::new();
                                for offset in 0..window {
                                    capture.record(
                                        sources[(leaf_index + offset) % sources.len()],
                                        ASPECT_A,
                                        None,
                                    );
                                }
                                Ok(PreparedEvaluation::from_result(
                                    NodeEvaluationResult::from_version(version_ab(
                                        (round + 2) as u64,
                                        leaf_index as u64,
                                    )),
                                )
                                .with_dependencies(capture))
                            })
                            .unwrap();
                    let access_after_execute = crate::data::access_counters::snapshot();
                    let execute_delta = access_after_execute.delta_since(access_before_execute);
                    execute_materialized_entry_reads += execute_delta.materialized_entry_reads;
                    execute_runtime_artifact_state_reads +=
                        execute_delta.runtime_artifact_state_reads;
                    execute_runtime_artifact_warm_reads +=
                        execute_delta.runtime_artifact_warm_reads;
                    execute_retained_artifact_reads += execute_delta.retained_artifact_reads;
                    report_precompute_nanos += report.stage_precompute_nanos;
                    report_apply_nanos += report.stage_apply_nanos;
                    report_semantic_finalize_nanos += report.semantic_finalize_nanos;
                }
                let elapsed = start.elapsed();
                let after = graph.observe().metrics();
                let loop_access_delta =
                    crate::data::access_counters::snapshot().delta_since(access_before_loop);

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
                    map.insert(
                        "planning_materialized_entry_reads".into(),
                        json!(planning_materialized_entry_reads),
                    );
                    map.insert(
                        "planning_runtime_artifact_state_reads".into(),
                        json!(planning_runtime_artifact_state_reads),
                    );
                    map.insert(
                        "planning_runtime_artifact_warm_reads".into(),
                        json!(planning_runtime_artifact_warm_reads),
                    );
                    map.insert(
                        "execute_materialized_entry_reads".into(),
                        json!(execute_materialized_entry_reads),
                    );
                    map.insert(
                        "execute_runtime_artifact_state_reads".into(),
                        json!(execute_runtime_artifact_state_reads),
                    );
                    map.insert(
                        "execute_runtime_artifact_warm_reads".into(),
                        json!(execute_runtime_artifact_warm_reads),
                    );
                    map.insert(
                        "execute_retained_artifact_reads".into(),
                        json!(execute_retained_artifact_reads),
                    );
                    map.insert(
                        "loop_materialized_entry_reads".into(),
                        json!(loop_access_delta.materialized_entry_reads),
                    );
                }
                PerfMeasurement::new(elapsed.as_micros(), metrics)
            },
        )
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
    assert!(samples.iter().all(|sample| {
        sample.metrics["dependency_input_stable_shape_count"]
            .as_u64()
            .unwrap_or(0)
            > sample.metrics["dependency_input_replacement_count"]
                .as_u64()
                .unwrap_or(u64::MAX / 2)
    }));
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture"]
fn perf_suppression_wide_fanout_serial() {
    let samples = with_perf_topology_asserts_disabled(|| {
        capture_and_certify_perf_samples(
            hot_family_contract(
                "suppression_wide_fanout",
                "balanced",
                PerfTimingPolicy::MedianOnly,
                &["leaf_reread_nanos", "stage_execution_nanos"],
                ZERO_BROAD_ENTRY_ACCESS,
            ),
            || {
                let mut runtime = SignalRuntime::builder(SignalGraph::new())
                    .with_kernel_defaults()
                    .build();
                runtime
                    .set_runtime_policy(SignalRuntimePolicy::operational().with_history_limit(4));

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
                    let current = ctx.graph().node_aspect_version(source)?.get(ASPECT_A);
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
                let access_before_transaction = crate::data::access_counters::snapshot();
                let transaction_start = Instant::now();
                runtime
                    .transaction(&mut (), |tx| {
                        tx.mark_dirty(source, ASPECT_A)?;
                        tx.read(source, &|ctx| {
                            Ok(ctx.finish(NodeEvaluationResult::from_version(version_ab(12, 0))))
                        })?;
                        Ok(())
                    })
                    .unwrap();
                let transaction_nanos = transaction_start.elapsed().as_nanos();
                let access_after_transaction = crate::data::access_counters::snapshot();

                let access_before_reread = crate::data::access_counters::snapshot();
                let leaf_reread_start = Instant::now();
                for &leaf in &leaves {
                    let _ = runtime
                        .read_with_executor(leaf, &(), &evaluator, StageExecutor::Serial)
                        .unwrap();
                }
                let leaf_reread_nanos = leaf_reread_start.elapsed().as_nanos();
                let access_after_reread = crate::data::access_counters::snapshot();
                let after = runtime.observe().metrics();

                let mut metrics = eval_metrics_delta(before, after);
                if let Value::Object(ref mut map) = metrics {
                    map.insert("transaction_nanos".into(), json!(transaction_nanos));
                    map.insert("leaf_reread_nanos".into(), json!(leaf_reread_nanos));
                    map.insert(
                        "transaction_materialized_entry_reads".into(),
                        json!(
                            access_after_transaction
                                .delta_since(access_before_transaction)
                                .materialized_entry_reads
                        ),
                    );
                    map.insert(
                        "transaction_runtime_artifact_state_reads".into(),
                        json!(
                            access_after_transaction
                                .delta_since(access_before_transaction)
                                .runtime_artifact_state_reads
                        ),
                    );
                    map.insert(
                        "reread_materialized_entry_reads".into(),
                        json!(
                            access_after_reread
                                .delta_since(access_before_reread)
                                .materialized_entry_reads
                        ),
                    );
                    map.insert(
                        "reread_runtime_artifact_state_reads".into(),
                        json!(
                            access_after_reread
                                .delta_since(access_before_reread)
                                .runtime_artifact_state_reads
                        ),
                    );
                }
                PerfMeasurement::new(
                    (transaction_nanos + leaf_reread_nanos) as u128 / 1_000,
                    metrics,
                )
            },
        )
    });

    assert!(samples.iter().all(|sample| sample.elapsed_micros > 0));
    assert!(samples.iter().all(|sample| {
        sample.metrics["tasks_pruned_before_execution"]
            .as_u64()
            .unwrap_or(0)
            > 0
            || sample.metrics["skipped_by_comparator"]
                .as_u64()
                .unwrap_or(0)
                > 0
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
        let samples = capture_and_certify_perf_samples(
            perf_contract(
                "harness_observability_profile",
                profile_name,
                PerfTimingPolicy::StructuralOnly,
                &["observe_loop_nanos"],
            ),
            || {
                let mut scenario = SignalScenario::new("perf-observability-profile");
                let mut sources = Vec::new();
                let mut dependents = Vec::new();
                for index in 0..12 {
                    let source = scenario.node(format!("source-{index}"));
                    let dependent = scenario.node(format!("dependent-{index}"));
                    scenario
                        .graph_mut()
                        .append_dependency(dependent, source, ASPECT_A)
                        .unwrap();
                    scenario
                        .graph_mut()
                        .set_causality(
                            dependent,
                            Some(CausalityMetadata {
                                kind: "perf-observe".to_string(),
                                fields: [
                                    ("source".to_string(), format!("source-{index}")),
                                    ("channel".to_string(), "obs".to_string()),
                                ]
                                .into_iter()
                                .collect(),
                            }),
                        )
                        .unwrap();
                    sources.push(source);
                    dependents.push(dependent);
                }

                let fixture = dependents
                    .iter()
                    .enumerate()
                    .fold(
                        scenario.with_evaluator(move |ctx: &mut EvaluationContext<'_, ()>| {
                            let node = ctx.node();
                            let version = sources
                                .iter()
                                .position(|source| *source == node)
                                .map(|index| (index + 1) as u64)
                                .unwrap_or(10_000 + node.index() as u64);
                            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(
                                version_ab(version, 0),
                            ))
                        }),
                        |builder, (index, _)| {
                            builder
                                .input(format!("source-{index}"))
                                .observe(format!("dependent-{index}"))
                        },
                    )
                    .fixture()
                    .unwrap();

                let request = worth_harness::facade::ExecutionRequest::new(
                    "observe-dependent-fanout",
                    (0..dependents.len())
                        .map(|index| format!("dependent-{index}"))
                        .collect(),
                );
                let profile = match profile_name {
                    "development" => SignalProfileCatalog::development("development"),
                    "forensic" => SignalProfileCatalog::forensic("forensic"),
                    other => panic!("unexpected profile for perf test: {other}"),
                };
                let iterations = 6_u64;

                let observe_start = Instant::now();
                let mut explanations = 0_u64;
                let mut provenance = 0_u64;
                let mut tasks_executed = 0_u64;
                let mut tasks_pruned = 0_u64;
                let mut diagnostics_seen = false;
                for _ in 0..iterations {
                    let bundle = signal_bench(fixture.clone(), request.clone())
                        .observe(&profile)
                        .unwrap();
                    explanations += bundle.explanations.len() as u64;
                    provenance += bundle.provenance.len() as u64;
                    diagnostics_seen |= bundle.diagnostics.is_some();
                    tasks_executed += bundle
                        .core
                        .run
                        .summary
                        .get("tasks_executed")
                        .and_then(|value| value.as_u64())
                        .unwrap_or(0);
                    tasks_pruned += bundle
                        .core
                        .run
                        .summary
                        .get("tasks_pruned")
                        .and_then(|value| value.as_u64())
                        .unwrap_or(0);
                }
                let observe_loop_nanos = observe_start.elapsed().as_nanos();

                let metrics = json!({
                    "iterations": iterations,
                    "targets": dependents.len(),
                    "explanations": explanations,
                    "provenance": provenance,
                    "has_diagnostics": diagnostics_seen,
                    "tasks_executed": tasks_executed,
                    "tasks_pruned": tasks_pruned,
                    "observe_loop_nanos": observe_loop_nanos,
                });

                PerfMeasurement::new(observe_loop_nanos as u128 / 1_000, metrics)
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
