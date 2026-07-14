use worth_store_layout_indexes::PublishedLsmCompaction;

fn skip_selection(witness: &PublishedLsmCompaction, sequence: u64) {
    let _ = witness.execute_lookup_latest_visible_record(sequence);
}
