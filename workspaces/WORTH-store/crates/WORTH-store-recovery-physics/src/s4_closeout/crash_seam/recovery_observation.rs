use crate::{PageLsn, RecoveryCounterSnapshot};

use super::{S4CrashFaultSchedulerEvidence, S4RecoveryCrashSeam};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashSeamRecoveryObservation {
    seam: S4RecoveryCrashSeam,
    lowered_plan_id: String,
    storage_boundary_id: String,
    observer_transcript_id: String,
    proof_oracle_id: String,
    seed: u64,
    backend_profile: String,
    fault_ordinal: u16,
    recovered_root: String,
    page_lsn_frontier: Option<PageLsn>,
    source_decision_digest: String,
    counters: RecoveryCounterSnapshot,
}

impl CrashSeamRecoveryObservation {
    pub(crate) fn from_fault_scheduler_evidence(evidence: S4CrashFaultSchedulerEvidence) -> Self {
        let crash_receipt = evidence.fresh_runtime_recovery.crash_receipt();
        let state = crash_receipt.execution().recovered_state();
        Self {
            seam: evidence.seam,
            lowered_plan_id: evidence.lowered_plan_id,
            storage_boundary_id: evidence.storage_boundary_id,
            observer_transcript_id: evidence.observer_transcript_id,
            proof_oracle_id: evidence.proof_oracle_id,
            seed: evidence.seed,
            backend_profile: evidence.backend_profile,
            fault_ordinal: evidence.fault_ordinal,
            recovered_root: state.recovered_physical_root().to_string(),
            page_lsn_frontier: state.page_lsn_frontier(),
            source_decision_digest: state.source_decision_digest().to_string(),
            counters: crash_receipt.counters(),
        }
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

    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub const fn page_lsn_frontier(&self) -> Option<PageLsn> {
        self.page_lsn_frontier
    }

    pub fn source_decision_digest(&self) -> &str {
        &self.source_decision_digest
    }

    pub const fn counters(&self) -> RecoveryCounterSnapshot {
        self.counters
    }
}
