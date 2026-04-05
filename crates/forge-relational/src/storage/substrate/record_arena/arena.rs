use std::collections::{BTreeMap, BTreeSet};

use crate::identity::data::{KindId, PartitionId, RecordId, VersionId};
use crate::payloads::data::RecordPayload;
use crate::storage::data::RecordLifecycleState;
use crate::storage::partition::DenseSlotBitSet;
use crate::symbols::data::Symbol;

use super::{
    slot_view::SlotView, EntityRecordKind, LifecycleCounts, RecordKind, RelationRecordKind,
    VersionedPayload,
};

#[derive(Debug)]
pub(crate) struct SlotInit<K: RecordKind> {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub payload: Option<RecordPayload>,
    pub version_id: VersionId,
    pub extra: K::Extra,
}

#[derive(Debug, Clone)]
pub(crate) struct RecordArena<K: RecordKind> {
    pub(crate) partition_ids: Vec<PartitionId>,
    pub(crate) generations: Vec<u32>,
    pub(crate) lifecycle: Vec<RecordLifecycleState>,
    pub(crate) kind_ids: Vec<Option<KindId>>,
    pub(crate) payloads: Vec<Option<RecordPayload>>,
    pub(crate) payload_history: Vec<Vec<VersionedPayload>>,
    pub(crate) metadata_history: Vec<Vec<K::Meta>>,
    pub(crate) created_at: Vec<VersionId>,
    pub(crate) retired_at: Vec<Option<VersionId>>,
    pub(crate) extra: Vec<K::Extra>,
    pub(crate) aspect_versions: Vec<BTreeMap<Symbol, u64>>,
    pub(crate) diagnostics_enrichment: Vec<BTreeMap<Symbol, String>>,
    pub(crate) branch_pins: Vec<u32>,
    pub(crate) replay_pins: Vec<u32>,
    pub(crate) snapshot_pins: Vec<u32>,
    pub(crate) live_bitset: DenseSlotBitSet,
    pub(crate) reclaimable_bitset: DenseSlotBitSet,
    pub(crate) free_list: Vec<u64>,
}

