use std::collections::BTreeSet;

use crate::capability::{CapabilityRegistrationDiagnostic, RegisteredCapabilitySet};

pub(crate) type AcceptedRegistrationKey = (&'static str, String);

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct RegistrationValidationReport {
    accepted_capabilities: RegisteredCapabilitySet,
    accepted_registration_keys: BTreeSet<AcceptedRegistrationKey>,
    diagnostics: Vec<CapabilityRegistrationDiagnostic>,
}

impl RegistrationValidationReport {
    pub(crate) fn new(
        accepted_capabilities: RegisteredCapabilitySet,
        accepted_registration_keys: BTreeSet<AcceptedRegistrationKey>,
        diagnostics: Vec<CapabilityRegistrationDiagnostic>,
    ) -> Self {
        Self {
            accepted_capabilities,
            accepted_registration_keys,
            diagnostics,
        }
    }

    #[cfg(test)]
    pub(crate) fn accepted_capabilities(&self) -> &RegisteredCapabilitySet {
        &self.accepted_capabilities
    }

    #[cfg(test)]
    pub(crate) fn diagnostics(&self) -> &[CapabilityRegistrationDiagnostic] {
        &self.diagnostics
    }

    pub(crate) fn accepted_identity_texts_for_family(
        &self,
        family_name: &'static str,
    ) -> BTreeSet<String> {
        self.accepted_registration_keys
            .iter()
            .filter(|(accepted_family_name, _)| *accepted_family_name == family_name)
            .map(|(_, identity_text)| identity_text.clone())
            .collect()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        RegisteredCapabilitySet,
        Vec<CapabilityRegistrationDiagnostic>,
    ) {
        (self.accepted_capabilities, self.diagnostics)
    }
}
