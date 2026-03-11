use std::collections::BTreeMap;

use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::logic::state::{
    DenseSlotBitSet, SnapshotPartitionPins, SnapshotState,
};

impl RelationalRuntime {
    pub(crate) fn build_partition_pins_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> BTreeMap<crate::identity::data::PartitionId, SnapshotPartitionPins> {
        if version_id == self.current_version_id() {
            let mut pinned_partitions = BTreeMap::new();
            for (partition_id, partition) in &self.partitions {
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
        self.build_visibility_state(
            version_id,
            SnapshotId(0),
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        )
        .pinned_partitions
    }

    pub(crate) fn build_visibility_state(
        &self,
        version_id: crate::identity::data::VersionId,
        snapshot_id: SnapshotId,
        read_policy: SnapshotReadPolicy,
    ) -> SnapshotState {
        let handle = SnapshotHandle {
            snapshot_id,
            version_id,
            read_policy,
        };
        let current_state = self.current_state();
        let entity_partitions = self.visible_entity_slots_from_state(&current_state, version_id);
        let relation_partitions = self.visible_relation_slots_from_state(&current_state, version_id);
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
}
