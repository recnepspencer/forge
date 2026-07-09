use crate::request_context::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerOperationRequestDenialCode {
    MissingOperationFamily,
    MissingOperationName,
    InvalidOperationName,
    InvalidBasisDigest,
    InvalidIdempotencyKey,
    InvalidProductSessionIdentity,
    InvalidPayloadEnvelope,
    InvalidDeclaredSchemaIdentity,
    UnknownOperationName,
    CompatibilityBindingInvalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationRequestDenial {
    code: WorthServerOperationRequestDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    detail: String,
}

impl WorthServerOperationRequestDenial {
    pub(crate) fn new(
        code: WorthServerOperationRequestDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> WorthServerOperationRequestDenialCode {
        self.code.clone()
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
