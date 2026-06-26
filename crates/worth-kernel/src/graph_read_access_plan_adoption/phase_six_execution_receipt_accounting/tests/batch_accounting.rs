use super::production_phase_six_closeout;
use crate::graph_read_access_plan_adoption::WorthGraphReadAccessCounterAccountingReport;

#[test]
fn batched_execution_reports_per_read_and_aggregate_counters() {
    let closeout = production_phase_six_closeout();
    let batch_report = closeout.batch_accounting_report();

    assert!(batch_report.per_read_association_preserved());
    assert_eq!(
        batch_report.rows()[0].per_read_receipt_row_count(),
        closeout.receipt_accounting_report().rows().len()
    );
    assert_eq!(
        batch_report.rows()[0].aggregate_counter_row_count(),
        closeout.counter_accounting_report().rows().len()
    );
    assert_eq!(
        batch_report.aggregate_counter_digest(),
        closeout.counter_accounting_report().report_digest()
    );
}

#[test]
fn batched_execution_detects_lost_per_read_counter_association() {
    let closeout = production_phase_six_closeout();
    let mut adversarial_counter_rows = closeout.counter_accounting_report().rows().to_vec();
    adversarial_counter_rows[0] = adversarial_counter_rows[0]
        .with_receipt_identity_digest_for_tests("adversarial-unmatched-receipt-identity");
    let adversarial_counter_report =
        WorthGraphReadAccessCounterAccountingReport::from_rows_for_tests(adversarial_counter_rows);
    let batch_report = super::super::batch_accounting::build_batch_accounting_report(
        closeout.receipt_accounting_report(),
        &adversarial_counter_report,
    );

    assert!(!batch_report.per_read_association_preserved());
    assert_ne!(
        batch_report.rows()[0].per_read_receipt_row_count(),
        0,
        "adversarial report must still exercise real receipt rows"
    );
}
