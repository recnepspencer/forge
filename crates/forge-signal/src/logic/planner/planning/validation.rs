use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::PartitionSubscription;
use crate::data::proof::DedupedNodeBatch;
use crate::logic::prepared::PreparedDependencyCapture;

#[derive(Debug, Clone, Default)]
pub(crate) struct MaybeStalePreview {
    pub(crate) unchanged: bool,
    pub(crate) requires_upstream_evaluation: Vec<NodeId>,
}

#[allow(dead_code)]
pub(crate) fn capture_current_dependencies(
    graph: &mut SignalGraph,
    node: NodeId,
) -> Result<PreparedDependencyCapture, SignalError> {
    let mut capture = PreparedDependencyCapture::new();
    for dependency in graph.runtime_dependencies_of(node)? {
        capture.record(
            dependency.source(),
            dependency.aspect(),
            dependency.scope_ref().cloned(),
        );
    }
    Ok(capture.into_sorted_unique())
}

pub(crate) fn capture_current_dependencies_without_refresh(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<PreparedDependencyCapture, SignalError> {
    let mut capture = PreparedDependencyCapture::new();
    for dependency in graph.current_runtime_dependencies_of(node)? {
        capture.record(
            dependency.source(),
            dependency.aspect(),
            dependency.scope_ref().cloned(),
        );
    }
    Ok(capture.into_sorted_unique())
}

pub(crate) fn runtime_sorted_dependencies(
    graph: &mut SignalGraph,
    node: NodeId,
) -> Result<Vec<DependencyEdge>, SignalError> {
    Ok(graph.runtime_dependencies_of(node)?.to_vec())
}

pub(crate) fn preview_maybe_stale(
    graph: &SignalGraph,
    node: NodeId,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<MaybeStalePreview, SignalError> {
    let snapshot = graph.get_dep_snapshot(node)?;
    let comparator =
        resolver.policy_for_node(node, graph.node_eval_config(node)?.comparator.as_ref());
    let mut requires_upstream_evaluation = Vec::new();
    let mut meaningful_change_detected = false;

    for snapshot_entry in snapshot.entries() {
        if !graph.is_alive(snapshot_entry.source) {
            meaningful_change_detected = true;
            continue;
        }

        if !matches!(graph.get_state(snapshot_entry.source)?, NodeState::Clean) {
            requires_upstream_evaluation.push(snapshot_entry.source);
            continue;
        }

        let current_version = graph.node_version_for_scope(
            snapshot_entry.source,
            snapshot_entry.aspect,
            snapshot_entry.scope.as_ref(),
        )?;
        if let Some(scope) = &snapshot_entry.scope {
            if current_version == snapshot_entry.cached_version {
                continue;
            }
            if partition_scope_untouched(
                graph.node_runtime_artifact_hot(snapshot_entry.source)?,
                scope,
            ) {
                continue;
            }
            meaningful_change_detected = true;
            continue;
        }

        if comparator.has_meaningful_change(
            snapshot_entry.aspect,
            snapshot_entry.cached_version,
            current_version,
            resolver,
        )? {
            meaningful_change_detected = true;
        }
    }

    requires_upstream_evaluation =
        DedupedNodeBatch::canonicalize_unordered(requires_upstream_evaluation).into_vec();

    Ok(MaybeStalePreview {
        unchanged: !meaningful_change_detected && requires_upstream_evaluation.is_empty(),
        requires_upstream_evaluation,
    })
}

pub(crate) fn partition_scope_untouched(
    trace_summary: Option<&crate::data::trace::RuntimeArtifactHot>,
    scope: &PartitionSubscription,
) -> bool {
    trace_summary.is_none_or(|summary| {
        !summary
            .changed_scopes
            .as_slice()
            .iter()
            .any(|changed_scope| {
                if scope.partition != changed_scope.partition {
                    return false;
                }
                match scope.match_mode {
                    crate::data::output::PartitionMatchMode::WholePartition => true,
                    crate::data::output::PartitionMatchMode::PartitionAndDetail => {
                        scope.detail == changed_scope.detail
                    }
                }
            })
    })
}
