use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionCloseoutCounters;

fn main() {
    let _ = WorthGraphReadAccessPlanAdoptionCloseoutCounters {
        executed_receipt_count: 0,
        receipt_row_count: 0,
        admitted_plan_count: 0,
        admitted_plan_requires_receipt_count: 0,
        required_posture_count: 0,
        denied_posture_count: 0,
        carried_gap_count: 0,
        visible_non_executed_posture_count: 0,
        required_future_receipt_count: 0,
        no_receipt_posture_count: 0,
        accounted_counter_row_count: 0,
        explicit_counter_gap_count: 0,
        no_execution_counter_required_count: 0,
        caller_owned_graph_work_count: 0,
        batch_row_count: 0,
        deleted_path_count: 0,
        capped_residue_count: 0,
        uncapped_residue_count: 0,
        source_firewall_region_count: 0,
        source_firewall_source_count: 0,
        source_firewall_violation_count: 0,
        posture_projection_count: 0,
        cap_row_count: 0,
        counter_digest: String::new(),
    };
}
