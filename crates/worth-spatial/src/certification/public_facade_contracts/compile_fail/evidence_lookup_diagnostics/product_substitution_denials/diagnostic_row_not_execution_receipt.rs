use worth_spatial::facade::evidence_lookup_diagnostics::EvidenceLookupDiagnosticRow;
use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;

fn require_execution_receipt(_: &EvidenceLookupExecutionReceipt) {}

fn main() {
    let _: fn(&EvidenceLookupDiagnosticRow) = require_execution_receipt;
}
