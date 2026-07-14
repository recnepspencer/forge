#![cfg(feature = "parallel")]

use std::num::NonZeroUsize;

use serde_json::json;
use worth_harness::facade::{ComparisonMode, ComparisonProfile, ExecutionRequest};

use crate::data::comparator::{
    DefaultComparatorPolicyResolver, DefaultComparatorResolver, VersionComparatorPolicy,
};
use crate::facade::*;
use crate::logic::planner::model::ParallelAdmissionReason;
use crate::logic::planner::{ParallelApplyMode, ParallelExecutionPolicy};
use crate::logic::prepared::{PreparedDependencyCapture, PreparedEvaluation};
use crate::tests::support::{version_ab, ASPECT_A};

use crate::presentation::harness::{signal_parity_suite, SignalProfileCatalog, SignalScenario};

fn canonical_runtime_artifacts(graph: &SignalGraph, node: NodeId) -> serde_json::Value {
    let explanation = graph.observe().explain(node).unwrap();
    let explanation_fact = graph.explanation_fact(node);
    let provenance = graph.provenance_fact(node).cloned();
    let diagnostics = graph
        .observe()
        .diagnostics_summary(DiagnosticsTier::Development);
    let replay = graph
        .replay_events()
        .iter()
        .map(|event| {
            json!({
                "cursor": event.cursor.0,
                "kind": format!("{:?}", event.kind),
                "branch_id": event.branch_id.0,
                "snapshot_id": event.snapshot_id.map(|id| id.0),
                "node": event.node.map(|node| node.to_string()),
                "execution_record_id": event.execution_record_id,
                "semantic_segment_id": event.semantic_segment_id,
                "lineage_artifact_id": event.lineage_artifact_id.map(|id| id.0),
                "detail": event.detail,
            })
        })
        .collect::<Vec<_>>();
    json!({
        "explanation": {
            "node": explanation.node.to_string(),
            "state": format!("{:?}", explanation.state),
            "execution_record_id": explanation.execution_record_id,
            "semantic_segment_id": explanation.semantic_segment_id,
            "upstream_count": explanation.upstream.len(),
            "propagation_suppressed": explanation.propagation_suppressed,
            "changed_region_count": explanation.changed_regions.len(),
            "output_change": explanation.output_change.map(|change| format!("{change:?}")),
            "fact_state": explanation_fact.map(|fact| fact.state.clone()),
            "fact_upstream_count": explanation_fact.map(|fact| fact.upstream_count),
        },
        "provenance": provenance,
        "replay": replay,
        "diagnostics": {
            "active_node_count": diagnostics.active_node_count,
            "clean_node_count": diagnostics.clean_node_count,
            "maybe_stale_node_count": diagnostics.maybe_stale_node_count,
            "dirty_node_count": diagnostics.dirty_node_count,
            "dependency_edge_count": diagnostics.dependency_edge_count,
            "subscriber_edge_count": diagnostics.subscriber_edge_count,
            "nodes_with_trace_summary": diagnostics.nodes_with_trace_summary,
            "nodes_with_execution_record": diagnostics.nodes_with_execution_record,
            "nodes_with_causality": diagnostics.nodes_with_causality,
            "partition_interner_size": diagnostics.partition_interner_size,
            "sample_dirty_nodes": diagnostics
                .sample_dirty_nodes
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            "sample_nodes_with_execution_record": diagnostics
                .sample_nodes_with_execution_record
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        },
    })
}

fn hostile_executor_matrix() -> Vec<(&'static str, StageExecutor)> {
    let policies = [
        ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
            .with_worker_count(1)
            .with_chunk_size(1)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(1),
        ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
            .with_worker_count(2)
            .with_chunk_size(1)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(2),
        ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
            .with_worker_count(3)
            .with_chunk_size(2)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(2),
        ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
            .with_worker_count(4)
            .with_chunk_size(2)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(4),
    ];
    let mut executors = vec![("serial", StageExecutor::Serial)];
    for (index, policy) in policies.into_iter().enumerate() {
        executors.push((
            match index {
                0 => "staged-1x1",
                1 => "staged-2x1",
                2 => "staged-3x2",
                _ => "staged-4x2",
            },
            StageExecutor::parallel(1).with_parallel_policy(policy),
        ));
        executors.push((
            match index {
                0 => "full-1x1",
                1 => "full-2x1",
                2 => "full-3x2",
                _ => "full-4x2",
            },
            StageExecutor::full_parallel(1).with_parallel_policy(policy),
        ));
    }
    executors
}

