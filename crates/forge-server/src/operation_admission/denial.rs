use forge_foundational::DiagnosticRichnessProfile;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationAdmissionDenialCode {
    AuthorityDenied,
    AuthorizationDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationAdmissionDenial {
    code: ForgeServerOperationAdmissionDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    detail: String,
}

impl ForgeServerOperationAdmissionDenial {
    pub(crate) fn new(
        code: ForgeServerOperationAdmissionDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> ForgeServerOperationAdmissionDenialCode {
        self.code
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
