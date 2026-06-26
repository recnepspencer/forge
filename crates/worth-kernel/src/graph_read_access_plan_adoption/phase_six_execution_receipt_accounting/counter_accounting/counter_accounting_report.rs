use crate::graph_read_access_plan_adoption::WorthGraphReadAccessSpatialDensePhaseSixSeed;

use super::super::receipt_accounting::WorthGraphReadAccessReceiptAccountingReport;
use super::super::stable_digest;
use super::{
    WorthGraphReadAccessCounterAccountingRow, WorthGraphReadAccessCounterAccountingStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessCounterAccountingReport {
    rows: Vec<WorthGraphReadAccessCounterAccountingRow>,
    accounted_counter_row_count: usize,
    explicit_counter_gap_count: usize,
    no_execution_counter_required_count: usize,
    caller_owned_graph_work_count: usize,
    report_digest: String,
}

pub(crate) fn build_counter_accounting_report(
    seed: &WorthGraphReadAccessSpatialDensePhaseSixSeed,
    receipt_report: &WorthGraphReadAccessReceiptAccountingReport,
) -> WorthGraphReadAccessCounterAccountingReport {
    let mut rows = Vec::new();
    if let Some(phase_four_receipt_row) = receipt_report.rows().first() {
        rows.push(
            WorthGraphReadAccessCounterAccountingRow::from_phase_four_receipt(
                phase_four_receipt_row,
                seed.phase_four_receipt_projection(),
            ),
        );
    }
    rows.extend(
        receipt_report
            .rows()
            .iter()
            .skip(1)
            .map(WorthGraphReadAccessCounterAccountingRow::from_receipt_row),
    );
    WorthGraphReadAccessCounterAccountingReport::from_rows(rows)
}

impl WorthGraphReadAccessCounterAccountingReport {
    fn from_rows(rows: Vec<WorthGraphReadAccessCounterAccountingRow>) -> Self {
        let accounted_counter_row_count = rows
            .iter()
            .filter(|row| {
                row.status() == WorthGraphReadAccessCounterAccountingStatus::QueryCountersAccounted
            })
            .count();
        let explicit_counter_gap_count = rows
            .iter()
            .filter(|row| {
                row.status()
                    == WorthGraphReadAccessCounterAccountingStatus::CounterGapRequiresQueryReceiptSurface
            })
            .count();
        let no_execution_counter_required_count = rows
            .iter()
            .filter(|row| {
                row.status()
                    == WorthGraphReadAccessCounterAccountingStatus::NoExecutionCountersRequired
            })
            .count();
        let caller_owned_graph_work_count = rows
            .iter()
            .map(|row| row.caller_owned_work().total_count())
            .sum();
        let report_digest = stable_digest(
            &std::iter::once("worth_graph_read_access_counter_accounting_report_v1".to_string())
                .chain(rows.iter().map(|row| format!("row:{}", row.row_digest())))
                .chain([
                    format!("accounted:{accounted_counter_row_count}"),
                    format!("explicit_gap:{explicit_counter_gap_count}"),
                    format!("not_required:{no_execution_counter_required_count}"),
                    format!("caller_work:{caller_owned_graph_work_count}"),
                ])
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            accounted_counter_row_count,
            explicit_counter_gap_count,
            no_execution_counter_required_count,
            caller_owned_graph_work_count,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[WorthGraphReadAccessCounterAccountingRow] {
        &self.rows
    }

    pub const fn accounted_counter_row_count(&self) -> usize {
        self.accounted_counter_row_count
    }

    pub const fn explicit_counter_gap_count(&self) -> usize {
        self.explicit_counter_gap_count
    }

    pub const fn no_execution_counter_required_count(&self) -> usize {
        self.no_execution_counter_required_count
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[cfg(test)]
mod adversarial_counter_accounting_report {
    use super::*;

    impl WorthGraphReadAccessCounterAccountingReport {
        pub(crate) fn from_rows_for_tests(
            rows: Vec<WorthGraphReadAccessCounterAccountingRow>,
        ) -> Self {
            Self::from_rows(rows)
        }

        pub(crate) fn with_caller_owned_graph_work_for_tests(&self) -> Self {
            let mut report = self.clone();
            report.caller_owned_graph_work_count = self.caller_owned_graph_work_count + 1;
            report.report_digest = stable_digest(&[
                "worth_graph_read_access_counter_accounting_report_adversarial_caller_work_v1"
                    .to_string(),
                format!("source:{}", self.report_digest),
                format!("caller_work:{}", report.caller_owned_graph_work_count),
            ]);
            report
        }
    }
}
