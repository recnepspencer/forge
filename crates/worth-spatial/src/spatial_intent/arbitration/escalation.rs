use std::collections::HashSet;

use super::blocked::{
    SpatialBlockedCapability, SpatialIntentCandidateAvailability, SpatialIntentCapabilitySet,
};
use super::candidates::SpatialIntentCandidate;
use super::conflicts::{
    SpatialAuthoredActKind, SpatialIntentConflictClass, SpatialObservedRelationFact,
};
use super::ranking::{SpatialIntentCandidateRank, SpatialIntentExplanationClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentEscalation {
    AutoResolve(SpatialIntentCandidate),
    PreserveCandidates,
    AskForClarification,
    BlockedByMissingCapability(SpatialBlockedCapability),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialIntentArbitrationAnalysis {
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: Vec<SpatialObservedRelationFact>,
    candidates: Vec<SpatialIntentCandidateRank>,
    conflict_class: SpatialIntentConflictClass,
    escalation: SpatialIntentEscalation,
    chosen_candidate: Option<SpatialIntentCandidate>,
}

impl SpatialIntentArbitrationAnalysis {
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
}

pub fn analyze_spatial_intent_conflict(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
) -> SpatialIntentArbitrationAnalysis {
    analyze_spatial_intent_conflict_with_capabilities(
        authored_act,
        observed_relation_facts,
        SpatialIntentCapabilitySet::blocked_defaults(),
    )
}

pub fn analyze_spatial_intent_conflict_with_capabilities(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialIntentCapabilitySet,
) -> SpatialIntentArbitrationAnalysis {
    let baseline = SpatialIntentCandidate::baseline_for(authored_act);
    let mut inserted = HashSet::new();
    let mut candidates = Vec::new();
    push_candidate(
        &mut candidates,
        &mut inserted,
        baseline,
        capabilities.availability_for(None),
        SpatialIntentExplanationClass::AuthoredBaseline,
    );

    for fact in observed_relation_facts.iter().copied() {
        match fact {
            SpatialObservedRelationFact::Overlap => {
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialIntentCandidate::MergeCandidate,
                    capabilities.availability_for(Some(SpatialBlockedCapability::MergeBoolean)),
                    SpatialIntentExplanationClass::BlockedFutureCapability,
                );
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialIntentCandidate::SubtractCandidate,
                    capabilities.availability_for(Some(SpatialBlockedCapability::SubtractBoolean)),
                    SpatialIntentExplanationClass::BlockedFutureCapability,
                );
            }
            SpatialObservedRelationFact::GrazingContact => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialIntentCandidate::SnapFlush,
                capabilities.availability_for(None),
                SpatialIntentExplanationClass::UnsafeBoundary,
            ),
            SpatialObservedRelationFact::FrameAligned => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialIntentCandidate::AlignFrames,
                capabilities.availability_for(None),
                SpatialIntentExplanationClass::RelationInferred,
            ),
            SpatialObservedRelationFact::InsideTarget => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialIntentCandidate::NestInside,
                capabilities.availability_for(None),
                SpatialIntentExplanationClass::UnsafeBoundary,
            ),
            SpatialObservedRelationFact::HostFaceContact => {
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialIntentCandidate::AttachRelationally,
                    capabilities.availability_for(Some(SpatialBlockedCapability::HostAttach)),
                    SpatialIntentExplanationClass::BlockedFutureCapability,
                );
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialIntentCandidate::JoinCandidate,
                    capabilities.availability_for(Some(SpatialBlockedCapability::Join)),
                    SpatialIntentExplanationClass::BlockedFutureCapability,
                );
            }
            SpatialObservedRelationFact::HostPenetration => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialIntentCandidate::CutOpeningCandidate,
                capabilities.availability_for(Some(SpatialBlockedCapability::CutOpening)),
                SpatialIntentExplanationClass::BlockedFutureCapability,
            ),
        }
    }

    candidates.sort_by(|left, right| right.priority().cmp(&left.priority()));
    let available = candidates
        .iter()
        .copied()
        .filter(|candidate| {
            candidate.availability() == SpatialIntentCandidateAvailability::Available
        })
        .collect::<Vec<_>>();
    let blocked = candidates
        .iter()
        .copied()
        .filter(|candidate| {
            matches!(
                candidate.availability(),
                SpatialIntentCandidateAvailability::Blocked(_)
            )
        })
        .collect::<Vec<_>>();

    let only_baseline_available =
        available.len() == 1 && available[0].candidate() == baseline && blocked.is_empty();
    let nonbaseline_available = available
        .iter()
        .any(|candidate| candidate.candidate() != baseline);
    let (conflict_class, escalation, chosen_candidate) = if only_baseline_available {
        (
            SpatialIntentConflictClass::SingleClearIntent,
            SpatialIntentEscalation::AutoResolve(baseline),
            Some(baseline),
        )
    } else if !blocked.is_empty() && (nonbaseline_available || available.len() == 1) {
        (
            SpatialIntentConflictClass::BlockedCandidateSet,
            SpatialIntentEscalation::BlockedByMissingCapability(first_blocked_capability(&blocked)),
            None,
        )
    } else if available.len() > 1
        && available
            .iter()
            .any(|candidate| candidate.candidate() == baseline)
    {
        (
            SpatialIntentConflictClass::UnsafeToAssume,
            SpatialIntentEscalation::AskForClarification,
            None,
        )
    } else if available.len() > 1 {
        (
            SpatialIntentConflictClass::MultiplePlausibleIntents,
            SpatialIntentEscalation::PreserveCandidates,
            None,
        )
    } else {
        (
            SpatialIntentConflictClass::BlockedCandidateSet,
            SpatialIntentEscalation::BlockedByMissingCapability(first_blocked_capability(&blocked)),
            None,
        )
    };

    SpatialIntentArbitrationAnalysis {
        authored_act,
        observed_relation_facts: observed_relation_facts.to_vec(),
        candidates,
        conflict_class,
        escalation,
        chosen_candidate,
    }
}

fn push_candidate(
    candidates: &mut Vec<SpatialIntentCandidateRank>,
    inserted: &mut HashSet<SpatialIntentCandidate>,
    candidate: SpatialIntentCandidate,
    availability: SpatialIntentCandidateAvailability,
    explanation: SpatialIntentExplanationClass,
) {
    if inserted.insert(candidate) {
        candidates.push(SpatialIntentCandidateRank::new(
            candidate,
            availability,
            explanation,
        ));
    }
}

fn first_blocked_capability(candidates: &[SpatialIntentCandidateRank]) -> SpatialBlockedCapability {
    candidates
        .iter()
        .find_map(|candidate| match candidate.availability() {
            SpatialIntentCandidateAvailability::Blocked(blocked) => Some(blocked),
            SpatialIntentCandidateAvailability::Available => None,
        })
        .expect("blocked candidate capability")
}
