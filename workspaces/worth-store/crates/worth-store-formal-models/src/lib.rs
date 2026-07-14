#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeledStateMachine {
    WalCheckpointFlushOrdering,
    RecoverySourcePrecedence,
    CompactionCutover,
    PhysicalReadLease,
    RepairQuarantine,
    ReplicationImportAdmission,
}
