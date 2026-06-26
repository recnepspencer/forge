use crate::{
    S3AcceptanceSuiteKind, S3CloseoutEvidenceFamily, S3CloseoutModuleKind,
    S3CorruptionLocalizationBoundary, SyntheticCloseoutShortcutAttempt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalIntegrityCloseoutDenial {
    MissingAcceptanceSuite(S3AcceptanceSuiteKind),
    DuplicateAcceptanceSuite(S3AcceptanceSuiteKind),
    MissingHarnessTranscript(S3AcceptanceSuiteKind),
    MissingHarnessFamily(S3AcceptanceSuiteKind),
    WrongHarnessLane(S3AcceptanceSuiteKind),
    MissingEvidenceFamily(S3CloseoutEvidenceFamily),
    MissingCorruptionLocalization,
    MissingBoundaryDenial(S3CloseoutDenialBoundary),
    UnexecutedCorruptionLocalization(S3CorruptionLocalizationBoundary),
    UnexecutedBoundaryDenial(S3CloseoutDenialBoundary),
    MissingSyntheticRejection(SyntheticCloseoutShortcutAttempt),
    SyntheticRejectionTranscriptMismatch(SyntheticCloseoutShortcutAttempt),
    MissingS4HandoffPayload,
    S4HandoffEvidenceMismatch,
    MissingLineCapComposition,
    MissingLineCapModule(S3CloseoutModuleKind),
    LineCapModuleOverBudget(S3CloseoutModuleKind),
    CollapsedCloseoutResponsibility(S3CloseoutModuleKind),
    MismatchedHarnessSuite(S3AcceptanceSuiteKind),
    HarnessExecutionFailed(S3AcceptanceSuiteKind),
    MissingExecutedSuiteOutput(S3AcceptanceSuiteKind),
    MissingS3OwnedCloseoutFile,
    S3OwnedCloseoutFileOverBudget(String),
    OmittedS3OwnedCloseoutFile(String),
    S4HandoffContainsRawBytes,
    S4HandoffClaimsRecovery,
    S3ReadinessClaimsLaterSequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S3CloseoutDenialBoundary {
    ForgedChecksum,
    DigestAsChecksum,
    ChecksumAsAuthenticity,
    RawByteEntry,
    CopiedQuarantineRecord,
    OverBudgetScrubPlan,
}

impl S3CloseoutDenialBoundary {
    pub const ALL: [Self; 6] = [
        Self::ForgedChecksum,
        Self::DigestAsChecksum,
        Self::ChecksumAsAuthenticity,
        Self::RawByteEntry,
        Self::CopiedQuarantineRecord,
        Self::OverBudgetScrubPlan,
    ];
}
