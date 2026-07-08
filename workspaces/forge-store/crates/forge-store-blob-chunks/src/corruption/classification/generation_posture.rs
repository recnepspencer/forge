use crate::{
    BlobChunkQuarantine, BlobCorruptionCounterSnapshot, BlobCorruptionDenial,
    BlobObjectClassification, BlobQuarantineLifecycleState, DerivedBlobRebuildAuthority,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCorruptionGenerationClassification {
    classification: BlobObjectClassification,
    state: BlobQuarantineLifecycleState,
    counters: BlobCorruptionCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivedBlobCorruptionRebuildReadiness {
    counters: BlobCorruptionCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthoritativeBlobCorruptionPosture {
    state: BlobQuarantineLifecycleState,
    counters: BlobCorruptionCounterSnapshot,
}

impl BlobCorruptionGenerationClassification {
    pub(crate) const fn construct_from_quarantine(
        quarantine: &BlobChunkQuarantine,
        classification: BlobObjectClassification,
    ) -> Self {
        Self {
            classification,
            state: quarantine.state(),
            counters: quarantine.counters(),
        }
    }

    pub fn admit_derived_rebuild(
        self,
        authority: DerivedBlobRebuildAuthority,
    ) -> Result<DerivedBlobCorruptionRebuildReadiness, BlobCorruptionDenial> {
        let _current_authority = authority.into_current_authority();
        if self.classification.is_derived() {
            Ok(DerivedBlobCorruptionRebuildReadiness {
                counters: self.counters.record_derived_rebuild_admission(),
            })
        } else {
            Err(BlobCorruptionDenial::DerivedRebuildRequiresDerivedBlob {
                counters: self.counters.record_denial(),
            })
        }
    }

    pub const fn authoritative_posture(
        self,
    ) -> Result<AuthoritativeBlobCorruptionPosture, BlobCorruptionDenial> {
        self.authoritative_posture_with_state(
            BlobQuarantineLifecycleState::RepairRequiredAuthoritative,
            self.counters.record_authoritative_repair_posture(),
        )
    }

    pub const fn authoritative_restore_posture(
        self,
    ) -> Result<AuthoritativeBlobCorruptionPosture, BlobCorruptionDenial> {
        self.authoritative_posture_with_state(
            BlobQuarantineLifecycleState::RestoreRequiredAuthoritative,
            self.counters.record_authoritative_restore_posture(),
        )
    }

    pub const fn authoritative_degraded_truth_posture(
        self,
    ) -> Result<AuthoritativeBlobCorruptionPosture, BlobCorruptionDenial> {
        self.authoritative_posture_with_state(
            BlobQuarantineLifecycleState::DegradedTruthAuthoritative,
            self.counters.record_authoritative_degraded_truth_posture(),
        )
    }

    const fn authoritative_posture_with_state(
        self,
        state: BlobQuarantineLifecycleState,
        counters: BlobCorruptionCounterSnapshot,
    ) -> Result<AuthoritativeBlobCorruptionPosture, BlobCorruptionDenial> {
        if self.classification.is_authoritative() {
            Ok(AuthoritativeBlobCorruptionPosture { state, counters })
        } else {
            Err(
                BlobCorruptionDenial::AuthoritativeRepairRequiresAuthoritativeBlob {
                    counters: self.counters.record_denial(),
                },
            )
        }
    }

    pub const fn classification(self) -> BlobObjectClassification {
        self.classification
    }

    pub const fn state(self) -> BlobQuarantineLifecycleState {
        self.state
    }

    pub const fn counters(self) -> BlobCorruptionCounterSnapshot {
        self.counters
    }
}

impl DerivedBlobCorruptionRebuildReadiness {
    pub const fn counters(self) -> BlobCorruptionCounterSnapshot {
        self.counters
    }
}

impl AuthoritativeBlobCorruptionPosture {
    pub const fn state(self) -> BlobQuarantineLifecycleState {
        self.state
    }

    pub const fn counters(self) -> BlobCorruptionCounterSnapshot {
        self.counters
    }
}
