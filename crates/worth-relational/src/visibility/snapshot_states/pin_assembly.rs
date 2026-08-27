use std::collections::BTreeMap;

use crate::identity::data::PartitionId;
use crate::snapshots::data::SnapshotHandle;
use crate::storage::overlay::SnapshotPartitionPins;
use crate::storage::partition::DenseSlotBitSet;

use super::{SnapshotState, SnapshotStateBasis};

pub(super) struct SelectedRelationSlots {
    pub(super) partition_id: PartitionId,
    pub(super) visible: DenseSlotBitSet,
    pub(super) retained: DenseSlotBitSet,
}

pub(super) fn assemble_snapshot_state(
    handle: SnapshotHandle,
    basis: SnapshotStateBasis,
    entity_partitions: Vec<(PartitionId, DenseSlotBitSet)>,
    relation_partitions: Vec<SelectedRelationSlots>,
) -> SnapshotState {
    let mut pinned_partitions = BTreeMap::new();
    let mut pinned_entity_count = 0;
    for (partition_id, entity_slots) in entity_partitions {
        pinned_entity_count += entity_slots.count_ones();
        let pins = pinned_partitions
            .entry(partition_id)
            .or_insert_with(empty_partition_pins);
        pins.entity_slots = entity_slots;
    }
    let mut pinned_relation_count = 0;
    for selected in relation_partitions {
        pinned_relation_count += selected.visible.count_ones();
        let pins = pinned_partitions
            .entry(selected.partition_id)
            .or_insert_with(empty_partition_pins);
        pins.relation_slots = selected.visible;
        pins.retained_relation_slots = selected.retained;
    }
    SnapshotState {
        handle,
        basis,
        pinned_entity_count,
        pinned_relation_count,
        pinned_partitions,
    }
}

fn empty_partition_pins() -> SnapshotPartitionPins {
    SnapshotPartitionPins {
        entity_slots: DenseSlotBitSet::with_capacity(0),
        relation_slots: DenseSlotBitSet::with_capacity(0),
        retained_relation_slots: DenseSlotBitSet::with_capacity(0),
    }
}
