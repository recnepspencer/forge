use worth_kernel::graph_read_access_inventory::inventory_lane::WorthGraphReadAccessScopeReport;

fn main() {
    let _report = WorthGraphReadAccessScopeReport {
        scoped_row_count: 1,
        selected_obligation_scoped_count: 1,
        touched_authority_scoped_count: 0,
        touch_descriptor_scoped_count: 0,
        topology_read_proof_scoped_count: 0,
        spatial_continuation_scoped_count: 0,
        deleted_graph_read_source_scoped_count: 0,
        certification_only_scoped_count: 0,
        out_of_scope_count: 0,
    };
}
