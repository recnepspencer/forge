use worth_kernel::graph_read_access_inventory::inventory_lane::WorthGraphReadAccessCoverageGuardReport;

fn main() {
    let _ = WorthGraphReadAccessCoverageGuardReport {
        discovered_surface_count: 1,
        covered_source_count: 1,
        admitted_surface_count: 1,
        unclassified_surface_count: 0,
        production_shaped_test_support_gap_count: 0,
    };
}
