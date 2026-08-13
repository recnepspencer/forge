use std::collections::BTreeMap;

use crate::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::overlay::{SnapshotPartitionPins, SnapshotState};
use crate::storage::partition::DenseSlotBitSet;

pub(crate) fn build_partition_pins_for_version(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) -> BTreeMap<crate::identity::data::PartitionId, SnapshotPartitionPins> {
    if version_id == runtime.current_version_id() {
        return runtime.storage_access().current_version_partition_pins();
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
        runtime_instance_id: runtime.runtime_instance_id(),
        branch_id: crate::visibility::branch_scope::authoritative_branch_for_version(
            runtime, version_id,
        ),
        snapshot_id,
        version_id,
        read_policy,
    };
    let current_state = runtime.storage_access().current_state();
    let reader = runtime.read_truth();
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
                retained_relation_slots: DenseSlotBitSet::with_capacity(0),
            });
        pins.entity_slots = entity_slots;
    }
    let mut pinned_relation_count = 0;
    for (partition_id, relation_slots) in relation_partitions {
        pinned_relation_count += relation_slots.count_ones();
        let retained_relation_slots = retained_relation_slots_for_version(
            runtime,
            &current_state,
            partition_id,
            &relation_slots,
            version_id,
        );
        let pins = pinned_partitions
            .entry(partition_id)
            .or_insert_with(|| SnapshotPartitionPins {
                entity_slots: DenseSlotBitSet::with_capacity(0),
                relation_slots: DenseSlotBitSet::with_capacity(relation_slots.words().len() * 64),
                retained_relation_slots: DenseSlotBitSet::with_capacity(
                    relation_slots.words().len() * 64,
                ),
            });
        pins.relation_slots = relation_slots;
        pins.retained_relation_slots = retained_relation_slots;
    }
    SnapshotState {
        handle,
        pinned_entity_count,
        pinned_relation_count,
        pinned_partitions,
    }
}

fn retained_relation_slots_for_version(
    runtime: &RelationalRuntime,
    state: &impl crate::storage::overlay::PartitionAccess,
    partition_id: crate::identity::data::PartitionId,
    relation_slots: &DenseSlotBitSet,
    version_id: crate::identity::data::VersionId,
) -> DenseSlotBitSet {
    let reader = runtime.read_truth();
    let mut retained = DenseSlotBitSet::with_capacity(relation_slots.words().len() * 64);
    for slot in relation_slots.iter_set_slots() {
        let relation_id = crate::identity::data::RelationId::new(partition_id, slot as u64, 0);
        if reader
            .authoritative_relation_record_for_id_at_version(state, relation_id, version_id)
            .is_some_and(|record| {
                record.lifecycle
                    == crate::storage::data::RecordLifecycleState::RetainedDanglingForAudit
            })
        {
            retained.set(slot, true);
        }
    }
    retained
}
