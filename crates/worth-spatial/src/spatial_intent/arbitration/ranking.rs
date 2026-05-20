use super::blocked::SpatialIntentCandidateAvailability;
use super::candidates::SpatialIntentCandidate;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentExplanationClass {
    AuthoredBaseline,
    RelationInferred,
    BlockedFutureCapability,
    UnsafeBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialIntentCandidateRank {
    candidate: SpatialIntentCandidate,
    availability: SpatialIntentCandidateAvailability,
    explanation: SpatialIntentExplanationClass,
    priority: u8,
}

impl SpatialIntentCandidateRank {
    pub fn new(
        candidate: SpatialIntentCandidate,
        availability: SpatialIntentCandidateAvailability,
        explanation: SpatialIntentExplanationClass,
    ) -> Self {
        Self {
            candidate,
            availability,
            explanation,
            priority: candidate.default_priority(),
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
}
