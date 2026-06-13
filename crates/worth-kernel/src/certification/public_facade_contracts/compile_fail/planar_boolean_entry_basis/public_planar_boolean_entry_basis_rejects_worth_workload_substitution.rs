use worth_kernel::workload_composition::{PlanarBooleanEntryBasis, WorthWorkload};

fn fake_workload() -> WorthWorkload {
    panic!("type-check only")
}

fn main() {
    let workload = fake_workload();
    let _ = PlanarBooleanEntryBasis::bind(workload, "worth workload basis");
}
