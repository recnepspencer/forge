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
    PreviewBasisAdmission,
    BranchBasisAdmission,
    PreviewIntentAdmission,
    PreviewIntentReceipt,
    BranchIntentAdmission,
    BranchIntentReceipt,
    IntentDenialEvidence,
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
            Self::PreviewBasisAdmission => "preview-basis-admission",
            Self::BranchBasisAdmission => "branch-basis-admission",
            Self::PreviewIntentAdmission => "preview-intent-admission",
            Self::PreviewIntentReceipt => "preview-intent-receipt",
            Self::BranchIntentAdmission => "branch-intent-admission",
            Self::BranchIntentReceipt => "branch-intent-receipt",
            Self::IntentDenialEvidence => "intent-denial-evidence",
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
