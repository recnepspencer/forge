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
    }

    pub(crate) fn merge_slot_chunks_from_owned(
        &mut self,
        overlay: &mut Self,
        touched_slots: &BTreeSet<usize>,
        chunk_width: usize,
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
        chunk_count
    }

    fn move_slot_from_overlay(&mut self, overlay: &mut Self, slot: usize)
    where
        K::Extra: Clone,
        K::Meta: Clone,
    {
        let overlay_physical = overlay
            .physical_index(slot)
            .expect("touched overlay slot must be materialized");
        let Some(physical) = self.physical_index(slot) else {
            self.slots
                .insert(slot as u64)
                .expect("new publication slot must be unique");
            self.partition_ids
                .push(overlay.partition_ids[overlay_physical]);
            self.generations.push(overlay.generations[overlay_physical]);
            self.lifecycle.push(overlay.lifecycle[overlay_physical]);
            self.kind_ids.push(overlay.kind_ids[overlay_physical]);
            self.metadata_history.push(std::mem::take(
                &mut overlay.metadata_history[overlay_physical],
            ));
            self.created_at.push(overlay.created_at[overlay_physical]);
            self.retired_at.push(overlay.retired_at[overlay_physical]);
            self.extra.push(std::mem::replace(
                &mut overlay.extra[overlay_physical],
                K::empty_extra(),
            ));
            self.aspect_versions.push(std::mem::take(
                &mut overlay.aspect_versions[overlay_physical],
            ));
            self.diagnostics_enrichment.push(std::mem::take(
                &mut overlay.diagnostics_enrichment[overlay_physical],
            ));
            self.branch_pins.push(0);
            self.replay_pins.push(0);
            self.snapshot_pins.push(0);
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
            return;
        };

        let retains_runtime_pins =
            self.generations[physical] == overlay.generations[overlay_physical];
        if !retains_runtime_pins {
            debug_assert_eq!(self.branch_pins[physical], 0);
            debug_assert_eq!(self.replay_pins[physical], 0);
            debug_assert_eq!(self.snapshot_pins[physical], 0);
        }
        self.partition_ids[physical] = overlay.partition_ids[overlay_physical];
        self.generations[physical] = overlay.generations[overlay_physical];
        self.lifecycle[physical] = overlay.lifecycle[overlay_physical];
        self.kind_ids[physical] = overlay.kind_ids[overlay_physical];
        self.metadata_history[physical] =
            std::mem::take(&mut overlay.metadata_history[overlay_physical]);
        self.created_at[physical] = overlay.created_at[overlay_physical];
        self.retired_at[physical] = overlay.retired_at[overlay_physical];
        self.extra[physical] =
            std::mem::replace(&mut overlay.extra[overlay_physical], K::empty_extra());
        self.aspect_versions[physical] =
            std::mem::take(&mut overlay.aspect_versions[overlay_physical]);
        self.diagnostics_enrichment[physical] =
            std::mem::take(&mut overlay.diagnostics_enrichment[overlay_physical]);
        // Pin counts belong to the live storage owner, not to a transaction's
        // sparsely cloned publication overlay. Preserve counts which may have
        // advanced after the overlay was opened. A new generation starts with
        // no inherited obligations; allocator admission proves the displaced
        // generation was reclaimable before reuse.
        if !retains_runtime_pins {
            self.branch_pins[physical] = 0;
            self.replay_pins[physical] = 0;
            self.snapshot_pins[physical] = 0;
        }
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
