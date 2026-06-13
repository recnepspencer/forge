use worth_kernel::workload_composition::PlanarBooleanEntryBasis;
use worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt;

fn fake_retained_planar_facts() -> RetainedPlanarFactsReceipt {
    panic!("type-check only")
}

fn main() {
    let retained = fake_retained_planar_facts();
    let _ = PlanarBooleanEntryBasis::bind(retained, "hand-built planar facts basis");
}
