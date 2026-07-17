use worth_store_physical_backend::PhysicalControlAppendReceipt;

use super::{
    BackupReachabilityLeaseHolderId, BackupReachabilityLeaseRegistry,
    BackupReachabilityLeaseRegistryDenial, BackupReachabilityLeaseReleaseRecord,
    PendingBackupLeaseAdmission, PendingBackupLeaseRelease, PersistedBackupReachabilityLease,
    ReleasedBackupReachabilityLease,
};

impl PendingBackupLeaseAdmission<'_> {
    pub fn acknowledge_durable_persistence(
        mut self,
        receipt: PhysicalControlAppendReceipt,
    ) -> Result<PersistedBackupReachabilityLease, BackupReachabilityLeaseRegistryDenial> {
        let persisted =
            self.registry
                .commit_admission(self.holder, &self.record, self.reserved, receipt)?;
        self.completed = true;
        Ok(persisted)
    }
}

impl Drop for PendingBackupLeaseAdmission<'_> {
    fn drop(&mut self) {
        if !self.completed && self.reserved {
            self.registry
                .rollback_admission(self.holder, self.record.cut_identity());
        }
    }
}

impl PendingBackupLeaseRelease<'_> {
    pub fn acknowledge_durable_release(
        mut self,
        receipt: PhysicalControlAppendReceipt,
    ) -> Result<ReleasedBackupReachabilityLease, BackupReachabilityLeaseRegistryDenial> {
        let released = self
            .registry
            .commit_release(self.holder, self.cut_identity, receipt)?;
        self.completed = true;
        Ok(released)
    }
}

impl Drop for PendingBackupLeaseRelease<'_> {
    fn drop(&mut self) {
        if !self.completed {
            self.registry
                .rollback_release(self.holder, self.cut_identity);
        }
    }
}

impl BackupReachabilityLeaseRegistry {
    pub fn release_after_durable_record(
        &self,
        holder: BackupReachabilityLeaseHolderId,
        release: BackupReachabilityLeaseReleaseRecord,
        receipt: PhysicalControlAppendReceipt,
    ) -> Result<ReleasedBackupReachabilityLease, BackupReachabilityLeaseRegistryDenial> {
        let reservation = self.reserve_release(holder, release.cut_identity())?;
        reservation.acknowledge_durable_release(receipt)
    }
}
