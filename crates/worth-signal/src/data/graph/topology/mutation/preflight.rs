use std::collections::BTreeMap;

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

    let mut changed = Vec::new();
    for (&node, desired) in &desired_by_node {
        if graph.raw_dependencies_of(node)? != desired.as_slice() {
            changed.push(node);
        }
    }
    reject_batch_cycles(graph, &changed, &desired_by_node)?;
    Ok(desired_by_node.into_iter().collect())
}

fn reject_batch_cycles(
    graph: &SignalGraph,
    roots: &[NodeId],
    desired_by_node: &BTreeMap<NodeId, CanonicalDependencies>,
) -> Result<(), SignalError> {
    let capacity = graph.arena_capacity();
    let mut visit_state = vec![0_u8; capacity];
    let mut active_positions = vec![None::<usize>; capacity];
    for &root in roots {
        if visit_state[root.index() as usize] == 2 {
            continue;
        }
        let mut stack = vec![DependencyFrame::new(
            root,
            dependency_sources(graph, root, desired_by_node)?,
        )];
        visit_state[root.index() as usize] = 1;
        active_positions[root.index() as usize] = Some(0);
        while let Some(frame) = stack.last_mut() {
            let Some(next) = frame.next_source() else {
                let completed = stack.pop().expect("active dependency frame must exist");
                let index = completed.node.index() as usize;
                active_positions[index] = None;
                visit_state[index] = 2;
                continue;
            };
            let next_index = next.index() as usize;
            if let Some(cycle_start) = active_positions[next_index] {
                let mut path = stack[cycle_start..]
                    .iter()
                    .map(|frame| frame.node)
                    .collect::<Vec<_>>();
                path.push(next);
                return Err(SignalError::cycle_detected(path));
            }
            if visit_state[next_index] == 2 {
                continue;
            }
            let next_position = stack.len();
            stack.push(DependencyFrame::new(
                next,
                dependency_sources(graph, next, desired_by_node)?,
            ));
            visit_state[next_index] = 1;
            active_positions[next_index] = Some(next_position);
        }
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
