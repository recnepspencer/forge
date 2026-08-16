use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use std::collections::BTreeSet;

pub(super) fn requested_dependency_order(
    graph: &SignalGraph,
    target: NodeId,
) -> Result<Vec<NodeId>, SignalError> {
    let mut discovered = BTreeSet::new();
    let mut order = Vec::new();
    let mut stack = vec![(target, false)];
    while let Some((node, expanded)) = stack.pop() {
        if expanded {
            order.push(node);
            continue;
        }
        if !discovered.insert(node) {
            continue;
        }
        stack.push((node, true));
        for dependency in graph.dependencies_of(node)?.iter().rev() {
            stack.push((dependency.source(), false));
        }
    }
    Ok(order)
}
