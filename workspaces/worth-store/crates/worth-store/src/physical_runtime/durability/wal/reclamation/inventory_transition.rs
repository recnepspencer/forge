use super::super::inventory::PhysicalWalSegmentInventoryEntry;
use super::super::PhysicalWalRuntimeOwner;
use crate::physical_runtime::CompletedPhysicalWalReclamationAction;

impl PhysicalWalRuntimeOwner {
    pub(super) fn complete_reclamation(
        &self,
        expected: PhysicalWalSegmentInventoryEntry,
        completed: &CompletedPhysicalWalReclamationAction,
    ) -> bool {
        if completed.segment() != expected.identity()
            || completed.lsn_range() != expected.lsn_range()
            || completed.byte_count() != expected.byte_count()
        {
            self.seal_for_inspection();
            return false;
        }
        let mut state = self
            .shared
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if state.segments.consume_reclaimed_head(expected).is_err() {
            state.sealed = true;
            return false;
        }
        state.segment_count = state.segment_count.saturating_sub(1);
        state.reclaimed_segments = state.reclaimed_segments.saturating_add(1);
        state.reclaimed_bytes = state.reclaimed_bytes.saturating_add(expected.byte_count());
        true
    }
}
