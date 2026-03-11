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
        let Ok(entry) = graph.get_entry(node) else {
            continue;
        };
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
        let Ok(dependencies) = graph.dependencies_of(node) else {
            continue;
        };
        for dependency in dependencies {
            let mut edge_label = format!("aspect:{}", dependency.aspect().index());
            if let Some(scope) = dependency.scope_ref() {
                edge_label.push_str(&format!("\\nscope:{:?}", scope));
            }
            let _ = writeln!(
                dot,
                "  \"{}\" -> \"{}\" [label=\"{}\"];",
                dependency.source(),
                node,
                edge_label
            );
        }
    }

    dot.push_str("}\n");
    dot
}
