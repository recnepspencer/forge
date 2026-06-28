use forge_store_recovery_physics::{
    BoundedRecoveryReceipt, RecoveryCounterSnapshot, S4RecoveryCrashSeam,
};

use crate::{FaultSchedulerDriver, StorageBoundaryInterposerDriver};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedS4CrashHarnessTranscript {
    seam: S4RecoveryCrashSeam,
    lowered_plan_id: String,
    storage_boundary_id: String,
    observer_transcript_id: String,
    proof_oracle_id: String,
    seed: u64,
    backend_profile: &'static str,
    fault_ordinal: u16,
    recovered_root: String,
    source_decision_digest: String,
    counters: RecoveryCounterSnapshot,
}

impl ExecutedS4CrashHarnessTranscript {
    pub fn execute(
        seam: S4RecoveryCrashSeam,
        lowered_plan_id: impl Into<String>,
        receipt: &BoundedRecoveryReceipt,
    ) -> Result<Self, ExecutedS4CrashHarnessDenial> {
        let scheduler = FaultSchedulerDriver::deterministic(13);
        let fault = scheduler.schedule_fault(seam.as_str());
        let interposer =
            StorageBoundaryInterposerDriver::production_like("strict-posix-fsync-dir-fsync");
        let boundary = interposer.lower_boundary_event(fault.seam(), fault.ordinal());
        let state = receipt.execution().recovered_state();
        let counters = receipt.counters();
        require_oracle_observations(state.recovered_physical_root(), counters)?;
        Ok(Self {
            seam,
            lowered_plan_id: lowered_plan_id.into(),
            storage_boundary_id: format!(
                "storage-boundary:{}:{}:{}",
                boundary.backend_profile(),
                boundary.seam(),
                boundary.fault_ordinal()
            ),
            observer_transcript_id: format!(
                "observer:{}:{}:{}",
                seam.as_str(),
                state.recovered_physical_root(),
                state.source_decision_digest()
            ),
            proof_oracle_id: format!(
                "oracle:{}:{}:{}",
                seam.as_str(),
                counters.replayed_frames(),
                counters.forbidden_full_store_scans()
            ),
            seed: fault.seed(),
            backend_profile: boundary.backend_profile(),
            fault_ordinal: boundary.fault_ordinal(),
            recovered_root: state.recovered_physical_root().to_string(),
            source_decision_digest: state.source_decision_digest().to_string(),
            counters,
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

    pub const fn backend_profile(&self) -> &'static str {
        self.backend_profile
    }

    pub const fn fault_ordinal(&self) -> u16 {
        self.fault_ordinal
    }

    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub fn source_decision_digest(&self) -> &str {
        &self.source_decision_digest
    }

    pub const fn counters(&self) -> RecoveryCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutedS4CrashHarnessDenial {
    EmptyRecoveredRoot,
    ForbiddenFullStoreScan,
}

fn require_oracle_observations(
    recovered_root: &str,
    counters: RecoveryCounterSnapshot,
) -> Result<(), ExecutedS4CrashHarnessDenial> {
    if recovered_root.is_empty() {
        return Err(ExecutedS4CrashHarnessDenial::EmptyRecoveredRoot);
    }
    if counters.forbidden_full_store_scans() != 0 {
        return Err(ExecutedS4CrashHarnessDenial::ForbiddenFullStoreScan);
    }
    Ok(())
}
