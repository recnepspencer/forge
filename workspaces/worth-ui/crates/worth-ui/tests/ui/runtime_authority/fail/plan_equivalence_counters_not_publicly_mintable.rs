use worth_ui::facade::WorthUiExecutionPlanEquivalenceCounters;

fn main() {
    let _ = WorthUiExecutionPlanEquivalenceCounters {
        plan_digest_count: 1,
        plan_node_digest_count: 1,
        child_range_digest_count: 1,
        lane_partition_digest_count: 1,
        lookup_index_digest_count: 1,
        egui_boundary_digest_count: 1,
        render_resource_digest_count: 1,
        equivalence_comparison_count: 1,
        artifact_tree_scan_count: 0,
        pointer_identity_comparison_count: 0,
        diagnostic_policy_read_count: 0,
    };
}
