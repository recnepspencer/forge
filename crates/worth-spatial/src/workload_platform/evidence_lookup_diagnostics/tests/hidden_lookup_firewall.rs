use crate::workload_platform::evidence_lookup_diagnostics::derive_evidence_lookup_diagnostics;

use super::fixtures::{
    supported_projection_path, supported_projection_path_with_extra_unrelated_receipts,
};

#[test]
fn diagnostic_projection_cannot_perform_hidden_lookup() {
    let baseline = supported_projection_path();
    let expanded = supported_projection_path_with_extra_unrelated_receipts(4);

    let diagnostics =
        derive_evidence_lookup_diagnostics(baseline.selected_plan(), baseline.execution_receipt())
            .expect("baseline diagnostics derive");
    let expanded_diagnostics =
        derive_evidence_lookup_diagnostics(expanded.selected_plan(), expanded.execution_receipt())
            .expect("expanded diagnostics derive");

    assert_eq!(
        diagnostics.counters().hidden_lookup_scan_count(),
        baseline
            .selected_plan()
            .counters()
            .raw_evidence_row_scan_count()
            + baseline
                .execution_receipt()
                .counters()
                .caller_owned_scan_count()
    );
    assert_eq!(
        diagnostics.counters().hidden_broad_receipt_scan_count(),
        baseline
            .selected_plan()
            .counters()
            .broad_receipt_scan_count()
    );
    assert_eq!(diagnostics.counters().hidden_lookup_scan_count(), 0);
    assert_eq!(diagnostics.counters().hidden_broad_receipt_scan_count(), 0);
    assert_eq!(diagnostics.counters(), expanded_diagnostics.counters());
}
