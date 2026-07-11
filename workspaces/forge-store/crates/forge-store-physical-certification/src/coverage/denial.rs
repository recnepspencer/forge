use super::{CoverageSurfaceKind, HarnessMaturityLevel, HarnessSubsystem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverageGapDenial {
    MissingRegistrationEvidence {
        surface: CoverageSurfaceKind,
    },
    MissingPlanBeforeDependentSurface {
        surface: CoverageSurfaceKind,
    },
    DuplicateRegistrationEvidence {
        surface: CoverageSurfaceKind,
    },
    PlanScenarioIdentityMismatch,
    MutationPlanIdentityMismatch,
    PlanScheduleIdentityMismatch,
    DriverContractPlanMismatch,
    CounterReceiptPlanMismatch,
    TranscriptPlanMismatch,
    EmptyDriverRegistration,
    EmptyActorRegistration,
    EmptyOracleVerdictRegistration,
    EmptyCounterReceiptRegistration,
    MissingMutationResult,
    MissingRequiredOracleVerdict,
    UnsatisfiedOracleVerdict,
    ManualCoverageProseDenied,
    EditedMatrixRowDenied,
    UncheckedMaturityClaimDenied,
    SmokeOnlyMaturityDenied {
        subsystem: HarnessSubsystem,
        actual: HarnessMaturityLevel,
    },
    MissingPhysicalIsolationCorrectnessNonClaim,
    WrongSequenceMaturityEvidence,
    UnsupportedProfileMaturityEvidence,
}

pub fn reject_manual_coverage_prose() -> Result<(), CoverageGapDenial> {
    Err(CoverageGapDenial::ManualCoverageProseDenied)
}

pub fn reject_edited_matrix_row() -> Result<(), CoverageGapDenial> {
    Err(CoverageGapDenial::EditedMatrixRowDenied)
}

pub fn reject_unchecked_maturity_claim() -> Result<(), CoverageGapDenial> {
    Err(CoverageGapDenial::UncheckedMaturityClaimDenied)
}
