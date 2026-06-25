use crate::graph_read_access_plan_adoption::WorthGraphReadAccessHardDeletionProofReport;

use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionDeletionExport {
    report: WorthGraphReadAccessHardDeletionProofReport,
    export_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionDeletionExport {
    pub(in crate::graph_read_access_plan_adoption::phase_eight_public_closeout) fn from_report(
        report: &WorthGraphReadAccessHardDeletionProofReport,
    ) -> Self {
        let export_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_deletion_export_v1".to_string(),
            format!("report:{}", report.report_digest()),
            format!("deleted:{}", report.deleted_count()),
            format!("unresolved:{}", report.unresolved_count()),
        ]);
        Self {
            report: report.clone(),
            export_digest,
        }
    }

    pub const fn report(&self) -> &WorthGraphReadAccessHardDeletionProofReport {
        &self.report
    }

    pub const fn deleted_count(&self) -> usize {
        self.report.deleted_count()
    }

    pub const fn unresolved_count(&self) -> usize {
        self.report.unresolved_count()
    }

    pub fn export_digest(&self) -> &str {
        &self.export_digest
    }
}
