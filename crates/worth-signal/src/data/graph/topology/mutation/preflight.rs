use std::collections::{BTreeMap, BTreeSet};

use crate::data::dependency::{CanonicalDependencies, DependencyEdge};
use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;

pub(super) fn canonicalize_and_preflight(
    graph: &SignalGraph,
    reconciliations: &[(NodeId, &[DependencyEdge])],
) -> Result<Vec<(NodeId, CanonicalDependencies)>, SignalError> {
    let mut desired_by_node = BTreeMap::new();
    for &(node, desired) in reconciliations {
        graph.validate_handle(node)?;
        if desired_by_node.contains_key(&node) {
            return Err(SignalError::invalid_input(format!(
                "dependency mutation contains duplicate target {node}"
            )));
        }
        for edge in desired {
            graph.validate_handle(edge.source())?;
        }
        desired_by_node.insert(node, CanonicalDependencies::new(desired.iter().cloned()));
    }

    for &node in desired_by_node.keys() {
        if graph.raw_dependencies_of(node)? == desired_by_node[&node].as_slice() {
            continue;
        }
        reject_reachable_cycle(graph, node, &desired_by_node)?;
    }
    Ok(desired_by_node.into_iter().collect())
}

fn reject_reachable_cycle(
    graph: &SignalGraph,
    root: NodeId,
    desired_by_node: &BTreeMap<NodeId, CanonicalDependencies>,
) -> Result<(), SignalError> {
    let mut active_positions = BTreeMap::<NodeId, usize>::new();
    let mut finished = BTreeSet::<NodeId>::new();
    let mut stack = vec![DependencyFrame::new(
        root,
        dependency_sources(graph, root, desired_by_node)?,
    )];
    active_positions.insert(root, 0);

    while let Some(frame) = stack.last_mut() {
        let Some(next) = frame.next_source() else {
            let completed = stack.pop().expect("active dependency frame must exist");
            active_positions.remove(&completed.node);
            finished.insert(completed.node);
            continue;
        };
        if let Some(&cycle_start) = active_positions.get(&next) {
            let mut path = stack[cycle_start..]
                .iter()
                .map(|frame| frame.node)
                .collect::<Vec<_>>();
            path.push(next);
            return Err(SignalError::cycle_detected(path));
        }
        if finished.contains(&next) {
            continue;
        }
        let next_position = stack.len();
        stack.push(DependencyFrame::new(
            next,
            dependency_sources(graph, next, desired_by_node)?,
        ));
        active_positions.insert(next, next_position);
    }
    Ok(())
}

fn dependency_sources(
    graph: &SignalGraph,
    node: NodeId,
    desired_by_node: &BTreeMap<NodeId, CanonicalDependencies>,
) -> Result<Vec<NodeId>, SignalError> {
    let edges = match desired_by_node.get(&node) {
        Some(desired) => desired.as_slice(),
        None => graph.raw_dependencies_of(node)?,
    };
    Ok(edges
        .iter()
        .map(DependencyEdge::source)
        .filter(|source| graph.is_alive(*source))
        .collect())
}

struct DependencyFrame {
    node: NodeId,
    sources: Vec<NodeId>,
    next_index: usize,
}

impl DependencyFrame {
    fn new(node: NodeId, sources: Vec<NodeId>) -> Self {
        Self {
            node,
            sources,
            next_index: 0,
        }
    }

    fn next_source(&mut self) -> Option<NodeId> {
        let source = self.sources.get(self.next_index).copied();
        self.next_index += usize::from(source.is_some());
        source
    }
}
