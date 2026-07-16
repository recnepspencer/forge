use std::collections::{BTreeMap, HashSet};
use std::sync::RwLock;

use worth_store_physical_backend::PhysicalControlAppendReceipt;

use super::{
    BackupReachabilityLeaseHolderId, BackupReachabilityLeaseIndexSnapshot,
    BackupReachabilityLeasePersistenceRecord, PersistedBackupReachabilityLease,
    ReleasedBackupReachabilityLease,
};

#[derive(Debug, Default)]
pub struct BackupReachabilityLeaseRegistry {
    active: RwLock<BTreeMap<[u8; 32], BackupLeaseRegistryEntry>>,
}

#[derive(Debug, Clone)]
struct BackupLeaseRegistryEntry {
    record: BackupReachabilityLeasePersistenceRecord,
    durable_holders: HashSet<BackupReachabilityLeaseHolderId>,
    pending_admissions: HashSet<BackupReachabilityLeaseHolderId>,
    pending_releases: HashSet<BackupReachabilityLeaseHolderId>,
}

pub struct PendingBackupLeaseAdmission<'a> {
    pub(super) registry: &'a BackupReachabilityLeaseRegistry,
    pub(super) holder: BackupReachabilityLeaseHolderId,
    pub(super) record: BackupReachabilityLeasePersistenceRecord,
    pub(super) reserved: bool,
    pub(super) completed: bool,
}

pub struct PendingBackupLeaseRelease<'a> {
    pub(super) registry: &'a BackupReachabilityLeaseRegistry,
    pub(super) holder: BackupReachabilityLeaseHolderId,
    pub(super) cut_identity: [u8; 32],
    pub(super) completed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupReachabilityLeaseRegistryDenial {
    ConflictingCutIdentity,
    LeaseNotActive,
    AllocationFailed,
    HolderCountOverflow,
    LeaseAdmissionInProgress,
    LeaseReleaseInProgress,
    HolderNotActive,
    ReservationLost,
    CorruptLeaseRecord,
}

impl BackupReachabilityLeaseRegistry {
    pub fn for_store_runtime() -> Self {
        Self::default()
    }

    pub fn recover_from_persisted(
        records: impl IntoIterator<
            Item = (
                BackupReachabilityLeaseHolderId,
                BackupReachabilityLeasePersistenceRecord,
            ),
        >,
    ) -> Result<Self, BackupReachabilityLeaseRegistryDenial> {
        Self::recover_from_persisted_results(records.into_iter().map(Ok))
    }

    pub fn recover_from_persisted_results(
        records: impl IntoIterator<
            Item = Result<
                (
                    BackupReachabilityLeaseHolderId,
                    BackupReachabilityLeasePersistenceRecord,
                ),
                BackupReachabilityLeaseRegistryDenial,
            >,
        >,
    ) -> Result<Self, BackupReachabilityLeaseRegistryDenial> {
        let registry = Self::for_store_runtime();
        {
            let mut active = registry.write_active();
            for record in records {
                let (holder, record) = record?;
                match active.get_mut(&record.cut_identity()) {
                    Some(existing) if existing.record != record => {
                        return Err(BackupReachabilityLeaseRegistryDenial::ConflictingCutIdentity)
                    }
                    Some(existing) => {
                        existing
                            .durable_holders
                            .try_reserve(1)
                            .map_err(|_| BackupReachabilityLeaseRegistryDenial::AllocationFailed)?;
                        existing.durable_holders.insert(holder);
                    }
                    None => {
                        active.insert(
                            record.cut_identity(),
                            BackupLeaseRegistryEntry::recovered(holder, record)?,
                        );
                    }
                }
            }
        }
        Ok(registry)
    }