fn aggressive_parallel_runtime_policy() -> SignalRuntimePolicy {
    SignalRuntimePolicy::operational().with_parallel_admission(ParallelAdmissionPolicy {
        operational_min_parallel_tasks: 1,
        development_min_parallel_tasks: 1,
        forensic_min_parallel_tasks: 1,
        full_parallel_min_tasks: 1,
    })
}

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

#[test]
fn full_parallel_rewires_dynamic_dependencies_without_losing_parity() {
    fn bootstrap_graph() -> Result<(SignalGraph, NodeId, NodeId, NodeId, [NodeId; 2]), SignalError>
    {
        let mut graph = SignalGraph::new();
        graph.set_runtime_policy(aggressive_parallel_runtime_policy());
        let selector = graph.node().build();
        let left = graph.node().build();
        let right = graph.node().build();
        let target_a = graph.node().build();
        let target_b = graph.node().build();
        let targets = [target_a, target_b];

        let bootstrap = graph.build_evaluation_plan(
            &[selector, left, right, target_a, target_b],
            EvaluationRequestMode::ForceOnDemand,
        )?;
        graph.execute_prepared_plan(&bootstrap, &(), &move |ctx| {
            let node = ctx.node();
            let result = if node == selector {
                ctx.finish(version_ab(0, 0))
            } else if node == left {
                ctx.finish(version_ab(10, 0))
            } else if node == right {
                ctx.finish(version_ab(20, 0))
            } else {
                let selector_version = ctx.read_aspect_version(selector, ASPECT_A)?;
                let source = if selector_version.get(ASPECT_A) == 0 {
                    left
                } else {
                    right
                };
                let chosen = ctx.read_aspect_version(source, ASPECT_A)?;
                ctx.finish(NodeEvaluationResult::from_version(chosen))
            };
            Ok(result)
        })?;

        Ok((graph, selector, left, right, targets))
    }

    fn rewire_targets(
        graph: &mut SignalGraph,
        selector: NodeId,
        right: NodeId,
        targets: &[NodeId; 2],
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError> {
        mark_dirty(&mut *graph, selector, ASPECT_A)?;
        let plan = graph.build_evaluation_plan(targets, EvaluationRequestMode::Default)?;
        graph.execute_prepared_plan_with_executor(
            &plan,
            &(),
            &move |ctx| {
                let node = ctx.node();
                let result = if node == selector {
                    ctx.finish(version_ab(1, 0))
                } else {
                    let selector_version = ctx.read_aspect_version(selector, ASPECT_A)?;
                    let source = if selector_version.get(ASPECT_A) == 0 {
                        unreachable!("selector should have recomputed before dynamic targets")
                    } else {
                        right
                    };
                    let chosen = ctx.read_aspect_version(source, ASPECT_A)?;
                    ctx.finish(NodeEvaluationResult::from_version(chosen))
                };
                Ok(result)
            },
            executor,
        )
    }

    let (base_graph, selector, left, right, targets) = bootstrap_graph().unwrap();
    let mut serial_graph = base_graph.clone();
    let mut parallel_graph = base_graph;

    let serial_report = rewire_targets(
        &mut serial_graph,
        selector,
        right,
        &targets,
        StageExecutor::Serial,
    )
    .unwrap();
    let parallel_report = rewire_targets(
        &mut parallel_graph,
        selector,
        right,
        &targets,
        StageExecutor::full_parallel(2),
    )
    .unwrap();

    for target in targets {
        assert_eq!(
            serial_graph.dependencies_of(target).unwrap(),
            parallel_graph.dependencies_of(target).unwrap()
        );
        assert!(parallel_graph
            .dependencies_of(target)
            .unwrap()
            .iter()
            .any(|edge| edge.source() == right));
        assert!(!parallel_graph
            .dependencies_of(target)
            .unwrap()
            .iter()
            .any(|edge| edge.source() == left));
        assert_eq!(
            serial_graph.get_entry(target).unwrap().get_aspect_version(),
            parallel_graph
                .get_entry(target)
                .unwrap()
                .get_aspect_version()
        );
    }

    assert_eq!(
        serial_graph.subscribers_of(left).unwrap(),
        parallel_graph.subscribers_of(left).unwrap()
    );
    assert_eq!(
        serial_graph.subscribers_of(right).unwrap(),
        parallel_graph.subscribers_of(right).unwrap()
    );
    assert!(parallel_graph.subscribers_of(left).unwrap().is_empty());
    assert_eq!(
        parallel_graph.subscribers_of(right).unwrap().len(),
        targets.len()
    );
    assert!(parallel_report
        .stages
        .iter()
        .any(|stage| { stage.apply_mode == Some(ParallelApplyMode::SerialApply) }));
    assert_eq!(serial_report.tasks_executed, parallel_report.tasks_executed);
}

#[test]
fn full_parallel_apply_failure_does_not_leak_partial_semantic_state() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(aggressive_parallel_runtime_policy());
    let stable = graph.node().build();
    let unstable = graph.node().build();
    let requested = [stable, unstable];

    let bootstrap = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|ctx| {
            Ok(ctx.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
        })
        .unwrap();

    let stable_baseline = graph.get_entry(stable).unwrap().get_aspect_version();
    let unstable_baseline = graph.get_entry(unstable).unwrap().get_aspect_version();
    let stable_fact_before = graph.explanation_fact(stable).cloned();
    let unstable_fact_before = graph.explanation_fact(unstable).cloned();
    let replay_len_before = graph.replay_events().len();

    mark_dirty(&mut graph, stable, ASPECT_A).unwrap();
    mark_dirty(&mut graph, unstable, ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::Default)
        .unwrap();
    let err = {
        let mut comparator = DefaultComparatorResolver;
        let mut resolver = DefaultComparatorPolicyResolver {
            fallback: VersionComparatorPolicy::Exact,
            custom: &mut comparator,
        };
        crate::logic::planner::execute_prepared_plan_with_precompute(
            &mut graph,
            &plan,
            &move |node, _view| {
                let mut prepared = PreparedEvaluation::from_result(
                    NodeEvaluationResult::from_version(version_ab(2, 0)),
                );
                if node == unstable {
                    let mut capture = PreparedDependencyCapture::new();
                    capture.record(NodeId::new(999_999, 0), ASPECT_A, None);
                    prepared = prepared.with_dependencies(capture);
                }
                Ok(prepared)
            },
            &mut resolver,
            crate::logic::planner::TemporalLoweringContext::graph_only(),
            StageExecutor::full_parallel(1).with_parallel_policy(
                ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
                    .with_worker_count(2)
                    .with_chunk_size(1)
                    .with_apply_group_min_width(1)
                    .with_max_concurrent_apply_groups(2),
            ),
        )
        .unwrap_err()
    };
    assert!(
        matches!(err, SignalError::StaleHandle { .. }),
        "apply failure should surface stale dependency-capture error, got: {err}"
    );

    assert_eq!(
        graph.get_entry(stable).unwrap().get_aspect_version(),
        stable_baseline,
        "stable node state must not commit when the parallel stage fails"
    );
    assert_eq!(
        graph.get_entry(unstable).unwrap().get_aspect_version(),
        unstable_baseline,
        "failing node state must be rewound"
    );
    assert_eq!(graph.explanation_fact(stable), stable_fact_before.as_ref());
    assert_eq!(
        graph.explanation_fact(unstable),
        unstable_fact_before.as_ref()
    );
    assert_eq!(
        graph.replay_events().len(),
        replay_len_before,
        "failed planner stage must not leak task-applied replay events"
    );
}

