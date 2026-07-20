use crate::evidence_identity::WorthQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct QuerySubscriptionDeclarationDigest(String);

impl QuerySubscriptionDeclarationDigest {
    pub(super) fn from_evidence_identity(identity: &WorthQueryEvidenceIdentity) -> Self {
        Self(identity.as_str().to_string())
    }
}
