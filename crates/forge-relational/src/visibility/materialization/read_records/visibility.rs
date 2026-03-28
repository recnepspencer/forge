use crate::identity::data::VersionBound;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{
    DenseSlotBitSet, HistoricalMetadata, PartitionAccess, PartitionState, RecordArena, RecordKind,
    VersionedValue,
};

pub(super) fn visible_payload_for_generation(
    history: &[VersionedValue],
    version_id: crate::identity::data::VersionId,
    generation: u32,
) -> Option<&crate::payloads::data::RecordPayload> {
    let bound = VersionBound::new(version_id);
    let end = history.partition_point(|entry| bound.includes_created(entry.effective_at));
    history[..end]
        .iter()
        .rev()
        .find(|entry| {
            entry.generation == generation
                && bound.includes_created(entry.effective_at)
                && entry
                    .retired_at
                    .is_none_or(|retired| bound.retains_retired(retired))
        })
        .map(|entry| &entry.value)
}

pub(super) fn visible_metadata<M: HistoricalMetadata>(
    history: &[M],
    version_id: crate::identity::data::VersionId,
) -> Option<&M> {
    let bound = VersionBound::new(version_id);
    let end = history.partition_point(|entry| bound.includes_created(entry.effective_at()));
    history[..end].iter().rev().find(|entry| {
        bound.includes_created(entry.effective_at())
            && entry
                .retired_at()
                .is_none_or(|retired| bound.retains_retired(retired))
    })
}

pub(super) fn entity_visible_in_partition_at_version(
    partition: &PartitionState,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool {
    partition
        .entity_arena
        .metadata_history_at(slot)
        .and_then(|history| visible_metadata(history, version_id))
        .is_some()
}

pub(super) fn relation_visible_in_partition_at_version(
    partition: &PartitionState,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool {
    record_visible_in_arena_at_version(&partition.relation_arena, slot, version_id)
}

pub(super) fn record_visible_in_arena_at_version<K: RecordKind>(
    arena: &RecordArena<K>,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool
where
    K::Meta: HistoricalMetadata,
{
    arena
        .metadata_history_at(slot)
        .and_then(|history| visible_metadata(history, version_id))
        .is_some()
}

pub(super) fn slot_kind_matches<K: RecordKind>(
    arena: &RecordArena<K>,
    slot: usize,
    kind_id: crate::identity::data::KindId,
) -> bool {
    arena
        .get_slot(slot)
        .and_then(|slot_view| slot_view.kind_id())
        == Some(kind_id)
}

pub(super) fn visible_slots_in_partition_from_state<K: RecordKind>(
    runtime: &RelationalRuntime,
    state: &impl PartitionAccess,
    partition_id: crate::identity::data::PartitionId,
    version_id: crate::identity::data::VersionId,
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

pub(super) fn lifecycle_storage_visible(lifecycle: RecordLifecycleState) -> bool {
    lifecycle != RecordLifecycleState::Reusable
}

pub(super) fn historical_lifecycle(
    retired_at: Option<crate::identity::data::VersionId>,
    version_id: crate::identity::data::VersionId,
) -> RecordLifecycleState {
    if retired_at.is_some_and(|retired_at| retired_at <= version_id) {
        RecordLifecycleState::DeletedRetained
    } else {
        RecordLifecycleState::Live
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::runtime::RelationalRuntime;
    use crate::logic::runtime::RelationalRuntimeConfig;
    use crate::payloads::data::RecordPayload;
    use crate::storage::data::RecordLifecycleState;
    use crate::storage::overlay::PartitionState;
    use crate::storage::partition::AdjacencySet;

    #[test]
    fn stale_entity_id_does_not_materialize_reused_slot_at_current_version() {
        let mut runtime = RelationalRuntime::new(RelationalRuntimeConfig::default());
        let partition_id = crate::identity::data::PartitionId(7);
        let adjacency_policy = runtime.config.storage.adjacency_policy.clone();
        let mut entity_arena = crate::storage::substrate::EntityArena::with_capacity(1);
        let (slot, generation, _) = entity_arena.push_slot(crate::storage::substrate::SlotInit {
            partition_id,
            kind_id: crate::identity::data::KindId(11),
            payload: Some(RecordPayload::OpaqueBytes(vec![1])),
            version_id: crate::identity::data::VersionId(1),
            extra: crate::storage::substrate::EntityExtra::default(),
        });
        let stale_id = crate::identity::data::EntityId::new(partition_id, slot as u64, generation);
        entity_arena.retire(slot, crate::identity::data::VersionId(2));
        entity_arena.lifecycle[slot] = RecordLifecycleState::Reusable;
        entity_arena.reset_slot(slot);
        let (_, reused_generation, _) =
            entity_arena.push_slot(crate::storage::substrate::SlotInit {
                partition_id,
                kind_id: crate::identity::data::KindId(12),
                payload: Some(RecordPayload::OpaqueBytes(vec![2])),
                version_id: crate::identity::data::VersionId(3),
                extra: crate::storage::substrate::EntityExtra::default(),
            });
        assert_eq!(reused_generation, 2);

        runtime.history.next_version_id = 4;
        runtime.partitions.insert(
            partition_id,
            PartitionState {
                partition_id,
                adjacency_policy: adjacency_policy.clone(),
                entity_arena,
                relation_arena: crate::storage::substrate::RelationArena::with_capacity(0),
                adjacency: vec![AdjacencySet::new(&adjacency_policy)],
                reverse_adjacency: vec![AdjacencySet::new(&adjacency_policy)],
            },
        );

        let current_state = runtime.storage_access().current_state();
        assert!(runtime
            .visibility_reads()
            .entity_record_for_id_at_version(&current_state, stale_id, runtime.current_version_id())
            .is_none());
    }
}