#[test]
fn full_parallel_policy_matrix_preserves_semantic_artifacts_on_tolerance_heavy_partition_graph() {
    fn build_graph() -> (SignalGraph, NodeId, NodeId, NodeId) {
        let mut graph = SignalGraph::new();
        let source = graph.node().build();
        let branch_a = graph.node().tolerance(1).build();
        let branch_b = graph.node().tolerance(1).build();
        graph
            .append_partition_dependency(branch_a, source, ASPECT_A, "shell")
            .unwrap();
        graph
            .append_partition_dependency(branch_b, source, ASPECT_A, "core")
            .unwrap();
        (graph, source, branch_a, branch_b)
    }

    fn bootstrap(graph: &mut SignalGraph, source: NodeId, branch_a: NodeId, branch_b: NodeId) {
        let plan = graph
            .build_evaluation_plan(
                &[source, branch_a, branch_b],
                EvaluationRequestMode::ForceOnDemand,
            )
            .unwrap();
        graph
            .execute_prepared_plan(&plan, &(), &move |ctx| {
                let node = ctx.node();
                let result = if node == source {
                    ctx.finish(
                        NodeEvaluationResult::from_version(version_ab(10, 0))
                            .with_changed_region(ChangedRegion::new("shell"))
                            .with_changed_region(ChangedRegion::new("core")),
                    )
                } else {
                    let version = ctx.read_aspect_version(source, ASPECT_A)?;
                    ctx.finish(NodeEvaluationResult::from_version(version))
                };
                Ok(result)
            })
            .unwrap();
    }

    fn run_with_executor(
        mut graph: SignalGraph,
        source: NodeId,
        target: NodeId,
        executor: StageExecutor,
    ) -> serde_json::Value {
        mark_dirty_with_regions(
            &mut graph,
            source,
            ASPECT_A,
            &[ChangedRegion::new("core"), ChangedRegion::new("shell")],
        )
        .unwrap();
        let plan = graph
            .build_evaluation_plan(&[target], EvaluationRequestMode::Default)
            .unwrap();
        graph
            .execute_prepared_plan_with_executor(
                &plan,
                &(),
                &move |ctx| {
                    let node = ctx.node();
                    let result = if node == source {
                        ctx.finish(
                            NodeEvaluationResult::from_version(version_ab(12, 0))
                                .with_changed_region(ChangedRegion::new("shell"))
                                .with_changed_region(ChangedRegion::new("core")),
                        )
                    } else {
                        let version = ctx.read_aspect_version(source, ASPECT_A)?;
                        ctx.finish(NodeEvaluationResult::from_version(version))
                    };
                    Ok(result)
                },
                executor,
            )
            .unwrap();
        canonical_runtime_artifacts(&graph, target)
    }

    let (base_graph, source, branch_a, branch_b) = build_graph();
    let mut seed_graph = base_graph.clone();
    bootstrap(&mut seed_graph, source, branch_a, branch_b);
    let baseline = run_with_executor(seed_graph.clone(), source, branch_b, StageExecutor::Serial);

    for (label, executor) in hostile_executor_matrix() {
        let observed = run_with_executor(seed_graph.clone(), source, branch_b, executor);
        assert_eq!(
            baseline, observed,
            "executor {label} drifted semantic artifacts"
        );
    }
}

