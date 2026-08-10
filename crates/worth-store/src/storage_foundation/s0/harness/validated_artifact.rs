use super::super::artifacts::S0ArtifactValidationCostSurface;
use super::report::HarnessMaturityReport;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ValidatedHarnessMaturityReportArtifact {
    pub(super) report: HarnessMaturityReport,
    pub(super) validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedHarnessMaturityReportArtifact {
    pub fn report(&self) -> &HarnessMaturityReport {
        &self.report
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}
