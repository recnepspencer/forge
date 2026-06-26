use super::super::counter_accounting::WorthGraphReadAccessCounterAccountingReport;
use super::super::receipt_accounting::WorthGraphReadAccessReceiptAccountingReport;
use super::super::stable_digest;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessBatchAccountingRow {
    batch_scope: String,
    per_read_receipt_row_count: usize,
    aggregate_counter_row_count: usize,
    per_read_association_preserved: bool,
    caller_owned_graph_work_count: usize,
    row_digest: String,
}

impl WorthGraphReadAccessBatchAccountingRow {
    pub(crate) fn from_reports(
        receipt_report: &WorthGraphReadAccessReceiptAccountingReport,
        counter_report: &WorthGraphReadAccessCounterAccountingReport,
    ) -> Self {
        let per_read_receipt_row_count = receipt_report.rows().len();
        let aggregate_counter_row_count = counter_report.rows().len();
        let per_read_association_preserved =
            receipt_identity_set(receipt_report) == counter_receipt_identity_set(counter_report);
        let caller_owned_graph_work_count = counter_report.caller_owned_graph_work_count();
        let batch_scope = "phase_six_receipt_accounting_batch".to_string();
        let row_digest = stable_digest(&[
            "worth_graph_read_access_batch_accounting_row_v1".to_string(),
            format!("scope:{batch_scope}"),
            format!("receipt_rows:{per_read_receipt_row_count}"),
            format!("counter_rows:{aggregate_counter_row_count}"),
            format!("association:{per_read_association_preserved}"),
            format!("caller_work:{caller_owned_graph_work_count}"),
        ]);
        Self {
            batch_scope,
            per_read_receipt_row_count,
            aggregate_counter_row_count,
            per_read_association_preserved,
            caller_owned_graph_work_count,
            row_digest,
        }
    }

    pub fn batch_scope(&self) -> &str {
        &self.batch_scope
    }

    pub const fn per_read_receipt_row_count(&self) -> usize {
        self.per_read_receipt_row_count
    }

    pub const fn aggregate_counter_row_count(&self) -> usize {
        self.aggregate_counter_row_count
    }

    pub const fn per_read_association_preserved(&self) -> bool {
        self.per_read_association_preserved
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

fn receipt_identity_set(
    receipt_report: &WorthGraphReadAccessReceiptAccountingReport,
) -> BTreeSet<&str> {
    receipt_report
        .rows()
        .iter()
        .map(|row| row.receipt_identity().identity_digest())
        .collect()
}

fn counter_receipt_identity_set(
    counter_report: &WorthGraphReadAccessCounterAccountingReport,
) -> BTreeSet<&str> {
    counter_report
        .rows()
        .iter()
        .map(|row| row.receipt_identity_digest())
        .collect()
}
