use std::collections::BTreeMap;

use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::data::RelationalReadView;
use crate::storage::logic::state::{
    DenseSlotBitSet, SnapshotPartitionPins, SnapshotState,
};

pub(crate) fn build_partition_pins_for_version(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) -> BTreeMap<crate::identity::data::PartitionId, SnapshotPartitionPins> {
    if version_id == runtime.current_version_id() {
        let mut pinned_partitions = BTreeMap::new();
        for (partition_id, partition) in &runtime.partitions {
            let mut entity_slots =
                DenseSlotBitSet::with_capacity(partition.entity_arena.slot_count());
            for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                entity_slots.set(slot, true);
            }
            let mut relation_slots =
                DenseSlotBitSet::with_capacity(partition.relation_arena.slot_count());
            for slot in partition.relation_arena.live_bitset.iter_set_slots() {
                relation_slots.set(slot, true);
            }
            if entity_slots.count_ones() > 0 || relation_slots.count_ones() > 0 {
                pinned_partitions.insert(
                    *partition_id,
                    SnapshotPartitionPins {
                        entity_slots,
                        relation_slots,
                    },
                );
            }
        }
        return pinned_partitions;
    }
    build_visibility_state(
        runtime,
        version_id,
        SnapshotId(0),
        SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
    )
    .pinned_partitions
}

pub(crate) fn build_visibility_state(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
    snapshot_id: SnapshotId,
    read_policy: SnapshotReadPolicy,
) -> SnapshotState {
    let handle = SnapshotHandle {
        snapshot_id,
        version_id,
        read_policy,
    };
    let current_state = runtime.current_state();
    let reader = runtime.visibility_reads();
    let entity_partitions = reader.visible_entity_slots_from_state(&current_state, version_id);
    let relation_partitions = reader.visible_relation_slots_from_state(&current_state, version_id);
    let mut pinned_partitions = BTreeMap::new();
    let mut pinned_entity_count = 0;
    for (partition_id, entity_slots) in entity_partitions {
        pinned_entity_count += entity_slots.count_ones();
        let pins = pinned_partitions
            .entry(partition_id)
            .or_insert_with(|| SnapshotPartitionPins {
                entity_slots: DenseSlotBitSet::with_capacity(entity_slots.words().len() * 64),
                relation_slots: DenseSlotBitSet::with_capacity(0),
            });
        pins.entity_slots = entity_slots;
    }
    let mut pinned_relation_count = 0;
    for (partition_id, relation_slots) in relation_partitions {
        pinned_relation_count += relation_slots.count_ones();
        let pins = pinned_partitions
            .entry(partition_id)
            .or_insert_with(|| SnapshotPartitionPins {
                entity_slots: DenseSlotBitSet::with_capacity(0),
                relation_slots: DenseSlotBitSet::with_capacity(relation_slots.words().len() * 64),
            });
        pins.relation_slots = relation_slots;
    }
    SnapshotState {
        handle,
        pinned_entity_count,
        pinned_relation_count,
        pinned_partitions,
    }
}

pub(crate) fn read_view_from_snapshot_state(
    runtime: &RelationalRuntime,
    state: &SnapshotState,
) -> RelationalReadView {
    let current_state = runtime.current_state();
    let reader = runtime.visibility_reads();
    let mut entities = Vec::with_capacity(state.pinned_entity_count);
    let mut relations = Vec::with_capacity(state.pinned_relation_count);
    for (partition_id, pins) in &state.pinned_partitions {
        for slot in pins.entity_slots.iter_set_slots() {
            let entity_id = crate::identity::data::EntityId::new(*partition_id, slot as u64, 0);
            if let Some(record) = reader.entity_record_for_id_at_version(
                &current_state,
                entity_id,
                state.handle.version_id,
            ) {
                entities.push(record);
            }
        }
        for slot in pins.relation_slots.iter_set_slots() {
            let relation_id = crate::identity::data::RelationId::new(*partition_id, slot as u64, 0);
            if let Some(record) = reader.relation_record_for_id_at_version(
                &current_state,
                relation_id,
                state.handle.version_id,
            ) {
                relations.push(record);
            }
        }
    }
    runtime.services.instrumentation.count(|counters| {
        counters.visible_entity_records_materialized += entities.len();
        counters.visible_relation_records_materialized += relations.len();
    });
    RelationalReadView {
        snapshot: state.handle.clone(),
        entities,
        relations,
    }
}
