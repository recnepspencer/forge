use forge_foundational::DiagnosticRichnessProfile;

use super::ForgeServerPipelineStep;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDenial {
    code: ForgeServerDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    priority: ForgeServerDenialPriority,
    step: ForgeServerPipelineStep,
    detail: String,
}

impl ForgeServerDenial {
    pub(crate) fn new(
        code: ForgeServerDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        priority: ForgeServerDenialPriority,
        step: ForgeServerPipelineStep,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            priority,
            step,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> ForgeServerDenialCode {
        self.code
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn priority(&self) -> ForgeServerDenialPriority {
        self.priority
    }

    pub fn step(&self) -> ForgeServerPipelineStep {
        self.step
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerDenialCode {
    CompatHttpDiagnosticsBudgetExceeded,
    PreviewBranchAccessDenied,
    QueryMutationDisabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ForgeServerDenialPriority {
    Authorization,
    Budget,
    Validation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerMiddlewareDeferred {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerMiddlewareStale {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerMiddlewareRebindRequired {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerMiddlewareFailure {
    reason: &'static str,
}
