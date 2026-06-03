use worth_spatial::facade::arbitration::{
    SpatialAuthoredActKind, SpatialBlockedCapability, SpatialIntentArbitrationAnalysis,
    SpatialIntentCandidate, SpatialIntentCandidateAvailability, SpatialIntentCandidateRank,
    SpatialIntentConflictClass, SpatialIntentEscalation, SpatialIntentExplanationClass,
    SpatialObservedRelationFact,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveIntentClarificationCandidate {
    candidate: SpatialIntentCandidate,
    availability: SpatialIntentCandidateAvailability,
    explanation: SpatialIntentExplanationClass,
    priority: u8,
}

impl PrimitiveIntentClarificationCandidate {
    fn new(rank: SpatialIntentCandidateRank) -> Self {
        Self {
            candidate: rank.candidate(),
            availability: rank.availability(),
            explanation: rank.explanation(),
            priority: rank.priority(),
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

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveIntentClarificationRequest {
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: Vec<SpatialObservedRelationFact>,
    candidates: Vec<PrimitiveIntentClarificationCandidate>,
    conflict_class: SpatialIntentConflictClass,
    escalation: SpatialIntentEscalation,
}

impl PrimitiveIntentClarificationRequest {
    fn new(analysis: SpatialIntentArbitrationAnalysis) -> Self {
        let candidates = analysis
            .candidates()
            .iter()
            .copied()
            .map(PrimitiveIntentClarificationCandidate::new)
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

    pub fn candidates(&self) -> &[PrimitiveIntentClarificationCandidate] {
        &self.candidates
    }

    pub fn conflict_class(&self) -> SpatialIntentConflictClass {
        self.conflict_class
    }

    pub fn escalation(&self) -> SpatialIntentEscalation {
        self.escalation
    }

    pub fn blocked_capability(&self) -> Option<SpatialBlockedCapability> {
        match self.escalation {
            SpatialIntentEscalation::BlockedByMissingCapability(blocked) => Some(blocked),
            SpatialIntentEscalation::AutoResolve(_)
            | SpatialIntentEscalation::PreserveCandidates
            | SpatialIntentEscalation::AskForClarification => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveIntentClarificationRequestError {
    NoClarificationBoundary(SpatialIntentEscalation),
}

impl std::fmt::Display for PrimitiveIntentClarificationRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoClarificationBoundary(escalation) => write!(
                f,
                "analysis does not require preserved ambiguity or human clarification: {escalation:?}"
            ),
        }
    }
}

impl std::error::Error for PrimitiveIntentClarificationRequestError {}

pub fn prepare_primitive_intent_clarification_request(
    analysis: SpatialIntentArbitrationAnalysis,
) -> Result<PrimitiveIntentClarificationRequest, PrimitiveIntentClarificationRequestError> {
    match analysis.escalation() {
        SpatialIntentEscalation::AutoResolve(candidate) => Err(
            PrimitiveIntentClarificationRequestError::NoClarificationBoundary(
                SpatialIntentEscalation::AutoResolve(candidate),
            ),
        ),
        SpatialIntentEscalation::PreserveCandidates
        | SpatialIntentEscalation::AskForClarification
        | SpatialIntentEscalation::BlockedByMissingCapability(_) => {
            Ok(PrimitiveIntentClarificationRequest::new(analysis))
        }
    }
}
