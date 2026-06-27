#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDiagnosticsProjectionDenial {
    reason: WorthUiDiagnosticsProjectionDenialReason,
    active_plan_digest: u64,
    evidence_digest: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiDiagnosticsProjectionDenialReason {
    MissingRuntimeDiagnosticReport,
    RuntimeReportDigestMismatch,
    PlanInspectionDigestMismatch,
    FreeformQueryStatusRow,
    HookAttemptedIdentityRewrite,
}

impl WorthUiDiagnosticsProjectionDenial {
    pub(crate) fn new(
        reason: WorthUiDiagnosticsProjectionDenialReason,
        active_plan_digest: u64,
        evidence_digest: Option<u64>,
    ) -> Self {
        Self {
            reason,
            active_plan_digest,
            evidence_digest,
        }
    }

    pub fn reason(&self) -> WorthUiDiagnosticsProjectionDenialReason {
        self.reason
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }

    pub fn evidence_digest(&self) -> Option<u64> {
        self.evidence_digest
    }
}
