use worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger;

fn main() {
    let _ = CompleteWorkloadEvidenceLedger {
        ledger: unconstructible(),
    };
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}
