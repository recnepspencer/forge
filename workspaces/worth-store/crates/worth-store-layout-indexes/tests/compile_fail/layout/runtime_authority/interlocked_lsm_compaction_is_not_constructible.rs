use worth_store_layout_indexes::InterlockedLsmCompaction;

fn worth() -> InterlockedLsmCompaction {
    InterlockedLsmCompaction {
        prepared: panic!(),
        physical: panic!(),
    }
}

fn main() {
    let _ = worth();
}
