use crate::graph_read_access_plan_adoption::WorthGraphReadAccessHardDeletionSourceFirewallReport;

use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionSourceFirewallExport {
    report: WorthGraphReadAccessHardDeletionSourceFirewallReport,
    export_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionSourceFirewallExport {
    pub(in crate::graph_read_access_plan_adoption::phase_eight_public_closeout) fn from_report(
        report: &WorthGraphReadAccessHardDeletionSourceFirewallReport,
    ) -> Self {
        let export_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_source_firewall_export_v1".to_string(),
            format!("report:{}", report.report_digest()),
            format!("regions:{}", report.scanned_region_count()),
            format!("sources:{}", report.scanned_source_count()),
            format!("violations:{}", report.violation_count()),
        ]);
        Self {
            report: report.clone(),
            export_digest,
        }
    }

    pub const fn report(&self) -> &WorthGraphReadAccessHardDeletionSourceFirewallReport {
        &self.report
    }

    pub const fn scanned_region_count(&self) -> usize {
        self.report.scanned_region_count()
    }

    pub const fn scanned_source_count(&self) -> usize {
        self.report.scanned_source_count()
    }

    pub const fn violation_count(&self) -> usize {
        self.report.violation_count()
    }

    pub fn export_digest(&self) -> &str {
        &self.export_digest
    }
}
