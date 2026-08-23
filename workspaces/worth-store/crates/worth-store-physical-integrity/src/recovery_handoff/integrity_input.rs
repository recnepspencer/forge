use crate::IntegrityVettedWalFrame;
use crate::{WalFrameIntegrityInputIdentity, WalTailIntegrityPosture};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsIntegrityInput {
    wal_identity: WalFrameIntegrityInputIdentity,
    tail_posture: WalTailIntegrityPosture,
}

impl RecoveryPhysicsIntegrityInput {
    pub fn from_vetted_wal_frame(record: &IntegrityVettedWalFrame) -> Self {
        Self {
            wal_identity: record.input_identity(),
            tail_posture: record.tail_posture(),
        }
    }

    pub const fn wal_identity(&self) -> WalFrameIntegrityInputIdentity {
        self.wal_identity
    }

    pub const fn tail_posture(&self) -> WalTailIntegrityPosture {
        self.tail_posture
    }
}
