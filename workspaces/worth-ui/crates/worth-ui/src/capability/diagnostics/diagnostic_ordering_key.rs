use super::CapabilityDiagnosticCode;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct DiagnosticOrderingKey {
    code: CapabilityDiagnosticCode,
    family_name: Option<&'static str>,
    identity_text: Option<String>,
    related_family_name: Option<&'static str>,
    related_identity_text: Option<String>,
}

impl DiagnosticOrderingKey {
    pub(crate) fn new(
        code: CapabilityDiagnosticCode,
        family_name: Option<&'static str>,
        identity_text: Option<&str>,
        related_family_name: Option<&'static str>,
        related_identity_text: Option<&str>,
    ) -> Self {
        Self {
            code,
            family_name,
            identity_text: identity_text.map(str::to_owned),
            related_family_name,
            related_identity_text: related_identity_text.map(str::to_owned),
        }
    }
}
