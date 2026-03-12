use crate::capabilities::VisibilityPolicySource;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{
    EntityRecordKind, PinClass, RecordKind, RelationRecordKind, SnapshotState,
};
use crate::storage::substrate::PinClass as SubstratePinClass;

use crate::visibility::retention::{
    refresh_entity_retention_state, refresh_relation_retention_state,
};
use crate::visibility::cache_state::{
    bump_visibility_ref, evict_cache_if_needed, maybe_remove_unprotected_state,
    protect_branch_head_version,
};
use crate::visibility::snapshot_states::build_partition_pins_for_version;

pub(crate) struct VisibilityPinAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> VisibilityPinAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

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

    pub(crate) fn pin_branch_version(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) {
        let pinned_partitions = build_partition_pins_for_version(self.runtime, version_id);
        for (partition_id, pins) in pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.pin_branch_entity(crate::identity::data::EntityId::new(
                    partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.pin_branch_relation(crate::identity::data::RelationId::new(
                    partition_id,
                    slot as u64,
                    0,
                ));
            }
        }
    }

    pub(crate) fn pin_replay_state(&mut self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.pin_replay_entity(crate::identity::data::EntityId::new(*partition_id, slot as u64, 0));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.pin_replay_relation(crate::identity::data::RelationId::new(*partition_id, slot as u64, 0));
            }
        }
    }

    pub(crate) fn unpin_replay_state(&mut self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.unpin_replay_entity(crate::identity::data::EntityId::new(*partition_id, slot as u64, 0));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.unpin_replay_relation(crate::identity::data::RelationId::new(*partition_id, slot as u64, 0));
            }
        }
    }

    pub(crate) fn advance_branch_pins_for_changed_records(
        &mut self,
        old_version: Option<crate::identity::data::VersionId>,
        new_version: crate::identity::data::VersionId,
        changed_records: &[crate::transactions::data::RecordRef],
    ) {
        let current_state = self.runtime.current_state();
        let reader = self.runtime.visibility_reads();
        let mut entity_actions = Vec::new();
        let mut relation_actions = Vec::new();
        for record in changed_records {
            match record {
                crate::transactions::data::RecordRef::Entity(entity_id) => {
                    let was_visible = old_version.is_some_and(|version_id| {
                        reader
                            .entity_record_for_id_at_version(&current_state, *entity_id, version_id)
                            .is_some()
                    });
                    let is_visible = reader
                        .entity_record_for_id_at_version(&current_state, *entity_id, new_version)
                        .is_some();
                    match (was_visible, is_visible) {
                        (false, true) => entity_actions.push((*entity_id, 1)),
                        (true, false) => entity_actions.push((*entity_id, -1)),
                        _ => {}
                    }
                }
                crate::transactions::data::RecordRef::Relation(relation_id) => {
                    let was_visible = old_version.is_some_and(|version_id| {
                        reader
                            .relation_record_for_id_at_version(
                                &current_state,
                                *relation_id,
                                version_id,
                            )
                            .is_some()
                    });
                    let is_visible = reader
                        .relation_record_for_id_at_version(&current_state, *relation_id, new_version)
                        .is_some();
                    match (was_visible, is_visible) {
                        (false, true) => relation_actions.push((*relation_id, 1)),
                        (true, false) => relation_actions.push((*relation_id, -1)),
                        _ => {}
                    }
                }
            }
        }
        drop(current_state);
        for (entity_id, delta) in entity_actions {
            if delta > 0 {
                self.pin_branch_entity(entity_id);
            } else {
                self.unpin_branch_entity(entity_id);
            }
        }
        for (relation_id, delta) in relation_actions {
            if delta > 0 {
                self.pin_branch_relation(relation_id);
            } else {
                self.unpin_branch_relation(relation_id);
            }
        }
    }

    pub(crate) fn rebuild_branch_pins_from_heads(&mut self) {
        for partition in self.runtime.partitions.values_mut() {
            partition.entity_arena.clear_named_pins(PinClass::Branch);
            partition.relation_arena.clear_named_pins(PinClass::Branch);
        }
        let head_versions = self.runtime.history_access().branch_head_versions();
        for version_id in head_versions {
            self.pin_branch_version(version_id);
        }
    }

    pub(crate) fn rebuild_branch_head_visibility_residency(&mut self) {
        let tracked_versions = self.runtime.visibility.cache.tracked_branch_head_versions();
        self.runtime
            .visibility
            .cache
            .clear_branch_head_residency(&tracked_versions);
        for version_id in tracked_versions {
            maybe_remove_unprotected_state(self.runtime, version_id);
        }
        if !self.runtime.protect_branch_heads() {
            evict_cache_if_needed(self.runtime);
            return;
        }
        let head_versions = self.runtime.history_access().branch_head_versions();
        for version_id in head_versions {
            protect_branch_head_version(self.runtime, version_id);
            self.runtime.services.instrumentation.count(|counters| {
                counters.visibility_cache_branch_head_promotions += 1;
            });
        }
        evict_cache_if_needed(self.runtime);
    }

    pub(crate) fn move_branch_head_visibility_residency(
        &mut self,
        previous_head: Option<crate::identity::data::VersionId>,
        next_head: Option<crate::identity::data::VersionId>,
    ) {
        if !self.runtime.protect_branch_heads() || previous_head == next_head {
            return;
        }
        if let Some(version_id) = previous_head {
            bump_visibility_ref(self.runtime, version_id, |residency| {
                residency.branch_head_refs = residency.branch_head_refs.saturating_sub(1);
            });
        }
        if let Some(version_id) = next_head {
            protect_branch_head_version(self.runtime, version_id);
            self.runtime.services.instrumentation.count(|counters| {
                counters.visibility_cache_branch_head_promotions += 1;
            });
        }
        evict_cache_if_needed(self.runtime);
    }

    fn pin_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        pin_snapshot_record::<EntityRecordKind>(self.runtime, entity_id);
    }

    fn unpin_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        unpin_snapshot_record::<EntityRecordKind>(
            self.runtime,
            entity_id,
            refresh_entity_retention_state,
        );
    }

    fn pin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        pin_snapshot_record::<RelationRecordKind>(self.runtime, relation_id);
    }

    fn unpin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        unpin_snapshot_record::<RelationRecordKind>(
            self.runtime,
            relation_id,
            refresh_relation_retention_state,
        );
    }

    fn pin_branch_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self.runtime, entity_id, SubstratePinClass::Branch, 1);
    }

    fn unpin_branch_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self.runtime, entity_id, SubstratePinClass::Branch, -1);
    }

    fn pin_replay_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self.runtime, entity_id, SubstratePinClass::Replay, 1);
    }

    fn unpin_replay_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self.runtime, entity_id, SubstratePinClass::Replay, -1);
    }

    fn pin_branch_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self.runtime, relation_id, SubstratePinClass::Branch, 1);
    }

    fn unpin_branch_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self.runtime, relation_id, SubstratePinClass::Branch, -1);
    }

    fn pin_replay_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self.runtime, relation_id, SubstratePinClass::Replay, 1);
    }

    fn unpin_replay_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self.runtime, relation_id, SubstratePinClass::Replay, -1);
    }
}