#[test]
fn logically_equivalent_region_orders_produce_identical_provenance_and_replay() {
    let mut graph_a = SignalGraph::new();
    let source_a = graph_a.node().build();
    let target_a = graph_a.node().tolerance(0).build();
    graph_a
        .append_partition_dependency(target_a, source_a, ASPECT_A, "face")
        .unwrap();

    let mut graph_b = graph_a.clone();
    let bootstrap = |graph: &mut SignalGraph, source: NodeId, target: NodeId| {
        let plan = graph
            .build_evaluation_plan(&[source, target], EvaluationRequestMode::ForceOnDemand)
            .unwrap();
        graph
            .execute_prepared_plan(&plan, &(), &move |ctx| {
                let node = ctx.node();
                let result = if node == source {
                    ctx.finish(
                        NodeEvaluationResult::from_version(version_ab(5, 0))
                            .with_changed_region(ChangedRegion::new("face"))
                            .with_changed_region(ChangedRegion::new("edge")),
                    )
                } else {
                    let version = ctx.read_aspect_version(source, ASPECT_A)?;
                    ctx.finish(NodeEvaluationResult::from_version(version))
                };
                Ok(result)
            })
            .unwrap();
    };
    bootstrap(&mut graph_a, source_a, target_a);
    bootstrap(&mut graph_b, source_a, target_a);

    mark_dirty_with_regions(
        &mut graph_a,
        source_a,
        ASPECT_A,
        &[ChangedRegion::new("face"), ChangedRegion::new("edge")],
    )
    .unwrap();
    mark_dirty_with_regions(
        &mut graph_b,
        source_a,
        ASPECT_A,
        &[ChangedRegion::new("edge"), ChangedRegion::new("face")],
    )
    .unwrap();

    let run = |graph: &mut SignalGraph| {
        let plan = graph
            .build_evaluation_plan(&[target_a], EvaluationRequestMode::Default)
            .unwrap();
        graph
            .execute_prepared_plan_with_executor(
                &plan,
                &(),
                &move |ctx| {
                    let node = ctx.node();
                    let result = if node == source_a {
                        ctx.finish(
                            NodeEvaluationResult::from_version(version_ab(6, 0))
                                .with_changed_region(ChangedRegion::new("edge"))
                                .with_changed_region(ChangedRegion::new("face")),
                        )
                    } else {
                        let version = ctx.read_aspect_version(source_a, ASPECT_A)?;
                        ctx.finish(NodeEvaluationResult::from_version(version))
                    };
                    Ok(result)
                },
                StageExecutor::full_parallel(1),
            )
            .unwrap();
        canonical_runtime_artifacts(graph, target_a)
    };

    assert_eq!(run(&mut graph_a), run(&mut graph_b));
}

