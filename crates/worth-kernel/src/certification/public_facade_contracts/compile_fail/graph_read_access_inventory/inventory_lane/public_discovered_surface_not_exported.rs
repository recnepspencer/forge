use worth_kernel::graph_read_access_inventory::inventory_lane::WorthGraphReadAccessDiscoveredSurface;

fn main() {
    let _ = WorthGraphReadAccessDiscoveredSurface::new(
        "crates/worth-spatial/src/workload_platform/new_boolean_frontier",
        "relation loop with local cache",
        false,
    );
}
