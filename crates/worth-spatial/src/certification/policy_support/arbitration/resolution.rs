use super::candidates::SpatialArbitrationCandidate;
use super::capabilities::{SpatialArbitrationCandidateAvailability, SpatialBlockedCapability};
use super::declaration::{SpatialArbitrationAnalysis, SpatialArbitrationEscalation};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialChosenArbitrationAuthority {
    PolicyAutoResolve,
    ExplicitChoice,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialChosenArbitrationResolution {
    analysis: SpatialArbitrationAnalysis,
    chosen_candidate: SpatialArbitrationCandidate,
    authority: SpatialChosenArbitrationAuthority,
}

impl SpatialChosenArbitrationResolution {
    fn new(
        analysis: SpatialArbitrationAnalysis,
        chosen_candidate: SpatialArbitrationCandidate,
        authority: SpatialChosenArbitrationAuthority,
    ) -> Self {
        Self {
            analysis,
            chosen_candidate,
            authority,
        }
    }

    pub fn analysis(&self) -> &SpatialArbitrationAnalysis {
        &self.analysis
    }

    pub fn chosen_candidate(&self) -> SpatialArbitrationCandidate {
        self.chosen_candidate
    }

    pub fn authority(&self) -> SpatialChosenArbitrationAuthority {
        self.authority
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationResolutionError {
    ClarificationRequired,
    CandidateSetPreserved,
    BlockedByMissingCapability(SpatialBlockedCapability),
    CandidateNotPresent(SpatialArbitrationCandidate),
    CandidateBlocked(SpatialArbitrationCandidate, SpatialBlockedCapability),
}

impl std::fmt::Display for SpatialArbitrationResolutionError {
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

impl std::error::Error for SpatialArbitrationResolutionError {}

pub(crate) fn resolve_spatial_arbitration_conflict_by_policy(
    analysis: SpatialArbitrationAnalysis,
) -> Result<SpatialChosenArbitrationResolution, SpatialArbitrationResolutionError> {
    match analysis.escalation() {
        SpatialArbitrationEscalation::AutoResolve(candidate) => {
            Ok(SpatialChosenArbitrationResolution::new(
                analysis,
                candidate,
                SpatialChosenArbitrationAuthority::PolicyAutoResolve,
            ))
        }
        SpatialArbitrationEscalation::PreserveCandidates => {
            Err(SpatialArbitrationResolutionError::CandidateSetPreserved)
        }
        SpatialArbitrationEscalation::AskForClarification => {
            Err(SpatialArbitrationResolutionError::ClarificationRequired)
        }
        SpatialArbitrationEscalation::BlockedByMissingCapability(capability) => {
            Err(SpatialArbitrationResolutionError::BlockedByMissingCapability(capability))
        }
    }
}

pub(crate) fn resolve_spatial_arbitration_conflict_by_choice(
    analysis: SpatialArbitrationAnalysis,
    chosen_candidate: SpatialArbitrationCandidate,
) -> Result<SpatialChosenArbitrationResolution, SpatialArbitrationResolutionError> {
    let candidate = analysis
        .candidates()
        .iter()
        .find(|candidate| candidate.candidate() == chosen_candidate)
        .copied()
        .ok_or(SpatialArbitrationResolutionError::CandidateNotPresent(
            chosen_candidate,
        ))?;
    match candidate.availability() {
        SpatialArbitrationCandidateAvailability::Available => {
            Ok(SpatialChosenArbitrationResolution::new(
                analysis,
                chosen_candidate,
                SpatialChosenArbitrationAuthority::ExplicitChoice,
            ))
        }
        SpatialArbitrationCandidateAvailability::Blocked(capability) => Err(
            SpatialArbitrationResolutionError::CandidateBlocked(chosen_candidate, capability),
        ),
    }
}
