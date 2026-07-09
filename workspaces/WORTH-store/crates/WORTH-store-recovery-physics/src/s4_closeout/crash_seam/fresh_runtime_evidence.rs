use crate::{BoundedRecoveryReceipt, RecoveryRuntimeClassification, RuntimeRecoveryReport};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshRuntimeCrashRecoveryEvidence {
    crash_receipt: BoundedRecoveryReceipt,
    runtime_report: RuntimeRecoveryReport,
}

impl FreshRuntimeCrashRecoveryEvidence {
    pub fn from_runtime_report(
        crash_receipt: BoundedRecoveryReceipt,
        runtime_report: RuntimeRecoveryReport,
    ) -> Result<Self, super::super::RecoveryPhysicsCloseoutDenial> {
        if runtime_report.classification() != RecoveryRuntimeClassification::Recovered
            || runtime_report.fresh_runtime_constructions() == 0
            || runtime_report.runtime_cache_reads() != 0
        {
            return Err(super::super::RecoveryPhysicsCloseoutDenial::SameProcessCrashObservation);
        }
        if runtime_report.recovered_state() != crash_receipt.execution().recovered_state()
            || runtime_report.counters() != crash_receipt.counters()
        {
            return Err(
                super::super::RecoveryPhysicsCloseoutDenial::FreshRuntimeCrashEvidenceMismatch,
            );
        }
        Ok(Self {
            crash_receipt,
            runtime_report,
        })
    }

    pub const fn crash_receipt(&self) -> &BoundedRecoveryReceipt {
        &self.crash_receipt
    }

    pub const fn runtime_report(&self) -> &RuntimeRecoveryReport {
        &self.runtime_report
    }
}
