use crate::{
    BlobChunkOrdinal, BlobCorruptedChunkLocalization, BlobCorruptionCounterSnapshot,
    BlobCorruptionDetectionSource, BlobCorruptionPlacementClass,
    BlobCorruptionReferenceSharingScope, BlobGeneration, BlobObjectId, BlobQuarantineAuthority,
    StoredChunkDigest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobQuarantineLifecycleState {
    DetectedUnquarantined,
    Quarantined,
    RebuildableDerived,
    RepairRequiredAuthoritative,
    RestoreRequiredAuthoritative,
    DegradedTruthAuthoritative,
    ColdUnavailableCorrupt,
    ImportCorrupt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkQuarantine {
    localization: BlobCorruptedChunkLocalization,
    state: BlobQuarantineLifecycleState,
    counters: BlobCorruptionCounterSnapshot,
}

impl BlobChunkQuarantine {
    pub fn seal(
        localization: BlobCorruptedChunkLocalization,
        authority: BlobQuarantineAuthority,
    ) -> Self {
        let _current_authority = authority.into_current_authority();
        let state = match localization.source() {
            BlobCorruptionDetectionSource::ColdFetch => {
                BlobQuarantineLifecycleState::ColdUnavailableCorrupt
            }
            BlobCorruptionDetectionSource::ImportReadmission => {
                BlobQuarantineLifecycleState::ImportCorrupt
            }
            _ => BlobQuarantineLifecycleState::Quarantined,
        };
        let counters = localization.counters().record_quarantine_hold();
        Self {
            localization,
            state,
            counters,
        }
    }

    pub const fn localization(&self) -> &BlobCorruptedChunkLocalization {
        &self.localization
    }

    pub const fn state(&self) -> BlobQuarantineLifecycleState {
        self.state
    }

    pub const fn source(&self) -> BlobCorruptionDetectionSource {
        self.localization.source()
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        self.localization.object_id()
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.localization.generation()
    }

    pub const fn ordinal(&self) -> BlobChunkOrdinal {
        self.localization.ordinal()
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        self.localization.stored_digest()
    }

    pub const fn placement_class(&self) -> BlobCorruptionPlacementClass {
        self.localization.placement_class()
    }

    pub const fn sharing_scope(&self) -> BlobCorruptionReferenceSharingScope {
        self.localization.sharing_scope()
    }

    pub const fn counters(&self) -> BlobCorruptionCounterSnapshot {
        self.counters
    }
}