impl<K: RecordKind> RecordArena<K> {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            partition_ids: Vec::with_capacity(capacity),
            generations: Vec::with_capacity(capacity),
            lifecycle: Vec::with_capacity(capacity),
            kind_ids: Vec::with_capacity(capacity),
            payloads: Vec::with_capacity(capacity),
            payload_history: Vec::with_capacity(capacity),
            metadata_history: Vec::with_capacity(capacity),
            created_at: Vec::with_capacity(capacity),
            retired_at: Vec::with_capacity(capacity),
            extra: Vec::with_capacity(capacity),
            aspect_versions: Vec::with_capacity(capacity),
            diagnostics_enrichment: Vec::with_capacity(capacity),
            branch_pins: Vec::with_capacity(capacity),
            replay_pins: Vec::with_capacity(capacity),
            snapshot_pins: Vec::with_capacity(capacity),
            live_bitset: DenseSlotBitSet::with_capacity(capacity),
            reclaimable_bitset: DenseSlotBitSet::with_capacity(capacity),
            free_list: Vec::new(),
        }
    }

    pub(crate) fn reserve_additional(&mut self, additional: usize) {
        /* unchanged */
        self.partition_ids.reserve(additional);
        self.generations.reserve(additional);
        self.lifecycle.reserve(additional);
        self.kind_ids.reserve(additional);
        self.payloads.reserve(additional);
        self.payload_history.reserve(additional);
        self.metadata_history.reserve(additional);
        self.created_at.reserve(additional);
        self.retired_at.reserve(additional);
        K::reserve_extra(&mut self.extra, additional);
        self.aspect_versions.reserve(additional);
        self.diagnostics_enrichment.reserve(additional);
        self.branch_pins.reserve(additional);
        self.replay_pins.reserve(additional);
        self.snapshot_pins.reserve(additional);
        self.free_list.reserve(additional);
    }

    pub(crate) fn apply_payload_update(
        &mut self,
        slot: usize,
        payload: RecordPayload,
        version_id: VersionId,
    ) {
        let payload = payload.canonicalized();
        self.payloads[slot] = Some(payload.clone());
        if let Some(current) = self.payload_history[slot].last_mut() {
            current.retired_at = Some(version_id);
        }
        self.payload_history[slot].push(VersionedPayload {
            effective_at: version_id,
            retired_at: None,
            generation: self.generations[slot],
            value: payload,
        });
    }

    pub(crate) fn retire(&mut self, slot: usize, version_id: VersionId) {
        self.retired_at[slot] = Some(version_id);
        self.lifecycle[slot] = RecordLifecycleState::DeletedRetained;
        self.live_bitset.set(slot, false);
        self.reclaimable_bitset.set(slot, true);
        if let Some(current) = self
            .payload_history
            .get_mut(slot)
            .and_then(|history| history.last_mut())
        {
            current.retired_at = Some(version_id);
        }
        if let Some(current) = self.metadata_history[slot].last_mut() {
            K::retire_metadata(current, version_id);
        }
    }

    pub(crate) fn lifecycle_counts(&self) -> LifecycleCounts {
        lifecycle_counts(&self.lifecycle)
    }

    pub(crate) fn push_slot(&mut self, init: SlotInit<K>) -> (usize, u32, bool) {
        let SlotInit {
            partition_id,
            kind_id,
            payload,
            version_id,
            extra,
        } = init;
        let canonical_payload = payload.map(|value| value.canonicalized());
        if let Some(slot) = self.free_list.pop() {
            let idx = slot as usize;
            if let Some(current) = self.metadata_history[idx].last_mut() {
                K::retire_metadata(current, version_id);
            }
            if let Some(current) = self.payload_history[idx].last_mut() {
                current.retired_at = Some(version_id);
            }
            self.partition_ids[idx] = partition_id;
            self.generations[idx] += 1;
            self.lifecycle[idx] = RecordLifecycleState::Live;
            self.kind_ids[idx] = Some(kind_id);
            self.payloads[idx] = canonical_payload.clone();
            if let Some(payload) = canonical_payload {
                self.payload_history[idx].push(VersionedPayload {
                    effective_at: version_id,
                    retired_at: None,
                    generation: self.generations[idx],
                    value: payload,
                });
            }
            self.metadata_history[idx].push(K::metadata_for_create(
                kind_id,
                self.generations[idx],
                version_id,
                &extra,
            ));
            self.created_at[idx] = version_id;
            self.retired_at[idx] = None;
            self.extra[idx] = extra;
            self.aspect_versions[idx].clear();
            self.diagnostics_enrichment[idx].clear();
            self.branch_pins[idx] = 0;
            self.replay_pins[idx] = 0;
            self.snapshot_pins[idx] = 0;
            self.live_bitset.set(idx, true);
            self.reclaimable_bitset.set(idx, false);
            return (idx, self.generations[idx], true);
        }

        let slot = self.generations.len();
        self.partition_ids.push(partition_id);
        self.generations.push(1);
        self.lifecycle.push(RecordLifecycleState::Live);
        self.kind_ids.push(Some(kind_id));
        self.payloads.push(canonical_payload.clone());
        self.payload_history.push(Vec::new());
        if let Some(payload) = canonical_payload {
            self.payload_history[slot].push(VersionedPayload {
                effective_at: version_id,
                retired_at: None,
                generation: 1,
                value: payload,
            });
        }
        self.metadata_history
            .push(vec![K::metadata_for_create(kind_id, 1, version_id, &extra)]);
        self.created_at.push(version_id);
        self.retired_at.push(None);
        self.extra.push(extra);
        self.aspect_versions.push(BTreeMap::new());
        self.diagnostics_enrichment.push(BTreeMap::new());
        self.branch_pins.push(0);
        self.replay_pins.push(0);
        self.snapshot_pins.push(0);
        self.live_bitset.set(slot, true);
        self.reclaimable_bitset.set(slot, false);
        (slot, 1, false)
    }

    pub(crate) fn sparse_clone_slots_for_overlay(
        &self,
        touched_slots: &std::collections::BTreeSet<usize>,
    ) -> Self
    where
        K::Extra: Clone,
        K::Meta: Clone,
    {
        let slot_count = self.slot_count();
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

        clone.payloads.resize(slot_count, None);
        clone.payload_history.resize_with(slot_count, Vec::new);
        clone.metadata_history.resize_with(slot_count, Vec::new);
        clone.extra.resize_with(slot_count, K::empty_extra);
        clone.aspect_versions.resize_with(slot_count, BTreeMap::new);
        clone
            .diagnostics_enrichment
            .resize_with(slot_count, BTreeMap::new);

        for &slot in touched_slots {
            if slot >= slot_count {
                continue;
            }
            clone.payloads[slot] = self.payloads[slot].clone();
            clone.payload_history[slot] = self.payload_history[slot].clone();
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
        let slot_count = self.slot_count();
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

        clone.payloads.resize(slot_count, None);
        clone.payload_history.resize_with(slot_count, Vec::new);
        clone.metadata_history.resize_with(slot_count, Vec::new);
        clone.extra.resize_with(slot_count, K::empty_extra);
        clone.aspect_versions.resize_with(slot_count, BTreeMap::new);
        clone
            .diagnostics_enrichment
            .resize_with(slot_count, BTreeMap::new);

        clone
    }

    pub(crate) fn reset_slot(&mut self, slot: usize) {
        self.kind_ids[slot] = None;
        self.payloads[slot] = None;
        self.extra[slot] = K::empty_extra();
        self.aspect_versions[slot].clear();
        self.diagnostics_enrichment[slot].clear();
        self.branch_pins[slot] = 0;
        self.replay_pins[slot] = 0;
        self.snapshot_pins[slot] = 0;
        self.retired_at[slot] = None;
        self.free_list.push(slot as u64);
    }

    pub(crate) fn get(&self, id: &RecordId<K::Domain>) -> Option<SlotView<'_, K>> {
        let slot = super::slot_of::<K>(id);
        self.get_slot(slot).filter(|view| {
            view.generation() == super::generation_of::<K>(id)
                && view.partition_id() == super::partition_of::<K>(id)
        })
    }

    pub(crate) fn get_slot(&self, slot: usize) -> Option<SlotView<'_, K>> {
        (slot < self.generations.len()).then(|| SlotView::new(self, slot))
    }

    pub(crate) fn slot_count(&self) -> usize {
        self.generations.len()
    }
    pub(crate) fn retired_at_for_slot(&self, slot: usize) -> Option<VersionId> {
        self.retired_at.get(slot).copied().flatten()
    }
    pub(crate) fn payload_history_at(&self, slot: usize) -> Option<&[VersionedPayload]> {
        self.payload_history.get(slot).map(Vec::as_slice)
    }
    pub(crate) fn payload_history_at_mut(
        &mut self,
        slot: usize,
    ) -> Option<&mut Vec<VersionedPayload>> {
        self.payload_history.get_mut(slot)
    }
    pub(crate) fn metadata_history_at(&self, slot: usize) -> Option<&[K::Meta]> {
        self.metadata_history.get(slot).map(Vec::as_slice)
    }
    pub(crate) fn metadata_history_at_mut(&mut self, slot: usize) -> Option<&mut Vec<K::Meta>> {
        self.metadata_history.get_mut(slot)
    }
    pub(crate) fn aspect_versions_at(&self, slot: usize) -> Option<&BTreeMap<Symbol, u64>> {
        self.aspect_versions.get(slot)
    }
    pub(crate) fn snapshot_pin_count(&self, slot: usize) -> Option<u32> {
        self.snapshot_pins.get(slot).copied()
    }
    pub(crate) fn branch_pin_count(&self, slot: usize) -> Option<u32> {
        self.branch_pins.get(slot).copied()
    }
    pub(crate) fn replay_pin_count(&self, slot: usize) -> Option<u32> {
        self.replay_pins.get(slot).copied()
    }

    pub(crate) fn increment_snapshot_pin(&mut self, slot: usize) -> Option<u32> {
        let count = self.snapshot_pins.get_mut(slot)?;
        *count = count.saturating_add(1);
        Some(*count)
    }

    pub(crate) fn decrement_snapshot_pin(&mut self, slot: usize) -> Option<u32> {
        let count = self.snapshot_pins.get_mut(slot)?;
        if *count == 0 {
            return None;
        }
        *count -= 1;
        Some(*count)
    }

    pub(crate) fn adjust_named_pin(&mut self, slot: usize, class: PinClass) -> Option<&mut u32> {
        match class {
            PinClass::Branch => self.branch_pins.get_mut(slot),
            PinClass::Replay => self.replay_pins.get_mut(slot),
        }
    }

    pub(crate) fn increment_named_pins_bulk(&mut self, slots: &BTreeSet<usize>, class: PinClass) {
        let pins = match class {
            PinClass::Branch => &mut self.branch_pins,
            PinClass::Replay => &mut self.replay_pins,
        };
        for &slot in slots {
            let Some(pin_count) = pins.get_mut(slot) else {
                continue;
            };
            *pin_count = pin_count.saturating_add(1);
        }
    }

    pub(crate) fn set_lifecycle_for_slot(
        &mut self,
        slot: usize,
        lifecycle: RecordLifecycleState,
    ) -> bool {
        let Some(current) = self.lifecycle.get_mut(slot) else {
            return false;
        };
        *current = lifecycle;
        true
    }

    pub(crate) fn clear_all_pins(&mut self) {
        self.snapshot_pins.fill(0);
        self.branch_pins.fill(0);
        self.replay_pins.fill(0);
    }

    pub(crate) fn clear_named_pins(&mut self, class: PinClass) {
        match class {
            PinClass::Branch => self.branch_pins.fill(0),
            PinClass::Replay => self.replay_pins.fill(0),
        }
    }

    pub(crate) fn contains_live_id(&self, id: &RecordId<K::Domain>) -> bool {
        self.get(id).is_some_and(|view| view.is_live())
    }

    pub(crate) fn merge_slots_from_owned(
        &mut self,
        overlay: &mut Self,
        touched_slots: &std::collections::BTreeSet<usize>,
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

        // Free-slot reuse is arena-global bookkeeping. Once a flat entity commit touches
        // entity slots, keep the authoritative free-list aligned with the committed overlay.
        if sync_free_list {
            self.free_list = std::mem::take(&mut overlay.free_list);
        }
    }

    pub(crate) fn merge_slot_chunks_from_owned(
        &mut self,
        overlay: &mut Self,
        touched_slots: &std::collections::BTreeSet<usize>,
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
            self.payloads.push(overlay.payloads[next].clone());
            self.payload_history
                .push(overlay.payload_history[next].clone());
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
        self.payloads[slot] = overlay.payloads[slot].take();
        self.payload_history[slot] = std::mem::take(&mut overlay.payload_history[slot]);
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

pub(crate) type EntityArena = RecordArena<EntityRecordKind>;
pub(crate) type RelationArena = RecordArena<RelationRecordKind>;

#[derive(Clone, Copy)]
pub(crate) enum PinClass {
    Branch,
    Replay,
}

pub(crate) fn lifecycle_counts(lifecycle: &[RecordLifecycleState]) -> LifecycleCounts {
    let mut counts = LifecycleCounts::default();
    for state in lifecycle {
        match state {
            RecordLifecycleState::Live => counts.live += 1,
            RecordLifecycleState::Reusable => counts.reusable += 1,
            RecordLifecycleState::DeletedRetained
            | RecordLifecycleState::RetainedDanglingForAudit
            | RecordLifecycleState::PinnedBySnapshot
            | RecordLifecycleState::PinnedByBranch
            | RecordLifecycleState::PinnedByReplayRetention
            | RecordLifecycleState::Reclaimable => counts.deleted += 1,
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::identity::data::{KindId, PartitionId, VersionId};
    use crate::payloads::data::RecordPayload;

    use super::{EntityArena, EntityRecordKind, RecordKind, SlotInit};

    #[test]
    fn chunked_owned_slot_merge_preserves_untouched_truth() {
        let partition_id = PartitionId(1);
        let mut base = EntityArena::with_capacity(2);
        let _ = base.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(1),
            payload: Some(RecordPayload::StructuredJson(
                serde_json::json!({"name":"left"}),
            )),
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });
        let _ = base.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(1),
            payload: Some(RecordPayload::StructuredJson(
                serde_json::json!({"name":"right"}),
            )),
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });

        let mut overlay = base.sparse_clone_slots_for_overlay(&BTreeSet::from([1usize]));
        overlay.apply_payload_update(
            1,
            RecordPayload::StructuredJson(serde_json::json!({"name":"right-updated"})),
            VersionId(2),
        );

        let published_chunks =
            base.merge_slot_chunks_from_owned(&mut overlay, &BTreeSet::from([1usize]), 128, false);

        assert_eq!(published_chunks, 1);
        assert_eq!(
            base.get_slot(0).and_then(|slot| slot.payload().cloned()),
            Some(RecordPayload::StructuredJson(
                serde_json::json!({"name":"left"})
            ))
        );
        assert_eq!(
            base.get_slot(1).and_then(|slot| slot.payload().cloned()),
            Some(RecordPayload::StructuredJson(
                serde_json::json!({"name":"right-updated"})
            ))
        );
    }
}
