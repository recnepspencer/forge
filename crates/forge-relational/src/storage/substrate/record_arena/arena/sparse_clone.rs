use std::collections::BTreeMap;

use super::{RecordArena, RecordKind};

impl<K: RecordKind> RecordArena<K> {
    pub(crate) fn sparse_clone_slots_for_overlay(
        &self,
        touched_slots: &std::collections::BTreeSet<usize>,
    ) -> Self
    where
        K::Extra: Clone,
        K::Meta: Clone,
    {
        let slot_count = self.slot_count();
        let mut clone = self.sparse_overlay_shell(slot_count);

        for &slot in touched_slots {
            if slot >= slot_count {
                continue;
            }
            clone.metadata_history[slot] = self.metadata_history[slot].clone();
            clone.extra[slot] = self.extra[slot].clone();
            clone.aspect_versions[slot] = self.aspect_versions[slot].clone();
            clone.diagnostics_enrichment[slot] = self.diagnostics_enrichment[slot].clone();
        }

        clone
    }

    pub(crate) fn sparse_shape_clone_for_overlay(&self) -> Self
    where
        K::Extra: Clone,
        K::Meta: Clone,
    {
        self.sparse_overlay_shell(self.slot_count())
    }

    fn sparse_overlay_shell(&self, slot_count: usize) -> Self
    where
        K::Extra: Clone,
        K::Meta: Clone,
    {
        let mut clone = Self::with_capacity(slot_count);
        clone.partition_ids.clone_from(&self.partition_ids);
        clone.generations.clone_from(&self.generations);
        clone.lifecycle.clone_from(&self.lifecycle);
        clone.kind_ids.clone_from(&self.kind_ids);
        clone.created_at.clone_from(&self.created_at);
        clone.retired_at.clone_from(&self.retired_at);
        clone.branch_pins.clone_from(&self.branch_pins);
        clone.replay_pins.clone_from(&self.replay_pins);
        clone.snapshot_pins.clone_from(&self.snapshot_pins);
        clone.live_bitset = self.live_bitset.clone();
        clone.reclaimable_bitset = self.reclaimable_bitset.clone();
        clone.free_list = self.free_list.clone();

        clone.metadata_history.resize_with(slot_count, Vec::new);
        clone.extra.resize_with(slot_count, K::empty_extra);
        clone.aspect_versions.resize_with(slot_count, BTreeMap::new);
        clone
            .diagnostics_enrichment
            .resize_with(slot_count, BTreeMap::new);

        clone
    }
}
