use super::decision::SubscriptionSupportPortabilityDecisionKind;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityRejection {
    rejection_kind: SubscriptionSupportPortabilityDecisionKind,
    manifest_digest: String,
    rejection_reason: String,
}

impl SupportPortabilityRejection {
    pub(super) fn new(
        rejection_kind: SubscriptionSupportPortabilityDecisionKind,
        manifest_digest: String,
        rejection_reason: String,
    ) -> Self {
        Self {
            rejection_kind,
            manifest_digest,
            rejection_reason,
        }
    }

    pub fn rejection_kind(&self) -> SubscriptionSupportPortabilityDecisionKind {
        self.rejection_kind
    }

    pub fn rejection_reason(&self) -> &str {
        &self.rejection_reason
    }
}
