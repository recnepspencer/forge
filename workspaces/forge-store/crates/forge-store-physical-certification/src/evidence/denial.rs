use crate::OracleFamilyKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalEvidenceBundleDenial {
    MissingOracleVerdict,
    MissingTranscriptReplayOracleVerdict,
    OracleFamilyNotRequired,
    RequiredOracleFamilyMissing(OracleFamilyKind),
    OracleFamilyMismatch,
    OracleVerdictPlanMismatch,
    TranscriptReplayVerdictMissingReplayEvidence,
    TranscriptReplayVerdictBasisMismatch,
    LooseLogDenied,
    TerminalJsonDenied,
    SameRunSelfComparisonDenied,
    FoundationalMaterializationIsNotStoreAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalProjectionOnlyEvidenceDenied {
    TerminalJsonProjection,
}

pub fn reject_loose_log_evidence_attempt() -> Result<(), PhysicalEvidenceBundleDenial> {
    Err(PhysicalEvidenceBundleDenial::LooseLogDenied)
}

pub fn reject_terminal_json_evidence_attempt() -> Result<(), TerminalProjectionOnlyEvidenceDenied> {
    Err(TerminalProjectionOnlyEvidenceDenied::TerminalJsonProjection)
}

pub fn reject_same_run_self_comparison_evidence_attempt() -> Result<(), PhysicalEvidenceBundleDenial>
{
    Err(PhysicalEvidenceBundleDenial::SameRunSelfComparisonDenied)
}

pub fn reject_foundational_materialization_as_store_authority(
) -> Result<(), PhysicalEvidenceBundleDenial> {
    Err(PhysicalEvidenceBundleDenial::FoundationalMaterializationIsNotStoreAuthority)
}
