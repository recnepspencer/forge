#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceWorkClass {
    SnapshotRefresh,
    Compaction,
    Reclaim,
    ReplicationPreparation,
    TierMovement,
}
