#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

#[cfg(feature = "parallel")]
use forge_signal::facade::diagnostics::{DiagnosticsAvailability, DiagnosticsTier};
#[cfg(feature = "parallel")]
use forge_signal::facade::{
    Aspect, AspectVersion, BatchChange, ChangedRegion, DependencyEdge, EvaluationRequestMode,
    NodeEvaluationResult, NodeId, SignalGraph,
};
#[cfg(feature = "parallel")]
use forge_signal::facade::advanced::{ParallelExecutionPolicy, StageExecutor};
#[cfg(feature = "parallel")]
use forge_signal::facade::runtime::{mark_dirty_batch, SignalRuntimePolicy};
#[cfg(feature = "parallel")]
use serde_json::json;

#[cfg(feature = "parallel")]
const ASPECT_A: Aspect = Aspect::new(0);

#[cfg(feature = "parallel")]
fn version_ab(a: u64, b: u64) -> AspectVersion {
    AspectVersion::from_updates([(ASPECT_A, a), (Aspect::new(1), b)])
}

#[cfg(feature = "parallel")]
fn policy_name(policy: SignalRuntimePolicy) -> &'static str {
    match policy.tier {
        DiagnosticsTier::Operational => "operational",
        DiagnosticsTier::Development => "development",
        DiagnosticsTier::Forensic => "forensic",
    }
}

#[cfg(feature = "parallel")]
fn parse_runtime_policy(label: &str) -> SignalRuntimePolicy {
    match label {
        "operational" => SignalRuntimePolicy::operational(),
        "development" => SignalRuntimePolicy::development(),
        "forensic" => SignalRuntimePolicy::forensic(),
        other => panic!("unsupported runtime policy: {other}"),
    }
}

#[cfg(feature = "parallel")]
fn materialization_label(mode: DiagnosticsAvailability) -> &'static str {
    match mode {
        DiagnosticsAvailability::RetainedAvailable => "retained",
        DiagnosticsAvailability::ReconstructedAvailable => "reconstructed",
        DiagnosticsAvailability::OmittedByTier
        | DiagnosticsAvailability::DeniedByBudget
        | DiagnosticsAvailability::UnavailableNotRetained
        | DiagnosticsAvailability::UnavailableNotReconstructable => "unavailable",
    }
}

#[cfg(feature = "parallel")]
fn canonical_runtime_artifacts(
    graph: &SignalGraph,
    node: NodeId,
    runtime_policy: SignalRuntimePolicy,
) -> serde_json::Value {
    let observer = graph.observe();
    let (explanation, explanation_mode) = observer
        .materialize()
        .materialize_explanation_artifact(node)
        .unwrap();
    let (provenance, provenance_mode) = observer
        .materialize()
        .materialize_provenance_artifact(node)
        .unwrap();
    let explanation = explanation.expect("snapshot fixture should have an explainable target");
    let explanation_fact = observer.explanation_fact(node);
    let diagnostics = observer.diagnostics_summary(DiagnosticsTier::Development);
    let replay = observer
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
        "runtime_policy": policy_name(runtime_policy),
        "core_storage_profile": "public-facade-default",
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
            "materialization": materialization_label(explanation_mode),
        },
        "provenance": {
            "materialization": materialization_label(provenance_mode),
            "artifact": provenance,
        },
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

#[cfg(feature = "parallel")]
fn parse_executor(label: &str) -> StageExecutor {
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
    match label {
        "serial" => StageExecutor::Serial,
        "staged-2x1" => StageExecutor::parallel(1).with_parallel_policy(policy_2x1),
        "staged-4x2" => StageExecutor::parallel(1).with_parallel_policy(policy_4x2),
        "full-2x1" => StageExecutor::full_parallel(1).with_parallel_policy(policy_2x1),
        "full-4x2" => StageExecutor::full_parallel(1).with_parallel_policy(policy_4x2),
        other => panic!("unsupported profile: {other}"),
    }
}

#[cfg(feature = "parallel")]
fn main() {
    let profile = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "serial".to_string());
    let runtime_policy = parse_runtime_policy(
        &std::env::args()
            .nth(2)
            .unwrap_or_else(|| "development".to_string()),
    );
    let executor = parse_executor(&profile);

    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(runtime_policy);
    let source = graph.node().output_identity().build();
    let shell = graph.node().tolerance(1).partitioned_output().build();
    let core = graph.node().tolerance(1).partitioned_output().build();
    let target = graph.node().output_identity().build();
    graph
        .set_dependencies(
            shell,
            [DependencyEdge::whole_partition(source, ASPECT_A, "shell")],
        )
        .unwrap();
    graph
        .set_dependencies(
            core,
            [DependencyEdge::whole_partition(source, ASPECT_A, "mesh")],
        )
        .unwrap();
    graph
        .set_dependencies(
            target,
            [
                DependencyEdge::new(shell, ASPECT_A),
                DependencyEdge::new(core, ASPECT_A),
            ],
        )
        .unwrap();

    let bootstrap = graph
        .build_evaluation_plan(
            &[source, shell, core, target],
            EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &move |ctx| {
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

    mark_dirty_batch(
        &mut graph,
        &BatchChange::singleton(
            source,
            ASPECT_A,
            vec![
                ChangedRegion::new("mesh").with_detail("face-b"),
                ChangedRegion::new("shell").with_detail("face-a"),
            ],
        ),
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
                        NodeEvaluationResult::from_version(version_ab(22, 0))
                            .with_output_identity("geom-v2")
                            .with_changed_region(ChangedRegion::new("shell").with_detail("face-a"))
                            .with_changed_region(ChangedRegion::new("mesh").with_detail("face-b")),
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

    println!(
        "{}",
        serde_json::to_string_pretty(&canonical_runtime_artifacts(&graph, target, runtime_policy))
            .unwrap()
    );
}

#[cfg(not(feature = "parallel"))]
fn main() {
    panic!("signal_semantic_snapshot requires the `parallel` feature");
}
