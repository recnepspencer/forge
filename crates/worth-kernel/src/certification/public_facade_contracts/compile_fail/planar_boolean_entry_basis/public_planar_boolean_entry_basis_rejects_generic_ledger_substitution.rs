use worth_kernel::workload_composition::PlanarBooleanEntryBasis;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceLedger;

fn main() {
    let ledger = WorkloadEvidenceLedger::from_rows(Vec::new()).expect("inspectable ledger");
    let _ = PlanarBooleanEntryBasis::bind(ledger, "generic ledger basis");
}
