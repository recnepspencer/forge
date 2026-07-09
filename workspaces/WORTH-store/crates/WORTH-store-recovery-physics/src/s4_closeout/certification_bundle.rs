use super::{
    RecoveryPhysicsCloseoutEvidence, RecoveryPhysicsCloseoutReport,
    RecoveryPhysicsCloseoutSuiteRequirement, S4RecoveryCrashSeam,
    S5PhysicalIsolationRecoveryReadiness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsCertificationBundle {
    closeout_report: RecoveryPhysicsCloseoutReport,
    evidence: RecoveryPhysicsCloseoutEvidence,
}

impl RecoveryPhysicsCertificationBundle {
    pub(crate) fn new(
        evidence: RecoveryPhysicsCloseoutEvidence,
        work_bound: super::RecoveryWorkBound,
        required_suites: Vec<RecoveryPhysicsCloseoutSuiteRequirement>,
        crash_seams: Vec<S4RecoveryCrashSeam>,
    ) -> Self {
        let closeout_report =
            RecoveryPhysicsCloseoutReport::new(&evidence, work_bound, required_suites, crash_seams);
        Self {
            closeout_report,
            evidence,
        }
    }

    pub const fn closeout_report(&self) -> &RecoveryPhysicsCloseoutReport {
        &self.closeout_report
    }

    pub const fn evidence(&self) -> &RecoveryPhysicsCloseoutEvidence {
        &self.evidence
    }

    pub fn publish_s5_readiness(&self) -> S5PhysicalIsolationRecoveryReadiness {
        S5PhysicalIsolationRecoveryReadiness::from_closeout_bundle(
            &self.closeout_report,
            self.evidence.receipt().execution().clone(),
            self.evidence.source_trace().clone(),
        )
    }
}
