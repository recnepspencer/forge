use crate::{
    PhysicalCounterExpectationKind, PhysicalOracleOutcome, PhysicalProofOracleKind,
    PhysicalStoryTranscript, ScenarioDenialBoundary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSubstrateCloseoutStoryRow {
    PhysicalSubstrateStoryTranscript,
    LegacyOverclaimRejected,
}

#[derive(Debug)]
pub struct PhysicalSubstrateCloseoutStoryReport {
    row: PhysicalSubstrateCloseoutStoryRow,
}

impl PhysicalSubstrateCloseoutStoryReport {
    pub fn from_transcript(
        row: PhysicalSubstrateCloseoutStoryRow,
        transcript: &PhysicalStoryTranscript,
    ) -> Result<Self, PhysicalSubstrateCloseoutStoryDenial> {
        match row {
            PhysicalSubstrateCloseoutStoryRow::PhysicalSubstrateStoryTranscript => {
                admit_physical_substrate_story(transcript)?
            }
            PhysicalSubstrateCloseoutStoryRow::LegacyOverclaimRejected => {
                admit_legacy_overclaim_rejection(transcript)?
            }
        }
        Ok(Self { row })
    }

    pub const fn row(&self) -> PhysicalSubstrateCloseoutStoryRow {
        self.row
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSubstrateCloseoutStoryDenial {
    MissingStoryEvidence,
    MissingLegacyOverclaimDenial,
    MissingLegacyOverclaimCounter,
    MissingSatisfiedOracle(PhysicalProofOracleKind),
}

fn admit_physical_substrate_story(
    transcript: &PhysicalStoryTranscript,
) -> Result<(), PhysicalSubstrateCloseoutStoryDenial> {
    if transcript.counter_trace().observed_counters().is_empty()
        || transcript.shortcut_trace().forbidden_shortcuts().is_empty()
    {
        return Err(PhysicalSubstrateCloseoutStoryDenial::MissingStoryEvidence);
    }
    admit_satisfied_oracle(transcript, PhysicalProofOracleKind::BoundedPhysicalLocate)
}

fn admit_legacy_overclaim_rejection(
    transcript: &PhysicalStoryTranscript,
) -> Result<(), PhysicalSubstrateCloseoutStoryDenial> {
    if !transcript
        .denial_trace()
        .observed_denials()
        .contains(&ScenarioDenialBoundary::LegacyPlatformClaim)
    {
        return Err(PhysicalSubstrateCloseoutStoryDenial::MissingLegacyOverclaimDenial);
    }
    if !transcript
        .counter_trace()
        .is_expected(PhysicalCounterExpectationKind::LegacyPlatformClaimRejections)
    {
        return Err(PhysicalSubstrateCloseoutStoryDenial::MissingLegacyOverclaimCounter);
    }
    admit_satisfied_oracle(
        transcript,
        PhysicalProofOracleKind::ForbiddenLegacyPlatformClaim,
    )
}

fn admit_satisfied_oracle(
    transcript: &PhysicalStoryTranscript,
    required: PhysicalProofOracleKind,
) -> Result<(), PhysicalSubstrateCloseoutStoryDenial> {
    if transcript.judgments().iter().any(|judgment| {
        judgment.oracle() == required && judgment.outcome() == PhysicalOracleOutcome::Satisfied
    }) {
        Ok(())
    } else {
        Err(PhysicalSubstrateCloseoutStoryDenial::MissingSatisfiedOracle(required))
    }
}
