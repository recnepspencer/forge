use worth_ui::facade::WorthUiPlanTopologyCounters;

fn main() {
    let _counters = WorthUiPlanTopologyCounters {
        plan_node_input_count: 0,
        topology_node_count: 0,
        child_range_count: 0,
        lane_partition_count: 0,
        lookup_entry_count: 0,
        render_resource_ref_count: 0,
        topology_validation_count: 0,
        artifact_tree_scan_count: 0,
        registry_string_lookup_count: 0,
        broad_registry_scan_count: 0,
        denial_count: 0,
    };
}
