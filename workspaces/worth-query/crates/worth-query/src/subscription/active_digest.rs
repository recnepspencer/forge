use crate::evidence_identity::WorthQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveSubscriptionLaneDigest(WorthQueryEvidenceIdentity);

impl ActiveSubscriptionLaneDigest {
    pub(super) fn new(value: WorthQueryEvidenceIdentity) -> Self {
        Self(value)
    }

    pub fn evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.0
    }
}

impl Ord for ActiveSubscriptionLaneDigest {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .scope()
            .cmp(&other.0.scope())
            .then_with(|| self.0.scheme().cmp(&other.0.scheme()))
            .then_with(|| self.0.as_str().cmp(other.0.as_str()))
    }
}

impl PartialOrd for ActiveSubscriptionLaneDigest {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
