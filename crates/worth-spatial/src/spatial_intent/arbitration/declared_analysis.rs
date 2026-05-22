use super::candidates::SpatialIntentCandidate;
use super::capabilities::{
    SpatialBlockedCapability, SpatialIntentCandidateAvailability, SpatialIntentCapabilitySummary,
};
use super::facts::{SpatialAuthoredActKind, SpatialObservedRelationFact};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentConflictClass {
    SingleClearIntent,
    MultiplePlausibleIntents,
    UnsafeToAssume,
    BlockedCandidateSet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentExplanationClass {
    AuthoredBaseline,
    RelationInferred,
    BlockedFutureCapability,
    UnsafeBoundary,
    PolicyPreferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentEscalation {
    AutoResolve(SpatialIntentCandidate),
    PreserveCandidates,
    AskForClarification,
    BlockedByMissingCapability(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationPreviewHint {
    AutoResolve(SpatialIntentCandidate),
    PreserveCandidates,
    ClarificationRequired,
    BlockedByCapability(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationContinuityHint {
    IdentityPreserved(SpatialIntentCandidate),
    AnchorContinuityPreserved(SpatialIntentCandidate),
    IdentityReinterpreted(SpatialIntentCandidate),
    IdentitySplit(SpatialIntentCandidate),
    IdentityMerged(SpatialIntentCandidate),
    PendingChoice,
    BlockedPendingChoice(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialIntentCandidateRank {
    candidate: SpatialIntentCandidate,
    availability: SpatialIntentCandidateAvailability,
    explanation: SpatialIntentExplanationClass,
    priority: u8,
    is_baseline: bool,
    is_policy_preferred: bool,
}

impl SpatialIntentCandidateRank {
    pub fn new(
        candidate: SpatialIntentCandidate,
        availability: SpatialIntentCandidateAvailability,
        explanation: SpatialIntentExplanationClass,
        is_baseline: bool,
        is_policy_preferred: bool,
    ) -> Self {
        Self {
            candidate,
            availability,
            explanation,
            priority: candidate.default_priority(),
            is_baseline,
            is_policy_preferred,
        }
    }

    pub fn candidate(&self) -> SpatialIntentCandidate {
        self.candidate
    }

    pub fn availability(&self) -> SpatialIntentCandidateAvailability {
        self.availability
    }

    pub fn explanation(&self) -> SpatialIntentExplanationClass {
        self.explanation
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }

    pub fn is_baseline(&self) -> bool {
        self.is_baseline
    }

    pub fn is_policy_preferred(&self) -> bool {
        self.is_policy_preferred
    }

    pub fn blocked_capability(&self) -> Option<SpatialBlockedCapability> {
        match self.availability {
            SpatialIntentCandidateAvailability::Available => None,
            SpatialIntentCandidateAvailability::Blocked(capability) => Some(capability),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialIntentArbitrationDeclaration {
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: Vec<SpatialObservedRelationFact>,
    candidates: Vec<SpatialIntentCandidateRank>,
    conflict_class: SpatialIntentConflictClass,
    escalation: SpatialIntentEscalation,
    chosen_candidate: Option<SpatialIntentCandidate>,
    policy_profile_name: &'static str,
    capability_summary: SpatialIntentCapabilitySummary,
}

impl SpatialIntentArbitrationDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: Vec<SpatialObservedRelationFact>,
        candidates: Vec<SpatialIntentCandidateRank>,
        conflict_class: SpatialIntentConflictClass,
        escalation: SpatialIntentEscalation,
        chosen_candidate: Option<SpatialIntentCandidate>,
        policy_profile_name: &'static str,
        capability_summary: SpatialIntentCapabilitySummary,
    ) -> Self {
        Self {
            authored_act,
            observed_relation_facts,
            candidates,
            conflict_class,
            escalation,
            chosen_candidate,
            policy_profile_name,
            capability_summary,
        }
    }

    pub fn authored_act(&self) -> SpatialAuthoredActKind {
        self.authored_act
    }

    pub fn observed_relation_facts(&self) -> &[SpatialObservedRelationFact] {
        &self.observed_relation_facts
    }

    pub fn candidates(&self) -> &[SpatialIntentCandidateRank] {
        &self.candidates
    }

    pub fn conflict_class(&self) -> SpatialIntentConflictClass {
        self.conflict_class
    }

    pub fn escalation(&self) -> SpatialIntentEscalation {
        self.escalation
    }

    pub fn chosen_candidate(&self) -> Option<SpatialIntentCandidate> {
        self.chosen_candidate
    }

    pub fn policy_profile_name(&self) -> &'static str {
        self.policy_profile_name
    }

    pub fn capability_summary(&self) -> &SpatialIntentCapabilitySummary {
        &self.capability_summary
    }

    pub fn preview_hint(&self) -> SpatialArbitrationPreviewHint {
        match self.escalation {
            SpatialIntentEscalation::AutoResolve(candidate) => {
                SpatialArbitrationPreviewHint::AutoResolve(candidate)
            }
            SpatialIntentEscalation::PreserveCandidates => {
                SpatialArbitrationPreviewHint::PreserveCandidates
            }
            SpatialIntentEscalation::AskForClarification => {
                SpatialArbitrationPreviewHint::ClarificationRequired
            }
            SpatialIntentEscalation::BlockedByMissingCapability(capability) => {
                SpatialArbitrationPreviewHint::BlockedByCapability(capability)
            }
        }
    }

    pub fn continuity_hint(&self) -> SpatialArbitrationContinuityHint {
        match self.escalation {
            SpatialIntentEscalation::AutoResolve(
                candidate
                @ (SpatialIntentCandidate::MoveOnly | SpatialIntentCandidate::AlignFrames),
            ) => SpatialArbitrationContinuityHint::IdentityPreserved(candidate),
            SpatialIntentEscalation::AutoResolve(candidate @ SpatialIntentCandidate::SnapFlush) => {
                SpatialArbitrationContinuityHint::AnchorContinuityPreserved(candidate)
            }
            SpatialIntentEscalation::AutoResolve(
                candidate @ (SpatialIntentCandidate::AttachRelationally
                | SpatialIntentCandidate::NestInside),
            ) => SpatialArbitrationContinuityHint::IdentityReinterpreted(candidate),
            SpatialIntentEscalation::AutoResolve(
                candidate @ (SpatialIntentCandidate::SubtractCandidate
                | SpatialIntentCandidate::CutOpeningCandidate),
            ) => SpatialArbitrationContinuityHint::IdentitySplit(candidate),
            SpatialIntentEscalation::AutoResolve(
                candidate @ (SpatialIntentCandidate::MergeCandidate
                | SpatialIntentCandidate::JoinCandidate),
            ) => SpatialArbitrationContinuityHint::IdentityMerged(candidate),
            SpatialIntentEscalation::PreserveCandidates
            | SpatialIntentEscalation::AskForClarification => {
                SpatialArbitrationContinuityHint::PendingChoice
            }
            SpatialIntentEscalation::BlockedByMissingCapability(capability) => {
                SpatialArbitrationContinuityHint::BlockedPendingChoice(capability)
            }
        }
    }

    pub fn policy_preferred_candidate(&self) -> Option<SpatialIntentCandidate> {
        self.candidates
            .iter()
            .find(|candidate| candidate.is_policy_preferred())
            .map(SpatialIntentCandidateRank::candidate)
    }
}

pub type SpatialIntentArbitrationAnalysis = SpatialIntentArbitrationDeclaration;
