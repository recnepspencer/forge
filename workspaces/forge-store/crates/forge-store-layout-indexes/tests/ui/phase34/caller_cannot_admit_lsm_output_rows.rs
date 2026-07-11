use forge_store_layout_indexes::layout_strategy_admission::S8LsmTombstoneLaw;

fn bypass(law: S8LsmTombstoneLaw, claimed_output: &[u8]) {
    let _ = law.admit_compaction_product(claimed_output);
}

fn main() {}
