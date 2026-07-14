use forge_store_layout_indexes::{DerivedIndexParityBasis, LayoutCoverageWitness};

fn forge(coverage: LayoutCoverageWitness) {
    let _ = DerivedIndexParityBasis::new(Vec::new(), coverage, true, Vec::new());
}

fn main() {}
