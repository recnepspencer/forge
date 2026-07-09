use worth_proof::TransitionOutcome;

use crate::{BlobCorruptionGuard, BlobCorruptionGuardDenial};

pub type BlobCorruptionCapsuleReadinessOutcome =
    TransitionOutcome<BlobCorruptionCapsuleReadiness, BlobCorruptionGuardDenial>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCorruptionCapsuleReadiness;

impl BlobCorruptionCapsuleReadiness {
    pub fn deny_for_quarantine(
        guard: &BlobCorruptionGuard,
    ) -> BlobCorruptionCapsuleReadinessOutcome {
        TransitionOutcome::denied(guard.deny_capsule_readiness())
    }
}
