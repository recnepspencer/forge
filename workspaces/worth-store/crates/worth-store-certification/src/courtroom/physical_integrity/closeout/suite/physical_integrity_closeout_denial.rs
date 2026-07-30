use crate::{
    CorruptionLocalizationBoundary, IntegrityCloseoutEvidenceFamily, IntegrityCloseoutModuleKind,
    PhysicalIntegrityAcceptanceSuite, SyntheticCloseoutShortcutAttempt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalIntegrityCloseoutDenial {
    MissingAcceptanceSuite(PhysicalIntegrityAcceptanceSuite),
    DuplicateAcceptanceSuite(PhysicalIntegrityAcceptanceSuite),
    MissingHarnessTranscript(PhysicalIntegrityAcceptanceSuite),
    MissingHarnessFamily(PhysicalIntegrityAcceptanceSuite),
    WrongHarnessLane(PhysicalIntegrityAcceptanceSuite),
    MissingEvidenceFamily(IntegrityCloseoutEvidenceFamily),
    MissingCorruptionLocalization,
    MissingBoundaryDenial(IntegrityCloseoutDenialBoundary),
    UnexecutedCorruptionLocalization(CorruptionLocalizationBoundary),
    UnexecutedBoundaryDenial(IntegrityCloseoutDenialBoundary),
    MissingSyntheticRejection(SyntheticCloseoutShortcutAttempt),
    SyntheticRejectionTranscriptMismatch(SyntheticCloseoutShortcutAttempt),
    MissingRecoveryHandoffPayload,
    RecoveryHandoffEvidenceMismatch,
    MissingLineCapComposition,
    MissingLineCapModule(IntegrityCloseoutModuleKind),
    LineCapModuleOverBudget(IntegrityCloseoutModuleKind),
    CollapsedCloseoutResponsibility(IntegrityCloseoutModuleKind),
    MismatchedHarnessSuite(PhysicalIntegrityAcceptanceSuite),
    HarnessExecutionFailed(PhysicalIntegrityAcceptanceSuite),
    MissingExecutedSuiteOutput(PhysicalIntegrityAcceptanceSuite),
    MissingOwnedCloseoutFile,
    IntegrityOwnedCloseoutFileOverBudget(String),
    OmittedOwnedCloseoutFile(String),
    RecoveryHandoffContainsRawBytes,
    RecoveryHandoffClaimsRecoveryAuthority,
    IntegrityReadinessClaimsRecoverySequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntegrityCloseoutDenialBoundary {
    ForgedChecksum,
    DigestAsChecksum,
    ChecksumAsAuthenticity,
    StoreAuthorityMismatch,
    VerificationAllocationCoverage,
    CopiedQuarantineRecord,
    OverBudgetScrubPlan,
}

impl IntegrityCloseoutDenialBoundary {
    pub const ALL: [Self; 7] = [
        Self::ForgedChecksum,
        Self::DigestAsChecksum,
        Self::ChecksumAsAuthenticity,
        Self::StoreAuthorityMismatch,
        Self::VerificationAllocationCoverage,
        Self::CopiedQuarantineRecord,
        Self::OverBudgetScrubPlan,
    ];
}
