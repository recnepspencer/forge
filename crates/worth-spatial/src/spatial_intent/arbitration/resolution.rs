use super::blocked::{SpatialBlockedCapability, SpatialIntentCandidateAvailability};
use super::candidates::SpatialIntentCandidate;
use super::escalation::{SpatialIntentArbitrationAnalysis, SpatialIntentEscalation};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialChosenIntentAuthority {
    PolicyAutoResolve,
    ExplicitChoice,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialChosenIntentResolution {
    analysis: SpatialIntentArbitrationAnalysis,
    chosen_candidate: SpatialIntentCandidate,
    authority: SpatialChosenIntentAuthority,
}

impl SpatialChosenIntentResolution {
    fn new(
        analysis: SpatialIntentArbitrationAnalysis,
        chosen_candidate: SpatialIntentCandidate,
        authority: SpatialChosenIntentAuthority,
    ) -> Self {
        Self {
            analysis,
            chosen_candidate,
            authority,
        }
    }

    pub fn analysis(&self) -> &SpatialIntentArbitrationAnalysis {
        &self.analysis
    }

    pub fn chosen_candidate(&self) -> SpatialIntentCandidate {
        self.chosen_candidate
    }

    pub fn authority(&self) -> SpatialChosenIntentAuthority {
        self.authority
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentResolutionError {
    ClarificationRequired,
    CandidateSetPreserved,
    BlockedByMissingCapability(SpatialBlockedCapability),
    CandidateNotPresent(SpatialIntentCandidate),
    CandidateBlocked(SpatialIntentCandidate, SpatialBlockedCapability),
}

impl std::fmt::Display for SpatialIntentResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClarificationRequired => write!(f, "clarification required before resolution"),
            Self::CandidateSetPreserved => {
                write!(f, "candidate set preserved without a chosen intent")
            }
            Self::BlockedByMissingCapability(capability) => {
                write!(f, "blocked by missing capability: {capability:?}")
            }
            Self::CandidateNotPresent(candidate) => {
                write!(
                    f,
                    "candidate not present in arbitration analysis: {candidate:?}"
                )
            }
            Self::CandidateBlocked(candidate, capability) => write!(
                f,
                "candidate {candidate:?} is blocked by missing capability {capability:?}"
            ),
        }
    }
}

impl std::error::Error for SpatialIntentResolutionError {}

pub fn resolve_spatial_intent_conflict_by_policy(
    analysis: SpatialIntentArbitrationAnalysis,
) -> Result<SpatialChosenIntentResolution, SpatialIntentResolutionError> {
    match analysis.escalation() {
        SpatialIntentEscalation::AutoResolve(candidate) => Ok(SpatialChosenIntentResolution::new(
            analysis,
            candidate,
            SpatialChosenIntentAuthority::PolicyAutoResolve,
        )),
        SpatialIntentEscalation::PreserveCandidates => {
            Err(SpatialIntentResolutionError::CandidateSetPreserved)
        }
        SpatialIntentEscalation::AskForClarification => {
            Err(SpatialIntentResolutionError::ClarificationRequired)
        }
        SpatialIntentEscalation::BlockedByMissingCapability(capability) => Err(
            SpatialIntentResolutionError::BlockedByMissingCapability(capability),
        ),
    }
}

pub fn resolve_spatial_intent_conflict_by_choice(
    analysis: SpatialIntentArbitrationAnalysis,
    chosen_candidate: SpatialIntentCandidate,
) -> Result<SpatialChosenIntentResolution, SpatialIntentResolutionError> {
    let candidate = analysis
        .candidates()
        .iter()
        .find(|candidate| candidate.candidate() == chosen_candidate)
        .copied()
        .ok_or(SpatialIntentResolutionError::CandidateNotPresent(
            chosen_candidate,
        ))?;
    match candidate.availability() {
        SpatialIntentCandidateAvailability::Available => Ok(SpatialChosenIntentResolution::new(
            analysis,
            chosen_candidate,
            SpatialChosenIntentAuthority::ExplicitChoice,
        )),
        SpatialIntentCandidateAvailability::Blocked(capability) => Err(
            SpatialIntentResolutionError::CandidateBlocked(chosen_candidate, capability),
        ),
    }
}
