use std::collections::{BTreeMap, BTreeSet};

use crate::capabilities::VisibilityPolicySource;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::{
    EntityRecordKind, HistoricalMetadata, PinClass, RecordKind, RelationRecordKind, SnapshotState,
};

impl RelationalRuntime {
    pub(crate) fn pin_branch_version(&mut self, version_id: crate::identity::data::VersionId) {
        let pinned_partitions = self.build_partition_pins_for_version(version_id);
        for (partition_id, pins) in pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.pin_branch_entity(crate::identity::data::EntityId::new(partition_id, slot as u64, 0));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.pin_branch_relation(crate::identity::data::RelationId::new(partition_id, slot as u64, 0));
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
        let current_state = self.current_state();
        let mut entity_actions = Vec::new();
        let mut relation_actions = Vec::new();
        for record in changed_records {
            match record {
                crate::transactions::data::RecordRef::Entity(entity_id) => {
                    let was_visible = old_version.is_some_and(|version_id| {
                        self.entity_record_for_id_at_version(&current_state, *entity_id, version_id)
                            .is_some()
                    });
                    let is_visible = self
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
                        self.relation_record_for_id_at_version(&current_state, *relation_id, version_id)
                            .is_some()
                    });
                    let is_visible = self
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
        for partition in self.partitions.values_mut() {
            partition.entity_arena.clear_named_pins(PinClass::Branch);
            partition.relation_arena.clear_named_pins(PinClass::Branch);
        }
        let head_versions = self.branch_head_versions();
        for version_id in head_versions {
            self.pin_branch_version(version_id);
        }
    }

    pub(crate) fn rebuild_branch_head_visibility_residency(&self) {
        let tracked_versions = self
            .snapshots
            .visibility_residency
            .read()
            .expect("visibility residency lock poisoned")
            .iter()
            .filter_map(|(version_id, residency)| (residency.branch_head_refs > 0).then_some(*version_id))
            .collect::<Vec<_>>();
        {
            let mut residency = self
                .snapshots
                .visibility_residency
                .write()
                .expect("visibility residency lock poisoned");
            for version_id in &tracked_versions {
                if let Some(entry) = residency.get_mut(version_id) {
                    entry.branch_head_refs = 0;
                    if entry.replay_refs == 0 && entry.active_snapshot_refs == 0 && !entry.recent_resident {
                        residency.remove(version_id);
                    }
                }
            }
        }
        for version_id in tracked_versions {
            self.maybe_remove_unprotected_visibility_state(version_id);
        }
        if !self.protect_branch_heads() {
            self.evict_visibility_cache_if_needed();
            return;
        }
        let head_versions = self.branch_head_versions();
        for version_id in head_versions {
            self.protect_branch_head_version(version_id);
            self.instrumentation
                .count(|counters| counters.visibility_cache_branch_head_promotions += 1);
        }
        self.evict_visibility_cache_if_needed();
    }

    pub(crate) fn move_branch_head_visibility_residency(
        &self,
        previous_head: Option<crate::identity::data::VersionId>,
        next_head: Option<crate::identity::data::VersionId>,
    ) {
        if !self.protect_branch_heads() || previous_head == next_head {
            return;
        }
        if let Some(version_id) = previous_head {
            self.bump_visibility_ref(version_id, |residency| {
                residency.branch_head_refs = residency.branch_head_refs.saturating_sub(1);
            });
        }
        if let Some(version_id) = next_head {
            self.protect_branch_head_version(version_id);
            self.instrumentation
                .count(|counters| counters.visibility_cache_branch_head_promotions += 1);
        }
        self.evict_visibility_cache_if_needed();
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

        trim_live_history::<EntityRecordKind>(self, entity_slots, oldest_pinned_version, |runtime, trimmed| {
            runtime
                .instrumentation
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned")
                .live_entity_history_entries_trimmed += trimmed;
        });

        trim_live_history::<RelationRecordKind>(self, relation_slots, oldest_pinned_version, |runtime, trimmed| {
            runtime
                .instrumentation
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned")
                .live_relation_history_entries_trimmed += trimmed;
        });
    }
}

fn trim_live_history<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    slots_by_partition: BTreeMap<crate::identity::data::PartitionId, BTreeSet<usize>>,
    oldest_pinned_version: crate::identity::data::VersionId,
    count_trimmed: impl Fn(&RelationalRuntime, usize),
) where
    K::Meta: HistoricalMetadata,
{
    let mut total_trimmed = 0usize;
    for (partition_id, slots) in slots_by_partition {
        let Some(partition) = runtime.partitions.get_mut(&partition_id) else {
            continue;
        };
        let arena = K::arena_mut(partition);
        for slot in slots {
            if arena
                .get_slot(slot)
                .is_none_or(|slot_view| slot_view.lifecycle() != RecordLifecycleState::Live)
            {
                continue;
            }
            if arena
                .metadata_history_at(slot)
                .is_some_and(|metadata_history| metadata_history.len() > 1)
            {
                continue;
            }
            let bound = crate::identity::data::VersionBound::new(oldest_pinned_version);
            let original_len = match arena.payload_history_at(slot) {
                Some(history) => history.len(),
                None => continue,
            };
            let trimmed_len = {
                let Some(history) = arena.payload_history_at_mut(slot) else {
                    continue;
                };
                history.retain(|entry| {
                    entry.retired_at.is_none_or(|retired| bound.retains_retired(retired))
                });
                history.len()
            };
            if let Some(metadata_history) = arena.metadata_history_at_mut(slot) {
                metadata_history.retain(|entry| {
                    entry.retired_at().is_none_or(|retired| bound.retains_retired(retired))
                });
            }
            total_trimmed += original_len.saturating_sub(trimmed_len);
        }
    }
    count_trimmed(runtime, total_trimmed);
}
