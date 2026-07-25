use super::RecoveryCandidateDiscoveryTrace;
use crate::{
    CheckpointCutoverReceipt, CheckpointId, CheckpointRecoveryCounterSnapshot,
    CheckpointValidation, WalLsnRange,
};
use worth_store_physical_format::PhysicalReference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointBaseAdmission {
    checkpoint_id: CheckpointId,
    root_reference: PhysicalReference,
    covered_lsn_range: WalLsnRange,
    trace: RecoveryCandidateDiscoveryTrace,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl CheckpointBaseAdmission {
    pub fn from_validated_checkpoint(
        validation: &CheckpointValidation,
        receipt: &CheckpointCutoverReceipt,
        trace: RecoveryCandidateDiscoveryTrace,
    ) -> Option<Self> {
        if validation.checkpoint_id() != receipt.checkpoint_id() {
            return None;
        }
        Some(Self {
            checkpoint_id: validation.checkpoint_id().clone(),
            root_reference: validation
                .manifest()
                .root_posture()
                .root_reference()
                .expect("validated checkpoint retains its exact root reference"),
            covered_lsn_range: receipt.covered_lsn_range().range(),
            trace,
            counters: receipt.counters(),
        })
    }

    pub(crate) fn from_reopened_artifact(
        checkpoint_id: CheckpointId,
        root_reference: PhysicalReference,
        covered_lsn_range: WalLsnRange,
        trace: RecoveryCandidateDiscoveryTrace,
        counters: CheckpointRecoveryCounterSnapshot,
    ) -> Self {
        Self {
            checkpoint_id,
            root_reference,
            covered_lsn_range,
            trace,
            counters,
        }
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn root_reference(&self) -> PhysicalReference {
        self.root_reference
    }

    pub const fn covered_lsn_range(&self) -> WalLsnRange {
        self.covered_lsn_range
    }

    pub const fn trace(&self) -> &RecoveryCandidateDiscoveryTrace {
        &self.trace
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}
