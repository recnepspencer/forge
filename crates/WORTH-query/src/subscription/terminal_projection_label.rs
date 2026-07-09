use std::fmt;

use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::evidence_projection::subscription_evidence_projection;

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

    #[allow(dead_code)]
    pub(crate) fn from_projection(
        projection: QueryProjectionIdentity<String, QuerySubscriptionIdentityKind>,
    ) -> Self {
        Self(projection.label().to_string())
    }

    #[allow(dead_code)]
    pub(crate) fn from_terminal_parts(parts: impl Into<String>) -> Self {
        Self(parts.into())
    }

    #[allow(dead_code)]
    pub(crate) fn as_terminal_label(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TerminalProjectionLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
