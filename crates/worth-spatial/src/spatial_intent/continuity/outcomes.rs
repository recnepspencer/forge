use crate::spatial_intent::arbitration::{SpatialBlockedCapability, SpatialIntentCandidate};

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
    pub fn new(
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
