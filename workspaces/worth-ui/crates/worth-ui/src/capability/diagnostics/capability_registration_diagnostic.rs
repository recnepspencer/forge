use super::{CapabilityDiagnosticCode, CapabilityDiagnosticSeverity, DiagnosticOrderingKey};

/// Structured diagnostic emitted while validating capability registration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityRegistrationDiagnostic {
    code: CapabilityDiagnosticCode,
    severity: CapabilityDiagnosticSeverity,
    family_name: Option<&'static str>,
    identity_text: Option<String>,
    related_family_name: Option<&'static str>,
    related_identity_text: Option<String>,
    detail: Option<String>,
}

impl CapabilityRegistrationDiagnostic {
    pub(crate) fn error(
        code: CapabilityDiagnosticCode,
        family_name: Option<&'static str>,
        identity_text: Option<&str>,
        related_family_name: Option<&'static str>,
        related_identity_text: Option<&str>,
        detail: Option<String>,
    ) -> Self {
        Self {
            code,
            severity: CapabilityDiagnosticSeverity::Error,
            family_name,
            identity_text: identity_text.map(str::to_owned),
            related_family_name,
            related_identity_text: related_identity_text.map(str::to_owned),
            detail,
        }
    }

    pub fn code(&self) -> CapabilityDiagnosticCode {
        self.code
    }

    pub fn severity(&self) -> CapabilityDiagnosticSeverity {
        self.severity
    }

    pub fn family_name(&self) -> Option<&str> {
        self.family_name
    }

    pub fn identity_text(&self) -> Option<&str> {
        self.identity_text.as_deref()
    }

    pub fn related_family_name(&self) -> Option<&str> {
        self.related_family_name
    }

    pub fn related_identity_text(&self) -> Option<&str> {
        self.related_identity_text.as_deref()
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    pub(crate) fn ordering_key(&self) -> DiagnosticOrderingKey {
        DiagnosticOrderingKey::new(
            self.code,
            self.family_name,
            self.identity_text(),
            self.related_family_name,
            self.related_identity_text(),
        )
    }
}
