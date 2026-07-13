use forge_store_layout_indexes::InterlockedLsmCompaction;

fn forge() -> InterlockedLsmCompaction {
    InterlockedLsmCompaction {
        prepared: panic!(),
        physical: panic!(),
    }
}

fn main() {
    let _ = forge();
}
