use worth_query::facade::runtime::{WorthQueryGraphIndexInventoryCounters, WorthQueryGraphIndexInventoryMatchReport};

fn main() {
    let _ = WorthQueryGraphIndexInventoryMatchReport {
        digest: String::new(),
        inventory_digest: String::new(),
        requirement_set_digest: String::new(),
        matches: Vec::new(),
        counters: WorthQueryGraphIndexInventoryCounters::new(0, 0, 0, 0, 0),
    };
}
