use crate::spatial_intent::arbitration::{
    SpatialBlockedCapability, SpatialChosenIntentResolution, SpatialIntentArbitrationAnalysis,
    SpatialIntentCandidate, SpatialIntentEscalation,
};

use super::outcomes::{
    SpatialIdentityContinuityAssessment, SpatialIdentityContinuityClass,
    SpatialIdentityContinuityExplanationClass,
};

pub fn assess_spatial_identity_continuity_from_analysis(
    analysis: &SpatialIntentArbitrationAnalysis,
) -> SpatialIdentityContinuityAssessment {
    match analysis.escalation() {
        SpatialIntentEscalation::AutoResolve(candidate) => {
            assessment_for_candidate(Some(candidate), None)
        }
        SpatialIntentEscalation::PreserveCandidates
        | SpatialIntentEscalation::AskForClarification => SpatialIdentityContinuityAssessment::new(
            SpatialIdentityContinuityClass::IdentityBlockedPendingChoice,
            SpatialIdentityContinuityExplanationClass::CandidateSetPendingChoice,
            None,
            None,
            false,
            false,
        ),
        SpatialIntentEscalation::BlockedByMissingCapability(capability) => {
            blocked_pending_choice(capability)
        }
    }
}

pub fn assess_spatial_identity_continuity_from_resolution(
    resolution: &SpatialChosenIntentResolution,
) -> SpatialIdentityContinuityAssessment {
    assessment_for_candidate(Some(resolution.chosen_candidate()), None)
}

fn assessment_for_candidate(
    candidate: Option<SpatialIntentCandidate>,
    blocked_capability: Option<SpatialBlockedCapability>,
) -> SpatialIdentityContinuityAssessment {
    match candidate {
        Some(SpatialIntentCandidate::MoveOnly | SpatialIntentCandidate::AlignFrames) => {
            SpatialIdentityContinuityAssessment::new(
                SpatialIdentityContinuityClass::IdentityPreserved,
                SpatialIdentityContinuityExplanationClass::BaselineIdentityPreserved,
                candidate,
                blocked_capability,
                true,
                true,
            )
        }
        Some(SpatialIntentCandidate::SnapFlush) => SpatialIdentityContinuityAssessment::new(
            SpatialIdentityContinuityClass::AnchorContinuityPreserved,
            SpatialIdentityContinuityExplanationClass::RelationalAnchorContinuity,
            candidate,
            blocked_capability,
            true,
            true,
        ),
        Some(SpatialIntentCandidate::AttachRelationally | SpatialIntentCandidate::NestInside) => {
            SpatialIdentityContinuityAssessment::new(
                SpatialIdentityContinuityClass::IdentityReinterpreted,
                SpatialIdentityContinuityExplanationClass::RelationalIdentityReinterpreted,
                candidate,
                blocked_capability,
                true,
                false,
            )
        }
        Some(
            SpatialIntentCandidate::SubtractCandidate | SpatialIntentCandidate::CutOpeningCandidate,
        ) => SpatialIdentityContinuityAssessment::new(
            SpatialIdentityContinuityClass::IdentitySplit,
            SpatialIdentityContinuityExplanationClass::TopologyIdentitySplit,
            candidate,
            blocked_capability,
            false,
            false,
        ),
        Some(SpatialIntentCandidate::MergeCandidate | SpatialIntentCandidate::JoinCandidate) => {
            SpatialIdentityContinuityAssessment::new(
                SpatialIdentityContinuityClass::IdentityMerged,
                SpatialIdentityContinuityExplanationClass::TopologyIdentityMerged,
                candidate,
                blocked_capability,
                false,
                false,
            )
        }
        None => blocked_pending_choice(
            blocked_capability.expect("blocked capability required for unresolved continuity"),
        ),
    }
}

fn blocked_pending_choice(
    capability: SpatialBlockedCapability,
) -> SpatialIdentityContinuityAssessment {
    SpatialIdentityContinuityAssessment::new(
        SpatialIdentityContinuityClass::IdentityBlockedPendingChoice,
        SpatialIdentityContinuityExplanationClass::CapabilityBlockedPendingChoice,
        None,
        Some(capability),
        false,
        false,
    )
}
