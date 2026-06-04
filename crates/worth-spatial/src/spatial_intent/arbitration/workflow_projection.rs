use crate::spatial_intent::policy::SpatialPreviewRichness;

use super::declared_analysis::{SpatialIntentArbitrationDeclaration, SpatialIntentEscalation};
use super::resolution::SpatialChosenIntentResolution;
use super::{SpatialBlockedCapability, SpatialIntentCandidate};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentPreviewCommitDisposition {
    WouldAutoResolve(SpatialIntentCandidate),
    WouldPreserveCandidates,
    WouldRequireClarification,
    WouldBlockOnCapability(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentPreviewWarning {
    ClarificationRequired,
    PreservedCandidateSet,
    BlockedFutureCandidate(SpatialBlockedCapability),
    ProfileDrivenAutoResolve(SpatialIntentCandidate),
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
    candidate: Option<SpatialIntentCandidate>,
    blocked_capability: Option<SpatialBlockedCapability>,
    preserves_subject_identity: bool,
    preserves_anchor_identity: bool,
}

impl SpatialIdentityContinuityAssessment {
    fn new(
        continuity_class: SpatialIdentityContinuityClass,
        explanation_class: SpatialIdentityContinuityExplanationClass,
        candidate: Option<SpatialIntentCandidate>,
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

    pub fn candidate(&self) -> Option<SpatialIntentCandidate> {
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

impl SpatialIntentArbitrationDeclaration {
    pub fn preview_richness(&self) -> SpatialPreviewRichness {
        self.policy_profile().preview_richness()
    }

    pub fn preview_commit_disposition(&self) -> SpatialIntentPreviewCommitDisposition {
        match self.escalation() {
            SpatialIntentEscalation::AutoResolve(candidate) => {
                SpatialIntentPreviewCommitDisposition::WouldAutoResolve(candidate)
            }
            SpatialIntentEscalation::PreserveCandidates => {
                SpatialIntentPreviewCommitDisposition::WouldPreserveCandidates
            }
            SpatialIntentEscalation::AskForClarification => {
                SpatialIntentPreviewCommitDisposition::WouldRequireClarification
            }
            SpatialIntentEscalation::BlockedByMissingCapability(capability) => {
                SpatialIntentPreviewCommitDisposition::WouldBlockOnCapability(capability)
            }
        }
    }

    pub fn preview_warnings(&self) -> Vec<SpatialIntentPreviewWarning> {
        let mut warnings = Vec::new();
        match self.preview_commit_disposition() {
            SpatialIntentPreviewCommitDisposition::WouldRequireClarification => {
                warnings.push(SpatialIntentPreviewWarning::ClarificationRequired)
            }
            SpatialIntentPreviewCommitDisposition::WouldPreserveCandidates => {
                warnings.push(SpatialIntentPreviewWarning::PreservedCandidateSet)
            }
            SpatialIntentPreviewCommitDisposition::WouldBlockOnCapability(capability) => warnings
                .push(SpatialIntentPreviewWarning::BlockedFutureCandidate(
                    capability,
                )),
            SpatialIntentPreviewCommitDisposition::WouldAutoResolve(candidate)
                if candidate != SpatialIntentCandidate::baseline_for(self.authored_act()) =>
            {
                warnings.push(SpatialIntentPreviewWarning::ProfileDrivenAutoResolve(
                    candidate,
                ))
            }
            SpatialIntentPreviewCommitDisposition::WouldAutoResolve(_) => {}
        }
        if self.preview_richness() == SpatialPreviewRichness::HighFidelity {
            warnings.push(SpatialIntentPreviewWarning::HighFidelityPreview);
        }
        warnings
    }

    pub fn identity_continuity_assessment(&self) -> SpatialIdentityContinuityAssessment {
        match self.escalation() {
            SpatialIntentEscalation::AutoResolve(candidate) => {
                assessment_for_candidate(Some(candidate), None)
            }
            SpatialIntentEscalation::PreserveCandidates
            | SpatialIntentEscalation::AskForClarification => {
                SpatialIdentityContinuityAssessment::new(
                    SpatialIdentityContinuityClass::IdentityBlockedPendingChoice,
                    SpatialIdentityContinuityExplanationClass::CandidateSetPendingChoice,
                    None,
                    None,
                    false,
                    false,
                )
            }
            SpatialIntentEscalation::BlockedByMissingCapability(capability) => {
                blocked_pending_choice(capability)
            }
        }
    }
}

impl SpatialChosenIntentResolution {
    pub fn identity_continuity_assessment(&self) -> SpatialIdentityContinuityAssessment {
        assessment_for_candidate(Some(self.chosen_candidate()), None)
    }
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
