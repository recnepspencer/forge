use crate::capabilities::VisibilityPolicySource;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::logic::state::{
    EntityRecordKind, PinClass, RecordKind, RelationRecordKind, SnapshotState,
};
use crate::storage::substrate::PinClass as SubstratePinClass;
use crate::visibility::cache_state::{
    bump_visibility_ref, evict_cache_if_needed, maybe_remove_unprotected_state,
    protect_branch_head_version,
};
use crate::visibility::snapshot_states::build_partition_pins_for_version;
use std::collections::{BTreeMap, BTreeSet};

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

    pub(crate) fn pin_branch_version(&mut self, version_id: crate::identity::data::VersionId) {
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
                self.pin_replay_entity(crate::identity::data::EntityId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.pin_replay_relation(crate::identity::data::RelationId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
        }
    }

    pub(crate) fn unpin_replay_state(&mut self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.unpin_replay_entity(crate::identity::data::EntityId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.unpin_replay_relation(crate::identity::data::RelationId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
        }
    }

    pub(crate) fn advance_branch_pins_for_changed_records(
        &mut self,
        old_version: Option<crate::identity::data::VersionId>,
        new_version: crate::identity::data::VersionId,
        changed_records: &[crate::transactions::data::RecordRef],
    ) {
        if old_version.is_none() {
            let mut entity_slots =
                BTreeMap::<crate::identity::data::PartitionId, BTreeSet<usize>>::new();
            let mut relation_slots =
                BTreeMap::<crate::identity::data::PartitionId, BTreeSet<usize>>::new();
            for record in changed_records {
                match record {
                    crate::transactions::data::RecordRef::Entity(entity_id) => {
                        entity_slots
                            .entry(entity_id.partition_id)
                            .or_default()
                            .insert(entity_id.local_slot.0 as usize);
                    }
                    crate::transactions::data::RecordRef::Relation(relation_id) => {
                        relation_slots
                            .entry(relation_id.partition_id)
                            .or_default()
                            .insert(relation_id.local_slot.0 as usize);
                    }
                }
            }
            self.runtime
                .storage_authority()
                .increment_named_pins_bulk::<EntityRecordKind>(&entity_slots, PinClass::Branch);
            self.runtime
                .storage_authority()
                .increment_named_pins_bulk::<RelationRecordKind>(&relation_slots, PinClass::Branch);
            return;
        }

        let current_state = self.runtime.storage_access().current_state();
        let reader = self.runtime.read_truth();
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
                        .relation_record_for_id_at_version(
                            &current_state,
                            *relation_id,
                            new_version,
                        )
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
        self.runtime
            .storage_authority()
            .clear_named_pins(PinClass::Branch);
        let head_versions = self.runtime.history().branch_head_versions();
        for version_id in head_versions {
            self.pin_branch_version(version_id);
        }
    }

    pub(crate) fn rebuild_branch_head_visibility_residency(&mut self) {
        let tracked_versions = self.runtime.visibility.tracked_branch_head_versions();
        self.runtime
            .visibility
            .clear_branch_head_residency(&tracked_versions);
        for version_id in tracked_versions {
            maybe_remove_unprotected_state(self.runtime, version_id);
        }
        if !self.runtime.protect_branch_heads() {
            evict_cache_if_needed(self.runtime);
            return;
        }
        let head_versions = self.runtime.history().branch_head_versions();
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
        self.runtime
            .storage_authority()
            .pin_snapshot_record::<EntityRecordKind>(entity_id);
    }

    fn unpin_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        let retention_fence = self
            .runtime
            .visibility
            .retention_fence_version(self.runtime.current_version_id());
        self.runtime
            .storage_authority()
            .unpin_snapshot_record::<EntityRecordKind>(entity_id, retention_fence);
    }

    fn pin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        self.runtime
            .storage_authority()
            .pin_snapshot_record::<RelationRecordKind>(relation_id);
    }

    fn unpin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        let retention_fence = self
            .runtime
            .visibility
            .retention_fence_version(self.runtime.current_version_id());
        self.runtime
            .storage_authority()
            .unpin_snapshot_record::<RelationRecordKind>(relation_id, retention_fence);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
    use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::storage::logic::state::PartitionState;
    use crate::storage::substrate::{
        EntityArena, EntityExtra, RelationArena, RelationEndpoints, RelationExtra, SlotInit,
    };
    use crate::transactions::data::RecordRef;

    #[test]
    fn initial_branch_head_bulk_pin_advancement_sets_branch_pins_for_changed_records() {
        let mut runtime = RelationalRuntimeBuilder::new().build();
        let partition_id = PartitionId(7);
        runtime.partitions.insert(
            partition_id,
            PartitionState {
                partition_id,
                adjacency_policy: AdjacencyPolicy {
                    backend: AdjacencyBackend::CompressedFanoutAdjacency,
                    small_degree_inline_capacity: 8,
                },
                relation_overlay_is_sparse: false,
                entity_arena: EntityArena::with_capacity(1),
                relation_arena: RelationArena::with_capacity(1),
                adjacency: Vec::new(),
                reverse_adjacency: Vec::new(),
            },
        );
        let partition = runtime.partitions.get_mut(&partition_id).unwrap();
        partition
            .entity_arena
            .push_slot(SlotInit::<EntityRecordKind> {
                partition_id,
                kind_id: KindId(1),
                payload: None,
                version_id: VersionId(1),
                extra: EntityExtra::default(),
            });
        let entity_id = EntityId::new(partition_id, 0, 1);
        partition
            .relation_arena
            .push_slot(SlotInit::<RelationRecordKind> {
                partition_id,
                kind_id: KindId(2),
                payload: None,
                version_id: VersionId(1),
                extra: RelationExtra::from(Some(RelationEndpoints {
                    source: entity_id,
                    target: entity_id,
                })),
            });

        let relation_id = RelationId::new(partition_id, 0, 1);
        runtime
            .visibility_pins()
            .advance_branch_pins_for_changed_records(
                None,
                VersionId(2),
                &[
                    RecordRef::Entity(entity_id),
                    RecordRef::Relation(relation_id),
                ],
            );

        let partition = runtime.partitions.get(&partition_id).unwrap();
        assert_eq!(partition.entity_arena.branch_pin_count(0), Some(1));
        assert_eq!(partition.relation_arena.branch_pin_count(0), Some(1));
    }
}

fn adjust_entity_pin(
    runtime: &mut RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    class: SubstratePinClass,
    delta: i32,
) {
    adjust_record_pin::<EntityRecordKind>(runtime, entity_id, class, delta);
}

fn adjust_relation_pin(
    runtime: &mut RelationalRuntime,
    relation_id: crate::identity::data::RelationId,
    class: SubstratePinClass,
    delta: i32,
) {
    adjust_record_pin::<RelationRecordKind>(runtime, relation_id, class, delta);
}

fn adjust_record_pin<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    record_id: crate::identity::data::RecordId<K::Domain>,
    class: SubstratePinClass,
    delta: i32,
) {
    let retention_fence = runtime
        .visibility
        .retention_fence_version(runtime.current_version_id());
    runtime
        .storage_authority()
        .adjust_named_pin::<K>(record_id, class, delta, retention_fence);
}

impl RelationalRuntime {
    pub(crate) fn visibility_pins(&mut self) -> VisibilityPinAuthority<'_> {
        VisibilityPinAuthority::new(self)
    }
}
