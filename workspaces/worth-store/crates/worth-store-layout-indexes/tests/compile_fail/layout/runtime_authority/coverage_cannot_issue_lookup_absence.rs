use worth_store_layout_indexes::{access_planning, LayoutCoverageWitness};

fn worth(coverage: LayoutCoverageWitness) {
    let _ = access_planning().prove_exact_index_absence(coverage);
}

fn main() {}
