#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryEvidenceScope {
    RuntimePublicSupportMatrixRow,
    RuntimePublicSupportMatrix,
    RuntimePublicApiFamilyContract,
    RuntimePublicApiContract,
    RuntimePublicApiTranscriptEvidence,
    RuntimeHostileCertificationArtifact,
    RuntimeStateSnapshot,
    SessionLabelIdentity,
    BasisAdmissionEvidenceRow,
    PreviewBasisAdmission,
    BranchBasisAdmission,
    PreviewIntentAdmission,
    PreviewIntentReceipt,
    BranchIntentAdmission,
    BranchIntentReceipt,
    IntentDenialEvidence,
    PreviewCloseoutEvidence,
    PreviewPromotionDenialEvidence,
    PreviewExecutionEvidence,
    PreviewPromotionRebinding,
    RuntimePublicApiNamingRow,
    RuntimePublicApiNamingContract,
    GraphCompositionDomainInvariantDenial,
    GraphCompositionInvariantViolation,
    ReadDomainInvariantDenial,
    ReadInvariantViolation,
    ApplicationSupportSectionPosture,
    ApplicationSupportReport,
    ApplicationEvidenceIdentityBoundaryClosure,
    ApplicationStopClassBoundaryClosure,
    ApplicationSessionLabelBoundaryClosure,
    ApplicationIdentityBoundaryClosure,
}

impl ForgeQueryEvidenceScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimePublicSupportMatrixRow => "runtime-public-support-matrix-row",
            Self::RuntimePublicSupportMatrix => "runtime-public-support-matrix",
            Self::RuntimePublicApiFamilyContract => "runtime-public-api-family-contract",
            Self::RuntimePublicApiContract => "runtime-public-api-contract",
            Self::RuntimePublicApiTranscriptEvidence => "runtime-public-api-transcript-evidence",
            Self::RuntimeHostileCertificationArtifact => "runtime-hostile-certification-artifact",
            Self::RuntimeStateSnapshot => "runtime-state-snapshot",
            Self::SessionLabelIdentity => "session-label-identity",
            Self::BasisAdmissionEvidenceRow => "basis-admission-evidence-row",
            Self::PreviewBasisAdmission => "preview-basis-admission",
            Self::BranchBasisAdmission => "branch-basis-admission",
            Self::PreviewIntentAdmission => "preview-intent-admission",
            Self::PreviewIntentReceipt => "preview-intent-receipt",
            Self::BranchIntentAdmission => "branch-intent-admission",
            Self::BranchIntentReceipt => "branch-intent-receipt",
            Self::IntentDenialEvidence => "intent-denial-evidence",
            Self::PreviewCloseoutEvidence => "preview-closeout-evidence",
            Self::PreviewPromotionDenialEvidence => "preview-promotion-denial-evidence",
            Self::PreviewExecutionEvidence => "preview-execution-evidence",
            Self::PreviewPromotionRebinding => "preview-promotion-rebinding",
            Self::RuntimePublicApiNamingRow => "runtime-public-api-naming-row",
            Self::RuntimePublicApiNamingContract => "runtime-public-api-naming-contract",
            Self::GraphCompositionDomainInvariantDenial => {
                "graph-composition-domain-invariant-denial"
            }
            Self::GraphCompositionInvariantViolation => "graph-composition-invariant-violation",
            Self::ReadDomainInvariantDenial => "read-domain-invariant-denial",
            Self::ReadInvariantViolation => "read-invariant-violation",
            Self::ApplicationSupportSectionPosture => "application-support-section-posture",
            Self::ApplicationSupportReport => "application-support-report",
            Self::ApplicationEvidenceIdentityBoundaryClosure => {
                "application-evidence-identity-boundary-closure"
            }
            Self::ApplicationStopClassBoundaryClosure => "application-stop-class-boundary-closure",
            Self::ApplicationSessionLabelBoundaryClosure => {
                "application-session-label-boundary-closure"
            }
            Self::ApplicationIdentityBoundaryClosure => "application-identity-boundary-closure",
        }
    }
}
