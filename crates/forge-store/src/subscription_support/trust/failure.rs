use super::drift::SupportTrustDriftReport;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustFailureKind {
    SupportTrustFamilyMismatch,
    SupportTrustRoleMismatch,
    SupportTrustBasisMismatch,
    SupportTrustCursorCheckpointMismatch,
    SupportTrustCompatibilityMismatch,
    SupportTrustResumeClassificationMismatch,
    SupportTrustOperationalVerdictMismatch,
    SupportTrustTranslationMismatch,
    SupportTrustPortabilityMismatch,
    SupportTrustEquivalenceMissing,
    SupportTrustEpochExpired,
    SupportTrustCoverageMissing,
    SupportTrustAccessStructureDebt,
    SupportTrustPayloadBudgetExceeded,
    SupportTrustForbiddenExactOverclaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustRecoveryPosture {
    RetryWithFresherReceipts,
    RebuildTrustCache,
    RerunCertification,
    WaitForMilestone14OrRoadmap2Evidence,
    UnsupportedByCurrentFamilyCatalog,
    PermanentlyRejectedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustFailure {
    kind: SupportTrustFailureKind,
    recovery_posture: SupportTrustRecoveryPosture,
    message: String,
    drift_report: Option<SupportTrustDriftReport>,
}

impl SupportTrustFailure {
    pub(crate) fn new(
        kind: SupportTrustFailureKind,
        recovery_posture: SupportTrustRecoveryPosture,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            recovery_posture,
            message: message.into(),
            drift_report: None,
        }
    }

    pub(crate) fn new_with_drift_report(
        kind: SupportTrustFailureKind,
        recovery_posture: SupportTrustRecoveryPosture,
        message: impl Into<String>,
        drift_report: SupportTrustDriftReport,
    ) -> Self {
        Self {
            kind,
            recovery_posture,
            message: message.into(),
            drift_report: Some(drift_report),
        }
    }

    pub fn kind(&self) -> SupportTrustFailureKind {
        self.kind
    }

    pub fn recovery_posture(&self) -> SupportTrustRecoveryPosture {
        self.recovery_posture
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn drift_report(&self) -> Option<&SupportTrustDriftReport> {
        self.drift_report.as_ref()
    }
}
