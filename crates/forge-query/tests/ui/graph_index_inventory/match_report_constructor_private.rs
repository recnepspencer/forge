use forge_query::facade::runtime::{
    ForgeQueryGraphIndexInventoryCounters, ForgeQueryGraphIndexInventoryMatchReport,
};

fn main() {
    let _ = ForgeQueryGraphIndexInventoryMatchReport {
        digest: String::new(),
        inventory_digest: String::new(),
        requirement_set_digest: String::new(),
        matches: Vec::new(),
        counters: ForgeQueryGraphIndexInventoryCounters::new(0, 0, 0, 0, 0),
    };
}
