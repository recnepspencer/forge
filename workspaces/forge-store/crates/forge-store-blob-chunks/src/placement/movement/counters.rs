use forge_store_budgets::CounterEvidenceStrength;

use crate::BlobPlacementClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobPlacementMovementCounterSnapshot {
    strength: CounterEvidenceStrength,
    source_class: BlobPlacementClass,
    target_class: BlobPlacementClass,
    placement_moves: u64,
    inline_reads: u64,
    external_reads: u64,
    cold_fetches: u64,
    unavailable_cold_chunks: u64,
    tier_move_retries: u64,
    protected_denials: u64,
    execution_receipts: u64,
    published_observations: u64,
}

impl BlobPlacementMovementCounterSnapshot {
    pub const fn start(source_class: BlobPlacementClass, target_class: BlobPlacementClass) -> Self {
        Self {
            strength: CounterEvidenceStrength::Exact,
            source_class,
            target_class,
            placement_moves: 0,
            inline_reads: 0,
            external_reads: 0,
            cold_fetches: 0,
            unavailable_cold_chunks: 0,
            tier_move_retries: 0,
            protected_denials: 0,
            execution_receipts: 0,
            published_observations: 0,
        }
    }

    pub const fn record_read(mut self, class: BlobPlacementClass) -> Self {
        match class {
            BlobPlacementClass::Inline => self.inline_reads += 1,
            BlobPlacementClass::External => self.external_reads += 1,
            BlobPlacementClass::Cold => self.cold_fetches += 1,
        }
        self
    }

    pub const fn record_move(mut self) -> Self {
        self.placement_moves += 1;
        self
    }

    pub const fn record_unavailable_cold_chunk(mut self) -> Self {
        self.unavailable_cold_chunks += 1;
        self
    }

    pub const fn record_tier_move_retry(mut self) -> Self {
        self.tier_move_retries += 1;
        self
    }

    pub const fn record_protected_denial(mut self) -> Self {
        self.protected_denials += 1;
        self
    }

    pub const fn record_execution_receipt(mut self) -> Self {
        self.execution_receipts += 1;
        self
    }

    pub const fn record_published_observation(mut self) -> Self {
        self.published_observations += 1;
        self
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.strength
    }

    pub const fn source_class(self) -> BlobPlacementClass {
        self.source_class
    }

    pub const fn target_class(self) -> BlobPlacementClass {
        self.target_class
    }

    pub const fn placement_moves(self) -> u64 {
        self.placement_moves
    }

    pub const fn inline_reads(self) -> u64 {
        self.inline_reads
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

    pub const fn tier_move_retries(self) -> u64 {
        self.tier_move_retries
    }

    pub const fn protected_denials(self) -> u64 {
        self.protected_denials
    }

    pub const fn execution_receipts(self) -> u64 {
        self.execution_receipts
    }

    pub const fn published_observations(self) -> u64 {
        self.published_observations
    }
}
