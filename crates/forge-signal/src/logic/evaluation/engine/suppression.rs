use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::scope_touched_by_trace;

use super::prepared_apply::revert_to_clean;

pub(super) fn suppress_downstream_if_identity_unchanged(
    graph: &mut SignalGraph,
    node: NodeId,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<u64, SignalError> {
    let mut suppressed = 0_u64;
    let mut stack: Vec<NodeId> = graph.runtime_subscribers_of(node)?.to_vec();
    let mut visited = std::collections::BTreeSet::new();
    while let Some(current) = stack.pop() {
        if !graph.is_alive(current) {
            continue;
        }
        if !visited.insert(current) {
            continue;
        }
        if matches!(graph.get_entry(current)?.get_state(), NodeState::Clean) {
            continue;
        }
        if check_upstream_unchanged_ignoring_source(graph, current, node, comparator_resolver)? {
            revert_to_clean(graph, current)?;
            suppressed += 1;
            stack.extend_from_slice(graph.runtime_subscribers_of(current)?);
        }
    }
    Ok(suppressed)
}

fn check_upstream_unchanged_ignoring_source(
    graph: &SignalGraph,
    node: NodeId,
    ignored_source: NodeId,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<bool, SignalError> {
    let entry = graph.get_entry(node)?;
    let snapshot = graph.get_dep_snapshot(node)?;
    let node_cfg = entry.get_eval_config();
    let comparator = resolver.policy_for_node(node, node_cfg.comparator.as_ref());

    for snapshot_entry in snapshot.entries() {
        if snapshot_entry.source == ignored_source {
            if let Some(scope) = &snapshot_entry.scope {
                if !matches!(
                    graph.get_entry(snapshot_entry.source)?.get_state(),
                    NodeState::Clean
                ) {
                    return Ok(false);
                }
                if scope_touched_by_trace(
                    graph.get_entry(snapshot_entry.source)?.get_trace_summary(),
                    scope,
                ) {
                    return Ok(false);
                }
            }
            continue;
        }
        if !graph.is_alive(snapshot_entry.source) {
            return Ok(false);
        }
        if !matches!(
            graph.get_entry(snapshot_entry.source)?.get_state(),
            NodeState::Clean
        ) {
            return Ok(false);
        }
        let current_version = graph
            .get_entry(snapshot_entry.source)?
            .get_aspect_version()
            .get(snapshot_entry.aspect);
        if let Some(scope) = &snapshot_entry.scope {
            if current_version == snapshot_entry.cached_version {
                continue;
            }
            if !scope_touched_by_trace(
                graph.get_entry(snapshot_entry.source)?.get_trace_summary(),
                scope,
            ) {
                continue;
            }
            return Ok(false);
        }
        if comparator.has_meaningful_change(
            snapshot_entry.aspect,
            snapshot_entry.cached_version,
            current_version,
            resolver,
        )? {
            return Ok(false);
        }
    }

    Ok(true)
}
