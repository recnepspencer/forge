use forge_store_budgets::CounterEvidenceStrength;

use super::intent::BlobPlacementClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobPlacementCounterSnapshot {
    strength: CounterEvidenceStrength,
    placement_class: Option<BlobPlacementClass>,
    inline_reads: u64,
    external_reads: u64,
    cold_fetches: u64,
    unavailable_cold_chunks: u64,
    placement_moves: u64,
    tier_move_protected_denials: u64,
}

impl BlobPlacementCounterSnapshot {
    pub const fn start() -> Self {
        Self {
            strength: CounterEvidenceStrength::Exact,
            placement_class: None,
            inline_reads: 0,
            external_reads: 0,
            cold_fetches: 0,
            unavailable_cold_chunks: 0,
            placement_moves: 0,
            tier_move_protected_denials: 0,
        }
    }

    pub const fn for_class(placement_class: BlobPlacementClass) -> Self {
        Self {
            placement_class: Some(placement_class),
            ..Self::start()
        }
    }

    pub const fn record_inline_read(mut self) -> Self {
        self.inline_reads += 1;
        self
    }

    pub const fn record_external_read(mut self) -> Self {
        self.external_reads += 1;
        self
    }

    pub const fn record_cold_fetch(mut self) -> Self {
        self.cold_fetches += 1;
        self
    }

    pub const fn record_unavailable_cold_chunk(mut self) -> Self {
        self.unavailable_cold_chunks += 1;
        self
    }

    pub const fn record_placement_move(mut self) -> Self {
        self.placement_moves += 1;
        self
    }

    pub const fn record_tier_move_protected_denial(mut self) -> Self {
        self.tier_move_protected_denials += 1;
        self
    }

    pub const fn inline_reads(self) -> u64 {
        self.inline_reads
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.strength
    }

    pub const fn placement_class(self) -> Option<BlobPlacementClass> {
        self.placement_class
    }

    pub const fn external_reads(self) -> u64 {
        self.external_reads
    }

    pub const fn cold_fetches(self) -> u64 {
        self.cold_fetches
    }

    pub const fn unavailable_cold_chunks(self) -> u64 {
        self.unavailable_cold_chunks
    }

    pub const fn placement_moves(self) -> u64 {
        self.placement_moves
    }

    pub const fn tier_move_protected_denials(self) -> u64 {
        self.tier_move_protected_denials
    }
}