fn adjust_entity_pin(
    runtime: &mut RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    class: SubstratePinClass,
    delta: i32,
) {
    adjust_record_pin::<EntityRecordKind>(
        runtime,
        entity_id,
        class,
        delta,
        refresh_entity_retention_state,
    );
}

fn adjust_relation_pin(
    runtime: &mut RelationalRuntime,
    relation_id: crate::identity::data::RelationId,
    class: SubstratePinClass,
    delta: i32,
) {
    adjust_record_pin::<RelationRecordKind>(
        runtime,
        relation_id,
        class,
        delta,
        refresh_relation_retention_state,
    );
}

fn pin_snapshot_record<K: RecordKind>(runtime: &mut RelationalRuntime, record_id: K::Id) {
    let slot = K::slot_of(&record_id);
    let Some(partition) = runtime.partitions.get_mut(&K::partition_of(&record_id)) else {
        return;
    };
    let arena = K::arena_mut(partition);
    if arena.snapshot_pin_count(slot).is_none() {
        return;
    }
    runtime
        .services
        .instrumentation
        .count(|counters| counters.snapshot_pin_adjustments += 1);
    arena.increment_snapshot_pin(slot);
    if arena.retired_at_for_slot(slot).is_some() {
        arena.set_lifecycle_for_slot(slot, RecordLifecycleState::PinnedBySnapshot);
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
    let slot = K::slot_of(&record_id);
    let partition_id = K::partition_of(&record_id);
    let Some(partition) = runtime.partitions.get_mut(&partition_id) else {
        return;
    };
    let arena = K::arena_mut(partition);
    if arena.snapshot_pin_count(slot).unwrap_or(0) == 0 {
        return;
    }
    runtime
        .services
        .instrumentation
        .count(|counters| counters.snapshot_pin_adjustments += 1);
    arena.decrement_snapshot_pin(slot);
    let retired_at = arena.retired_at_for_slot(slot);
    let retention_fence = runtime.retention_fence_version(runtime.current_version_id());
    refresh_retention(runtime, partition_id, slot, retired_at, retention_fence);
}

fn adjust_record_pin<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    record_id: K::Id,
    class: SubstratePinClass,
    delta: i32,
    refresh_retention: fn(
        &mut RelationalRuntime,
        crate::identity::data::PartitionId,
        usize,
        Option<crate::identity::data::VersionId>,
        crate::identity::data::VersionId,
    ),
) {
    let slot = K::slot_of(&record_id);
    let partition_id = K::partition_of(&record_id);
    let Some(partition_len) = runtime
        .partitions
        .get(&partition_id)
        .map(|partition| K::arena(partition).slot_count())
    else {
        return;
    };
    if slot >= partition_len {
        return;
    }
    {
        let partition = runtime
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
    let retired_at = runtime
        .partitions
        .get(&partition_id)
        .and_then(|partition| K::arena(partition).retired_at_for_slot(slot));
    let retention_fence = runtime.retention_fence_version(runtime.current_version_id());
    refresh_retention(runtime, partition_id, slot, retired_at, retention_fence);
}

impl RelationalRuntime {
    pub(crate) fn visibility_pins(&mut self) -> VisibilityPinAuthority<'_> {
        VisibilityPinAuthority::new(self)
    }
}
