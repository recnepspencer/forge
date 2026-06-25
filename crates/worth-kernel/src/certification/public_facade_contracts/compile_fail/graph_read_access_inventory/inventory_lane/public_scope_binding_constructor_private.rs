use worth_kernel::graph_read_access_inventory::inventory_lane::{
    WorthGraphReadAccessScopeBinding, WorthGraphReadAccessScopeFamily,
};

fn main() {
    let _binding = WorthGraphReadAccessScopeBinding::selected_obligation(
        "crates/worth-topo/src/projection/read_views/domain",
        0,
        WorthGraphReadAccessScopeFamily::TopologyReadLedger,
        "authority-a",
        "touch-a",
        "execution-a",
        "registration-a",
    );
}
