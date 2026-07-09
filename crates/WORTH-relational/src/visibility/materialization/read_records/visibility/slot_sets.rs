use crate::identity::data::{PartitionId, VersionId};
use crate::logic::runtime::RelationalRuntime;
use crate::storage::logic::state::{
    DenseSlotBitSet, HistoricalMetadata, PartitionAccess, PartitionState, RecordArena, RecordKind,
};

use super::visible_metadata;

pub(in super::super) fn entity_visible_in_partition_at_version(
    partition: &PartitionState,
    slot: usize,
    version_id: VersionId,
) -> bool {
    partition
        .entity_arena
        .metadata_history_at(slot)
        .and_then(|history| visible_metadata(history, version_id))
        .is_some()
}

pub(in super::super) fn relation_visible_in_partition_at_version(
    partition: &PartitionState,
    slot: usize,
    version_id: VersionId,
) -> bool {
    record_visible_in_arena_at_version(&partition.relation_arena, slot, version_id)
}

pub(in super::super) fn record_visible_in_arena_at_version<K: RecordKind>(
    arena: &RecordArena<K>,
    slot: usize,
    version_id: VersionId,
) -> bool
where
    K::Meta: HistoricalMetadata,
{
    arena
        .metadata_history_at(slot)
        .and_then(|history| visible_metadata(history, version_id))
        .is_some()
}

pub(in super::super) fn visible_slots_in_partition_from_state<K: RecordKind>(
    runtime: &RelationalRuntime,
    state: &impl PartitionAccess,
    partition_id: PartitionId,
    version_id: VersionId,
    count_scans: impl Fn(&RelationalRuntime, usize),
) -> Option<DenseSlotBitSet>
where
    K::Meta: HistoricalMetadata,
{
    let current_version = runtime.current_version_id();
    let partition = state.get_partition(partition_id)?;
    let arena = K::arena(partition);
    let mut visible_slots = DenseSlotBitSet::with_capacity(arena.slot_count());
    if version_id == current_version {
        for slot in arena.live_bitset.iter_set_slots() {
            visible_slots.set(slot, true);
        }
    } else {
        count_scans(runtime, arena.slot_count());
        for slot in 0..arena.slot_count() {
            if record_visible_in_arena_at_version(arena, slot, version_id) {
                visible_slots.set(slot, true);
            }
        }
    }
    (visible_slots.count_ones() > 0).then_some(visible_slots)
}
