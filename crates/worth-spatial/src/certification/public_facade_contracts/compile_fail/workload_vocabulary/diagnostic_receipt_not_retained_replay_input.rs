use worth_spatial::facade::workload_vocabulary::{DiagnosticWorkload, DiagnosticWorkloadReceipt};

fn main() {
    let diagnostics: DiagnosticWorkloadReceipt = todo!();
    let _ = DiagnosticWorkload::for_retained_replay(&diagnostics);
}
