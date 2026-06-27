use crate::{LogSequenceNumber, PhysicalRecoverySource, RecoveryPhysicsIntegrityInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalReplayReceipt {
    input: RecoveryPhysicsIntegrityInput,
    replayed_through: LogSequenceNumber,
}

impl WalReplayReceipt {
    pub const fn input(&self) -> &RecoveryPhysicsIntegrityInput {
        &self.input
    }

    pub const fn replayed_through(&self) -> LogSequenceNumber {
        self.replayed_through
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointValidityDecision {
    source: PhysicalRecoverySource,
    valid_through: LogSequenceNumber,
}

impl CheckpointValidityDecision {
    pub const fn source(&self) -> PhysicalRecoverySource {
        self.source
    }

    pub const fn valid_through(&self) -> LogSequenceNumber {
        self.valid_through
    }
}
