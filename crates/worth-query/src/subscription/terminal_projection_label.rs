use std::fmt;

use super::evidence_projection::subscription_evidence_projection;
use crate::evidence_identity::WorthQueryEvidenceIdentity;

/// Terminal-only label quarantine. Does not implement `AsRef<str>` so it cannot
/// satisfy authority APIs or be composed back into evidence without admission.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TerminalProjectionLabel(String);

impl TerminalProjectionLabel {
    pub fn from_identity(identity: &WorthQueryEvidenceIdentity) -> Self {
        Self(
            subscription_evidence_projection(identity)
                .label()
                .to_string(),
        )
    }
}

impl fmt::Display for TerminalProjectionLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
