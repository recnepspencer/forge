use worth_ui::facade::WorthUiRuntimeHandleAllocationCounters;

fn main() {
    let _counters = WorthUiRuntimeHandleAllocationCounters {
        plan_node_input_count: 1,
        component_handle_count: 1,
        command_handle_count: 0,
        token_handle_count: 0,
        child_range_handle_count: 0,
        view_binding_handle_count: 0,
        lane_handle_count: 0,
        state_slot_handle_count: 0,
        collision_check_count: 1,
        collision_denial_count: 0,
        source_parse_count: 0,
        registry_string_lookup_count: 0,
        broad_registry_scan_count: 0,
    };
}
