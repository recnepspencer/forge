use crate::corruption::types::BlobCorruptionDetectionSource;
use crate::corruption::types::BlobQuarantineLifecycleState;
use crate::{
    BlobChunkOrdinal, BlobCorruptedChunkLocalization, BlobCorruptionCounterSnapshot,
    BlobCorruptionPlacementClass, BlobCorruptionReferenceSharingScope, BlobGeneration,
    BlobObjectId, StoredChunkDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkQuarantine {
    localization: BlobCorruptedChunkLocalization,
    state: BlobQuarantineLifecycleState,
    counters: BlobCorruptionCounterSnapshot,
}

pub(crate) fn construct_quarantine_receipt(
    localization: BlobCorruptedChunkLocalization,
    state: BlobQuarantineLifecycleState,
) -> BlobChunkQuarantine {
    BlobChunkQuarantine {
        counters: localization.counters().record_quarantine_hold(),
        localization,
        state,
    }
}

impl BlobChunkQuarantine {
    pub fn seal(
        localization: BlobCorruptedChunkLocalization,
        authority: crate::BlobQuarantineAuthority,
    ) -> Self {
        crate::corruption::transitions::seal(localization, authority)
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

    pub fn repair_capability(&self) -> crate::BlobQuarantineRepairCapability {
        crate::corruption::receipt_construction::repair_capability::classify_repair_capability_from_quarantine(self)
    }
}
