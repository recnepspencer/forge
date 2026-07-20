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

    pub fn is_launch(&self) -> bool {
        matches!(self, Self::Launch)
    }

    pub fn replacement_readiness(&self) -> Option<crate::runtime::WorthUiActivationReadiness> {
        match self {
            Self::Launch => None,
            Self::Replacement { readiness, .. } => Some(*readiness),
        }
    }

    pub fn replacement_staging_report(
        &self,
    ) -> Option<&crate::runtime::WorthUiActivationStagingReport> {
        match self {
            Self::Launch => None,
            Self::Replacement { staging_report, .. } => Some(staging_report),
        }
    }
}
