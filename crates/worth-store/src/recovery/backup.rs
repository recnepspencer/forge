use crate::{
    backend::records::StoreState,
    failure::StoreError,
    snapshot::{SNAPSHOT_BASIS_VERSION, SNAPSHOT_FAMILY_VERSION, SNAPSHOT_IMAGE_FORMAT_VERSION},
    DurableBackendFamily,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackupRestoreCompatibilityReport {
    backend_family: DurableBackendFamily,
    local_restart_only: bool,
    external_restore_requires_explicit_mode: bool,
    canonicalization_version: u32,
    admitted_wal_version: u32,
    admitted_snapshot_family_version: u32,
    admitted_snapshot_basis_version: u32,
    admitted_snapshot_image_format_version: u32,
    observed_snapshot_version_tuples: Vec<ObservedSnapshotVersionTuple>,
    incompatibilities: Vec<BackupRestoreIncompatibility>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ObservedSnapshotVersionTuple {
    snapshot_id: u64,
    family_version: u32,
    basis_version: u32,
    image_format_version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BackupRestoreIncompatibilityKind {
    SnapshotFamilyVersionUnsupported,
    SnapshotBasisVersionUnsupported,
    SnapshotImageFormatVersionUnsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackupRestoreIncompatibility {
    kind: BackupRestoreIncompatibilityKind,
    snapshot_id: u64,
    observed_version: u32,
    admitted_version: u32,
}

impl BackupRestoreCompatibilityReport {
    pub fn backend_family(&self) -> DurableBackendFamily {
        self.backend_family
    }

    pub fn local_restart_only(&self) -> bool {
        self.local_restart_only
    }

    pub fn external_restore_requires_explicit_mode(&self) -> bool {
        self.external_restore_requires_explicit_mode
    }

    pub fn canonicalization_version(&self) -> u32 {
        self.canonicalization_version
    }

    pub fn admitted_wal_version(&self) -> u32 {
        self.admitted_wal_version
    }

    pub fn admitted_snapshot_family_version(&self) -> u32 {
        self.admitted_snapshot_family_version
    }

    pub fn admitted_snapshot_basis_version(&self) -> u32 {
        self.admitted_snapshot_basis_version
    }

    pub fn admitted_snapshot_image_format_version(&self) -> u32 {
        self.admitted_snapshot_image_format_version
    }

    pub fn observed_snapshot_version_tuples(&self) -> &[ObservedSnapshotVersionTuple] {
        &self.observed_snapshot_version_tuples
    }

    pub fn incompatibilities(&self) -> &[BackupRestoreIncompatibility] {
        &self.incompatibilities
    }
}

impl ObservedSnapshotVersionTuple {
    pub fn snapshot_id(&self) -> u64 {
        self.snapshot_id
    }

    pub fn family_version(&self) -> u32 {
        self.family_version
    }

    pub fn basis_version(&self) -> u32 {
        self.basis_version
    }

    pub fn image_format_version(&self) -> u32 {
        self.image_format_version
    }
}

impl BackupRestoreIncompatibility {
    pub fn kind(&self) -> BackupRestoreIncompatibilityKind {
        self.kind
    }

    pub fn snapshot_id(&self) -> u64 {
        self.snapshot_id
    }

    pub fn observed_version(&self) -> u32 {
        self.observed_version
    }

    pub fn admitted_version(&self) -> u32 {
        self.admitted_version
    }
}

pub(crate) fn build_backup_restore_compatibility_report(
    state: &StoreState,
    backend_family: DurableBackendFamily,
) -> Result<BackupRestoreCompatibilityReport, StoreError> {
    let observed_snapshot_version_tuples = state
        .snapshot_basis_records
        .values()
        .map(|basis| ObservedSnapshotVersionTuple {
            snapshot_id: basis.snapshot_id.0,
            family_version: basis.snapshot_family_version,
            basis_version: basis.snapshot_basis_version,
            image_format_version: basis.snapshot_image_format_version,
        })
        .collect::<Vec<_>>();

    let mut incompatibilities = Vec::new();
    for observed in &observed_snapshot_version_tuples {
        if observed.family_version != SNAPSHOT_FAMILY_VERSION {
            incompatibilities.push(BackupRestoreIncompatibility {
                kind: BackupRestoreIncompatibilityKind::SnapshotFamilyVersionUnsupported,
                snapshot_id: observed.snapshot_id,
                observed_version: observed.family_version,
                admitted_version: SNAPSHOT_FAMILY_VERSION,
            });
        }
        if observed.basis_version != SNAPSHOT_BASIS_VERSION {
            incompatibilities.push(BackupRestoreIncompatibility {
                kind: BackupRestoreIncompatibilityKind::SnapshotBasisVersionUnsupported,
                snapshot_id: observed.snapshot_id,
                observed_version: observed.basis_version,
                admitted_version: SNAPSHOT_BASIS_VERSION,
            });
        }
        if observed.image_format_version != SNAPSHOT_IMAGE_FORMAT_VERSION {
            incompatibilities.push(BackupRestoreIncompatibility {
                kind: BackupRestoreIncompatibilityKind::SnapshotImageFormatVersionUnsupported,
                snapshot_id: observed.snapshot_id,
                observed_version: observed.image_format_version,
                admitted_version: SNAPSHOT_IMAGE_FORMAT_VERSION,
            });
        }
    }

    Ok(BackupRestoreCompatibilityReport {
        backend_family,
        local_restart_only: true,
        external_restore_requires_explicit_mode: true,
        canonicalization_version: state.canonicalization_version,
        admitted_wal_version: crate::wal::CURRENT_WAL_VERSION,
        admitted_snapshot_family_version: SNAPSHOT_FAMILY_VERSION,
        admitted_snapshot_basis_version: SNAPSHOT_BASIS_VERSION,
        admitted_snapshot_image_format_version: SNAPSHOT_IMAGE_FORMAT_VERSION,
        observed_snapshot_version_tuples,
        incompatibilities,
    })
}
