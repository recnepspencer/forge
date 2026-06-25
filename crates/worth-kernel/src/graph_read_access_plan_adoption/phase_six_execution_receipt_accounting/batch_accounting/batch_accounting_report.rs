use super::super::counter_accounting::WorthGraphReadAccessCounterAccountingReport;
use super::super::receipt_accounting::WorthGraphReadAccessReceiptAccountingReport;
use super::super::stable_digest;
use super::WorthGraphReadAccessBatchAccountingRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessBatchAccountingReport {
    rows: Vec<WorthGraphReadAccessBatchAccountingRow>,
    per_read_association_preserved: bool,
    aggregate_counter_digest: String,
    caller_owned_graph_work_count: usize,
    report_digest: String,
}

pub(crate) fn build_batch_accounting_report(
    receipt_report: &WorthGraphReadAccessReceiptAccountingReport,
    counter_report: &WorthGraphReadAccessCounterAccountingReport,
) -> WorthGraphReadAccessBatchAccountingReport {
    WorthGraphReadAccessBatchAccountingReport::from_rows(
        vec![WorthGraphReadAccessBatchAccountingRow::from_reports(
            receipt_report,
            counter_report,
        )],
        counter_report.report_digest().to_string(),
    )
}

impl WorthGraphReadAccessBatchAccountingReport {
    fn from_rows(
        rows: Vec<WorthGraphReadAccessBatchAccountingRow>,
        aggregate_counter_digest: String,
    ) -> Self {
        let per_read_association_preserved = rows
            .iter()
            .all(WorthGraphReadAccessBatchAccountingRow::per_read_association_preserved);
        let caller_owned_graph_work_count = rows
            .iter()
            .map(WorthGraphReadAccessBatchAccountingRow::caller_owned_graph_work_count)
            .sum();
        let report_digest = stable_digest(
            &std::iter::once("worth_graph_read_access_batch_accounting_report_v1".to_string())
                .chain(rows.iter().map(|row| format!("row:{}", row.row_digest())))
                .chain([
                    format!("association:{per_read_association_preserved}"),
                    format!("aggregate_counter:{aggregate_counter_digest}"),
                    format!("caller_work:{caller_owned_graph_work_count}"),
                ])
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            per_read_association_preserved,
            aggregate_counter_digest,
            caller_owned_graph_work_count,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[WorthGraphReadAccessBatchAccountingRow] {
        &self.rows
    }

    pub const fn per_read_association_preserved(&self) -> bool {
        self.per_read_association_preserved
    }

    pub fn aggregate_counter_digest(&self) -> &str {
        &self.aggregate_counter_digest
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[cfg(test)]
mod adversarial_batch_accounting_report {
    use super::*;

    impl WorthGraphReadAccessBatchAccountingReport {
        pub(crate) fn empty_for_tests() -> Self {
            Self::from_rows(Vec::new(), "adversarial-empty-counter-digest".to_string())
        }

        pub(crate) fn with_lost_per_read_association_for_tests(&self) -> Self {
            let mut report = self.clone();
            report.per_read_association_preserved = false;
            report.report_digest = stable_digest(&[
                "worth_graph_read_access_batch_accounting_report_adversarial_lost_association_v1"
                    .to_string(),
                format!("source:{}", self.report_digest),
            ]);
            report
        }

        pub(crate) fn with_caller_owned_graph_work_for_tests(&self) -> Self {
            let mut report = self.clone();
            report.caller_owned_graph_work_count = self.caller_owned_graph_work_count + 1;
            report.report_digest = stable_digest(&[
                "worth_graph_read_access_batch_accounting_report_adversarial_caller_work_v1"
                    .to_string(),
                format!("source:{}", self.report_digest),
                format!("caller_work:{}", report.caller_owned_graph_work_count),
            ]);
            report
        }
    }
}
