use super::{BackupReachabilityLeaseHolderId, BackupReachabilityLeasePersistenceRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedBackupReachabilityLease {
    holder: BackupReachabilityLeaseHolderId,
    record: BackupReachabilityLeasePersistenceRecord,
    control_generation: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReleasedBackupReachabilityLease {
    holder: BackupReachabilityLeaseHolderId,
    cut_identity: [u8; 32],
    control_generation: u64,
}

impl PersistedBackupReachabilityLease {
    pub(super) const fn new(
        holder: BackupReachabilityLeaseHolderId,
        record: BackupReachabilityLeasePersistenceRecord,
        control_generation: u64,
    ) -> Self {
        Self {
            holder,
            record,
            control_generation,
        }
    }

    pub const fn cut_identity(&self) -> [u8; 32] {
        self.record.cut_identity()
    }

    pub const fn holder(&self) -> BackupReachabilityLeaseHolderId {
        self.holder
    }

    pub const fn control_generation(&self) -> u64 {
        self.control_generation
    }
}

impl ReleasedBackupReachabilityLease {
    pub(super) const fn new(
        holder: BackupReachabilityLeaseHolderId,
        cut_identity: [u8; 32],
        control_generation: u64,
    ) -> Self {
        Self {
            holder,
            cut_identity,
            control_generation,
        }
    }

    pub const fn cut_identity(self) -> [u8; 32] {
        self.cut_identity
    }

    pub const fn holder(self) -> BackupReachabilityLeaseHolderId {
        self.holder
    }

    pub const fn control_generation(self) -> u64 {
        self.control_generation
    }
}
