use worth_store_offline_verifier::RecoveryObserverReport;
use worth_store_recovery_runtime::{RecoveryReportEnvelope, RecoveryReportOutcome};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshRuntimeCrashRecoveryEvidenceDenial {
    RuntimeDidNotRecover,
    ObserverSawNoArtifacts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshRuntimeCrashRecoveryEvidence {
    runtime_report: RecoveryReportEnvelope,
    observer_report: RecoveryObserverReport,
}

impl FreshRuntimeCrashRecoveryEvidence {
    pub fn from_reports(
        runtime_report: RecoveryReportEnvelope,
        observer_report: RecoveryObserverReport,
    ) -> Result<Self, FreshRuntimeCrashRecoveryEvidenceDenial> {
        if runtime_report.outcome() != RecoveryReportOutcome::Recovered {
            return Err(FreshRuntimeCrashRecoveryEvidenceDenial::RuntimeDidNotRecover);
        }
        if observer_report.artifact_count() == 0 {
            return Err(FreshRuntimeCrashRecoveryEvidenceDenial::ObserverSawNoArtifacts);
        }
        Ok(Self {
            runtime_report,
            observer_report,
        })
    }

    pub const fn runtime_report(&self) -> &RecoveryReportEnvelope {
        &self.runtime_report
    }

    pub const fn observer_report(&self) -> RecoveryObserverReport {
        self.observer_report
    }
}
