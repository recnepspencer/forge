use super::super::*;
use worth_store_physical_format::RecordFrameCoordinate;

/// Proof that one frame was selected from the pool's legal eviction order
/// while the pool state remained exclusively locked.
///
/// Only the eviction selector can construct this token. Release paths consume
/// it, so a raw coordinate cannot invoke eviction execution.
#[derive(Debug)]
pub(in crate::physical_residency::pool) struct LegalEvictionVictim {
    coordinate: RecordFrameCoordinate,
}

impl LegalEvictionVictim {
    pub(super) const fn coordinate(&self) -> RecordFrameCoordinate {
        self.coordinate
    }
}

impl PoolState {
    pub(in crate::physical_residency::pool) fn select_oldest_legal_victim(
        &mut self,
    ) -> Option<LegalEvictionVictim> {
        let coordinate = self.evictable_head?;
        let candidate = self
            .frames
            .get(&coordinate)
            .expect("an eviction-order identity remains resident");
        assert!(
            candidate.is_evictable(),
            "eviction order contained an illegal victim"
        );
        self.detach_evictable(coordinate);
        Some(LegalEvictionVictim { coordinate })
    }
}
