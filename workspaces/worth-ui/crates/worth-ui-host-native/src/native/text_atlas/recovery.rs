//! Native atlas denials, move-only recovery, and effects-indeterminate outcomes.

use worth_ui_host_contract::UiGlyphRasterDemandIdentity;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiNativeTextAtlasGeneration(u64);

/// Stable identity for the atlas owner that issued a recovery authority.
///
/// A generation alone is not an ownership proof: two freshly-created hosts
/// can legitimately have the same generation.  Recovery therefore carries
/// this non-forgeable-in-practice lineage value as well as its generation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiNativeTextAtlasLineageIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeTextAtlasDenial {
    MalformedDemand,
    StaleDemand,
    GlyphExtentExceeded,
    EntryCapacityExceeded,
    PageCapacityExceeded,
    TexelCapacityExceeded,
    StagingCapacityExceeded,
    LivePinConflict,
    StalePlan,
    StalePin,
    StaleAffinity,
    ReconstructionRequired,
    ReservationConflict,
    GenerationExhausted,
    RasterGeometryMismatch,
    RasterBatchMismatch,
    UploadRejected,
    PinnedCapacityExceeded,
    PinConflict,
}

#[derive(Debug, PartialEq)]
pub struct UiNativeTextAtlasRecovery {
    demand: UiGlyphRasterDemandIdentity,
    generation: UiNativeTextAtlasGeneration,
    lineage: UiNativeTextAtlasLineageIdentity,
}

impl UiNativeTextAtlasGeneration {
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

impl UiNativeTextAtlasLineageIdentity {
    pub(crate) const fn from_native_host(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

impl UiNativeTextAtlasRecovery {
    #[allow(dead_code, reason = "reserved for native atlas effect ownership")]
    pub(crate) const fn from_native_host(
        demand: UiGlyphRasterDemandIdentity,
        generation: UiNativeTextAtlasGeneration,
        lineage: UiNativeTextAtlasLineageIdentity,
    ) -> Self {
        Self {
            demand,
            generation,
            lineage,
        }
    }

    pub const fn demand_identity(&self) -> UiGlyphRasterDemandIdentity {
        self.demand
    }

    pub const fn generation(&self) -> UiNativeTextAtlasGeneration {
        self.generation
    }

    pub const fn lineage_identity(&self) -> UiNativeTextAtlasLineageIdentity {
        self.lineage
    }

    pub fn snapshot(&self) -> UiNativeTextAtlasRecoverySnapshot {
        UiNativeTextAtlasRecoverySnapshot {
            demand: self.demand,
            generation: self.generation,
            lineage: self.lineage,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeTextAtlasRecoverySnapshot {
    demand: UiGlyphRasterDemandIdentity,
    generation: UiNativeTextAtlasGeneration,
    lineage: UiNativeTextAtlasLineageIdentity,
}

impl UiNativeTextAtlasRecoverySnapshot {
    pub const fn demand_identity(self) -> UiGlyphRasterDemandIdentity {
        self.demand
    }

    pub const fn generation(self) -> UiNativeTextAtlasGeneration {
        self.generation
    }

    pub const fn lineage_identity(self) -> UiNativeTextAtlasLineageIdentity {
        self.lineage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovery_is_move_only_and_snapshot_is_copyable() {
        let recovery = UiNativeTextAtlasRecovery::from_native_host(
            UiGlyphRasterDemandIdentity::from_text_mechanics([4; 32]),
            UiNativeTextAtlasGeneration::new(2).unwrap(),
            UiNativeTextAtlasLineageIdentity::from_native_host(1).unwrap(),
        );
        let snapshot = recovery.snapshot();
        let copied = snapshot;
        assert_eq!(copied.generation().get(), 2);
        assert_eq!(
            UiNativeTextAtlasDenial::StalePlan,
            UiNativeTextAtlasDenial::StalePlan
        );
        let _consumed = recovery;
    }
}
