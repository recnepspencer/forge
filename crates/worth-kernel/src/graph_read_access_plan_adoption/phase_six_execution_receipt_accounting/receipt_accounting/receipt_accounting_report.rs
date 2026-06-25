use crate::graph_read_access_plan_adoption::WorthGraphReadAccessSpatialDensePhaseSixSeed;

use super::super::stable_digest;
use super::{WorthGraphReadAccessReceiptAccountingRow, WorthGraphReadAccessReceiptStatus};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessReceiptAccountingReport {
    rows: Vec<WorthGraphReadAccessReceiptAccountingRow>,
    executed_receipt_count: usize,
    required_future_receipt_count: usize,
    no_receipt_posture_count: usize,
    report_digest: String,
}

pub(crate) fn build_receipt_accounting_report(
    seed: &WorthGraphReadAccessSpatialDensePhaseSixSeed,
) -> WorthGraphReadAccessReceiptAccountingReport {
    let mut rows = vec![
        WorthGraphReadAccessReceiptAccountingRow::from_phase_four_receipt(
            seed.phase_four_receipt_projection(),
        ),
    ];
    rows.extend(
        seed.posture_projections()
            .iter()
            .map(WorthGraphReadAccessReceiptAccountingRow::from_spatial_dense_projection),
    );
    WorthGraphReadAccessReceiptAccountingReport::from_rows(rows)
}

impl WorthGraphReadAccessReceiptAccountingReport {
    fn from_rows(rows: Vec<WorthGraphReadAccessReceiptAccountingRow>) -> Self {
        let executed_receipt_count = rows
            .iter()
            .filter(|row| row.status().claims_query_receipt())
            .count();
        let required_future_receipt_count = rows
            .iter()
            .filter(|row| row.status().requires_future_receipt())
            .count();
        let no_receipt_posture_count = rows
            .iter()
            .filter(|row| {
                matches!(
                    row.status(),
                    WorthGraphReadAccessReceiptStatus::RequiredQueryPostureNoReceipt
                        | WorthGraphReadAccessReceiptStatus::DeniedByQueryPostureNoReceipt
                        | WorthGraphReadAccessReceiptStatus::CarriedCapabilityGapNoReceipt
                )
            })
            .count();
        let report_digest = stable_digest(
            &std::iter::once("worth_graph_read_access_receipt_accounting_report_v1".to_string())
                .chain(rows.iter().map(|row| format!("row:{}", row.row_digest())))
                .chain([
                    format!("executed:{executed_receipt_count}"),
                    format!("future_receipt:{required_future_receipt_count}"),
                    format!("no_receipt_posture:{no_receipt_posture_count}"),
                ])
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            executed_receipt_count,
            required_future_receipt_count,
            no_receipt_posture_count,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[WorthGraphReadAccessReceiptAccountingRow] {
        &self.rows
    }

    pub const fn executed_receipt_count(&self) -> usize {
        self.executed_receipt_count
    }

    pub const fn required_future_receipt_count(&self) -> usize {
        self.required_future_receipt_count
    }

    pub const fn no_receipt_posture_count(&self) -> usize {
        self.no_receipt_posture_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[cfg(test)]
mod adversarial_receipt_accounting_report {
    use super::*;

    impl WorthGraphReadAccessReceiptAccountingReport {
        pub(crate) fn from_rows_for_tests(
            rows: Vec<WorthGraphReadAccessReceiptAccountingRow>,
        ) -> Self {
            Self::from_rows(rows)
        }
    }
}
