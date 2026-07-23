use crate::identity::{FailureDigest, ResultDigest};

use super::super::{
    families::IdentityEvolutionAmbiguityReason, metadata::IdentityEvolutionMetadata,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryIdentityCandidateSet {
    metadata: IdentityEvolutionMetadata,
    advisory_candidate_identities: Vec<String>,
    advisory_digest: ResultDigest,
}

impl AdvisoryIdentityCandidateSet {
    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        &self.metadata
    }

    pub fn advisory_candidate_identities(&self) -> &[String] {
        &self.advisory_candidate_identities
    }

    pub fn advisory_digest(&self) -> &ResultDigest {
        &self.advisory_digest
    }

    pub(crate) fn new(
        metadata: IdentityEvolutionMetadata,
        advisory_candidate_identities: Vec<String>,
    ) -> Self {
        let mut parts = vec![format!(
            "metadata_digest:{}",
            metadata.metadata_digest().as_str()
        )];
        parts.extend(
            advisory_candidate_identities
                .iter()
                .map(|candidate| format!("advisory_candidate_identity:{candidate}")),
        );
        let advisory_digest = ResultDigest::from_parts(&parts);
        Self {
            metadata,
            advisory_candidate_identities,
            advisory_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionAmbiguityBundle {
    metadata: IdentityEvolutionMetadata,
    ambiguity_reason: IdentityEvolutionAmbiguityReason,
    ambiguity_digest: FailureDigest,
}

impl IdentityEvolutionAmbiguityBundle {
    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        &self.metadata
    }

    pub fn ambiguity_reason(&self) -> IdentityEvolutionAmbiguityReason {
        self.ambiguity_reason
    }

    pub fn ambiguity_digest(&self) -> &FailureDigest {
        &self.ambiguity_digest
    }

    pub(crate) fn new(
        metadata: IdentityEvolutionMetadata,
        ambiguity_reason: IdentityEvolutionAmbiguityReason,
    ) -> Self {
        let ambiguity_digest = FailureDigest::from_parts(&[
            format!("metadata_digest:{}", metadata.metadata_digest().as_str()),
            format!("ambiguity_reason:{}", ambiguity_reason.as_str()),
        ]);
        Self {
            metadata,
            ambiguity_reason,
            ambiguity_digest,
        }
    }
}
