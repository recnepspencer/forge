use std::fmt::Write;

use crate::data::graph::SignalGraph;
use crate::data::node::NodeState;

fn node_color(state: NodeState) -> &'static str {
    match state {
        NodeState::Clean => "green",
        NodeState::MaybeStale => "yellow",
        NodeState::Dirty => "red",
    }
}

/// Render the current signal graph as Graphviz DOT.
pub fn to_dot(graph: &SignalGraph) -> String {
    let mut dot = String::from("digraph SignalGraph {\n  rankdir=LR;\n");

    for index in 0..graph.arena_capacity() {
        let Some(node) = graph.live_node_id_at(index) else {
            continue;
        };
        let entry = graph
            .get_entry(node)
            .expect("live node should always resolve during DOT export");
        let mut label = format!(
            "{}\\nstate={:?}\\ncondition={:?}",
            node,
            entry.get_state(),
            entry.get_eval_config().condition
        );
        if !entry.get_dirty_aspects().is_empty() {
            label.push_str(&format!("\\ndirty={:?}", entry.get_dirty_aspects()));
        }
        let _ = writeln!(
            dot,
            "  \"{}\" [label=\"{}\", style=filled, fillcolor={}];",
            node,
            label,
            node_color(*entry.get_state())
        );
    }

    for index in 0..graph.arena_capacity() {
        let Some(node) = graph.live_node_id_at(index) else {
            continue;
        };
        let entry = graph
            .get_entry(node)
            .expect("live node should always resolve during DOT export");
        for dependency in entry.get_dependencies() {
            let _ = writeln!(
                dot,
                "  \"{}\" -> \"{}\" [label=\"aspect:{}\"];",
                dependency.source(),
                node,
                dependency.aspect().index()
            );
        }
    }

    dot.push_str("}\n");
    dot
}
