use std::collections::HashSet;

use crate::certification::policy_support::{
    SpatialArbitrationPolicyProfile, SpatialArbitrationPosture,
};

use super::candidates::SpatialArbitrationCandidate;
use super::capabilities::{
    SpatialArbitrationCandidateAvailability, SpatialArbitrationCapabilitySet,
    SpatialBlockedCapability,
};
use super::declaration::{
    SpatialArbitrationCandidateRank, SpatialArbitrationConflictClass,
    SpatialArbitrationDeclaration, SpatialArbitrationEscalation,
    SpatialArbitrationExplanationClass,
};
use super::facts::{SpatialAuthoredActKind, SpatialObservedRelationFact};

pub(crate) fn analyze_spatial_arbitration_conflict(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
) -> SpatialArbitrationDeclaration {
    analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        SpatialArbitrationCapabilitySet::blocked_defaults(),
        SpatialArbitrationPolicyProfile::conservative_exact_modeling(),
    )
}

pub(crate) fn analyze_spatial_arbitration_conflict_with_capabilities(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialArbitrationCapabilitySet,
) -> SpatialArbitrationDeclaration {
    analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        capabilities,
        SpatialArbitrationPolicyProfile::conservative_exact_modeling(),
    )
}

pub(crate) fn analyze_spatial_arbitration_conflict_with_profile(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    profile: SpatialArbitrationPolicyProfile,
) -> SpatialArbitrationDeclaration {
    analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        SpatialArbitrationCapabilitySet::blocked_defaults(),
        profile,
    )
}

pub(crate) fn analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialArbitrationCapabilitySet,
    profile: SpatialArbitrationPolicyProfile,
) -> SpatialArbitrationDeclaration {
    compute_spatial_arbitration_declaration(
        authored_act,
        observed_relation_facts,
        capabilities,
        profile,
    )
}

pub(crate) fn compute_spatial_arbitration_declaration(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialArbitrationCapabilitySet,
    profile: SpatialArbitrationPolicyProfile,
) -> SpatialArbitrationDeclaration {
    let baseline = SpatialArbitrationCandidate::baseline_for(authored_act);
    let mut inserted = HashSet::new();
    let mut candidates = Vec::new();
    push_candidate(
        &mut candidates,
        &mut inserted,
        baseline,
        capabilities.availability_for(None),
        SpatialArbitrationExplanationClass::AuthoredBaseline,
        true,
        false,
    );

    for fact in observed_relation_facts.iter().copied() {
        match fact {
            SpatialObservedRelationFact::Overlap => {
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialArbitrationCandidate::MergeCandidate,
                    capabilities.availability_for(Some(SpatialBlockedCapability::MergeBoolean)),
                    SpatialArbitrationExplanationClass::BlockedFutureCapability,
                    false,
                    false,
                );
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialArbitrationCandidate::SubtractCandidate,
                    capabilities.availability_for(Some(SpatialBlockedCapability::SubtractBoolean)),
                    SpatialArbitrationExplanationClass::BlockedFutureCapability,
                    false,
                    false,
                );
            }
            SpatialObservedRelationFact::GrazingContact => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialArbitrationCandidate::SnapFlush,
                capabilities.availability_for(None),
                SpatialArbitrationExplanationClass::UnsafeBoundary,
                false,
                false,
            ),
            SpatialObservedRelationFact::FrameAligned => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialArbitrationCandidate::AlignFrames,
                capabilities.availability_for(None),
                SpatialArbitrationExplanationClass::RelationInferred,
                false,
                false,
            ),
            SpatialObservedRelationFact::InsideTarget => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialArbitrationCandidate::NestInside,
                capabilities.availability_for(None),
                SpatialArbitrationExplanationClass::UnsafeBoundary,
                false,
                false,
            ),
            SpatialObservedRelationFact::HostFaceContact => {
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialArbitrationCandidate::AttachRelationally,
                    capabilities.availability_for(Some(SpatialBlockedCapability::HostAttach)),
                    SpatialArbitrationExplanationClass::BlockedFutureCapability,
                    false,
                    false,
                );
                push_candidate(
                    &mut candidates,
                    &mut inserted,
                    SpatialArbitrationCandidate::JoinCandidate,
                    capabilities.availability_for(Some(SpatialBlockedCapability::Join)),
                    SpatialArbitrationExplanationClass::BlockedFutureCapability,
                    false,
                    false,
                );
            }
            SpatialObservedRelationFact::HostPenetration => push_candidate(
                &mut candidates,
                &mut inserted,
                SpatialArbitrationCandidate::CutOpeningCandidate,
                capabilities.availability_for(Some(SpatialBlockedCapability::CutOpening)),
                SpatialArbitrationExplanationClass::BlockedFutureCapability,
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
            candidate.availability() == SpatialArbitrationCandidateAvailability::Available
        })
        .collect::<Vec<_>>();
    let blocked = candidates
        .iter()
        .copied()
        .filter(|candidate| {
            matches!(
                candidate.availability(),
                SpatialArbitrationCandidateAvailability::Blocked(_)
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
            SpatialArbitrationConflictClass::SingleClearIntent,
            SpatialArbitrationEscalation::AutoResolve(baseline),
            Some(baseline),
        )
    } else if let Some(candidate) = preferred_available {
        (
            if available.len() > 1 {
                SpatialArbitrationConflictClass::UnsafeToAssume
            } else {
                SpatialArbitrationConflictClass::SingleClearIntent
            },
            SpatialArbitrationEscalation::AutoResolve(candidate),
            Some(candidate),
        )
    } else if !blocked.is_empty() && (nonbaseline_available || available.len() == 1) {
        (
            SpatialArbitrationConflictClass::BlockedCandidateSet,
            SpatialArbitrationEscalation::BlockedByMissingCapability(first_blocked_capability(
                &blocked,
            )),
            None,
        )
    } else if available.len() > 1
        && available
            .iter()
            .any(|candidate| candidate.candidate() == baseline)
    {
        (
            SpatialArbitrationConflictClass::UnsafeToAssume,
            match profile.arbitration_posture() {
                SpatialArbitrationPosture::PreserveAmbiguity => {
                    SpatialArbitrationEscalation::PreserveCandidates
                }
                _ => SpatialArbitrationEscalation::AskForClarification,
            },
            None,
        )
    } else if available.len() > 1 {
        (
            SpatialArbitrationConflictClass::MultiplePlausibleIntents,
            match profile.arbitration_posture() {
                SpatialArbitrationPosture::AskFirst => {
                    SpatialArbitrationEscalation::AskForClarification
                }
                _ => SpatialArbitrationEscalation::PreserveCandidates,
            },
            None,
        )
    } else {
        (
            SpatialArbitrationConflictClass::BlockedCandidateSet,
            SpatialArbitrationEscalation::BlockedByMissingCapability(first_blocked_capability(
                &blocked,
            )),
            None,
        )
    };

    SpatialArbitrationDeclaration::new(
        authored_act,
        observed_relation_facts.to_vec(),
        candidates,
        conflict_class,
        escalation,
        chosen_candidate,
        profile,
        capabilities.summary(),
    )
}

