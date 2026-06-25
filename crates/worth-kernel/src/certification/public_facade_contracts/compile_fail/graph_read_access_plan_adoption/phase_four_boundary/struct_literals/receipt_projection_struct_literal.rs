use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessSliceReceiptProjection;

fn main() {
    let _ = WorthGraphReadAccessSliceReceiptProjection {
        selected_slice_digest: String::new(),
        plan_projection_digest: String::new(),
        status: unimplemented!(),
        plan_consumption_digest: None,
        execution_basis: String::new(),
        requirement_row_digest: None,
        declared_read_family_identity_digest: None,
        executed_read_family_digest: None,
        admitted_plan_digest: None,
        query_admission_digest: None,
        query_requirement_set_digest: None,
        executor_entry_count: 0,
        materialized_row_count: 0,
        local_strategy_recompute_count: 0,
        local_edge_scan_count: 0,
        local_neighbor_lookup_count: 0,
        persistent_artifact_bypass_count: 0,
        required_query_surface: None,
        required_worth_surface: None,
        existing_worth_execution_surface: None,
        blocker: None,
        projection_digest: String::new(),
    };
}
