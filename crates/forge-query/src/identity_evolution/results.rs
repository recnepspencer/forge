use crate::identity::{FailureDigest, ResultDigest};

use super::{
    families::IdentityEvolutionOutcomeFamily,
    metadata::IdentityEvolutionMetadata,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SingularIdentityContinuityResult {
    metadata: IdentityEvolutionMetadata,
    authoritative_identity: String,
    continuity_digest: ResultDigest,
}

impl SingularIdentityContinuityResult {
    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        &self.metadata
    }

    pub fn authoritative_identity(&self) -> &str {
        &self.authoritative_identity
    }

    pub fn continuity_digest(&self) -> &ResultDigest {
        &self.continuity_digest
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new(
        metadata: IdentityEvolutionMetadata,
        authoritative_identity: impl Into<String>,
    ) -> Self {
        let authoritative_identity = authoritative_identity.into();
        let continuity_digest = ResultDigest::from_parts(&[
            format!("metadata_digest:{}", metadata.metadata_digest().as_str()),
            format!("authoritative_identity:{}", authoritative_identity),
        ]);
        Self {
            metadata,
            authoritative_identity,
            continuity_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluralIdentitySuccessorSet {
    metadata: IdentityEvolutionMetadata,
    successor_identities: Vec<String>,
    successor_set_digest: ResultDigest,
}

impl PluralIdentitySuccessorSet {
    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        &self.metadata
    }

    pub fn successor_identities(&self) -> &[String] {
        &self.successor_identities
    }

    pub fn successor_set_digest(&self) -> &ResultDigest {
        &self.successor_set_digest
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new(
        metadata: IdentityEvolutionMetadata,
        successor_identities: Vec<String>,
    ) -> Self {
        let mut parts = vec![format!("metadata_digest:{}", metadata.metadata_digest().as_str())];
        parts.extend(
            successor_identities
                .iter()
                .map(|successor| format!("successor_identity:{successor}")),
        );
        let successor_set_digest = ResultDigest::from_parts(&parts);
        Self {
            metadata,
            successor_identities,
            successor_set_digest,
        }
    }
}

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

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new(
        metadata: IdentityEvolutionMetadata,
        advisory_candidate_identities: Vec<String>,
    ) -> Self {
        let mut parts = vec![format!("metadata_digest:{}", metadata.metadata_digest().as_str())];
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
    ambiguity_reason: &'static str,
    ambiguity_digest: FailureDigest,
}

impl IdentityEvolutionAmbiguityBundle {
    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        &self.metadata
    }

    pub fn ambiguity_reason(&self) -> &'static str {
        self.ambiguity_reason
    }

    pub fn ambiguity_digest(&self) -> &FailureDigest {
        &self.ambiguity_digest
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new(
        metadata: IdentityEvolutionMetadata,
        ambiguity_reason: &'static str,
    ) -> Self {
        let ambiguity_digest = FailureDigest::from_parts(&[
            format!("metadata_digest:{}", metadata.metadata_digest().as_str()),
            format!("ambiguity_reason:{}", ambiguity_reason),
        ]);
        Self {
            metadata,
            ambiguity_reason,
            ambiguity_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionDeniedBundle {
    metadata: IdentityEvolutionMetadata,
    denial_reason: &'static str,
    denial_digest: FailureDigest,
}

impl IdentityEvolutionDeniedBundle {
    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        &self.metadata
    }

    pub fn denial_reason(&self) -> &'static str {
        self.denial_reason
    }

    pub fn denial_digest(&self) -> &FailureDigest {
        &self.denial_digest
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new(metadata: IdentityEvolutionMetadata, denial_reason: &'static str) -> Self {
        let denial_digest = FailureDigest::from_parts(&[
            format!("metadata_digest:{}", metadata.metadata_digest().as_str()),
            format!("denial_reason:{}", denial_reason),
        ]);
        Self {
            metadata,
            denial_reason,
            denial_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(not(test), allow(dead_code))]
enum IdentityEvolutionResultEnvelope {
    SingularIdentityContinuity(SingularIdentityContinuityResult),
    PluralIdentitySuccessorSet(PluralIdentitySuccessorSet),
    AdvisoryIdentityCandidateSet(AdvisoryIdentityCandidateSet),
    Ambiguity(IdentityEvolutionAmbiguityBundle),
    Denied(IdentityEvolutionDeniedBundle),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionResultBundle {
    envelope: IdentityEvolutionResultEnvelope,
}

impl IdentityEvolutionResultBundle {
    pub fn outcome_family(&self) -> IdentityEvolutionOutcomeFamily {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(_) => {
                IdentityEvolutionOutcomeFamily::SingularIdentityContinuity
            }
            IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(_) => {
                IdentityEvolutionOutcomeFamily::PluralIdentitySuccessorSet
            }
            IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(_) => {
                IdentityEvolutionOutcomeFamily::AdvisoryIdentityCandidateSet
            }
            IdentityEvolutionResultEnvelope::Ambiguity(_) => IdentityEvolutionOutcomeFamily::Ambiguity,
            IdentityEvolutionResultEnvelope::Denied(_) => IdentityEvolutionOutcomeFamily::Denied,
        }
    }

    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(result) => result.metadata(),
            IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(result) => result.metadata(),
            IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(result) => result.metadata(),
            IdentityEvolutionResultEnvelope::Ambiguity(result) => result.metadata(),
            IdentityEvolutionResultEnvelope::Denied(result) => result.metadata(),
        }
    }

    pub fn as_singular_identity_continuity(&self) -> Option<&SingularIdentityContinuityResult> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(result) => Some(result),
            IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(_)
            | IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(_)
            | IdentityEvolutionResultEnvelope::Ambiguity(_)
            | IdentityEvolutionResultEnvelope::Denied(_) => None,
        }
    }

    pub fn as_plural_identity_successor_set(&self) -> Option<&PluralIdentitySuccessorSet> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(result) => Some(result),
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(_)
            | IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(_)
            | IdentityEvolutionResultEnvelope::Ambiguity(_)
            | IdentityEvolutionResultEnvelope::Denied(_) => None,
        }
    }

    pub fn as_advisory_identity_candidate_set(&self) -> Option<&AdvisoryIdentityCandidateSet> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(result) => Some(result),
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(_)
            | IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(_)
            | IdentityEvolutionResultEnvelope::Ambiguity(_)
            | IdentityEvolutionResultEnvelope::Denied(_) => None,
        }
    }

    pub fn as_ambiguity(&self) -> Option<&IdentityEvolutionAmbiguityBundle> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::Ambiguity(result) => Some(result),
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(_)
            | IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(_)
            | IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(_)
            | IdentityEvolutionResultEnvelope::Denied(_) => None,
        }
    }

    pub fn as_denied(&self) -> Option<&IdentityEvolutionDeniedBundle> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::Denied(result) => Some(result),
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(_)
            | IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(_)
            | IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(_)
            | IdentityEvolutionResultEnvelope::Ambiguity(_) => None,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn singular_identity_continuity(result: SingularIdentityContinuityResult) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::SingularIdentityContinuity(result),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn plural_identity_successor_set(result: PluralIdentitySuccessorSet) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(result),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn advisory_identity_candidate_set(result: AdvisoryIdentityCandidateSet) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(result),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn ambiguity(result: IdentityEvolutionAmbiguityBundle) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::Ambiguity(result),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn denied(result: IdentityEvolutionDeniedBundle) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::Denied(result),
        }
    }
}
