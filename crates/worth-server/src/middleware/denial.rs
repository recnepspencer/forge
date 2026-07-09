use worth_foundational::DiagnosticRichnessProfile;

use super::WorthServerPipelineStep;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDenial {
    code: WorthServerDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    priority: WorthServerDenialPriority,
    step: WorthServerPipelineStep,
    detail: String,
}

impl WorthServerDenial {
    pub(crate) fn new(
        code: WorthServerDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        priority: WorthServerDenialPriority,
        step: WorthServerPipelineStep,
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

    pub fn code(&self) -> WorthServerDenialCode {
        self.code
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn priority(&self) -> WorthServerDenialPriority {
        self.priority
    }

    pub fn step(&self) -> WorthServerPipelineStep {
        self.step
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerDenialCode {
    CompatHttpDiagnosticsBudgetExceeded,
    PreviewBranchAccessDenied,
    QueryMutationDisabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthServerDenialPriority {
    Authorization,
    Budget,
    Validation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerMiddlewareDeferred {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerMiddlewareStale {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerMiddlewareRebindRequired {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerMiddlewareFailure {
    reason: &'static str,
}
