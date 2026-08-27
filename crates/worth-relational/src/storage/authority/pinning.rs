use std::collections::{BTreeMap, BTreeSet};

use crate::identity::data::{PartitionId, RecordId, VersionId};
use crate::storage::data::RecordLifecycleState;
use crate::storage::substrate::{PinClass, RecordKind};

use super::{partition_of, slot_of, StorageAuthority};

impl<'runtime> StorageAuthority<'runtime> {
    pub(crate) fn clear_named_pins(&mut self, class: PinClass) {
        for partition in self.runtime.partitions.values_mut() {
            partition.entity_arena.clear_named_pins(class);
            partition.relation_arena.clear_named_pins(class);
        }
    }

    pub(crate) fn pin_snapshot_record<K: RecordKind>(&mut self, record_id: RecordId<K::Domain>) {
        let slot = slot_of::<K>(&record_id);
        let Some(partition) = self
            .runtime
            .partitions
            .get_mut(&partition_of::<K>(&record_id))
        else {
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
        &mut self,
        record_id: RecordId<K::Domain>,
        retention_fence: VersionId,
    ) {
        let slot = slot_of::<K>(&record_id);
        let partition_id = partition_of::<K>(&record_id);
        let Some(partition) = self.runtime.partitions.get_mut(&partition_id) else {
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
        let retired_at = arena.retired_at_for_slot(slot);
        self.refresh_retention_state::<K>(partition_id, slot, retired_at, retention_fence);
    }

    pub(crate) fn adjust_named_pin<K: RecordKind>(
        &mut self,
        record_id: RecordId<K::Domain>,
        class: PinClass,
        delta: i32,
        retention_fence: VersionId,
    ) {
        let slot = slot_of::<K>(&record_id);
        let partition_id = partition_of::<K>(&record_id);
        if !self.runtime.partitions.contains_key(&partition_id) {
            return;
        }
        {
            let partition = self
                .runtime
                .partitions
                .get_mut(&partition_id)
                .expect("pin partition present");
            let arena = K::arena_mut(partition);
            if arena.snapshot_pin_count(slot).is_none() {
                return;
            }
            if let Some(pin_count) = arena.adjust_named_pin(slot, class) {
                *pin_count = pin_count.saturating_add_signed(delta);
            }
        }
        let retired_at = self
            .runtime
            .partitions
            .get(&partition_id)
            .and_then(|partition| K::arena(partition).retired_at_for_slot(slot));
        self.refresh_retention_state::<K>(partition_id, slot, retired_at, retention_fence);
    }

    pub(crate) fn increment_named_pins_bulk<K: RecordKind>(
        &mut self,
        slots_by_partition: &BTreeMap<PartitionId, BTreeSet<usize>>,
        class: PinClass,
    ) {
        for (partition_id, slots) in slots_by_partition {
            if !self.runtime.partitions.contains_key(partition_id) {
                continue;
            }
            {
                let partition = self
                    .runtime
                    .partitions
                    .get_mut(partition_id)
                    .expect("pin partition present");
                let arena = K::arena_mut(partition);
                arena.increment_named_pins_bulk(slots, class);
            }
            for &slot in slots {
                if self
                    .runtime
                    .partitions
                    .get(partition_id)
                    .and_then(|partition| K::arena(partition).get_slot(slot))
                    .is_none()
                {
                    continue;
                }
                let retired_at = self
                    .runtime
                    .partitions
                    .get(partition_id)
                    .and_then(|partition| K::arena(partition).retired_at_for_slot(slot));
                self.refresh_retention_state::<K>(
                    *partition_id,
                    slot,
                    retired_at,
                    self.runtime
                        .visibility
                        .retention_fence_version(self.runtime.current_version_id()),
                );
            }
        }
    }
}
