use crate::graph_read_access_plan_adoption::WorthGraphReadAccessHardDeletionCappedResidueReport;

use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionResidueExport {
    report: WorthGraphReadAccessHardDeletionCappedResidueReport,
    export_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionResidueExport {
    pub(in crate::graph_read_access_plan_adoption::phase_eight_public_closeout) fn from_report(
        report: &WorthGraphReadAccessHardDeletionCappedResidueReport,
    ) -> Self {
        let export_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_residue_export_v1".to_string(),
            format!("report:{}", report.report_digest()),
            format!("residue:{}", report.residue_count()),
            format!("uncapped:{}", report.uncapped_residue_count()),
        ]);
        Self {
            report: report.clone(),
            export_digest,
        }
    }

    pub const fn report(&self) -> &WorthGraphReadAccessHardDeletionCappedResidueReport {
        &self.report
    }

    pub const fn residue_count(&self) -> usize {
        self.report.residue_count()
    }

    pub const fn uncapped_residue_count(&self) -> usize {
        self.report.uncapped_residue_count()
    }

    pub fn export_digest(&self) -> &str {
        &self.export_digest
    }
}
