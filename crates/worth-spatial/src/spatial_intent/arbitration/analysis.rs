use std::collections::HashSet;

use forge_proof::TransitionOutcome;

use crate::spatial_intent::policy::{SpatialArbitrationPosture, SpatialIntentPolicyProfile};

use super::candidates::SpatialIntentCandidate;
use super::capabilities::{
    SpatialBlockedCapability, SpatialIntentCandidateAvailability, SpatialIntentCapabilitySet,
};
use super::declared_analysis::{
    SpatialIntentArbitrationDeclaration, SpatialIntentCandidateRank, SpatialIntentConflictClass,
    SpatialIntentEscalation, SpatialIntentExplanationClass,
};
use super::facts::{SpatialAuthoredActKind, SpatialObservedRelationFact};
use super::progression::{
    admit_requested_spatial_arbitration_intent, declare_admitted_spatial_arbitration_intent,
    request_spatial_arbitration_intent,
};

pub fn analyze_spatial_intent_conflict(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
) -> SpatialIntentArbitrationDeclaration {
    analyze_spatial_intent_conflict_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    )
}

pub fn analyze_spatial_intent_conflict_with_capabilities(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialIntentCapabilitySet,
) -> SpatialIntentArbitrationDeclaration {
    analyze_spatial_intent_conflict_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        capabilities,
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    )
}

pub fn analyze_spatial_intent_conflict_with_profile(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    profile: SpatialIntentPolicyProfile,
) -> SpatialIntentArbitrationDeclaration {
    analyze_spatial_intent_conflict_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        SpatialIntentCapabilitySet::blocked_defaults(),
        profile,
    )
}

pub fn analyze_spatial_intent_conflict_with_capabilities_and_profile(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialIntentCapabilitySet,
    profile: SpatialIntentPolicyProfile,
) -> SpatialIntentArbitrationDeclaration {
    let requested = request_spatial_arbitration_intent(
        authored_act,
        observed_relation_facts,
        capabilities,
        profile,
    );
    let admitted = match admit_requested_spatial_arbitration_intent(requested) {
        TransitionOutcome::Success(admitted) => admitted,
        _ => unreachable!("spatial arbitration admission is infallible"),
    };
    let declared = match declare_admitted_spatial_arbitration_intent(admitted) {
        TransitionOutcome::Success(declared) => declared,
        _ => unreachable!("spatial arbitration declaration is infallible"),
    };
    let (payload, _, _) = declared.into_parts().into_parts();
    payload
}

