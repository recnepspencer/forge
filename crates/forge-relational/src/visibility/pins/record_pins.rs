use crate::identity::data::VersionBound;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{
    EntityRecordKind, RecordId, RecordKind, RelationRecordKind, SnapshotState,
};

impl RelationalRuntime {
    pub(crate) fn pin_snapshot_state(&mut self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.pin_entity(crate::identity::data::EntityId::new(*partition_id, slot as u64, 0));
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
                self.unpin_entity(crate::identity::data::EntityId::new(*partition_id, slot as u64, 0));
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
        pin_snapshot_record::<EntityRecordKind>(self, entity_id);
    }

    pub(crate) fn unpin_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        unpin_snapshot_record::<EntityRecordKind>(
            self,
            entity_id,
            RelationalRuntime::refresh_entity_retention_state,
        );
    }

    pub(crate) fn pin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        pin_snapshot_record::<RelationRecordKind>(self, relation_id);
    }

    pub(crate) fn unpin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        unpin_snapshot_record::<RelationRecordKind>(
            self,
            relation_id,
            RelationalRuntime::refresh_relation_retention_state,
        );
    }

    pub(crate) fn pin_branch_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self, entity_id, PinClass::Branch, 1);
    }

    pub(crate) fn unpin_branch_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self, entity_id, PinClass::Branch, -1);
    }

    pub(crate) fn pin_replay_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self, entity_id, PinClass::Replay, 1);
    }

    pub(crate) fn unpin_replay_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self, entity_id, PinClass::Replay, -1);
    }

    pub(crate) fn pin_branch_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self, relation_id, PinClass::Branch, 1);
    }

    pub(crate) fn unpin_branch_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self, relation_id, PinClass::Branch, -1);
    }

    pub(crate) fn pin_replay_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self, relation_id, PinClass::Replay, 1);
    }

    pub(crate) fn unpin_replay_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self, relation_id, PinClass::Replay, -1);
    }

    pub(crate) fn refresh_entity_retention_state(
        &mut self,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
        retired_at: Option<crate::identity::data::VersionId>,
        retention_fence: crate::identity::data::VersionId,
    ) {
        refresh_retention_state::<EntityRecordKind>(
            self,
            partition_id,
            slot,
            retired_at,
            retention_fence,
        );
    }

    pub(crate) fn refresh_relation_retention_state(
        &mut self,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
        retired_at: Option<crate::identity::data::VersionId>,
        retention_fence: crate::identity::data::VersionId,
    ) {
        refresh_retention_state::<RelationRecordKind>(
            self,
            partition_id,
            slot,
            retired_at,
            retention_fence,
        );
    }
}

#[derive(Clone, Copy)]
enum PinClass {
    Branch,
    Replay,
}

fn adjust_entity_pin(
    runtime: &mut RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    class: PinClass,
    delta: i32,
) {
    adjust_record_pin::<EntityRecordKind>(
        runtime,
        entity_id,
        class,
        delta,
        RelationalRuntime::refresh_entity_retention_state,
    );
}

fn adjust_relation_pin(
    runtime: &mut RelationalRuntime,
    relation_id: crate::identity::data::RelationId,
    class: PinClass,
    delta: i32,
) {
    adjust_record_pin::<RelationRecordKind>(
        runtime,
        relation_id,
        class,
        delta,
        RelationalRuntime::refresh_relation_retention_state,
    );
}

fn pin_snapshot_record<K: RecordKind>(runtime: &mut RelationalRuntime, record_id: K::Id) {
    let slot = record_id.local_slot();
    let Some(partition) = runtime.partitions.get_mut(&record_id.partition_id()) else {
        return;
    };
    let arena = K::arena_mut(partition);
    if slot >= arena.snapshot_pins.len() {
        return;
    }
    runtime
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned")
        .snapshot_pin_adjustments += 1;
    arena.snapshot_pins[slot] = arena.snapshot_pins[slot].saturating_add(1);
    if arena.retired_at[slot].is_some() {
        arena.lifecycle[slot] = RecordLifecycleState::PinnedBySnapshot;
    }
}

