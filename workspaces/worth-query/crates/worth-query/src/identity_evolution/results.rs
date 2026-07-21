use crate::identity::{FailureDigest, ResultDigest};

use super::{
    families::{
        IdentityEvolutionDenialReason, IdentityEvolutionIdentityBreakReason,
        IdentityEvolutionOutcomeFamily,
    },
    metadata::IdentityEvolutionMetadata,
};

#[path = "results/correspondence.rs"]
mod correspondence;
pub use correspondence::{AdvisoryIdentityCandidateSet, IdentityEvolutionAmbiguityBundle};
#[path = "results/lifecycle.rs"]
mod lifecycle;
pub use lifecycle::IdentityLifecycleResult;

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

    pub(crate) fn new(
        metadata: IdentityEvolutionMetadata,
        successor_identities: Vec<String>,
    ) -> Self {
        let mut parts = vec![format!(
            "metadata_digest:{}",
            metadata.metadata_digest().as_str()
        )];
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
pub struct IdentityEvolutionIdentityBreakBundle {
    metadata: IdentityEvolutionMetadata,
    identity_break_reason: IdentityEvolutionIdentityBreakReason,
    identity_break_digest: ResultDigest,
}

impl IdentityEvolutionIdentityBreakBundle {
    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        &self.metadata
    }

    pub fn identity_break_reason(&self) -> IdentityEvolutionIdentityBreakReason {
        self.identity_break_reason
    }

    pub fn identity_break_digest(&self) -> &ResultDigest {
        &self.identity_break_digest
    }

    pub(crate) fn new(
        metadata: IdentityEvolutionMetadata,
        identity_break_reason: IdentityEvolutionIdentityBreakReason,
    ) -> Self {
        let identity_break_digest = ResultDigest::from_parts(&[
            format!("metadata_digest:{}", metadata.metadata_digest().as_str()),
            format!("identity_break_reason:{}", identity_break_reason.as_str()),
        ]);
        Self {
            metadata,
            identity_break_reason,
            identity_break_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionDeniedBundle {
    metadata: IdentityEvolutionMetadata,
    denial_reason: IdentityEvolutionDenialReason,
    denial_digest: FailureDigest,
}

impl IdentityEvolutionDeniedBundle {
    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        &self.metadata
    }

    pub fn denial_reason(&self) -> IdentityEvolutionDenialReason {
        self.denial_reason
    }

    pub fn denial_digest(&self) -> &FailureDigest {
        &self.denial_digest
    }

    pub(crate) fn new(
        metadata: IdentityEvolutionMetadata,
        denial_reason: IdentityEvolutionDenialReason,
    ) -> Self {
        let denial_digest = FailureDigest::from_parts(&[
            format!("metadata_digest:{}", metadata.metadata_digest().as_str()),
            format!("denial_reason:{}", denial_reason.as_str()),
        ]);
        Self {
            metadata,
            denial_reason,
            denial_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum IdentityEvolutionResultEnvelope {
    SingularIdentityContinuity(SingularIdentityContinuityResult),
    PluralIdentitySuccessorSet(PluralIdentitySuccessorSet),
    AdvisoryIdentityCandidateSet(AdvisoryIdentityCandidateSet),
    Ambiguity(IdentityEvolutionAmbiguityBundle),
    IdentityBreak(IdentityEvolutionIdentityBreakBundle),
    GeneratedIdentity(IdentityLifecycleResult),
    RetiredIdentity(IdentityLifecycleResult),
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
            IdentityEvolutionResultEnvelope::Ambiguity(_) => {
                IdentityEvolutionOutcomeFamily::Ambiguity
            }
            IdentityEvolutionResultEnvelope::IdentityBreak(_) => {
                IdentityEvolutionOutcomeFamily::IdentityBreak
            }
            IdentityEvolutionResultEnvelope::GeneratedIdentity(_) => {
                IdentityEvolutionOutcomeFamily::GeneratedIdentity
            }
            IdentityEvolutionResultEnvelope::RetiredIdentity(_) => {
                IdentityEvolutionOutcomeFamily::RetiredIdentity
            }
            IdentityEvolutionResultEnvelope::Denied(_) => IdentityEvolutionOutcomeFamily::Denied,
        }
    }

    pub fn metadata(&self) -> &IdentityEvolutionMetadata {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(result) => {
                result.metadata()
            }
            IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(result) => {
                result.metadata()
            }
            IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(result) => {
                result.metadata()
            }
            IdentityEvolutionResultEnvelope::Ambiguity(result) => result.metadata(),
            IdentityEvolutionResultEnvelope::IdentityBreak(result) => result.metadata(),
            IdentityEvolutionResultEnvelope::GeneratedIdentity(result)
            | IdentityEvolutionResultEnvelope::RetiredIdentity(result) => result.metadata(),
            IdentityEvolutionResultEnvelope::Denied(result) => result.metadata(),
        }
    }

    pub fn as_singular_identity_continuity(&self) -> Option<&SingularIdentityContinuityResult> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(result) => Some(result),
            IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(_)
            | IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(_)
            | IdentityEvolutionResultEnvelope::Ambiguity(_)
            | IdentityEvolutionResultEnvelope::IdentityBreak(_)
            | IdentityEvolutionResultEnvelope::GeneratedIdentity(_)
            | IdentityEvolutionResultEnvelope::RetiredIdentity(_)
            | IdentityEvolutionResultEnvelope::Denied(_) => None,
        }
    }

    pub fn as_plural_identity_successor_set(&self) -> Option<&PluralIdentitySuccessorSet> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(result) => Some(result),
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(_)
            | IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(_)
            | IdentityEvolutionResultEnvelope::Ambiguity(_)
            | IdentityEvolutionResultEnvelope::IdentityBreak(_)
            | IdentityEvolutionResultEnvelope::GeneratedIdentity(_)
            | IdentityEvolutionResultEnvelope::RetiredIdentity(_)
            | IdentityEvolutionResultEnvelope::Denied(_) => None,
        }
    }

    pub fn as_advisory_identity_candidate_set(&self) -> Option<&AdvisoryIdentityCandidateSet> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(result) => Some(result),
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(_)
            | IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(_)
            | IdentityEvolutionResultEnvelope::Ambiguity(_)
            | IdentityEvolutionResultEnvelope::IdentityBreak(_)
            | IdentityEvolutionResultEnvelope::GeneratedIdentity(_)
            | IdentityEvolutionResultEnvelope::RetiredIdentity(_)
            | IdentityEvolutionResultEnvelope::Denied(_) => None,
        }
    }

    pub fn as_ambiguity(&self) -> Option<&IdentityEvolutionAmbiguityBundle> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::Ambiguity(result) => Some(result),
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(_)
            | IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(_)
            | IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(_)
            | IdentityEvolutionResultEnvelope::IdentityBreak(_)
            | IdentityEvolutionResultEnvelope::GeneratedIdentity(_)
            | IdentityEvolutionResultEnvelope::RetiredIdentity(_)
            | IdentityEvolutionResultEnvelope::Denied(_) => None,
        }
    }

    pub fn as_identity_break(&self) -> Option<&IdentityEvolutionIdentityBreakBundle> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::IdentityBreak(result) => Some(result),
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(_)
            | IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(_)
            | IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(_)
            | IdentityEvolutionResultEnvelope::Ambiguity(_)
            | IdentityEvolutionResultEnvelope::Denied(_) => None,
            IdentityEvolutionResultEnvelope::GeneratedIdentity(_)
            | IdentityEvolutionResultEnvelope::RetiredIdentity(_) => None,
        }
    }

    pub fn as_denied(&self) -> Option<&IdentityEvolutionDeniedBundle> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::Denied(result) => Some(result),
            IdentityEvolutionResultEnvelope::SingularIdentityContinuity(_)
            | IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(_)
            | IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(_)
            | IdentityEvolutionResultEnvelope::Ambiguity(_) => None,
            IdentityEvolutionResultEnvelope::IdentityBreak(_) => None,
            IdentityEvolutionResultEnvelope::GeneratedIdentity(_)
            | IdentityEvolutionResultEnvelope::RetiredIdentity(_) => None,
        }
    }

    pub fn as_generated_identity(&self) -> Option<&IdentityLifecycleResult> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::GeneratedIdentity(result) => Some(result),
            _ => None,
        }
    }

    pub fn as_retired_identity(&self) -> Option<&IdentityLifecycleResult> {
        match &self.envelope {
            IdentityEvolutionResultEnvelope::RetiredIdentity(result) => Some(result),
            _ => None,
        }
    }

    pub(crate) fn singular_identity_continuity(result: SingularIdentityContinuityResult) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::SingularIdentityContinuity(result),
        }
    }

    pub(crate) fn plural_identity_successor_set(result: PluralIdentitySuccessorSet) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::PluralIdentitySuccessorSet(result),
        }
    }

    pub(crate) fn advisory_identity_candidate_set(result: AdvisoryIdentityCandidateSet) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::AdvisoryIdentityCandidateSet(result),
        }
    }

    pub(crate) fn ambiguity(result: IdentityEvolutionAmbiguityBundle) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::Ambiguity(result),
        }
    }

    pub(crate) fn identity_break(result: IdentityEvolutionIdentityBreakBundle) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::IdentityBreak(result),
        }
    }

    pub(crate) fn generated_identity(result: IdentityLifecycleResult) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::GeneratedIdentity(result),
        }
    }

    pub(crate) fn retired_identity(result: IdentityLifecycleResult) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::RetiredIdentity(result),
        }
    }

    pub(crate) fn denied(result: IdentityEvolutionDeniedBundle) -> Self {
        Self {
            envelope: IdentityEvolutionResultEnvelope::Denied(result),
        }
    }
}