#[test]
fn reordered_dependency_and_region_orders_stay_canonical_across_executor_matrix() {
    fn build_graph(reverse_dependencies: bool) -> (SignalGraph, NodeId, NodeId, NodeId, NodeId) {
        let mut graph = SignalGraph::new();
        let source = graph.node().output_identity().build();
        let shell = graph.node().tolerance(1).partitioned_output().build();
        let core = graph.node().tolerance(1).partitioned_output().build();
        let target = graph.node().output_identity().build();

        if reverse_dependencies {
            graph.append_dependency(target, core, ASPECT_A).unwrap();
            graph.append_dependency(target, shell, ASPECT_A).unwrap();
            graph
                .append_partition_dependency(core, source, ASPECT_A, "mesh")
                .unwrap();
            graph
                .append_partition_dependency(shell, source, ASPECT_A, "shell")
                .unwrap();
        } else {
            graph
                .append_partition_dependency(shell, source, ASPECT_A, "shell")
                .unwrap();
            graph
                .append_partition_dependency(core, source, ASPECT_A, "mesh")
                .unwrap();
            graph.append_dependency(target, shell, ASPECT_A).unwrap();
            graph.append_dependency(target, core, ASPECT_A).unwrap();
        }

        (graph, source, shell, core, target)
    }

    fn bootstrap(
        graph: &mut SignalGraph,
        source: NodeId,
        shell: NodeId,
        core: NodeId,
        target: NodeId,
    ) {
        let plan = graph
            .build_evaluation_plan(
                &[source, shell, core, target],
                EvaluationRequestMode::ForceOnDemand,
            )
            .unwrap();
        graph
            .execute_prepared_plan(&plan, &(), &move |ctx| {
                let node = ctx.node();
                let result = if node == source {
                    ctx.finish(
                        NodeEvaluationResult::from_version(version_ab(20, 0))
                            .with_output_identity("geom-v1")
                            .with_changed_region(ChangedRegion::new("mesh").with_detail("face-b"))
                            .with_changed_region(ChangedRegion::new("shell").with_detail("face-a")),
                    )
                } else if node == shell || node == core {
                    let version = ctx.read_aspect_version(source, ASPECT_A)?;
                    ctx.finish(NodeEvaluationResult::from_version(version))
                } else {
                    let shell_v = ctx.read_aspect_version(shell, ASPECT_A)?;
                    let core_v = ctx.read_aspect_version(core, ASPECT_A)?;
                    ctx.finish(
                        NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                            ASPECT_A,
                            shell_v.get(ASPECT_A) + core_v.get(ASPECT_A),
                        )]))
                        .with_output_identity("geom-aggregate"),
                    )
                };
                Ok(result)
            })
            .unwrap();
    }

    fn execute(
        mut graph: SignalGraph,
        source: NodeId,
        shell: NodeId,
        core: NodeId,
        target: NodeId,
        region_order: &[ChangedRegion],
        executor: StageExecutor,
    ) -> serde_json::Value {
        mark_dirty_with_regions(&mut graph, source, ASPECT_A, region_order).unwrap();
        let plan = graph
            .build_evaluation_plan(&[target], EvaluationRequestMode::Default)
            .unwrap();
        graph
            .execute_prepared_plan_with_executor(
                &plan,
                &(),
                &move |ctx| {
                    let node = ctx.node();
                    let result = if node == source {
                        ctx.finish(
                            NodeEvaluationResult::from_version(version_ab(22, 0))
                                .with_output_identity("geom-v2")
                                .with_changed_region(
                                    ChangedRegion::new("shell").with_detail("face-a"),
                                )
                                .with_changed_region(
                                    ChangedRegion::new("mesh").with_detail("face-b"),
                                ),
                        )
                    } else if node == shell || node == core {
                        let version = ctx.read_aspect_version(source, ASPECT_A)?;
                        ctx.finish(NodeEvaluationResult::from_version(version))
                    } else {
                        let shell_v = ctx.read_aspect_version(shell, ASPECT_A)?;
                        let core_v = ctx.read_aspect_version(core, ASPECT_A)?;
                        ctx.finish(
                            NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                                ASPECT_A,
                                shell_v.get(ASPECT_A) + core_v.get(ASPECT_A),
                            )]))
                            .with_output_identity("geom-aggregate"),
                        )
                    };
                    Ok(result)
                },
                executor,
            )
            .unwrap();
        canonical_runtime_artifacts(&graph, target)
    }

    let (mut graph_a, source_a, shell_a, core_a, target_a) = build_graph(false);
    let (mut graph_b, source_b, shell_b, core_b, target_b) = build_graph(true);
    bootstrap(&mut graph_a, source_a, shell_a, core_a, target_a);
    bootstrap(&mut graph_b, source_b, shell_b, core_b, target_b);

    let baseline = execute(
        graph_a.clone(),
        source_a,
        shell_a,
        core_a,
        target_a,
        &[
            ChangedRegion::new("mesh").with_detail("face-b"),
            ChangedRegion::new("shell").with_detail("face-a"),
        ],
        StageExecutor::Serial,
    );
    let reordered = execute(
        graph_b.clone(),
        source_b,
        shell_b,
        core_b,
        target_b,
        &[
            ChangedRegion::new("shell").with_detail("face-a"),
            ChangedRegion::new("mesh").with_detail("face-b"),
        ],
        StageExecutor::Serial,
    );
    assert_eq!(baseline, reordered, "serial canonicalization drifted");

    for (label, executor) in hostile_executor_matrix() {
        let observed = execute(
            graph_b.clone(),
            source_b,
            shell_b,
            core_b,
            target_b,
            &[
                ChangedRegion::new("shell").with_detail("face-a"),
                ChangedRegion::new("mesh").with_detail("face-b"),
            ],
            executor,
        );
        assert_eq!(
            baseline, observed,
            "executor {label} drifted reordered topology artifacts"
        );
    }
}

