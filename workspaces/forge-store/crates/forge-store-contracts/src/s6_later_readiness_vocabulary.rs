#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6LaterMilestoneDestination {
    S7Placement,
    S10Compaction,
    S10BackupExport,
    S10RepairScan,
    S11OperatorReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S7PlacementReadinessNonClaim {
    BlobLifecycleCorrectness,
    ChunkDedupeCorrectness,
    PlacementPolicyCorrectness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10CompactionReadinessNonClaim {
    CompactionProductCorrectness,
    ForensicCorrectness,
    PlacementCorrectness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10BackupExportReadinessNonClaim {
    BackupRestoreCorrectness,
    ExportFormatCorrectness,
    PointInTimeRecoveryCorrectness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10RepairScanReadinessNonClaim {
    RepairOperatorAuthorization,
    RepairPlanCorrectness,
    ForensicCorrectness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S11OperatorReadinessNonClaim {
    EncryptionAlgorithm,
    KeyRotation,
    AuditCorrectness,
    OperatorAuthorization,
}

impl S7PlacementReadinessNonClaim {
    pub const fn required() -> [Self; 3] {
        [
            Self::BlobLifecycleCorrectness,
            Self::ChunkDedupeCorrectness,
            Self::PlacementPolicyCorrectness,
        ]
    }
}

impl S10CompactionReadinessNonClaim {
    pub const fn required() -> [Self; 3] {
        [
            Self::CompactionProductCorrectness,
            Self::ForensicCorrectness,
            Self::PlacementCorrectness,
        ]
    }
}

impl S10BackupExportReadinessNonClaim {
    pub const fn required() -> [Self; 3] {
        [
            Self::BackupRestoreCorrectness,
            Self::ExportFormatCorrectness,
            Self::PointInTimeRecoveryCorrectness,
        ]
    }
}

impl S10RepairScanReadinessNonClaim {
    pub const fn required() -> [Self; 3] {
        [
            Self::RepairOperatorAuthorization,
            Self::RepairPlanCorrectness,
            Self::ForensicCorrectness,
        ]
    }
}

impl S11OperatorReadinessNonClaim {
    pub const fn required() -> [Self; 4] {
        [
            Self::EncryptionAlgorithm,
            Self::KeyRotation,
            Self::AuditCorrectness,
            Self::OperatorAuthorization,
        ]
    }
}
