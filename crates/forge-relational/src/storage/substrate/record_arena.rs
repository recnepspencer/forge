use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use crate::identity::data::{
    EntityId, KindId, LineageId, PartitionId, RelationId, StructuralFingerprint,
    VersionBound, VersionId,
};
use crate::payloads::data::RecordPayload;
use crate::storage::overlay::PartitionState;
use crate::storage::partition::DenseSlotBitSet;
use crate::storage::data::{
    EntityReadRecord, RecordLifecycleState, RelationReadRecord,
};
use crate::symbols::data::Symbol;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct LifecycleCounts {
    pub(crate) live: usize,
    pub(crate) deleted: usize,
    pub(crate) reusable: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct VersionedPayload {
    pub(crate) effective_at: VersionId,
    pub(crate) retired_at: Option<VersionId>,
    pub(crate) generation: u32,
    pub(crate) value: RecordPayload,
}

pub(crate) type VersionedValue = VersionedPayload;

#[derive(Debug, Clone)]
pub(crate) struct VersionedEntityMetadata {
    pub(crate) effective_at: VersionId,
    pub(crate) retired_at: Option<VersionId>,
    pub(crate) generation: u32,
    pub(crate) kind_id: KindId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RelationEndpoints {
    pub(crate) source: EntityId,
    pub(crate) target: EntityId,
}

#[derive(Debug, Clone)]
pub(crate) struct VersionedRelationMetadata {
    pub(crate) effective_at: VersionId,
    pub(crate) retired_at: Option<VersionId>,
    pub(crate) generation: u32,
    pub(crate) kind_id: KindId,
    pub(crate) endpoints: RelationEndpoints,
}

pub(crate) trait RecordId: Copy + Ord + Hash + Debug + 'static {
    fn partition_id(&self) -> PartitionId;
    fn local_slot(&self) -> usize;
    fn generation(&self) -> u32;
    #[allow(dead_code)]
    fn with_slot_and_generation(partition: PartitionId, slot: u64, generation: u32) -> Self;
}

impl RecordId for EntityId {
    fn partition_id(&self) -> PartitionId {
        self.partition_id
    }

    fn local_slot(&self) -> usize {
        self.local_slot.0 as usize
    }

    fn generation(&self) -> u32 {
        self.generation.0
    }

    fn with_slot_and_generation(partition: PartitionId, slot: u64, generation: u32) -> Self {
        Self::new(partition, slot, generation)
    }

}

impl RecordId for RelationId {
    fn partition_id(&self) -> PartitionId {
        self.partition_id
    }

    fn local_slot(&self) -> usize {
        self.local_slot.0 as usize
    }

    fn generation(&self) -> u32 {
        self.generation.0
    }

    fn with_slot_and_generation(partition: PartitionId, slot: u64, generation: u32) -> Self {
        Self::new(partition, slot, generation)
    }

}

#[derive(Debug, Clone, Default)]
pub(crate) struct EntityExtra {
    pub(crate) structural_fingerprint: Option<StructuralFingerprint>,
    pub(crate) lineage_id: Option<LineageId>,
}

pub(crate) type RelationExtra = Option<RelationEndpoints>;

pub(crate) trait RecordKind: Clone + Debug + 'static {
    type Id: RecordId;
    type Meta: Clone + Debug;
    type Extra: Clone + Debug;
    type ReadRecord: Clone + Debug;

    fn arena(partition: &PartitionState) -> &RecordArena<Self>;
    fn arena_mut(partition: &mut PartitionState) -> &mut RecordArena<Self>;
    fn empty_extra() -> Self::Extra;
    fn reserve_extra(extra: &mut Vec<Self::Extra>, additional: usize);
    fn retire_metadata(metadata: &mut Self::Meta, version_id: VersionId);
    fn reset_reclaimed_slot(arena: &mut RecordArena<Self>, slot: usize);
    fn metadata_for_create(
        kind_id: KindId,
        generation: u32,
        version_id: VersionId,
        extra: &Self::Extra,
    ) -> Self::Meta;
}

pub(crate) trait HistoricalMetadata {
    fn effective_at(&self) -> VersionId;
    fn retired_at(&self) -> Option<VersionId>;
}

#[derive(Debug, Clone)]
pub(crate) struct EntityRecordKind;

impl RecordKind for EntityRecordKind {
    type Id = EntityId;
    type Meta = VersionedEntityMetadata;
    type Extra = EntityExtra;
    type ReadRecord = EntityReadRecord;

    fn arena(partition: &PartitionState) -> &RecordArena<Self> {
        &partition.entity_arena
    }

    fn arena_mut(partition: &mut PartitionState) -> &mut RecordArena<Self> {
        &mut partition.entity_arena
    }

    fn empty_extra() -> Self::Extra {
        EntityExtra::default()
    }

    fn reserve_extra(extra: &mut Vec<Self::Extra>, additional: usize) {
        extra.reserve(additional);
    }

    fn retire_metadata(metadata: &mut Self::Meta, version_id: VersionId) {
        metadata.retired_at = Some(version_id);
    }