#[test]
fn repeated_executor_policy_churn_keeps_tolerance_boundary_artifacts_stable() {
    fn build_graph() -> (SignalGraph, NodeId, NodeId, NodeId, NodeId) {
        let mut graph = SignalGraph::new();
        let source = graph.node().build();
        let shell = graph.node().tolerance(2).build();
        let core = graph.node().tolerance(2).build();
        let target = graph.node().output_identity().build();
        graph.append_dependency(shell, source, ASPECT_A).unwrap();
        graph.append_dependency(core, source, ASPECT_A).unwrap();
        graph.append_dependency(target, shell, ASPECT_A).unwrap();
        graph.append_dependency(target, core, ASPECT_A).unwrap();
        (graph, source, shell, core, target)
    }

    fn bootstrap(
        graph: &mut SignalGraph,
        source: NodeId,
        shell: NodeId,
        core: NodeId,
        target: NodeId,
    ) {
        let plan = graph
            .build_evaluation_plan(
                &[source, shell, core, target],
                EvaluationRequestMode::ForceOnDemand,
            )
            .unwrap();
        graph
            .execute_prepared_plan(&plan, &(), &move |ctx| {
                let node = ctx.node();
                let result = if node == source {
                    ctx.finish(NodeEvaluationResult::from_version(version_ab(100, 0)))
                } else if node == shell || node == core {
                    let version = ctx.read_aspect_version(source, ASPECT_A)?;
                    ctx.finish(NodeEvaluationResult::from_version(version))
                } else {
                    let shell_v = ctx.read_aspect_version(shell, ASPECT_A)?;
                    let core_v = ctx.read_aspect_version(core, ASPECT_A)?;
                    ctx.finish(
                        NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                            ASPECT_A,
                            shell_v.get(ASPECT_A) + core_v.get(ASPECT_A),
                        )]))
                        .with_output_identity("topology-target"),
                    )
                };
                Ok(result)
            })
            .unwrap();
    }

    fn run_once(
        graph: &mut SignalGraph,
        source: NodeId,
        shell: NodeId,
        core: NodeId,
        target: NodeId,
        next_version: u64,
        executor: StageExecutor,
    ) -> serde_json::Value {
        mark_dirty(&mut *graph, source, ASPECT_A).unwrap();
        let plan = graph
            .build_evaluation_plan(&[target], EvaluationRequestMode::Default)
            .unwrap();
        graph
            .execute_prepared_plan_with_executor(
                &plan,
                &(),
                &move |ctx| {
                    let node = ctx.node();
                    let result = if node == source {
                        ctx.finish(NodeEvaluationResult::from_version(version_ab(
                            next_version,
                            0,
                        )))
                    } else if node == shell || node == core {
                        let version = ctx.read_aspect_version(source, ASPECT_A)?;
                        ctx.finish(NodeEvaluationResult::from_version(version))
                    } else {
                        let shell_v = ctx.read_aspect_version(shell, ASPECT_A)?;
                        let core_v = ctx.read_aspect_version(core, ASPECT_A)?;
                        ctx.finish(
                            NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                                ASPECT_A,
                                shell_v.get(ASPECT_A) + core_v.get(ASPECT_A),
                            )]))
                            .with_output_identity("topology-target"),
                        )
                    };
                    Ok(result)
                },
                executor,
            )
            .unwrap();
        canonical_runtime_artifacts(graph, target)
    }

    let (mut seed_graph, source, shell, core, target) = build_graph();
    bootstrap(&mut seed_graph, source, shell, core, target);

    for next_version in [101_u64, 102, 101, 102] {
        let mut baseline_graph = seed_graph.clone();
        let baseline = run_once(
            &mut baseline_graph,
            source,
            shell,
            core,
            target,
            next_version,
            StageExecutor::Serial,
        );
        for (label, executor) in hostile_executor_matrix() {
            let mut candidate_graph = seed_graph.clone();
            let observed = run_once(
                &mut candidate_graph,
                source,
                shell,
                core,
                target,
                next_version,
                executor,
            );
            assert_eq!(
                baseline, observed,
                "executor {label} drifted at tolerance boundary {next_version}"
            );
        }
    }
}

