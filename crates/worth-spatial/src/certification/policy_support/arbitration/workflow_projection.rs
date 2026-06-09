use crate::certification::policy_support::SpatialPreviewRichness;

use super::declaration::{SpatialArbitrationDeclaration, SpatialArbitrationEscalation};
use super::resolution::SpatialChosenArbitrationResolution;
use super::{SpatialArbitrationCandidate, SpatialBlockedCapability};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationPreviewCommitDisposition {
    WouldAutoResolve(SpatialArbitrationCandidate),
    WouldPreserveCandidates,
    WouldRequireClarification,
    WouldBlockOnCapability(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationPreviewWarning {
    ClarificationRequired,
    PreservedCandidateSet,
    BlockedFutureCandidate(SpatialBlockedCapability),
    ProfileDrivenAutoResolve(SpatialArbitrationCandidate),
    HighFidelityPreview,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIdentityContinuityClass {
    IdentityPreserved,
    AnchorContinuityPreserved,
    IdentityReinterpreted,
    IdentitySplit,
    IdentityMerged,
    IdentityBlockedPendingChoice,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIdentityContinuityExplanationClass {
    BaselineIdentityPreserved,
    RelationalAnchorContinuity,
    RelationalIdentityReinterpreted,
    TopologyIdentitySplit,
    TopologyIdentityMerged,
    CandidateSetPendingChoice,
    CapabilityBlockedPendingChoice,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialIdentityContinuityAssessment {
    continuity_class: SpatialIdentityContinuityClass,
    explanation_class: SpatialIdentityContinuityExplanationClass,
    candidate: Option<SpatialArbitrationCandidate>,
    blocked_capability: Option<SpatialBlockedCapability>,
    preserves_subject_identity: bool,
    preserves_anchor_identity: bool,
}

impl SpatialIdentityContinuityAssessment {
    fn new(
        continuity_class: SpatialIdentityContinuityClass,
        explanation_class: SpatialIdentityContinuityExplanationClass,
        candidate: Option<SpatialArbitrationCandidate>,
        blocked_capability: Option<SpatialBlockedCapability>,
        preserves_subject_identity: bool,
        preserves_anchor_identity: bool,
    ) -> Self {
        Self {
            continuity_class,
            explanation_class,
            candidate,
            blocked_capability,
            preserves_subject_identity,
            preserves_anchor_identity,
        }
    }

    pub fn continuity_class(&self) -> SpatialIdentityContinuityClass {
        self.continuity_class
    }

    pub fn explanation_class(&self) -> SpatialIdentityContinuityExplanationClass {
        self.explanation_class
    }

    pub fn candidate(&self) -> Option<SpatialArbitrationCandidate> {
        self.candidate
    }

    pub fn blocked_capability(&self) -> Option<SpatialBlockedCapability> {
        self.blocked_capability
    }

    pub fn preserves_subject_identity(&self) -> bool {
        self.preserves_subject_identity
    }

    pub fn preserves_anchor_identity(&self) -> bool {
        self.preserves_anchor_identity
    }
}

impl SpatialArbitrationDeclaration {
    pub fn preview_richness(&self) -> SpatialPreviewRichness {
        self.policy_profile().preview_richness()
    }

    pub fn preview_commit_disposition(&self) -> SpatialArbitrationPreviewCommitDisposition {
        match self.escalation() {
            SpatialArbitrationEscalation::AutoResolve(candidate) => {
                SpatialArbitrationPreviewCommitDisposition::WouldAutoResolve(candidate)
            }
            SpatialArbitrationEscalation::PreserveCandidates => {
                SpatialArbitrationPreviewCommitDisposition::WouldPreserveCandidates
            }
            SpatialArbitrationEscalation::AskForClarification => {
                SpatialArbitrationPreviewCommitDisposition::WouldRequireClarification
            }
            SpatialArbitrationEscalation::BlockedByMissingCapability(capability) => {
                SpatialArbitrationPreviewCommitDisposition::WouldBlockOnCapability(capability)
            }
        }
    }

    pub fn preview_warnings(&self) -> Vec<SpatialArbitrationPreviewWarning> {
        let mut warnings = Vec::new();
        match self.preview_commit_disposition() {
            SpatialArbitrationPreviewCommitDisposition::WouldRequireClarification => {
                warnings.push(SpatialArbitrationPreviewWarning::ClarificationRequired)
            }
            SpatialArbitrationPreviewCommitDisposition::WouldPreserveCandidates => {
                warnings.push(SpatialArbitrationPreviewWarning::PreservedCandidateSet)
            }
            SpatialArbitrationPreviewCommitDisposition::WouldBlockOnCapability(capability) => {
                warnings.push(SpatialArbitrationPreviewWarning::BlockedFutureCandidate(
                    capability,
                ))
            }
            SpatialArbitrationPreviewCommitDisposition::WouldAutoResolve(candidate)
                if candidate != SpatialArbitrationCandidate::baseline_for(self.authored_act()) =>
            {
                warnings.push(SpatialArbitrationPreviewWarning::ProfileDrivenAutoResolve(
                    candidate,
                ))
            }
            SpatialArbitrationPreviewCommitDisposition::WouldAutoResolve(_) => {}
        }
        if self.preview_richness() == SpatialPreviewRichness::HighFidelity {
            warnings.push(SpatialArbitrationPreviewWarning::HighFidelityPreview);
        }
        warnings
    }

    pub fn identity_continuity_assessment(&self) -> SpatialIdentityContinuityAssessment {
        match self.escalation() {
            SpatialArbitrationEscalation::AutoResolve(candidate) => {
                assessment_for_candidate(Some(candidate), None)
            }
            SpatialArbitrationEscalation::PreserveCandidates
            | SpatialArbitrationEscalation::AskForClarification => {
                SpatialIdentityContinuityAssessment::new(
                    SpatialIdentityContinuityClass::IdentityBlockedPendingChoice,
                    SpatialIdentityContinuityExplanationClass::CandidateSetPendingChoice,
                    None,
                    None,
                    false,
                    false,
                )
            }
            SpatialArbitrationEscalation::BlockedByMissingCapability(capability) => {
                blocked_pending_choice(capability)
            }
        }
    }
}

impl SpatialChosenArbitrationResolution {
    pub fn identity_continuity_assessment(&self) -> SpatialIdentityContinuityAssessment {
        assessment_for_candidate(Some(self.chosen_candidate()), None)
    }
}

fn assessment_for_candidate(
    candidate: Option<SpatialArbitrationCandidate>,
    blocked_capability: Option<SpatialBlockedCapability>,
) -> SpatialIdentityContinuityAssessment {
    match candidate {
        Some(SpatialArbitrationCandidate::MoveOnly | SpatialArbitrationCandidate::AlignFrames) => {
            SpatialIdentityContinuityAssessment::new(
                SpatialIdentityContinuityClass::IdentityPreserved,
                SpatialIdentityContinuityExplanationClass::BaselineIdentityPreserved,
                candidate,
                blocked_capability,
                true,
                true,
            )
        }
        Some(SpatialArbitrationCandidate::SnapFlush) => SpatialIdentityContinuityAssessment::new(
            SpatialIdentityContinuityClass::AnchorContinuityPreserved,
            SpatialIdentityContinuityExplanationClass::RelationalAnchorContinuity,
            candidate,
            blocked_capability,
            true,
            true,
        ),
        Some(
            SpatialArbitrationCandidate::AttachRelationally
            | SpatialArbitrationCandidate::NestInside,
        ) => SpatialIdentityContinuityAssessment::new(
            SpatialIdentityContinuityClass::IdentityReinterpreted,
            SpatialIdentityContinuityExplanationClass::RelationalIdentityReinterpreted,
            candidate,
            blocked_capability,
            true,
            false,
        ),
        Some(
            SpatialArbitrationCandidate::SubtractCandidate
            | SpatialArbitrationCandidate::CutOpeningCandidate,
        ) => SpatialIdentityContinuityAssessment::new(
            SpatialIdentityContinuityClass::IdentitySplit,
            SpatialIdentityContinuityExplanationClass::TopologyIdentitySplit,
            candidate,
            blocked_capability,
            false,
            false,
        ),
        Some(
            SpatialArbitrationCandidate::MergeCandidate
            | SpatialArbitrationCandidate::JoinCandidate,
        ) => SpatialIdentityContinuityAssessment::new(
            SpatialIdentityContinuityClass::IdentityMerged,
            SpatialIdentityContinuityExplanationClass::TopologyIdentityMerged,
            candidate,
            blocked_capability,
            false,
            false,
        ),
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