    fn reset_reclaimed_slot(arena: &mut RecordArena<Self>, slot: usize) {
        arena.kind_ids[slot] = None;
        arena.payloads[slot] = None;
        arena.aspect_versions[slot].clear();
        arena.extra[slot].structural_fingerprint = None;
        arena.extra[slot].lineage_id = None;
        arena.diagnostics_enrichment[slot].clear();
        arena.snapshot_pins[slot] = 0;
        arena.branch_pins[slot] = 0;
        arena.replay_pins[slot] = 0;
        arena.retired_at[slot] = None;
        arena.free_list.push(slot as u64);
    }

    fn metadata_for_create(
        kind_id: KindId,
        generation: u32,
        version_id: VersionId,
        _extra: &Self::Extra,
    ) -> Self::Meta {
        VersionedEntityMetadata {
            effective_at: version_id,
            retired_at: None,
            generation,
            kind_id,
        }
    }
}

impl HistoricalMetadata for VersionedEntityMetadata {
    fn effective_at(&self) -> VersionId {
        self.effective_at
    }

    fn retired_at(&self) -> Option<VersionId> {
        self.retired_at
    }

}

#[derive(Debug, Clone)]
pub(crate) struct RelationRecordKind;

impl RecordKind for RelationRecordKind {
    type Id = RelationId;
    type Meta = VersionedRelationMetadata;
    type Extra = RelationExtra;
    type ReadRecord = RelationReadRecord;

    fn arena(partition: &PartitionState) -> &RecordArena<Self> {
        &partition.relation_arena
    }

    fn arena_mut(partition: &mut PartitionState) -> &mut RecordArena<Self> {
        &mut partition.relation_arena
    }

    fn empty_extra() -> Self::Extra {
        None
    }

    fn reserve_extra(extra: &mut Vec<Self::Extra>, additional: usize) {
        extra.reserve(additional);
    }

    fn retire_metadata(metadata: &mut Self::Meta, version_id: VersionId) {
        metadata.retired_at = Some(version_id);
    }

    fn reset_reclaimed_slot(arena: &mut RecordArena<Self>, slot: usize) {
        arena.kind_ids[slot] = None;
        arena.payloads[slot] = None;
        arena.aspect_versions[slot].clear();
        arena.diagnostics_enrichment[slot].clear();
        arena.branch_pins[slot] = 0;
        arena.replay_pins[slot] = 0;
        arena.snapshot_pins[slot] = 0;
        arena.extra[slot] = None;
        arena.retired_at[slot] = None;
        arena.free_list.push(slot as u64);
    }

    fn metadata_for_create(
        kind_id: KindId,
        generation: u32,
        version_id: VersionId,
        extra: &Self::Extra,
    ) -> Self::Meta {
        VersionedRelationMetadata {
            effective_at: version_id,
            retired_at: None,
            generation,
            kind_id,
            endpoints: extra
                .clone()
                .expect("relation metadata requires endpoints"),
        }
    }
}

impl HistoricalMetadata for VersionedRelationMetadata {
    fn effective_at(&self) -> VersionId {
        self.effective_at
    }

    fn retired_at(&self) -> Option<VersionId> {
        self.retired_at
    }

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

