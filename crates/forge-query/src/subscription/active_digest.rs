use crate::evidence_identity::ForgeQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveSubscriptionLaneDigest(ForgeQueryEvidenceIdentity);

impl ActiveSubscriptionLaneDigest {
    pub(super) fn new(value: ForgeQueryEvidenceIdentity) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.0
    }
}
