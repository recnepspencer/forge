use crate::facade::{
    mark_dirty_with_regions, AspectVersion, ChangedRegion, EvaluationRequestMode,
    NodeEvaluationResult, NodeId, SignalGraph, StageExecutor,
};
use crate::tests::support::{version_ab, GraphDependencyBatchExt, ASPECT_A};

use super::canonical_artifact_oracle::canonical_runtime_artifacts;
use super::executor_policy::hostile_executor_matrix;

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
