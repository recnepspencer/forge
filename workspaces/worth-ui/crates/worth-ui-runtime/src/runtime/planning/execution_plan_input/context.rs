#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiPlanLoweringContext {
    Launch,
    Replacement {
        readiness: crate::runtime::WorthUiActivationReadiness,
        staging_report: crate::runtime::WorthUiActivationStagingReport,
    },
}

impl WorthUiPlanLoweringContext {
    pub(crate) fn launch() -> Self {
        Self::Launch
    }

    pub(crate) fn replacement(
        readiness: crate::runtime::WorthUiActivationReadiness,
        staging_report: crate::runtime::WorthUiActivationStagingReport,
    ) -> Self {
        Self::Replacement {
            readiness,
            staging_report,
        }
    }
}
