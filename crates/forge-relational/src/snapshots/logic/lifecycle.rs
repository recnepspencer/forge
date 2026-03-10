use std::collections::{BTreeMap, BTreeSet};

use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::data::{RecordLifecycleState, RelationalReadView};
use crate::storage::logic::state::{DenseSlotBitSet, SnapshotPartitionPins, SnapshotState};

impl RelationalRuntime {
    pub(crate) fn retention_fence_version(
        &self,
        published_version: crate::identity::data::VersionId,
    ) -> crate::identity::data::VersionId {
        self.snapshots
            .active
            .values()
            .map(|state| state.handle.version_id)
            .min()
            .unwrap_or(published_version)
    }

    pub(crate) fn snapshot_state_for_current(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> (SnapshotHandle, SnapshotState) {
        let snapshot_id = SnapshotId(self.snapshots.next_snapshot_id);
        self.snapshots.next_snapshot_id += 1;
        let handle = SnapshotHandle {
            snapshot_id,
            version_id,
            read_policy: SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        };
        let current_state = self.current_state();
        let entities = self.visible_entities_from_state(&current_state, version_id);
        let relations = self.visible_relations_from_state(&current_state, version_id);
        let mut pinned_partitions: BTreeMap<
            crate::identity::data::PartitionId,
            SnapshotPartitionPins,
        > = BTreeMap::new();
        for entity_id in entities.iter().map(|record| record.entity_id) {
            let pins = pinned_partitions
                .entry(entity_id.partition_id)
                .or_insert_with(|| SnapshotPartitionPins {
                    entity_slots: DenseSlotBitSet::with_capacity(
                        entity_id.local_slot.0 as usize + 1,
                    ),
                    relation_slots: DenseSlotBitSet::with_capacity(0),
                });
            pins.entity_slots.set(entity_id.local_slot.0 as usize, true);
        }
        for relation_id in relations.iter().map(|record| record.relation_id) {
            let pins = pinned_partitions
                .entry(relation_id.partition_id)
                .or_insert_with(|| SnapshotPartitionPins {
                    entity_slots: DenseSlotBitSet::with_capacity(0),
                    relation_slots: DenseSlotBitSet::with_capacity(
                        relation_id.local_slot.0 as usize + 1,
                    ),
                });
            pins.relation_slots
                .set(relation_id.local_slot.0 as usize, true);
        }
        let state = SnapshotState {
            handle: handle.clone(),
            pinned_entity_count: entities.len(),
            pinned_relation_count: relations.len(),
            pinned_partitions,
        };
        self.pin_snapshot_state(&state);
        (handle.clone(), state)
    }

    pub(crate) fn read_from_snapshot_state(&self, state: &SnapshotState) -> RelationalReadView {
        let current_state = self.current_state();
        let mut entities = Vec::with_capacity(state.pinned_entity_count);
        let mut relations = Vec::with_capacity(state.pinned_relation_count);
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                let entity_id = crate::identity::data::EntityId::new(*partition_id, slot as u64, 0);
                if let Some(record) = self.entity_record_for_id_at_version(
                    &current_state,
                    entity_id,
                    state.handle.version_id,
                ) {
                    entities.push(record);
                }
            }
            for slot in pins.relation_slots.iter_set_slots() {
                let relation_id =
                    crate::identity::data::RelationId::new(*partition_id, slot as u64, 0);
                if let Some(record) = self.relation_record_for_id_at_version(
                    &current_state,
                    relation_id,
                    state.handle.version_id,
                ) {
                    relations.push(record);
                }
            }
        }
        {
            let mut counters = self.instrumentation.complexity_counters.borrow_mut();
            counters.visible_entity_records_materialized += entities.len();
            counters.visible_relation_records_materialized += relations.len();
        }
        RelationalReadView {
            snapshot: state.handle.clone(),
            entities,
            relations,
        }
    }

    pub(crate) fn pin_snapshot_state(&mut self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.pin_entity(crate::identity::data::EntityId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.pin_relation(crate::identity::data::RelationId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
        }
    }

    pub(crate) fn unpin_snapshot_state(&mut self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.unpin_entity(crate::identity::data::EntityId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.unpin_relation(crate::identity::data::RelationId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
        }
    }

    pub(crate) fn pin_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        let slot = entity_id.local_slot.0 as usize;
        let Some(partition) = self.partitions.get_mut(&entity_id.partition_id) else {
            return;
        };
        if slot >= partition.entity_arena.snapshot_pins.len() {
            return;
        }
        self.instrumentation
            .complexity_counters
            .borrow_mut()
            .snapshot_pin_adjustments += 1;
        partition.entity_arena.snapshot_pins[slot] += 1;
        if partition.entity_arena.retired_at[slot].is_some() {
            partition.entity_arena.lifecycle[slot] = RecordLifecycleState::PinnedBySnapshot;
        }
    }

    pub(crate) fn unpin_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        let slot = entity_id.local_slot.0 as usize;
        let Some(partition) = self.partitions.get_mut(&entity_id.partition_id) else {
            return;
        };
        if slot >= partition.entity_arena.snapshot_pins.len()
            || partition.entity_arena.snapshot_pins[slot] == 0
        {
            return;
        }
        self.instrumentation
            .complexity_counters
            .borrow_mut()
            .snapshot_pin_adjustments += 1;
        partition.entity_arena.snapshot_pins[slot] -= 1;
        let retired_at = partition.entity_arena.retired_at[slot];
        let retention_fence = self.retention_fence_version(self.current_version_id());
        self.refresh_entity_retention_state(
            entity_id.partition_id,
            slot,
            retired_at,
            retention_fence,
        );
    }

    pub(crate) fn pin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        let slot = relation_id.local_slot.0 as usize;
        let Some(partition) = self.partitions.get_mut(&relation_id.partition_id) else {
            return;
        };
        if slot >= partition.relation_arena.snapshot_pins.len() {
            return;
        }
        self.instrumentation
            .complexity_counters
            .borrow_mut()
            .snapshot_pin_adjustments += 1;
        partition.relation_arena.snapshot_pins[slot] += 1;
        if partition.relation_arena.retired_at[slot].is_some() {
            partition.relation_arena.lifecycle[slot] = RecordLifecycleState::PinnedBySnapshot;
        }
    }

    pub(crate) fn unpin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        let slot = relation_id.local_slot.0 as usize;
        let Some(partition) = self.partitions.get_mut(&relation_id.partition_id) else {
            return;
        };
        if slot >= partition.relation_arena.snapshot_pins.len()
            || partition.relation_arena.snapshot_pins[slot] == 0
        {
            return;
        }
        self.instrumentation
            .complexity_counters
            .borrow_mut()
            .snapshot_pin_adjustments += 1;
        partition.relation_arena.snapshot_pins[slot] -= 1;
        let retired_at = partition.relation_arena.retired_at[slot];
        let retention_fence = self.retention_fence_version(self.current_version_id());
        self.refresh_relation_retention_state(
            relation_id.partition_id,
            slot,
            retired_at,
            retention_fence,
        );
    }

    pub(crate) fn refresh_entity_retention_state(
        &mut self,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
        retired_at: Option<crate::identity::data::VersionId>,
        retention_fence: crate::identity::data::VersionId,
    ) {
        let Some(_retired_at) = retired_at else {
            return;
        };
        let partition = self
            .partitions
            .get_mut(&partition_id)
            .expect("entity retention partition present");
        partition.entity_arena.lifecycle[slot] = match self.config.retention_policy.backend {
            crate::config::data::RetentionBackend::PinTrackedRetention => {
                if partition.entity_arena.snapshot_pins[slot] > 0 {
                    RecordLifecycleState::PinnedBySnapshot
                } else if partition.entity_arena.branch_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByBranch
                } else if partition.entity_arena.replay_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByReplayRetention
                } else {
                    RecordLifecycleState::Reclaimable
                }
            }
            crate::config::data::RetentionBackend::EpochChunkRetention => {
                if partition.entity_arena.branch_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByBranch
                } else if partition.entity_arena.replay_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByReplayRetention
                } else if retired_at.is_some_and(|retired| retired <= retention_fence) {
                    RecordLifecycleState::Reclaimable
                } else {
                    RecordLifecycleState::PinnedBySnapshot
                }
            }
        };
    }

    pub(crate) fn refresh_relation_retention_state(
        &mut self,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
        retired_at: Option<crate::identity::data::VersionId>,
        retention_fence: crate::identity::data::VersionId,
    ) {
        let Some(_retired_at) = retired_at else {
            return;
        };
        let partition = self
            .partitions
            .get_mut(&partition_id)
            .expect("relation retention partition present");
        partition.relation_arena.lifecycle[slot] = match self.config.retention_policy.backend {
            crate::config::data::RetentionBackend::PinTrackedRetention => {
                if partition.relation_arena.snapshot_pins[slot] > 0 {
                    RecordLifecycleState::PinnedBySnapshot
                } else {
                    RecordLifecycleState::Reclaimable
                }
            }
            crate::config::data::RetentionBackend::EpochChunkRetention => {
                if retired_at.is_some_and(|retired| retired <= retention_fence) {
                    RecordLifecycleState::Reclaimable
                } else {
                    RecordLifecycleState::PinnedBySnapshot
                }
            }
        };
    }

    pub(crate) fn trim_live_history_for_records(
        &mut self,
        changed_records: &[crate::transactions::data::RecordRef],
        published_version: crate::identity::data::VersionId,
    ) {
        let oldest_pinned_version = self.retention_fence_version(published_version);

        let mut entity_slots = BTreeMap::new();
        let mut relation_slots = BTreeMap::new();
        for record in changed_records {
            match record {
                crate::transactions::data::RecordRef::Entity(entity_id) => {
                    entity_slots
                        .entry(entity_id.partition_id)
                        .or_insert_with(BTreeSet::new)
                        .insert(entity_id.local_slot.0 as usize);
                }
                crate::transactions::data::RecordRef::Relation(relation_id) => {
                    relation_slots
                        .entry(relation_id.partition_id)
                        .or_insert_with(BTreeSet::new)
                        .insert(relation_id.local_slot.0 as usize);
                }
            }
        }

        for (partition_id, slots) in entity_slots {
            let Some(partition) = self.partitions.get_mut(&partition_id) else {
                continue;
            };
            for slot in slots {
                if slot >= partition.entity_arena.payload_history.len()
                    || partition.entity_arena.lifecycle[slot] != RecordLifecycleState::Live
                {
                    continue;
                }
                let history = &mut partition.entity_arena.payload_history[slot];
                let original_len = history.len();
                history.retain(|entry| {
                    entry
                        .retired_at
                        .is_none_or(|retired| retired > oldest_pinned_version)
                });
                self.instrumentation
                    .complexity_counters
                    .borrow_mut()
                    .live_entity_history_entries_trimmed +=
                    original_len.saturating_sub(history.len());
            }
        }

        for (partition_id, slots) in relation_slots {
            let Some(partition) = self.partitions.get_mut(&partition_id) else {
                continue;
            };
            for slot in slots {
                if !partition.relation_arena.payload_history.contains_key(&slot)
                    || partition.relation_arena.lifecycle[slot] != RecordLifecycleState::Live
                {
                    continue;
                }
                let history = partition
                    .relation_arena
                    .payload_history
                    .get_mut(&slot)
                    .expect("relation history present after key check");
                let original_len = history.len();
                history.retain(|entry| {
                    entry
                        .retired_at
                        .is_none_or(|retired| retired > oldest_pinned_version)
                });
                self.instrumentation
                    .complexity_counters
                    .borrow_mut()
                    .live_relation_history_entries_trimmed +=
                    original_len.saturating_sub(history.len());
                if history.is_empty() {
                    partition.relation_arena.payload_history.remove(&slot);
                }
            }
        }
    }
}
