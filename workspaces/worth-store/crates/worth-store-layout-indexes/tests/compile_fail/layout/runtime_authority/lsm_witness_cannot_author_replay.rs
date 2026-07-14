use worth_store_layout_indexes::PublishedLsmCompaction;

fn bypass(witness: &PublishedLsmCompaction) {
    let _ = witness.execute_replay_wal_tail();
}

fn main() {}
