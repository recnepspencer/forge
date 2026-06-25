use crate::graph_read_access_plan_adoption::WorthGraphReadAccessSpatialDensePhaseSixSeed;

use super::batch_accounting::WorthGraphReadAccessBatchAccountingReport;
use super::counter_accounting::WorthGraphReadAccessCounterAccountingReport;
use super::receipt_accounting::WorthGraphReadAccessReceiptAccountingReport;
use super::stable_digest;

pub(crate) fn execution_receipt_accounting_closeout_digest(
    seed: &WorthGraphReadAccessSpatialDensePhaseSixSeed,
    receipt_report: &WorthGraphReadAccessReceiptAccountingReport,
    counter_report: &WorthGraphReadAccessCounterAccountingReport,
    batch_report: &WorthGraphReadAccessBatchAccountingReport,
) -> String {
    stable_digest(&[
        "worth_graph_read_access_execution_receipt_accounting_closeout_v1".to_string(),
        format!("seed:{}", seed.seed_digest()),
        format!("phase_five:{}", seed.phase_five_closeout_digest()),
        format!("receipt:{}", receipt_report.report_digest()),
        format!("counter:{}", counter_report.report_digest()),
        format!("batch:{}", batch_report.report_digest()),
    ])
}
