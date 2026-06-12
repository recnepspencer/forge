use crate::capability::CapabilitySnapshot;

use super::CapabilityRegistrationDiagnostic;

/// Registration validation report paired with accepted snapshot meaning.
#[derive(Debug, Eq, PartialEq)]
pub struct CapabilityRegistrationReport {
    accepted_snapshot: CapabilitySnapshot,
    registration_diagnostics: Vec<CapabilityRegistrationDiagnostic>,
}

impl CapabilityRegistrationReport {
    pub(crate) fn new(
        accepted_snapshot: CapabilitySnapshot,
        registration_diagnostics: Vec<CapabilityRegistrationDiagnostic>,
    ) -> Self {
        Self {
            accepted_snapshot,
            registration_diagnostics,
        }
    }

    pub fn accepted_snapshot(&self) -> &CapabilitySnapshot {
        &self.accepted_snapshot
    }

    pub(crate) fn into_accepted_snapshot(self) -> CapabilitySnapshot {
        self.accepted_snapshot
    }

    pub fn registration_diagnostics(&self) -> &[CapabilityRegistrationDiagnostic] {
        &self.registration_diagnostics
    }

    pub fn has_errors(&self) -> bool {
        self.registration_diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity().is_error())
    }
}