fn unpin_snapshot_record<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    record_id: K::Id,
    refresh_retention: fn(
        &mut RelationalRuntime,
        crate::identity::data::PartitionId,
        usize,
        Option<crate::identity::data::VersionId>,
        crate::identity::data::VersionId,
    ),
) {
    let slot = record_id.local_slot();
    let Some(partition) = runtime.partitions.get_mut(&record_id.partition_id()) else {
        return;
    };
    let arena = K::arena_mut(partition);
    if slot >= arena.snapshot_pins.len() || arena.snapshot_pins[slot] == 0 {
        return;
    }
    runtime
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned")
        .snapshot_pin_adjustments += 1;
    arena.snapshot_pins[slot] -= 1;
    let retired_at = arena.retired_at[slot];
    let retention_fence = runtime.retention_fence_version(runtime.current_version_id());
    refresh_retention(runtime, record_id.partition_id(), slot, retired_at, retention_fence);
}

fn refresh_retention_state<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
    retired_at: Option<crate::identity::data::VersionId>,
    retention_fence: crate::identity::data::VersionId,
) {
    let Some(_retired_at) = retired_at else {
        return;
    };
    let partition = runtime
        .partitions
        .get_mut(&partition_id)
        .expect("retention partition present");
    let arena = K::arena_mut(partition);
    arena.lifecycle[slot] = match runtime.config.retention_policy.backend {
        crate::config::data::RetentionBackend::PinTrackedRetention => {
            if arena.snapshot_pins[slot] > 0 {
                RecordLifecycleState::PinnedBySnapshot
            } else if arena.branch_pins[slot] > 0 {
                RecordLifecycleState::PinnedByBranch
            } else if arena.replay_pins[slot] > 0 {
                RecordLifecycleState::PinnedByReplayRetention
            } else {
                RecordLifecycleState::Reclaimable
            }
        }
        crate::config::data::RetentionBackend::EpochChunkRetention => {
            if arena.branch_pins[slot] > 0 {
                RecordLifecycleState::PinnedByBranch
            } else if arena.replay_pins[slot] > 0 {
                RecordLifecycleState::PinnedByReplayRetention
            } else if retired_at.is_some_and(|retired| {
                !VersionBound::new(retention_fence).retains_retired(retired)
            }) {
                RecordLifecycleState::Reclaimable
            } else {
                RecordLifecycleState::PinnedBySnapshot
            }
        }
    };
}

fn adjust_record_pin<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    record_id: K::Id,
    class: PinClass,
    delta: i32,
    refresh_retention: fn(
        &mut RelationalRuntime,
        crate::identity::data::PartitionId,
        usize,
        Option<crate::identity::data::VersionId>,
        crate::identity::data::VersionId,
    ),
) {
    let slot = record_id.local_slot();
    let Some(partition_len) = runtime
        .partitions
        .get(&record_id.partition_id())
        .map(|partition| K::arena(partition).snapshot_pins.len())
    else {
        return;
    };
    if slot >= partition_len {
        return;
    }
    {
        let partition = runtime
            .partitions
            .get_mut(&record_id.partition_id())
            .expect("partition present while adjusting pin");
        let arena = K::arena_mut(partition);
        let counter = match class {
            PinClass::Branch => &mut arena.branch_pins[slot],
            PinClass::Replay => &mut arena.replay_pins[slot],
        };
        if delta < 0 {
            if *counter == 0 {
                return;
            }
            *counter -= 1;
        } else {
            *counter = (*counter).saturating_add(delta as u32);
        }
    }
    let retired_at = runtime
        .partitions
        .get(&record_id.partition_id())
        .and_then(|partition| K::arena(partition).retired_at.get(slot).copied())
        .flatten();
    let retention_fence = runtime.retention_fence_version(runtime.current_version_id());
    refresh_retention(runtime, record_id.partition_id(), slot, retired_at, retention_fence);
}