fn preferred_profile_candidate(
    available: &[SpatialArbitrationCandidateRank],
    profile: SpatialArbitrationPolicyProfile,
    observed_relation_facts: &[SpatialObservedRelationFact],
) -> Option<SpatialArbitrationCandidate> {
    match profile.arbitration_posture() {
        SpatialArbitrationPosture::PreferSnap
            if observed_relation_facts.contains(&SpatialObservedRelationFact::GrazingContact) =>
        {
            available
                .iter()
                .find(|candidate| candidate.candidate() == SpatialArbitrationCandidate::SnapFlush)
                .map(SpatialArbitrationCandidateRank::candidate)
        }
        SpatialArbitrationPosture::PreferHostRelationships
            if observed_relation_facts.contains(&SpatialObservedRelationFact::HostFaceContact) =>
        {
            available
                .iter()
                .find(|candidate| {
                    candidate.candidate() == SpatialArbitrationCandidate::AttachRelationally
                })
                .map(SpatialArbitrationCandidateRank::candidate)
        }
        _ => None,
    }
}

fn push_candidate(
    candidates: &mut Vec<SpatialArbitrationCandidateRank>,
    inserted: &mut HashSet<SpatialArbitrationCandidate>,
    candidate: SpatialArbitrationCandidate,
    availability: SpatialArbitrationCandidateAvailability,
    explanation: SpatialArbitrationExplanationClass,
    is_baseline: bool,
    is_policy_preferred: bool,
) {
    if inserted.insert(candidate) {
        candidates.push(SpatialArbitrationCandidateRank::new(
            candidate,
            availability,
            explanation,
            is_baseline,
            is_policy_preferred,
        ));
    }
}

fn mark_policy_preferred(
    candidates: &mut [SpatialArbitrationCandidateRank],
    candidate: SpatialArbitrationCandidate,
) {
    if let Some(rank) = candidates
        .iter_mut()
        .find(|rank| rank.candidate() == candidate)
    {
        *rank = SpatialArbitrationCandidateRank::new(
            rank.candidate(),
            rank.availability(),
            SpatialArbitrationExplanationClass::PolicyPreferred,
            rank.is_baseline(),
            true,
        );
    }
}

fn first_blocked_capability(
    candidates: &[SpatialArbitrationCandidateRank],
) -> SpatialBlockedCapability {
    candidates
        .iter()
        .find_map(SpatialArbitrationCandidateRank::blocked_capability)
        .expect("blocked candidate capability")
}