    pub fn reserve_admission(
        &self,
        holder: BackupReachabilityLeaseHolderId,
        record: BackupReachabilityLeasePersistenceRecord,
    ) -> Result<PendingBackupLeaseAdmission<'_>, BackupReachabilityLeaseRegistryDenial> {
        let mut active = self.write_active();
        match active.get_mut(&record.cut_identity()) {
            Some(existing) if existing.record != record => {
                return Err(BackupReachabilityLeaseRegistryDenial::ConflictingCutIdentity)
            }
            Some(existing) if existing.pending_releases.contains(&holder) => {
                return Err(BackupReachabilityLeaseRegistryDenial::LeaseReleaseInProgress)
            }
            Some(existing) if existing.pending_admissions.contains(&holder) => {
                return Err(BackupReachabilityLeaseRegistryDenial::LeaseAdmissionInProgress)
            }
            Some(existing) => {
                let reserved = !existing.durable_holders.contains(&holder);
                if reserved {
                    existing
                        .durable_holders
                        .try_reserve(1)
                        .map_err(|_| BackupReachabilityLeaseRegistryDenial::AllocationFailed)?;
                    existing
                        .pending_admissions
                        .try_reserve(1)
                        .map_err(|_| BackupReachabilityLeaseRegistryDenial::AllocationFailed)?;
                    existing.pending_admissions.insert(holder);
                }
                drop(active);
                return Ok(PendingBackupLeaseAdmission {
                    registry: self,
                    holder,
                    record,
                    reserved,
                    completed: false,
                });
            }
            None => {
                active.insert(
                    record.cut_identity(),
                    BackupLeaseRegistryEntry::pending(holder, record.clone())?,
                );
            }
        }
        drop(active);
        Ok(PendingBackupLeaseAdmission {
            registry: self,
            holder,
            record,
            reserved: true,
            completed: false,
        })
    }

    pub fn reserve_release(
        &self,
        holder: BackupReachabilityLeaseHolderId,
        cut_identity: [u8; 32],
    ) -> Result<PendingBackupLeaseRelease<'_>, BackupReachabilityLeaseRegistryDenial> {
        let mut active = self.write_active();
        let entry = active
            .get_mut(&cut_identity)
            .ok_or(BackupReachabilityLeaseRegistryDenial::LeaseNotActive)?;
        if !entry.durable_holders.contains(&holder) {
            return Err(BackupReachabilityLeaseRegistryDenial::HolderNotActive);
        }
        if !entry.pending_admissions.is_empty() {
            return Err(BackupReachabilityLeaseRegistryDenial::LeaseAdmissionInProgress);
        }
        if entry.pending_releases.contains(&holder) {
            return Err(BackupReachabilityLeaseRegistryDenial::LeaseReleaseInProgress);
        }
        entry
            .pending_releases
            .try_reserve(1)
            .map_err(|_| BackupReachabilityLeaseRegistryDenial::AllocationFailed)?;
        entry.pending_releases.insert(holder);
        Ok(PendingBackupLeaseRelease {
            registry: self,
            holder,
            cut_identity,
            completed: false,
        })
    }

    pub fn live_index_snapshot(
        &self,
    ) -> Result<BackupReachabilityLeaseIndexSnapshot, BackupReachabilityLeaseRegistryDenial> {
        let active = self.read_active();
        let active_holders = active.values().try_fold(0u64, |total, entry| {
            let holders = u64::try_from(entry.durable_holders.len())
                .map_err(|_| BackupReachabilityLeaseRegistryDenial::HolderCountOverflow)?;
            total
                .checked_add(holders)
                .ok_or(BackupReachabilityLeaseRegistryDenial::HolderCountOverflow)
        })?;
        let mut records = Vec::new();
        records
            .try_reserve_exact(active.len())
            .map_err(|_| BackupReachabilityLeaseRegistryDenial::AllocationFailed)?;
        for entry in active.values() {
            let recovered =
                BackupReachabilityLeasePersistenceRecord::recover(entry.record.recovery_bytes())
                    .map_err(|denial| match denial {
                        super::BackupReachabilityLeaseRecoveryDenial::AllocationFailed => {
                            BackupReachabilityLeaseRegistryDenial::AllocationFailed
                        }
                        _ => BackupReachabilityLeaseRegistryDenial::CorruptLeaseRecord,
                    })?;
            records.push(recovered);
        }
        Ok(BackupReachabilityLeaseIndexSnapshot::from_active(
            records,
            active_holders,
        ))
    }

    pub(super) fn commit_admission(
        &self,
        holder: BackupReachabilityLeaseHolderId,
        record: &BackupReachabilityLeasePersistenceRecord,
        reserved: bool,
        receipt: PhysicalControlAppendReceipt,
    ) -> Result<PersistedBackupReachabilityLease, BackupReachabilityLeaseRegistryDenial> {
        let mut active = self.write_active();
        let entry = active
            .get_mut(&record.cut_identity())
            .ok_or(BackupReachabilityLeaseRegistryDenial::ReservationLost)?;
        if reserved && !entry.pending_admissions.remove(&holder) {
            return Err(BackupReachabilityLeaseRegistryDenial::ReservationLost);
        }
        entry.durable_holders.insert(holder);
        Ok(PersistedBackupReachabilityLease::new(
            holder,
            record.clone(),
            receipt.generation().get(),
        ))
    }

    pub(super) fn commit_release(
        &self,
        holder: BackupReachabilityLeaseHolderId,
        cut_identity: [u8; 32],
        receipt: PhysicalControlAppendReceipt,
    ) -> Result<ReleasedBackupReachabilityLease, BackupReachabilityLeaseRegistryDenial> {
        let mut active = self.write_active();
        let entry = active
            .get_mut(&cut_identity)
            .ok_or(BackupReachabilityLeaseRegistryDenial::ReservationLost)?;
        if !entry.pending_releases.contains(&holder) || !entry.durable_holders.contains(&holder) {
            return Err(BackupReachabilityLeaseRegistryDenial::ReservationLost);
        }
        entry.pending_releases.remove(&holder);
        entry.durable_holders.remove(&holder);
        let remove = entry.durable_holders.is_empty() && entry.pending_admissions.is_empty();
        if remove {
            active.remove(&cut_identity);
        }
        Ok(ReleasedBackupReachabilityLease::new(
            holder,
            cut_identity,
            receipt.generation().get(),
        ))
    }

    pub(super) fn rollback_admission(
        &self,
        holder: BackupReachabilityLeaseHolderId,
        cut_identity: [u8; 32],
    ) {
        let mut active = self.write_active();
        let Some(entry) = active.get_mut(&cut_identity) else {
            return;
        };
        entry.pending_admissions.remove(&holder);
        if entry.durable_holders.is_empty() && entry.pending_admissions.is_empty() {
            active.remove(&cut_identity);
        }
    }

    pub(super) fn rollback_release(
        &self,
        holder: BackupReachabilityLeaseHolderId,
        cut_identity: [u8; 32],
    ) {
        if let Some(entry) = self.write_active().get_mut(&cut_identity) {
            entry.pending_releases.remove(&holder);
        }
    }

    fn write_active(
        &self,
    ) -> std::sync::RwLockWriteGuard<'_, BTreeMap<[u8; 32], BackupLeaseRegistryEntry>> {
        match self.active.write() {
            Ok(active) => active,
            Err(poisoned) => {
                self.active.clear_poison();
                poisoned.into_inner()
            }
        }
    }

    fn read_active(
        &self,
    ) -> std::sync::RwLockReadGuard<'_, BTreeMap<[u8; 32], BackupLeaseRegistryEntry>> {
        match self.active.read() {
            Ok(active) => active,
            Err(poisoned) => {
                self.active.clear_poison();
                poisoned.into_inner()
            }
        }
    }
}

