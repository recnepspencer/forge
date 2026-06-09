use super::{
    SpatialArbitrationAnalysis, SpatialArbitrationCandidate,
    SpatialArbitrationCandidateAvailability, SpatialArbitrationCandidateRank,
    SpatialArbitrationConflictClass, SpatialArbitrationEscalation,
    SpatialArbitrationExplanationClass, SpatialAuthoredActKind, SpatialBlockedCapability,
    SpatialObservedRelationFact,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialArbitrationClarificationCandidate {
    candidate: SpatialArbitrationCandidate,
    availability: SpatialArbitrationCandidateAvailability,
    explanation: SpatialArbitrationExplanationClass,
    priority: u8,
}

impl SpatialArbitrationClarificationCandidate {
    fn new(rank: SpatialArbitrationCandidateRank) -> Self {
        Self {
            candidate: rank.candidate(),
            availability: rank.availability(),
            explanation: rank.explanation(),
            priority: rank.priority(),
        }
    }

    pub fn candidate(&self) -> SpatialArbitrationCandidate {
        self.candidate
    }

    pub fn availability(&self) -> SpatialArbitrationCandidateAvailability {
        self.availability
    }

    pub fn explanation(&self) -> SpatialArbitrationExplanationClass {
        self.explanation
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialArbitrationClarificationRequest {
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: Vec<SpatialObservedRelationFact>,
    candidates: Vec<SpatialArbitrationClarificationCandidate>,
    conflict_class: SpatialArbitrationConflictClass,
    escalation: SpatialArbitrationEscalation,
}

impl SpatialArbitrationClarificationRequest {
    fn new(analysis: SpatialArbitrationAnalysis) -> Self {
        let candidates = analysis
            .candidates()
            .iter()
            .copied()
            .map(SpatialArbitrationClarificationCandidate::new)
            .collect::<Vec<_>>();
        Self {
            authored_act: analysis.authored_act(),
            observed_relation_facts: analysis.observed_relation_facts().to_vec(),
            candidates,
            conflict_class: analysis.conflict_class(),
            escalation: analysis.escalation(),
        }
    }

    pub fn authored_act(&self) -> SpatialAuthoredActKind {
        self.authored_act
    }

    pub fn observed_relation_facts(&self) -> &[SpatialObservedRelationFact] {
        &self.observed_relation_facts
    }

    pub fn candidates(&self) -> &[SpatialArbitrationClarificationCandidate] {
        &self.candidates
    }

    pub fn conflict_class(&self) -> SpatialArbitrationConflictClass {
        self.conflict_class
    }

    pub fn escalation(&self) -> SpatialArbitrationEscalation {
        self.escalation
    }

    pub fn blocked_capability(&self) -> Option<SpatialBlockedCapability> {
        match self.escalation {
            SpatialArbitrationEscalation::BlockedByMissingCapability(blocked) => Some(blocked),
            SpatialArbitrationEscalation::AutoResolve(_)
            | SpatialArbitrationEscalation::PreserveCandidates
            | SpatialArbitrationEscalation::AskForClarification => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationClarificationRequestError {
    NoClarificationBoundary(SpatialArbitrationEscalation),
}

impl std::fmt::Display for SpatialArbitrationClarificationRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoClarificationBoundary(escalation) => write!(
                f,
                "analysis does not require preserved ambiguity or human clarification: {escalation:?}"
            ),
        }
    }
}

impl std::error::Error for SpatialArbitrationClarificationRequestError {}

pub(crate) fn prepare_spatial_arbitration_clarification_request(
    analysis: SpatialArbitrationAnalysis,
) -> Result<SpatialArbitrationClarificationRequest, SpatialArbitrationClarificationRequestError> {
    match analysis.escalation() {
        SpatialArbitrationEscalation::AutoResolve(candidate) => Err(
            SpatialArbitrationClarificationRequestError::NoClarificationBoundary(
                SpatialArbitrationEscalation::AutoResolve(candidate),
            ),
        ),
        SpatialArbitrationEscalation::PreserveCandidates
        | SpatialArbitrationEscalation::AskForClarification
        | SpatialArbitrationEscalation::BlockedByMissingCapability(_) => {
            Ok(SpatialArbitrationClarificationRequest::new(analysis))
        }
    }
}
