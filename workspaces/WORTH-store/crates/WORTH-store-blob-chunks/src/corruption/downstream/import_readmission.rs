use worth_proof::TransitionOutcome;
use worth_store_authority::StoreCurrentAuthorityWitness;

use crate::corruption::transitions::verify_current_store_authority_for_readmission;
use crate::{
    AuthoritativeBlobCorruptionPosture, BlobCorruptionGuard, BlobCorruptionGuardDenial,
    BlobQuarantineLifecycleState,
};

pub type BlobCorruptionImportReadmissionOutcome =
    TransitionOutcome<BlobCorruptionImportReadmission, BlobCorruptionGuardDenial>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCorruptionImportReadmission;

impl BlobCorruptionImportReadmission {
    pub fn deny_for_quarantine(
        guard: &BlobCorruptionGuard,
    ) -> BlobCorruptionImportReadmissionOutcome {
        TransitionOutcome::denied(guard.deny_import_readmission())
    }

    pub fn admit_from_posture(
        posture: AuthoritativeBlobCorruptionPosture,
        current_store_authority: StoreCurrentAuthorityWitness,
    ) -> BlobCorruptionImportReadmissionOutcome {
        if verify_current_store_authority_for_readmission(&current_store_authority).is_err() {
            return TransitionOutcome::denied(BlobCorruptionGuardDenial::ImportReadmissionDenied {
                source: crate::BlobCorruptionDetectionSource::ImportReadmission,
                counters: posture.counters().record_denial(),
            });
        }
        if !matches!(
            posture.state(),
            BlobQuarantineLifecycleState::ImportCorrupt
                | BlobQuarantineLifecycleState::RestoreRequiredAuthoritative
        ) {
            return TransitionOutcome::denied(BlobCorruptionGuardDenial::ImportReadmissionDenied {
                source: crate::BlobCorruptionDetectionSource::ImportReadmission,
                counters: posture.counters().record_denial(),
            });
        }
        let _witness = current_store_authority;
        TransitionOutcome::success(BlobCorruptionImportReadmission)
    }
}
