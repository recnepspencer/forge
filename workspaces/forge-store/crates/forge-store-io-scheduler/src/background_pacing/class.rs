use super::BackgroundDebtKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundIoPressureClass {
    CompactionRewrite,
    CheckpointFlush,
    ScrubScan,
    ReplicationPrepRead,
    BlobIngestPressure,
    BlobMigrationPressure,
    BackupPrepRead,
    RepairScan,
    VerificationPressure,
}

impl BackgroundIoPressureClass {
    pub const fn debt_kind(self) -> BackgroundDebtKind {
        match self {
            Self::CompactionRewrite => BackgroundDebtKind::CompactionDebt,
            Self::CheckpointFlush => BackgroundDebtKind::CheckpointFlushDebt,
            Self::ScrubScan => BackgroundDebtKind::ScrubPressure,
            Self::ReplicationPrepRead => BackgroundDebtKind::ReplicationPrepPressure,
            Self::BlobIngestPressure | Self::BlobMigrationPressure => {
                BackgroundDebtKind::BlobContention
            }
            Self::BackupPrepRead => BackgroundDebtKind::BackupPressure,
            Self::RepairScan | Self::VerificationPressure => BackgroundDebtKind::RepairPressure,
        }
    }
}
