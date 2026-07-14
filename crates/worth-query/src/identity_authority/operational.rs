use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::identity::CanonicalQueryDigest;

/// Sealed Query-owned canonical identity authority.
///
/// Consumers may carry and inspect this handle, but only Query artifacts can
/// mint it. Its digest is a reporting projection, not a constructor input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryCanonicalAuthority {
    identity: WorthQueryEvidenceIdentity,
    digest: CanonicalQueryDigest,
}

impl QueryCanonicalAuthority {
    pub fn identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity
    }

    pub fn digest(&self) -> &CanonicalQueryDigest {
        &self.digest
    }

    pub(crate) fn from_query_artifact(digest: &CanonicalQueryDigest) -> Self {
        Self {
            identity: digest.evidence_identity(),
            digest: digest.clone(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_digest_for_test(digest: CanonicalQueryDigest) -> Self {
        Self {
            identity: digest.evidence_identity(),
            digest,
        }
    }
}
