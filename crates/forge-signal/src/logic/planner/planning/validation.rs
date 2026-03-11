use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::PartitionSubscription;
use crate::logic::prepared::PreparedDependencyCapture;

#[derive(Debug, Clone, Default)]
pub(crate) struct MaybeStalePreview {
    pub(crate) unchanged: bool,
    pub(crate) requires_upstream_evaluation: Vec<NodeId>,
}

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
    let entry = graph.get_entry(node)?;
    let snapshot = graph.get_dep_snapshot(node)?;
    let comparator = resolver.policy_for_node(node, entry.get_eval_config().comparator.as_ref());
    let mut requires_upstream_evaluation = Vec::new();
    let mut meaningful_change_detected = false;

    for snapshot_entry in snapshot.entries() {
        if !graph.is_alive(snapshot_entry.source) {
            meaningful_change_detected = true;
            continue;
        }

        let source_entry = graph.get_entry(snapshot_entry.source)?;
        if !matches!(source_entry.get_state(), NodeState::Clean) {
            requires_upstream_evaluation.push(snapshot_entry.source);
            continue;
        }

        let current_version = source_entry.get_aspect_version().get(snapshot_entry.aspect);
        if let Some(scope) = &snapshot_entry.scope {
            if current_version == snapshot_entry.cached_version {
                continue;
            }
            if partition_scope_untouched(source_entry.get_trace_summary(), scope) {
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

    requires_upstream_evaluation.sort_by_key(|node| (node.index(), node.generation()));
    requires_upstream_evaluation.dedup();

    Ok(MaybeStalePreview {
        unchanged: !meaningful_change_detected && requires_upstream_evaluation.is_empty(),
        requires_upstream_evaluation,
    })
}

pub(crate) fn partition_scope_untouched(
    trace_summary: Option<&crate::data::trace::TraceSummary>,
    scope: &PartitionSubscription,
) -> bool {
    trace_summary.is_none_or(|summary| {
        !summary.changed_regions.iter().any(|region| {
            if scope.partition != region.partition {
                return false;
            }
            match scope.match_mode {
                crate::data::output::PartitionMatchMode::WholePartition => true,
                crate::data::output::PartitionMatchMode::PartitionAndDetail => {
                    scope.detail == region.detail
                }
            }
        })
    })
}
