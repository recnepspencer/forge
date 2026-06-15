use worth_ui::facade::WorthUiPlanLoweringCounters;

fn main() {
    let _ = WorthUiPlanLoweringCounters {
        staged_node_input_count: 1,
        query_binding_input_count: 1,
        reconciliation_receipt_input_count: 1,
        component_hook_input_count: 0,
        rejected_component_hook_count: 0,
        readiness_verification_count: 1,
        epoch_verification_count: 1,
        source_parse_count: 0,
        registry_string_lookup_count: 0,
    };
}
