use forge_store_layout_indexes::DegradedScanReady;

fn forge() -> DegradedScanReady {
    DegradedScanReady {
        recipe: panic!(),
        current_materialization: panic!(),
    }
}

fn main() {
    let _ = forge();
}