impl BackupLeaseRegistryEntry {
    fn pending(
        holder: BackupReachabilityLeaseHolderId,
        record: BackupReachabilityLeasePersistenceRecord,
    ) -> Result<Self, BackupReachabilityLeaseRegistryDenial> {
        let mut durable_holders = HashSet::new();
        let mut pending_admissions = HashSet::new();
        durable_holders
            .try_reserve(1)
            .map_err(|_| BackupReachabilityLeaseRegistryDenial::AllocationFailed)?;
        pending_admissions
            .try_reserve(1)
            .map_err(|_| BackupReachabilityLeaseRegistryDenial::AllocationFailed)?;
        pending_admissions.insert(holder);
        Ok(Self {
            record,
            durable_holders,
            pending_admissions,
            pending_releases: HashSet::new(),
        })
    }

    fn recovered(
        holder: BackupReachabilityLeaseHolderId,
        record: BackupReachabilityLeasePersistenceRecord,
    ) -> Result<Self, BackupReachabilityLeaseRegistryDenial> {
        let mut durable_holders = HashSet::new();
        durable_holders
            .try_reserve(1)
            .map_err(|_| BackupReachabilityLeaseRegistryDenial::AllocationFailed)?;
        durable_holders.insert(holder);
        Ok(Self {
            record,
            durable_holders,
            pending_admissions: HashSet::new(),
            pending_releases: HashSet::new(),
        })
    }
}
