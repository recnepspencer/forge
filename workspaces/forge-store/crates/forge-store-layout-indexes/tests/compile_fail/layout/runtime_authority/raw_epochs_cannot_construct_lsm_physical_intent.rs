use forge_store_layout_indexes::LsmPhysicalCompactionIntent;

fn bypass() -> LsmPhysicalCompactionIntent {
    LsmPhysicalCompactionIntent::new(1, 2, 3).unwrap()
}

fn main() {
    let _ = bypass();
}
