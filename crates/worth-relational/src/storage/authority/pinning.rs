use std::collections::{BTreeMap, BTreeSet};

use crate::identity::data::{PartitionId, RecordId, VersionId};
use crate::storage::data::RecordLifecycleState;
use crate::storage::substrate::{PinClass, RecordKind};

use super::{partition_of, slot_of, StorageAuthority};

impl StorageAuthority<'_> {
    pub(crate) fn clear_named_pins(&self, class: PinClass) {
        let mut writer = self.runtime.edit_partitions();
        for partition in writer.partitions_mut() {
            partition.entity_arena.clear_named_pins(class);
            partition.relation_arena.clear_named_pins(class);
        }
    }

    pub(crate) fn pin_snapshot_record<K: RecordKind>(&self, record_id: RecordId<K::Domain>) {
        let slot = slot_of::<K>(&record_id);
        let mut writer = self.runtime.edit_partitions();
        let Some(partition) = writer.partition_mut(partition_of::<K>(&record_id)) else {
            return;
        };
        let arena = K::arena_mut(partition);
        if arena.snapshot_pin_count(slot).is_none() {
            return;
        }
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.snapshot_pin_adjustments += 1);
        arena.increment_snapshot_pin(slot);
        if arena.retired_at_for_slot(slot).is_some() {
            arena.set_lifecycle_for_slot(slot, RecordLifecycleState::PinnedBySnapshot);
        }
    }

    pub(crate) fn unpin_snapshot_record<K: RecordKind>(
        &self,
        record_id: RecordId<K::Domain>,
        retention_fence: VersionId,
    ) {
        let slot = slot_of::<K>(&record_id);
        let partition_id = partition_of::<K>(&record_id);
        let retired_at = {
            let mut writer = self.runtime.edit_partitions();
            let Some(partition) = writer.partition_mut(partition_id) else {
                return;
            };
            let arena = K::arena_mut(partition);
            if arena.snapshot_pin_count(slot).unwrap_or(0) == 0 {
                return;
            }
            self.runtime
                .services
                .instrumentation
                .count(|counters| counters.snapshot_pin_adjustments += 1);
            arena.decrement_snapshot_pin(slot);
            arena.retired_at_for_slot(slot)
        };
        self.refresh_retention_state::<K>(partition_id, slot, retired_at, retention_fence);
    }

    pub(crate) fn adjust_named_pin<K: RecordKind>(
        &self,
        record_id: RecordId<K::Domain>,
        class: PinClass,
        delta: i32,
        retention_fence: VersionId,
    ) {
        let slot = slot_of::<K>(&record_id);
        let partition_id = partition_of::<K>(&record_id);
        let retired_at = {
            let mut writer = self.runtime.edit_partitions();
            let Some(partition) = writer.partition_mut(partition_id) else {
                return;
            };
            let arena = K::arena_mut(partition);
            if arena.snapshot_pin_count(slot).is_none() {
                return;
            }
            if let Some(pin_count) = arena.adjust_named_pin(slot, class) {
                *pin_count = pin_count.saturating_add_signed(delta);
            }
            arena.retired_at_for_slot(slot)
        };
        self.refresh_retention_state::<K>(partition_id, slot, retired_at, retention_fence);
    }

    pub(crate) fn increment_named_pins_bulk<K: RecordKind>(
        &self,
        slots_by_partition: &BTreeMap<PartitionId, BTreeSet<usize>>,
        class: PinClass,
    ) {
        for (partition_id, slots) in slots_by_partition {
            let retired_slots = {
                let mut writer = self.runtime.edit_partitions();
                let Some(partition) = writer.partition_mut(*partition_id) else {
                    continue;
                };
                let arena = K::arena_mut(partition);
                arena.increment_named_pins_bulk(slots, class);
                slots
                    .iter()
                    .filter(|slot| arena.get_slot(**slot).is_some())
                    .map(|slot| (*slot, arena.retired_at_for_slot(*slot)))
                    .collect::<Vec<_>>()
            };
            let retention_fence = self
                .runtime
                .visibility
                .retention_fence_version(self.runtime.current_version_id());
            for (slot, retired_at) in retired_slots {
                self.refresh_retention_state::<K>(*partition_id, slot, retired_at, retention_fence);
            }
        }
    }
}