    pub(crate) fn allocate_common(
        &mut self,
        partition_id: PartitionId,
        kind_id: KindId,
        payload: Option<RecordPayload>,
        version_id: VersionId,
        extra: K::Extra,
    ) -> (usize, u32, bool) {
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
        self.metadata_history.push(vec![K::metadata_for_create(
            kind_id, 1, version_id, &extra,
        )]);
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

    pub(crate) fn get(&self, id: &K::Id) -> Option<SlotView<'_, K>> {
        let slot = id.local_slot();
        self.get_slot(slot)
            .filter(|view| {
                view.generation() == id.generation()
                    && view.partition_id() == id.partition_id()
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

    pub(crate) fn payload_history_at_mut(&mut self, slot: usize) -> Option<&mut Vec<VersionedPayload>> {
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

    pub(crate) fn contains_live_id(&self, id: &K::Id) -> bool {
        self.get(id).is_some_and(|view| view.is_live())
    }
}

pub(crate) type EntityArena = RecordArena<EntityRecordKind>;
pub(crate) type RelationArena = RecordArena<RelationRecordKind>;

#[derive(Clone, Copy)]
pub(crate) enum PinClass {
    Branch,
    Replay,
}

#[allow(dead_code)]
pub(crate) struct SlotView<'a, K: RecordKind> {
    arena: &'a RecordArena<K>,
    index: usize,
    _marker: PhantomData<K>,
}

#[allow(dead_code)]
impl<'a, K: RecordKind> SlotView<'a, K> {
    pub(crate) fn new(arena: &'a RecordArena<K>, index: usize) -> Self {
        Self {
            arena,
            index,
            _marker: PhantomData,
        }
    }

    pub(crate) fn generation(&self) -> u32 {
        self.arena.generations[self.index]
    }

    pub(crate) fn partition_id(&self) -> PartitionId {
        self.arena.partition_ids[self.index]
    }

    pub(crate) fn lifecycle(&self) -> RecordLifecycleState {
        self.arena.lifecycle[self.index]
    }

    pub(crate) fn is_live(&self) -> bool {
        self.lifecycle() == RecordLifecycleState::Live
    }

    pub(crate) fn kind_id(&self) -> Option<KindId> {
        self.arena.kind_ids[self.index]
    }

    pub(crate) fn payload(&self) -> Option<&RecordPayload> {
        self.arena.payloads[self.index].as_ref()
    }

    pub(crate) fn snapshot_pins(&self) -> u32 {
        self.arena.snapshot_pins[self.index]
    }

    pub(crate) fn branch_pins(&self) -> u32 {
        self.arena.branch_pins[self.index]
    }

    pub(crate) fn replay_pins(&self) -> u32 {
        self.arena.replay_pins[self.index]
    }

    pub(crate) fn retired_at(&self) -> Option<VersionId> {
        self.arena.retired_at[self.index]
    }

    pub(crate) fn extra(&self) -> &K::Extra {
        &self.arena.extra[self.index]
    }

    pub(crate) fn is_current(&self, id: &K::Id) -> bool {
        self.generation() == id.generation() && self.is_live()
    }

    pub(crate) fn is_visible_at(&self, bound: VersionBound) -> bool {
        bound.includes_created(self.arena.created_at[self.index])
            && self.arena.retired_at[self.index]
                .is_none_or(|retired| bound.retains_retired(retired))
    }
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
    use super::{
        EntityArena, EntityExtra, RelationArena, RelationEndpoints,
    };
    use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
    use crate::payloads::data::RecordPayload;
    use crate::storage::data::RecordLifecycleState;
    use crate::symbols::data::Symbol;

    #[test]
    fn reusing_entity_slot_clears_entity_sidecars_and_increments_generation() {
        let mut arena = EntityArena::with_capacity(1);
        let partition_id = PartitionId(7);
        let version_one = VersionId(1);
        let version_two = VersionId(2);
        let payload = RecordPayload::OpaqueBytes(vec![1, 2, 3]);

        let (slot, generation, _) = arena.allocate_common(
            partition_id,
            KindId(11),
            Some(payload.clone()),
            version_one,
            EntityExtra::default(),
        );
        assert_eq!(generation, 1);
        arena.extra[slot] = EntityExtra {
            structural_fingerprint: Some(crate::identity::data::StructuralFingerprint {
                family: Symbol(9),
                value: 42,
            }),
            lineage_id: Some(crate::identity::data::LineageId(12)),
        };
        arena.retire(slot, version_two);
        arena.lifecycle[slot] = RecordLifecycleState::Reusable;
        arena.free_list.push(slot as u64);

        let (_, reused_generation, reused) = arena.allocate_common(
            partition_id,
            KindId(12),
            Some(payload),
            VersionId(3),
            EntityExtra::default(),
        );
        assert!(reused);
        assert_eq!(reused_generation, 2);
        assert!(arena.extra[slot].structural_fingerprint.is_none());
        assert!(arena.extra[slot].lineage_id.is_none());
    }

    #[test]
    fn reusing_relation_slot_replaces_endpoints_and_increments_generation() {
        let mut arena = RelationArena::with_capacity(1);
        let partition_id = PartitionId(3);
        let first = RelationEndpoints {
            source: EntityId::new(partition_id, 1, 1),
            target: EntityId::new(partition_id, 2, 1),
        };
        let second = RelationEndpoints {
            source: EntityId::new(partition_id, 3, 1),
            target: EntityId::new(partition_id, 4, 1),
        };

        let (slot, generation, _) = arena.allocate_common(
            partition_id,
            KindId(21),
            Some(RecordPayload::OpaqueBytes(vec![9])),
            VersionId(1),
            Some(first),
        );
        assert_eq!(generation, 1);
        arena.retire(slot, VersionId(2));
        arena.lifecycle[slot] = RecordLifecycleState::Reusable;
        arena.free_list.push(slot as u64);

        let (_, reused_generation, reused) = arena.allocate_common(
            partition_id,
            KindId(22),
            None,
            VersionId(3),
            Some(second.clone()),
        );
        assert!(reused);
        assert_eq!(reused_generation, 2);
        assert_eq!(arena.extra[slot], Some(second));
    }

    #[test]
    fn get_rejects_id_from_different_partition_even_with_same_slot_and_generation() {
        let mut arena = RelationArena::with_capacity(1);
        let partition_id = PartitionId(3);
        let other_partition_id = PartitionId(4);
        let (slot, generation, _) = arena.allocate_common(
            partition_id,
            KindId(21),
            Some(RecordPayload::OpaqueBytes(vec![9])),
            VersionId(1),
            Some(RelationEndpoints {
                source: EntityId::new(partition_id, 1, 1),
                target: EntityId::new(partition_id, 2, 1),
            }),
        );

        let wrong_partition_id = RelationId::new(other_partition_id, slot as u64, generation);
        assert!(arena.get(&wrong_partition_id).is_none());
    }
}
