use crate::runtime::{WorthUiActivationReadiness, WorthUiActivationStagingReport};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanLoweringContext {
    readiness: WorthUiActivationReadiness,
    staging_report: WorthUiActivationStagingReport,
}

impl WorthUiPlanLoweringContext {
    pub(crate) fn new(
        readiness: WorthUiActivationReadiness,
        staging_report: WorthUiActivationStagingReport,
    ) -> Self {
        Self {
            readiness,
            staging_report,
        }
    }

    pub fn readiness(&self) -> WorthUiActivationReadiness {
        self.readiness
    }

    pub fn staging_report(&self) -> &WorthUiActivationStagingReport {
        &self.staging_report
    }
}
