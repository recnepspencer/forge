use worth_ui::facade::WorthUiPlanInspectionCounters;

fn main() {
    let _counters = WorthUiPlanInspectionCounters {
        inspection_count: 0,
        plan_digest_count: 0,
        node_inspection_count: 0,
        lane_inspection_count: 0,
        provenance_link_count: 0,
        query_link_preservation_count: 0,
        projection_consumption_link_count: 0,
        causal_inspection_reference_count: 0,
        ordinary_outcome_reference_count: 0,
        artifact_tree_scan_count: 0,
        source_archaeology_count: 0,
        registry_lookup_count: 0,
        diagnostic_policy_read_count: 0,
        frame_path_materialization_count: 0,
        denial_count: 0,
    };
}