#[test]
fn harness_parity_holds_for_branchy_partitioned_output_identity_graph() {
    let mut scenario = SignalScenario::new("adversarial-branchy-parity");
    let source = scenario.build_node("source", |graph| graph.node().output_identity().build());
    let left = scenario.build_node("left", |graph| graph.node().partitioned_output().build());
    let right = scenario.build_node("right", |graph| graph.node().partitioned_output().build());
    let _dependent = scenario.node("dependent");
    scenario
        .partition_detail_dependency("left", "source", ASPECT_A, "wing", "rib-a")
        .unwrap();
    scenario
        .partition_detail_dependency("right", "source", ASPECT_A, "wing", "rib-b")
        .unwrap();
    scenario.dependency("dependent", "left", ASPECT_A).unwrap();
    scenario.dependency("dependent", "right", ASPECT_A).unwrap();

    let fixture = scenario
        .observe("dependent")
        .with_evaluator(move |ctx: &mut EvaluationContext<'_, ()>| {
            let node = ctx.node();
            let result = if node == source {
                ctx.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("wing-artifact")
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-a")),
                )
            } else if node == left || node == right {
                let version = ctx.read_aspect_version(source, ASPECT_A)?;
                ctx.finish(NodeEvaluationResult::from_version(version))
            } else {
                let left_v = ctx.read_aspect_version(left, ASPECT_A)?;
                let right_v = ctx.read_aspect_version(right, ASPECT_A)?;
                ctx.finish(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(
                        ASPECT_A,
                        left_v.get(ASPECT_A) + right_v.get(ASPECT_A),
                    )]),
                ))
            };
            Ok(result)
        })
        .fixture()
        .unwrap();

    let request = ExecutionRequest::target("observe-dependent", "dependent".to_string());
    let report = signal_parity_suite(
        fixture,
        request,
        SignalProfileCatalog::serial("serial-baseline"),
    )
    .comparison_profile(ComparisonProfile {
        mode: ComparisonMode::Semantic,
        include_extensions: false,
        numeric_tolerance: None,
    })
    .candidates([
        SignalProfileCatalog::staged_parallel("staged-parallel-candidate"),
        SignalProfileCatalog::full_parallel("full-parallel-candidate"),
    ])
    .compare()
    .unwrap();

    assert!(report.matched);
}

