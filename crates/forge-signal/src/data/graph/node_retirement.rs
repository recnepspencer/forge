use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;

use super::scratch::ScratchLeaseKind;
use super::signal_graph::SignalGraph;

impl SignalGraph {
    pub fn unregister_node(&mut self, id: NodeId) -> Result<(), SignalError> {
        self.validate_handle(id)?;
        self.with_scratch(ScratchLeaseKind::Churn, |graph, scratch| {
            scratch.node_buffer_a.clear();
            scratch.node_buffer_b.clear();

            scratch
                .node_buffer_a
                .extend(graph.runtime_dependencies_of(id)?.iter().map(|edge| edge.source()));
            scratch
                .node_buffer_b
                .extend(graph.runtime_subscribers_of(id)?.iter().copied());

            for &source in &scratch.node_buffer_a {
                if graph.is_alive(source) {
                    graph.remove_subscriber_edge(source, id)?;
                }
            }

            for &subscriber in &scratch.node_buffer_b {
                if graph.is_alive(subscriber) {
                    graph.remove_dependencies_on(subscriber, id)?;
                    graph.get_entry_mut(subscriber)?.set_state(NodeState::Dirty);
                }
            }

            debug_assert!(
                !graph.free_slots.contains(id.index() as usize),
                "free list already contained slot {} before unregister",
                id.index()
            );
            graph.nodes[id.index() as usize].vacate();
            graph.active_nodes = graph.active_nodes.saturating_sub(1);
            graph.tombstone_count += 1;
            graph.gc_compaction_debt = graph.gc_compaction_debt.saturating_add(1);
            if !graph.nodes[id.index() as usize].is_retired() {
                graph.free_list.push(id.index());
                graph.free_slots.mark(id.index() as usize);
            }
            Ok(())
        })
    }
}
