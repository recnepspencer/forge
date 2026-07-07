pub use forge_store_contracts::{
    S10BackupExportReadinessNonClaim, S10CompactionReadinessNonClaim,
    S10RepairScanReadinessNonClaim, S11OperatorReadinessNonClaim, S7PlacementReadinessNonClaim,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S7CapsuleReadinessNonClaim {
    FullReplicationCorrectness,
    BackupRestoreCorrectness,
    ProductBlobApiCorrectness,
    RestoreWorkflowCorrectness,
}

impl S7CapsuleReadinessNonClaim {
    pub const fn required() -> [Self; 4] {
        [
            Self::FullReplicationCorrectness,
            Self::BackupRestoreCorrectness,
            Self::ProductBlobApiCorrectness,
            Self::RestoreWorkflowCorrectness,
        ]
    }
}
