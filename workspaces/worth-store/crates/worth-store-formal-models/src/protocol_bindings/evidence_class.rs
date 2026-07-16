#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerEvidenceClass {
    DurableAuthoritativeReceipt,
    ReopenedObservedReceipt,
    EphemeralDiagnosticTrace,
    ForbiddenAuthoritySubstitute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerCrashSurvivalPosture {
    DurableAcrossProcessLoss,
    ReconstructedAfterReopen,
    LostWithProcess,
    ForbiddenAsProtocolEvidence,
}

impl OwnerEvidenceClass {
    pub const fn survives_crash_as_protocol_evidence(self) -> bool {
        matches!(self, Self::DurableAuthoritativeReceipt)
    }

    pub const fn crash_survival_posture(self) -> OwnerCrashSurvivalPosture {
        match self {
            Self::DurableAuthoritativeReceipt => {
                OwnerCrashSurvivalPosture::DurableAcrossProcessLoss
            }
            Self::ReopenedObservedReceipt => OwnerCrashSurvivalPosture::ReconstructedAfterReopen,
            Self::EphemeralDiagnosticTrace => OwnerCrashSurvivalPosture::LostWithProcess,
            Self::ForbiddenAuthoritySubstitute => {
                OwnerCrashSurvivalPosture::ForbiddenAsProtocolEvidence
            }
        }
    }
}
