use forge_proof::TransitionOutcome;

use crate::{BlobCorruptionGuard, BlobCorruptionGuardDenial};

pub type BlobCorruptionExportAdmissionOutcome =
    TransitionOutcome<BlobCorruptionExportAdmission, BlobCorruptionGuardDenial>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCorruptionExportAdmission;

impl BlobCorruptionExportAdmission {
    pub fn deny_for_quarantine(
        guard: &BlobCorruptionGuard,
    ) -> BlobCorruptionExportAdmissionOutcome {
        TransitionOutcome::denied(guard.deny_export())
    }
}