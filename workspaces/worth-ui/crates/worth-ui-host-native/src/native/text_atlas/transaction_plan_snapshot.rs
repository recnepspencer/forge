//! Copyable inspection of one native atlas transaction plan.

use worth_ui_host_contract::UiGlyphRasterDemandIdentity;

use super::recovery::UiNativeTextAtlasGeneration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeTextAtlasTransactionPlanSnapshot {
    pub(super) demand_identity: UiGlyphRasterDemandIdentity,
    pub(super) peak_entries: u32,
    pub(super) peak_texel_bytes: u64,
    pub(super) predecessor_generation: UiNativeTextAtlasGeneration,
    pub(super) candidate_generation: UiNativeTextAtlasGeneration,
    pub(super) misses: u32,
    pub(super) hits: u32,
    pub(super) evictions: u32,
    pub(super) staged_bytes: u64,
    pub(super) physical_staged_bytes: u64,
    pub(super) key_lookups: u32,
    pub(super) page_probes: u32,
    pub(super) placement_probes: u32,
    pub(super) eviction_candidates: u32,
}

impl UiNativeTextAtlasTransactionPlanSnapshot {
    pub(crate) const fn demand_identity(self) -> UiGlyphRasterDemandIdentity {
        self.demand_identity
    }

    pub(crate) const fn predecessor_generation(self) -> UiNativeTextAtlasGeneration {
        self.predecessor_generation
    }

    pub(crate) const fn peak_entries(self) -> u32 {
        self.peak_entries
    }

    pub(crate) const fn peak_texel_bytes(self) -> u64 {
        self.peak_texel_bytes
    }

    pub(crate) const fn candidate_generation(self) -> UiNativeTextAtlasGeneration {
        self.candidate_generation
    }

    pub(crate) const fn misses(self) -> u32 {
        self.misses
    }

    pub(crate) const fn hits(self) -> u32 {
        self.hits
    }

    pub(crate) const fn evictions(self) -> u32 {
        self.evictions
    }

    pub(crate) const fn staged_bytes(self) -> u64 {
        self.staged_bytes
    }

    pub(crate) const fn physical_staged_bytes(self) -> u64 {
        self.physical_staged_bytes
    }

    pub(crate) const fn key_lookups(self) -> u32 {
        self.key_lookups
    }
    pub(crate) const fn page_probes(self) -> u32 {
        self.page_probes
    }
    pub(crate) const fn placement_probes(self) -> u32 {
        self.placement_probes
    }
    pub(crate) const fn eviction_candidates(self) -> u32 {
        self.eviction_candidates
    }
}
