use worth_ui::facade::WorthUiNodeReplacementCounters;

fn main() {
    let _ = WorthUiNodeReplacementCounters {
        active_nodes_classified: 0,
        candidate_nodes_classified: 0,
        preserved_node_count: 0,
        replaced_node_count: 0,
        dropped_node_count: 0,
        created_node_count: 0,
        moved_node_count: 0,
        rebound_node_count: 0,
        lane_changed_node_count: 0,
        ambiguous_node_count: 0,
    };
}