#[test]
#[ignore = "stress coverage for wide-graph full-parallel parity loops"]
fn stress_repeated_parallel_parity_on_wide_branch_graph() {
    let mut scenario = SignalScenario::new("stress-parity");
    let source = scenario.node("source");
    let mids: Vec<_> = (0..24)
        .map(|index| scenario.node(format!("mid-{index}")))
        .collect();
    let _target = scenario.node("target");
    for (index, _) in mids.iter().enumerate() {
        scenario
            .dependency(&format!("mid-{index}"), "source", ASPECT_A)
            .unwrap();
        scenario
            .dependency("target", &format!("mid-{index}"), ASPECT_A)
            .unwrap();
    }

    let fixture = scenario
        .observe("target")
        .with_evaluator(move |ctx: &mut EvaluationContext<'_, ()>| {
            let node = ctx.node();
            let result = if node == source {
                ctx.finish(version_ab(1, 0))
            } else if mids.contains(&node) {
                let version = ctx.read_aspect_version(source, ASPECT_A)?;
                ctx.finish(NodeEvaluationResult::from_version(version))
            } else {
                let mut total = 0_u64;
                for &mid in &mids {
                    total += ctx.read_aspect_version(mid, ASPECT_A)?.get(ASPECT_A);
                }
                ctx.finish(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(ASPECT_A, total)]),
                ))
            };
            Ok(result)
        })
        .fixture()
        .unwrap();

    let request = ExecutionRequest::target("target", "target".to_string());
    for _ in 0..25 {
        let report = signal_parity_suite(
            fixture.clone(),
            request.clone(),
            SignalProfileCatalog::serial("serial-baseline"),
        )
        .comparison_profile(ComparisonProfile {
            mode: ComparisonMode::Semantic,
            include_extensions: false,
            numeric_tolerance: None,
        })
        .candidates([
            SignalProfileCatalog::staged_parallel("staged-parallel-candidate"),
            SignalProfileCatalog::full_parallel("full-parallel-candidate"),
        ])
        .compare()
        .unwrap();
        assert!(report.matched);
    }
}
