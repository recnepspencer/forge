use crate::request_context::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationRequestDenialCode {
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
pub struct ForgeServerOperationRequestDenial {
    code: ForgeServerOperationRequestDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    detail: String,
}

impl ForgeServerOperationRequestDenial {
    pub(crate) fn new(
        code: ForgeServerOperationRequestDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> ForgeServerOperationRequestDenialCode {
        self.code.clone()
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
