use forge_proof::TransitionOutcome;

use crate::{BlobCorruptionGuard, BlobCorruptionGuardDenial};

pub type BlobCorruptionExportAdmissionOutcome =
    TransitionOutcome<BlobCorruptionExportAdmission, BlobCorruptionGuardDenial>;
pub type BlobCorruptionImportReadmissionOutcome =
    TransitionOutcome<BlobCorruptionImportReadmission, BlobCorruptionGuardDenial>;
pub type BlobCorruptionCapsuleReadinessOutcome =
    TransitionOutcome<BlobCorruptionCapsuleReadiness, BlobCorruptionGuardDenial>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCorruptionExportAdmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCorruptionImportReadmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCorruptionCapsuleReadiness;

impl BlobCorruptionExportAdmission {
    pub fn deny_for_quarantine(
        guard: &BlobCorruptionGuard,
    ) -> BlobCorruptionExportAdmissionOutcome {
        TransitionOutcome::denied(guard.deny_export())
    }
}

impl BlobCorruptionImportReadmission {
    pub fn deny_for_quarantine(
        guard: &BlobCorruptionGuard,
    ) -> BlobCorruptionImportReadmissionOutcome {
        TransitionOutcome::denied(guard.deny_import_readmission())
    }
}

impl BlobCorruptionCapsuleReadiness {
    pub fn deny_for_quarantine(
        guard: &BlobCorruptionGuard,
    ) -> BlobCorruptionCapsuleReadinessOutcome {
        TransitionOutcome::denied(guard.deny_capsule_readiness())
    }
}
