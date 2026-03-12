use crate::data::error::SignalError;
use crate::data::graph::{SignalGraph, TraversalScratch};
use crate::data::handle::NodeId;

pub(super) fn detect_reachable_cycles(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    candidates: &[NodeId],
) -> Result<(), SignalError> {
    for &candidate in candidates {
        detect_cycle_from(graph, scratch, candidate)?;
    }
    Ok(())
}

fn detect_cycle_from(
    graph: &mut SignalGraph,
    scratch: &mut TraversalScratch,
    node: NodeId,
) -> Result<(), SignalError> {
    scratch.cycle_stack.clear();
    scratch.cycle_stack.push((node, false));
    while let Some((current, expanded)) = scratch.cycle_stack.pop() {
        let index = current.index() as usize;
        if expanded {
            scratch.cycle_visiting.clear_mark(index);
            scratch.cycle_finished.mark(index);
            continue;
        }
        if scratch.cycle_finished.is_marked(index) {
            continue;
        }
        if scratch.cycle_visiting.is_marked(index) {
            return Err(circular_reference_error(scratch, current));
        }

        scratch.cycle_visiting.mark(index);
        scratch.cycle_stack.push((current, true));
        if let Ok(subscribers) = graph.runtime_subscribers_of(current) {
            for &subscriber in subscribers.iter().rev() {
                scratch.cycle_stack.push((subscriber, false));
            }
        }
    }
    Ok(())
}

fn circular_reference_error(scratch: &TraversalScratch, node: NodeId) -> SignalError {
    let mut path = scratch
        .cycle_stack
        .iter()
        .map(|(cycle_node, _)| *cycle_node)
        .collect::<Vec<_>>();
    path.push(node);
    SignalError::cycle_detected(path)
}
