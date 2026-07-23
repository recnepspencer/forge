use crate::identity::ResultDigest;

use super::super::IdentityEvolutionMetadata;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityLifecycleResult {
    metadata: IdentityEvolutionMetadata,
    authoritative_identity: String,
    lifecycle_digest: ResultDigest,
}

impl IdentityLifecycleResult {
    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        &self.metadata
    }

    pub fn authoritative_identity(&self) -> &str {
        &self.authoritative_identity
    }

    pub fn lifecycle_digest(&self) -> &ResultDigest {
        &self.lifecycle_digest
    }

    pub(crate) fn new(
        metadata: IdentityEvolutionMetadata,
        authoritative_identity: impl Into<String>,
    ) -> Self {
        let authoritative_identity = authoritative_identity.into();
        let lifecycle_digest = ResultDigest::from_parts(&[
            format!("metadata_digest:{}", metadata.metadata_digest().as_str()),
            format!("authoritative_identity:{authoritative_identity}"),
        ]);
        Self {
            metadata,
            authoritative_identity,
            lifecycle_digest,
        }
    }
}
