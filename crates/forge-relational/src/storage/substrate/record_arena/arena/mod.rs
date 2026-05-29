mod merge;
mod pinning;
mod slot_lifecycle;
mod sparse_clone;

use std::collections::BTreeMap;

use crate::identity::data::{KindId, PartitionId, RecordId, VersionId};
use crate::storage::data::RecordLifecycleState;
use crate::storage::partition::DenseSlotBitSet;
use crate::symbols::data::Symbol;

use super::{
    slot_view::SlotView, EntityRecordKind, LifecycleCounts, RecordKind, RelationRecordKind,
};

#[derive(Debug)]
pub(crate) struct SlotInit<K: RecordKind> {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub version_id: VersionId,
    pub extra: K::Extra,
}

#[derive(Debug, Clone)]
pub(crate) struct RecordArena<K: RecordKind> {
    pub(crate) partition_ids: Vec<PartitionId>,
    pub(crate) generations: Vec<u32>,
    pub(crate) lifecycle: Vec<RecordLifecycleState>,
    pub(crate) kind_ids: Vec<Option<KindId>>,
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

    pub(crate) fn lifecycle_counts(&self) -> LifecycleCounts {
        lifecycle_counts(&self.lifecycle)
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

    pub(crate) fn metadata_history_at(&self, slot: usize) -> Option<&[K::Meta]> {
        self.metadata_history.get(slot).map(Vec::as_slice)
    }

    pub(crate) fn metadata_history_at_mut(&mut self, slot: usize) -> Option<&mut Vec<K::Meta>> {
        self.metadata_history.get_mut(slot)
    }

    pub(crate) fn aspect_versions_at(&self, slot: usize) -> Option<&BTreeMap<Symbol, u64>> {
        self.aspect_versions.get(slot)
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
