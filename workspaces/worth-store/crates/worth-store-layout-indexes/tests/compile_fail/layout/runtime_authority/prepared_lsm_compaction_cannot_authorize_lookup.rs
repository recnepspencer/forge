use worth_store_layout_indexes::PreparedLsmCompaction;

fn bypass(prepared: &PreparedLsmCompaction) {
    let _ = prepared.admit_lookup_source();
}

fn main() {}
