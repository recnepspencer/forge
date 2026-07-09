use std::collections::BTreeSet;

use super::{RecordArena, RecordKind};

impl<K: RecordKind> RecordArena<K> {
    pub(crate) fn contains_live_id(&self, id: &crate::identity::data::RecordId<K::Domain>) -> bool {
        self.get(id).is_some_and(|view| view.is_live())
    }

    pub(crate) fn merge_slots_from_owned(
        &mut self,
        overlay: &mut Self,
        touched_slots: &BTreeSet<usize>,
        sync_free_list: bool,
    ) where
        K::Extra: Clone,
        K::Meta: Clone,
    {
        if touched_slots.is_empty() {
            return;
        }

        for &slot in touched_slots {
            self.move_slot_from_overlay(overlay, slot);
        }

        if sync_free_list {
            self.free_list = std::mem::take(&mut overlay.free_list);
        }
    }

    pub(crate) fn merge_slot_chunks_from_owned(
        &mut self,
        overlay: &mut Self,
        touched_slots: &BTreeSet<usize>,
        chunk_width: usize,
        sync_free_list: bool,
    ) -> usize
    where
        K::Extra: Clone,
        K::Meta: Clone,
    {
        if touched_slots.is_empty() {
            return 0;
        }

        let chunk_width = chunk_width.max(1);
        let mut chunk_count = 0usize;
        let mut current_chunk = None;
        for &slot in touched_slots {
            let chunk_index = slot / chunk_width;
            if current_chunk != Some(chunk_index) {
                current_chunk = Some(chunk_index);
                chunk_count += 1;
            }
            self.move_slot_from_overlay(overlay, slot);
        }

        if sync_free_list {
            self.free_list = std::mem::take(&mut overlay.free_list);
        }

        chunk_count
    }

    fn move_slot_from_overlay(&mut self, overlay: &mut Self, slot: usize)
    where
        K::Extra: Clone,
        K::Meta: Clone,
    {
        while self.generations.len() <= slot {
            let next = self.generations.len();
            self.partition_ids.push(overlay.partition_ids[next]);
            self.generations.push(overlay.generations[next]);
            self.lifecycle.push(overlay.lifecycle[next]);
            self.kind_ids.push(overlay.kind_ids[next]);
            self.metadata_history
                .push(overlay.metadata_history[next].clone());
            self.created_at.push(overlay.created_at[next]);
            self.retired_at.push(overlay.retired_at[next]);
            self.extra.push(overlay.extra[next].clone());
            self.aspect_versions
                .push(overlay.aspect_versions[next].clone());
            self.diagnostics_enrichment
                .push(overlay.diagnostics_enrichment[next].clone());
            self.branch_pins.push(overlay.branch_pins[next]);
            self.replay_pins.push(overlay.replay_pins[next]);
            self.snapshot_pins.push(overlay.snapshot_pins[next]);
            self.live_bitset.set(
                next,
                overlay.live_bitset.count_ones_in_range(next, next + 1) == 1,
            );
            self.reclaimable_bitset.set(
                next,
                overlay
                    .reclaimable_bitset
                    .count_ones_in_range(next, next + 1)
                    == 1,
            );
        }

        self.partition_ids[slot] = overlay.partition_ids[slot];
        self.generations[slot] = overlay.generations[slot];
        self.lifecycle[slot] = overlay.lifecycle[slot];
        self.kind_ids[slot] = overlay.kind_ids[slot];
        self.metadata_history[slot] = std::mem::take(&mut overlay.metadata_history[slot]);
        self.created_at[slot] = overlay.created_at[slot];
        self.retired_at[slot] = overlay.retired_at[slot];
        self.extra[slot] = std::mem::replace(&mut overlay.extra[slot], K::empty_extra());
        self.aspect_versions[slot] = std::mem::take(&mut overlay.aspect_versions[slot]);
        self.diagnostics_enrichment[slot] =
            std::mem::take(&mut overlay.diagnostics_enrichment[slot]);
        self.branch_pins[slot] = overlay.branch_pins[slot];
        self.replay_pins[slot] = overlay.replay_pins[slot];
        self.snapshot_pins[slot] = overlay.snapshot_pins[slot];
        self.live_bitset.set(
            slot,
            overlay.live_bitset.count_ones_in_range(slot, slot + 1) == 1,
        );
        self.reclaimable_bitset.set(
            slot,
            overlay
                .reclaimable_bitset
                .count_ones_in_range(slot, slot + 1)
                == 1,
        );
    }
}
