use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessReceiptAccountingReport, WorthGraphReadAccessReceiptStatus,
};

use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionReceiptExport {
    report: WorthGraphReadAccessReceiptAccountingReport,
    executed_receipt_count: usize,
    admitted_plan_requires_receipt_count: usize,
    required_posture_count: usize,
    denied_posture_count: usize,
    carried_gap_count: usize,
    export_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionReceiptExport {
    pub(in crate::graph_read_access_plan_adoption::phase_eight_public_closeout) fn from_report(
        report: &WorthGraphReadAccessReceiptAccountingReport,
    ) -> Self {
        let status_counts =
            WorthGraphReadAccessPlanAdoptionReceiptStatusCounts::from_report(report);
        let executed_receipt_count = status_counts.executed_receipt_count;
        let admitted_plan_requires_receipt_count =
            status_counts.admitted_plan_requires_receipt_count;
        let required_posture_count = status_counts.required_posture_count;
        let denied_posture_count = status_counts.denied_posture_count;
        let carried_gap_count = status_counts.carried_gap_count;
        let export_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_receipt_export_v1".to_string(),
            format!("report:{}", report.report_digest()),
            format!("executed:{executed_receipt_count}"),
            format!("pending_plan:{admitted_plan_requires_receipt_count}"),
            format!("required_posture:{required_posture_count}"),
            format!("denied_posture:{denied_posture_count}"),
            format!("carried_gap:{carried_gap_count}"),
        ]);
        Self {
            report: report.clone(),
            executed_receipt_count,
            admitted_plan_requires_receipt_count,
            required_posture_count,
            denied_posture_count,
            carried_gap_count,
            export_digest,
        }
    }

    pub const fn report(&self) -> &WorthGraphReadAccessReceiptAccountingReport {
        &self.report
    }

    pub const fn executed_receipt_count(&self) -> usize {
        self.executed_receipt_count
    }

    pub const fn admitted_plan_requires_receipt_count(&self) -> usize {
        self.admitted_plan_requires_receipt_count
    }

    pub const fn required_posture_count(&self) -> usize {
        self.required_posture_count
    }

    pub const fn denied_posture_count(&self) -> usize {
        self.denied_posture_count
    }

    pub const fn carried_gap_count(&self) -> usize {
        self.carried_gap_count
    }

    pub const fn visible_non_executed_posture_count(&self) -> usize {
        self.required_posture_count + self.denied_posture_count + self.carried_gap_count
    }

    pub const fn has_executed_receipts_or_visible_postures(&self) -> bool {
        self.executed_receipt_count > 0 || self.visible_non_executed_posture_count() > 0
    }

    pub fn export_digest(&self) -> &str {
        &self.export_digest
    }
}

struct WorthGraphReadAccessPlanAdoptionReceiptStatusCounts {
    executed_receipt_count: usize,
    admitted_plan_requires_receipt_count: usize,
    required_posture_count: usize,
    denied_posture_count: usize,
    carried_gap_count: usize,
}

impl WorthGraphReadAccessPlanAdoptionReceiptStatusCounts {
    fn from_report(report: &WorthGraphReadAccessReceiptAccountingReport) -> Self {
        let mut executed_receipt_count = 0;
        let mut admitted_plan_requires_receipt_count = 0;
        let mut required_posture_count = 0;
        let mut denied_posture_count = 0;
        let mut carried_gap_count = 0;
        for row in report.rows() {
            match row.status() {
                WorthGraphReadAccessReceiptStatus::ExecutedThroughQueryReceipt => {
                    executed_receipt_count += 1;
                }
                WorthGraphReadAccessReceiptStatus::AdmittedPlanRequiresExecutionReceipt => {
                    admitted_plan_requires_receipt_count += 1;
                }
                WorthGraphReadAccessReceiptStatus::RequiredQueryPostureNoReceipt => {
                    required_posture_count += 1;
                }
                WorthGraphReadAccessReceiptStatus::DeniedByQueryPostureNoReceipt => {
                    denied_posture_count += 1;
                }
                WorthGraphReadAccessReceiptStatus::CarriedCapabilityGapNoReceipt => {
                    carried_gap_count += 1;
                }
            }
        }
        Self {
            executed_receipt_count,
            admitted_plan_requires_receipt_count,
            required_posture_count,
            denied_posture_count,
            carried_gap_count,
        }
    }
}
