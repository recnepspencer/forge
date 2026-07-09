use super::{
    FreshRuntimeCrashRecoveryEvidence, S4LoweredCrashHarnessEvidence, S4RecoveryCrashSeam,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S4CrashFaultSchedulerEvidence {
    pub(super) seam: S4RecoveryCrashSeam,
    pub(super) lowered_plan_id: String,
    pub(super) storage_boundary_id: String,
    pub(super) observer_transcript_id: String,
    pub(super) proof_oracle_id: String,
    pub(super) seed: u64,
    pub(super) backend_profile: String,
    pub(super) fault_ordinal: u16,
    pub(super) fresh_runtime_recovery: FreshRuntimeCrashRecoveryEvidence,
}

impl S4CrashFaultSchedulerEvidence {
    pub fn from_lowered_crash_plan(
        harness: S4LoweredCrashHarnessEvidence,
        fresh_runtime_recovery: FreshRuntimeCrashRecoveryEvidence,
    ) -> Result<Self, super::super::RecoveryPhysicsCloseoutDenial> {
        Ok(Self {
            seam: harness.seam,
            lowered_plan_id: harness.lowered_plan_id,
            storage_boundary_id: harness.storage_boundary_id,
            observer_transcript_id: harness.observer_transcript_id,
            proof_oracle_id: harness.proof_oracle_id,
            seed: harness.seed,
            backend_profile: harness.backend_profile,
            fault_ordinal: harness.fault_ordinal,
            fresh_runtime_recovery,
        })
    }

    pub const fn seam(&self) -> S4RecoveryCrashSeam {
        self.seam
    }

    pub fn lowered_plan_id(&self) -> &str {
        &self.lowered_plan_id
    }

    pub fn storage_boundary_id(&self) -> &str {
        &self.storage_boundary_id
    }

    pub fn observer_transcript_id(&self) -> &str {
        &self.observer_transcript_id
    }

    pub fn proof_oracle_id(&self) -> &str {
        &self.proof_oracle_id
    }

    pub const fn seed(&self) -> u64 {
        self.seed
    }

    pub fn backend_profile(&self) -> &str {
        &self.backend_profile
    }

    pub const fn fault_ordinal(&self) -> u16 {
        self.fault_ordinal
    }

    pub const fn fresh_runtime_recovery(&self) -> &FreshRuntimeCrashRecoveryEvidence {
        &self.fresh_runtime_recovery
    }
}
