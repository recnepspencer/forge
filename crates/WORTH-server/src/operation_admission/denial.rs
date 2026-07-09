use worth_foundational::DiagnosticRichnessProfile;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerOperationAdmissionDenialCode {
    AuthorityDenied,
    AuthorizationDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationAdmissionDenial {
    code: WorthServerOperationAdmissionDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    detail: String,
}

impl WorthServerOperationAdmissionDenial {
    pub(crate) fn new(
        code: WorthServerOperationAdmissionDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> WorthServerOperationAdmissionDenialCode {
        self.code
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
