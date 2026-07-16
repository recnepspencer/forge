use super::lease_persistence::BackupProtectedPhysicalOwner;
use super::{
    AdmittedBackupCut, BackupCutManifest, BackupReachabilityLeasePersistenceRecord,
    BackupReachabilityLeaseReleaseRecord,
};
use crate::ReclaimCandidateSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupReachabilityLease {
    cut_identity: [u8; 32],
    persistence: BackupReachabilityLeasePersistenceRecord,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BackupReachabilityLeaseIndexSnapshot {
    active: Vec<BackupReachabilityLeasePersistenceRecord>,
    active_holders: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupLeaseOverlap {
    cut_identity: [u8; 32],
    protected_artifacts: u64,
    overlapping_artifacts: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BackupLeaseScan {
    pub(crate) active_leases: u64,
    pub(crate) protected_artifacts: u64,
    pub(crate) artifact_comparisons: u64,
    pub(crate) first_overlap: Option<BackupLeaseOverlap>,
}

impl BackupReachabilityLease {
    pub(crate) fn for_admitted_cut(
        cut_identity: [u8; 32],
        manifest: &BackupCutManifest,
    ) -> Result<Self, super::BackupReachabilityLeaseRecoveryDenial> {
        Ok(Self {
            cut_identity,
            persistence: BackupReachabilityLeasePersistenceRecord::from_manifest(
                cut_identity,
                manifest,
            )?,
        })
    }

    pub const fn cut_identity(&self) -> [u8; 32] {
        self.cut_identity
    }

    pub fn persistence_record(
        &self,
    ) -> Result<
        BackupReachabilityLeasePersistenceRecord,
        super::BackupReachabilityLeaseRecoveryDenial,
    > {
        BackupReachabilityLeasePersistenceRecord::recover(self.persistence.recovery_bytes())
    }

    pub fn release_record(&self) -> BackupReachabilityLeaseReleaseRecord {
        BackupReachabilityLeaseReleaseRecord::from_lease(self)
    }
}

impl BackupReachabilityLeaseIndexSnapshot {
    pub(super) fn from_active(
        mut active: Vec<BackupReachabilityLeasePersistenceRecord>,
        active_holders: u64,
    ) -> Self {
        active.sort_by_key(BackupReachabilityLeasePersistenceRecord::cut_identity);
        active.dedup_by_key(|record| record.cut_identity());
        Self {
            active,
            active_holders,
        }
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub const fn empty() -> Self {
        Self {
            active: Vec::new(),
            active_holders: 0,
        }
    }

    pub fn active_leases(&self) -> usize {
        self.active.len()
    }

    pub const fn active_holders(&self) -> u64 {
        self.active_holders
    }

    pub(crate) fn first_overlap(
        &self,
        candidates: &ReclaimCandidateSet,
    ) -> Result<BackupLeaseScan, crate::ReclaimDenial> {
        let mut candidate_owners = Vec::new();
        candidate_owners
            .try_reserve_exact(candidates.candidate_owners().len())
            .map_err(|_| crate::ReclaimDenial::AllocationFailed)?;
        candidate_owners.extend(
            candidates
                .candidate_owners()
                .iter()
                .copied()
                .map(BackupProtectedPhysicalOwner::from_owner),
        );
        let mut protected_artifacts = 0u64;
        let mut artifact_comparisons = 0u64;
        for lease in &self.active {
            let mut overlapping_artifacts = 0u64;
            for protected in lease.protection() {
                protected_artifacts += 1;
                artifact_comparisons += 1;
                let overlaps = candidate_owners.binary_search(protected).is_ok();
                if overlaps {
                    overlapping_artifacts += 1;
                }
            }
            if overlapping_artifacts > 0 {
                return Ok(BackupLeaseScan {
                    active_leases: self.active.len() as u64,
                    protected_artifacts,
                    artifact_comparisons,
                    first_overlap: Some(BackupLeaseOverlap {
                        cut_identity: lease.cut_identity(),
                        protected_artifacts: lease.protected_artifacts() as u64,
                        overlapping_artifacts,
                    }),
                });
            }
        }
        Ok(BackupLeaseScan {
            active_leases: self.active.len() as u64,
            protected_artifacts,
            artifact_comparisons,
            first_overlap: None,
        })
    }
}

impl BackupLeaseOverlap {
    pub const fn cut_identity(self) -> [u8; 32] {
        self.cut_identity
    }

    pub const fn protected_artifacts(self) -> u64 {
        self.protected_artifacts
    }

    pub const fn overlapping_artifacts(self) -> u64 {
        self.overlapping_artifacts
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupCutAbandonmentReceipt {
    cut_identity: [u8; 32],
    release_control_generation: u64,
    reason: String,
}

#[derive(Debug)]
pub struct PreparedBackupCutAbandonment {
    cut: AdmittedBackupCut,
    released: super::ReleasedBackupReachabilityLease,
}

#[derive(Debug)]
pub struct BackupCutReleaseMismatch {
    cut: AdmittedBackupCut,
    released: super::ReleasedBackupReachabilityLease,
}

impl BackupCutAbandonmentReceipt {
    pub const fn cut_identity(&self) -> [u8; 32] {
        self.cut_identity
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub const fn release_control_generation(&self) -> u64 {
        self.release_control_generation
    }
}

pub fn prepare_backup_cut_abandonment(
    cut: AdmittedBackupCut,
    released: super::ReleasedBackupReachabilityLease,
) -> Result<PreparedBackupCutAbandonment, BackupCutReleaseMismatch> {
    if cut.identity() != released.cut_identity() {
        return Err(BackupCutReleaseMismatch { cut, released });
    }
    Ok(PreparedBackupCutAbandonment { cut, released })
}

pub fn abandon_backup_cut(
    prepared: PreparedBackupCutAbandonment,
    reason: impl Into<String>,
) -> BackupCutAbandonmentReceipt {
    BackupCutAbandonmentReceipt {
        cut_identity: prepared.cut.identity(),
        release_control_generation: prepared.released.control_generation(),
        reason: reason.into(),
    }
}

impl BackupCutReleaseMismatch {
    pub fn into_parts(self) -> (AdmittedBackupCut, super::ReleasedBackupReachabilityLease) {
        (self.cut, self.released)
    }
}
