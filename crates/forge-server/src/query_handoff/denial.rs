use forge_foundational::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffDenial {
    code: ForgeServerQueryHandoffDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    detail: String,
}

impl ForgeServerQueryHandoffDenial {
    pub(crate) fn new(
        code: ForgeServerQueryHandoffDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> ForgeServerQueryHandoffDenialCode {
        self.code
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryHandoffDenialCode {
    PreparedIntentMismatch,
    UnsupportedQueryFacadeFamily,
    DownstreamDeliveryRequiresReadIntent,
    RuntimeBackedResumeUnsupported,
    DurableResumeDeferred,
    WorkspaceBindingFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffDeferred {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffStale {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffRebindRequired {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffFailure {
    reason: &'static str,
}