pub(crate) fn compute_spatial_intent_arbitration_declaration(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialIntentCapabilitySet,
    profile: SpatialIntentPolicyProfile,
) -> SpatialIntentArbitrationDeclaration {
    let baseline = SpatialIntentCandidate::baseline_for(authored_act);
    let mut inserted = HashSet::new();
    let mut candidates = Vec::new();
    push_candidate(
        &mut candidates,
        &mut inserted,
        baseline,
        capabilities.availability_for(None),
        SpatialIntentExplanationClass::AuthoredBaseline,
        true,
        false,
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
                    false,
                    false,
                );
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialIntentCandidate::SubtractCandidate,
                    capabilities.availability_for(Some(SpatialBlockedCapability::SubtractBoolean)),
                    SpatialIntentExplanationClass::BlockedFutureCapability,
                    false,
                    false,
                );
            }
            SpatialObservedRelationFact::GrazingContact => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialIntentCandidate::SnapFlush,
                capabilities.availability_for(None),
                SpatialIntentExplanationClass::UnsafeBoundary,
                false,
                false,
            ),
            SpatialObservedRelationFact::FrameAligned => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialIntentCandidate::AlignFrames,
                capabilities.availability_for(None),
                SpatialIntentExplanationClass::RelationInferred,
                false,
                false,
            ),
            SpatialObservedRelationFact::InsideTarget => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialIntentCandidate::NestInside,
                capabilities.availability_for(None),
                SpatialIntentExplanationClass::UnsafeBoundary,
                false,
                false,
            ),
            SpatialObservedRelationFact::HostFaceContact => {
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialIntentCandidate::AttachRelationally,
                    capabilities.availability_for(Some(SpatialBlockedCapability::HostAttach)),
                    SpatialIntentExplanationClass::BlockedFutureCapability,
                    false,
                    false,
                );
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialIntentCandidate::JoinCandidate,
                    capabilities.availability_for(Some(SpatialBlockedCapability::Join)),
                    SpatialIntentExplanationClass::BlockedFutureCapability,
                    false,
                    false,
                );
            }
            SpatialObservedRelationFact::HostPenetration => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialIntentCandidate::CutOpeningCandidate,
                capabilities.availability_for(Some(SpatialBlockedCapability::CutOpening)),
                SpatialIntentExplanationClass::BlockedFutureCapability,
                false,
                false,
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

    let preferred_available =
        preferred_profile_candidate(&available, profile, observed_relation_facts);
    if let Some(candidate) = preferred_available {
        mark_policy_preferred(&mut candidates, candidate);
    }

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
    } else if let Some(candidate) = preferred_available {
        (
            if available.len() > 1 {
                SpatialIntentConflictClass::UnsafeToAssume
            } else {
                SpatialIntentConflictClass::SingleClearIntent
            },
            SpatialIntentEscalation::AutoResolve(candidate),
            Some(candidate),
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
            match profile.arbitration_posture() {
                SpatialArbitrationPosture::PreserveAmbiguity => {
                    SpatialIntentEscalation::PreserveCandidates
                }
                _ => SpatialIntentEscalation::AskForClarification,
            },
            None,
        )
    } else if available.len() > 1 {
        (
            SpatialIntentConflictClass::MultiplePlausibleIntents,
            match profile.arbitration_posture() {
                SpatialArbitrationPosture::AskFirst => SpatialIntentEscalation::AskForClarification,
                _ => SpatialIntentEscalation::PreserveCandidates,
            },
            None,
        )
    } else {
        (
            SpatialIntentConflictClass::BlockedCandidateSet,
            SpatialIntentEscalation::BlockedByMissingCapability(first_blocked_capability(&blocked)),
            None,
        )
    };

    SpatialIntentArbitrationDeclaration::new(
        authored_act,
        observed_relation_facts.to_vec(),
        candidates,
        conflict_class,
        escalation,
        chosen_candidate,
        profile.name(),
        capabilities.summary(),
    )
}

fn preferred_profile_candidate(
    available: &[SpatialIntentCandidateRank],
    profile: SpatialIntentPolicyProfile,
    observed_relation_facts: &[SpatialObservedRelationFact],
) -> Option<SpatialIntentCandidate> {
    match profile.arbitration_posture() {
        SpatialArbitrationPosture::PreferSnap
            if observed_relation_facts.contains(&SpatialObservedRelationFact::GrazingContact) =>
        {
            available
                .iter()
                .find(|candidate| candidate.candidate() == SpatialIntentCandidate::SnapFlush)
                .map(SpatialIntentCandidateRank::candidate)
        }
        SpatialArbitrationPosture::PreferHostRelationships
            if observed_relation_facts.contains(&SpatialObservedRelationFact::HostFaceContact) =>
        {
            available
                .iter()
                .find(|candidate| {
                    candidate.candidate() == SpatialIntentCandidate::AttachRelationally
                })
                .map(SpatialIntentCandidateRank::candidate)
        }
        _ => None,
    }
}

fn push_candidate(
    candidates: &mut Vec<SpatialIntentCandidateRank>,
    inserted: &mut HashSet<SpatialIntentCandidate>,
    candidate: SpatialIntentCandidate,
    availability: SpatialIntentCandidateAvailability,
    explanation: SpatialIntentExplanationClass,
    is_baseline: bool,
    is_policy_preferred: bool,
) {
    if inserted.insert(candidate) {
        candidates.push(SpatialIntentCandidateRank::new(
            candidate,
            availability,
            explanation,
            is_baseline,
            is_policy_preferred,
        ));
    }
}

fn mark_policy_preferred(
    candidates: &mut [SpatialIntentCandidateRank],
    candidate: SpatialIntentCandidate,
) {
    if let Some(rank) = candidates
        .iter_mut()
        .find(|rank| rank.candidate() == candidate)
    {
        *rank = SpatialIntentCandidateRank::new(
            rank.candidate(),
            rank.availability(),
            SpatialIntentExplanationClass::PolicyPreferred,
            rank.is_baseline(),
            true,
        );
    }
}

fn first_blocked_capability(candidates: &[SpatialIntentCandidateRank]) -> SpatialBlockedCapability {
    candidates
        .iter()
        .find_map(SpatialIntentCandidateRank::blocked_capability)
        .expect("blocked candidate capability")
}
