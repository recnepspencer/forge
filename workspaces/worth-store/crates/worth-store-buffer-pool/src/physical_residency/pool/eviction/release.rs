use super::super::*;
use super::LegalEvictionVictim;

impl PoolState {
    pub(in crate::physical_residency::pool) fn drain_all_legal_clean_frames(&mut self) -> u64 {
        let mut drained = 0;
        while let Some(victim) = self.select_oldest_legal_victim() {
            self.drain_selected_clean_frame(victim);
            drained += 1;
        }
        drained
    }

    pub(in crate::physical_residency::pool) fn evict_selected_victim(
        &mut self,
        victim: LegalEvictionVictim,
    ) {
        let removed = self
            .frames
            .remove(&victim.coordinate())
            .expect("a selected legal eviction victim remains resident");
        self.accounting.remove_frame(
            removed.bytes,
            removed.pins,
            removed.dirty,
            removed.origin.is_candidate(),
        );
        self.accounting.record_eviction();
    }

    pub(in crate::physical_residency::pool) fn drain_selected_clean_frame(
        &mut self,
        victim: LegalEvictionVictim,
    ) {
        let removed = self
            .frames
            .remove(&victim.coordinate())
            .expect("a selected clean drain victim remains resident");
        self.accounting.remove_frame(
            removed.bytes,
            removed.pins,
            removed.dirty,
            removed.origin.is_candidate(),
        );
        self.accounting.record_administrative_drain();
    }
}

const _: fn(&mut PoolState, LegalEvictionVictim) = PoolState::evict_selected_victim;
