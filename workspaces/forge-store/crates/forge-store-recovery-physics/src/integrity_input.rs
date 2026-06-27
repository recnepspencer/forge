use forge_store_physical_integrity::{
    WalFrameIntegrityInputIdentity, WalFrameIntegrityReport, WalTailIntegrityPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsIntegrityInput {
    wal_identity: WalFrameIntegrityInputIdentity,
    tail_posture: WalTailIntegrityPosture,
}

impl RecoveryPhysicsIntegrityInput {
    pub fn from_wal_integrity_report(report: &WalFrameIntegrityReport) -> Self {
        Self {
            wal_identity: report.input_identity(),
            tail_posture: report.tail_posture(),
        }
    }

    pub const fn wal_identity(&self) -> WalFrameIntegrityInputIdentity {
        self.wal_identity
    }

    pub const fn tail_posture(&self) -> WalTailIntegrityPosture {
        self.tail_posture
    }
}
