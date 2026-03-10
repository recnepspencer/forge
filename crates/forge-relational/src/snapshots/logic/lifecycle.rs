use std::collections::{BTreeMap, BTreeSet};

use crate::logic::runtime::{RelationalRuntime, VisibilityResidency};
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::data::{RecordLifecycleState, RelationalReadView};
use crate::storage::logic::state::{DenseSlotBitSet, SnapshotPartitionPins, SnapshotState};

impl RelationalRuntime {
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
        SnapshotState {
            handle,
            pinned_entity_count: entities.len(),
            pinned_relation_count: relations.len(),
            pinned_partitions,
        }
    }

    pub(crate) fn retention_fence_version(
        &self,
        published_version: crate::identity::data::VersionId,
    ) -> crate::identity::data::VersionId {
        self.snapshots
            .active
            .values()
            .map(|binding| binding.version_id)
            .chain(self.snapshots.replay_retained.keys().copied())
            .min()
            .unwrap_or(published_version)
    }

    pub(crate) fn snapshot_state_for_current(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> (SnapshotHandle, SnapshotState) {
        let snapshot_id = SnapshotId(self.snapshots.next_snapshot_id);
        self.snapshots.next_snapshot_id += 1;
        let state = self.build_visibility_state(
            version_id,
            snapshot_id,
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        );
        self.pin_snapshot_state(&state);
        (state.handle.clone(), state)
    }

    pub(crate) fn visibility_state_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<SnapshotState> {
        self.snapshots
            .visibility_states
            .borrow()
            .get(&version_id)
            .cloned()
    }

    pub(crate) fn insert_visibility_state(&self, state: SnapshotState) {
        self.snapshots
            .visibility_states
            .borrow_mut()
            .insert(state.handle.version_id, state);
    }

    pub(crate) fn visibility_residency_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> VisibilityResidency {
        self.snapshots
            .visibility_residency
            .borrow()
            .get(&version_id)
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn bump_active_snapshot_ref(
        &self,
        version_id: crate::identity::data::VersionId,
        delta: i32,
    ) {
        self.bump_visibility_ref(version_id, delta, |residency| {
            residency.active_snapshot_refs =
                residency.active_snapshot_refs.saturating_add_signed(delta);
        });
        if delta > 0 {
            self.instrumentation
                .complexity_counters
                .borrow_mut()
                .visibility_cache_snapshot_promotions += delta as usize;
        }
    }

    pub(crate) fn bump_replay_ref(&self, version_id: crate::identity::data::VersionId, delta: i32) {
        self.bump_visibility_ref(version_id, delta, |residency| {
            residency.replay_refs = residency.replay_refs.saturating_add_signed(delta);
        });
        if delta > 0 {
            self.instrumentation
                .complexity_counters
                .borrow_mut()
                .visibility_cache_replay_promotions += delta as usize;
        }
    }

    fn bump_visibility_ref(
        &self,
        version_id: crate::identity::data::VersionId,
        _delta: i32,
        update: impl FnOnce(&mut VisibilityResidency),
    ) {
        let mut residency = self.snapshots.visibility_residency.borrow_mut();
        let entry = residency.entry(version_id).or_default();
        update(entry);
        if entry.branch_head_refs == 0
            && entry.replay_refs == 0
            && entry.active_snapshot_refs == 0
            && !entry.recent_resident
        {
            residency.remove(&version_id);
        }
        drop(residency);
        self.maybe_remove_unprotected_visibility_state(version_id);
    }

    fn protect_branch_head_version(&self, version_id: crate::identity::data::VersionId) {
        self.ensure_visibility_state(version_id, false);
        self.bump_visibility_ref(version_id, 1, |residency| {
            residency.branch_head_refs += 1;
        });
    }

    pub(crate) fn ensure_visibility_state(
        &self,
        version_id: crate::identity::data::VersionId,
        recent_candidate: bool,
    ) -> SnapshotState {
        if let Some(state) = self.visibility_state_for_version(version_id) {
            self.instrumentation
                .complexity_counters
                .borrow_mut()
                .visibility_cache_hits += 1;
            return state;
        }
        self.instrumentation
            .complexity_counters
            .borrow_mut()
            .visibility_cache_miss_reconstructions += 1;
        let state = self.build_visibility_state(
            version_id,
            SnapshotId(0),
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        );
        self.insert_visibility_state(state.clone());
        if recent_candidate {
            self.mark_recent_visibility_state(version_id);
        }
        state
    }

    pub(crate) fn read_or_reconstruct_visibility_state(
        &self,
        version_id: crate::identity::data::VersionId,
        allow_recent_admission: bool,
    ) -> Option<SnapshotState> {
        if version_id.0 == 0 || version_id.0 > self.current_version_id().0 {
            return None;
        }
        if let Some(state) = self.visibility_state_for_version(version_id) {
            self.instrumentation
                .complexity_counters
                .borrow_mut()
                .visibility_cache_hits += 1;
            return Some(state);
        }
        let recent_candidate = allow_recent_admission
            && self.config.visibility_cache_policy.enabled
            && self.snapshots.recent_policy.borrow().recent_version_window > 0
            && !self.is_protected_visibility_version(version_id);
        if recent_candidate || self.is_protected_visibility_version(version_id) {
            return Some(self.ensure_visibility_state(version_id, recent_candidate));
        }
        self.instrumentation
            .complexity_counters
            .borrow_mut()
            .visibility_cache_miss_reconstructions += 1;
        Some(self.build_visibility_state(
            version_id,
            SnapshotId(0),
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        ))
    }

    pub(crate) fn is_protected_visibility_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        let residency = self.visibility_residency_for_version(version_id);
        residency.branch_head_refs > 0
            || residency.replay_refs > 0
            || residency.active_snapshot_refs > 0
    }

    pub(crate) fn mark_recent_visibility_state(
        &self,
        version_id: crate::identity::data::VersionId,
    ) {
        if !self.config.visibility_cache_policy.enabled
            || self.snapshots.recent_policy.borrow().recent_version_window == 0
        {
            return;
        }
        {
            let mut residency = self.snapshots.visibility_residency.borrow_mut();
            let entry = residency.entry(version_id).or_default();
            if entry.recent_resident {
                return;
            }
            entry.recent_resident = true;
        }
        {
            let mut recent_policy = self.snapshots.recent_policy.borrow_mut();
            recent_policy.order.push_back(version_id);
            recent_policy.resident_count += 1;
        }
        self.evict_visibility_cache_if_needed();
    }

    pub(crate) fn evict_visibility_cache_if_needed(&self) {
        let window = self.snapshots.recent_policy.borrow().recent_version_window;
        if !self.config.visibility_cache_policy.enabled || window == 0 {
            return;
        }
        loop {
            if self.snapshots.recent_policy.borrow().resident_count <= window {
                break;
            }
            let scan_len = self.snapshots.recent_policy.borrow().order.len();
            if scan_len == 0 {
                break;
            }
            let mut evicted = false;
            for _ in 0..scan_len {
                let candidate = self.snapshots.recent_policy.borrow_mut().order.pop_front();
                let Some(version_id) = candidate else {
                    break;
                };
                let mut residency = self.snapshots.visibility_residency.borrow_mut();
                let Some(entry) = residency.get_mut(&version_id) else {
                    continue;
                };
                if !entry.recent_resident {
                    continue;
                }
                if entry.branch_head_refs > 0
                    || entry.replay_refs > 0
                    || entry.active_snapshot_refs > 0
                {
                    drop(residency);
                    self.snapshots
                        .recent_policy
                        .borrow_mut()
                        .order
                        .push_back(version_id);
                    continue;
                }
                entry.recent_resident = false;
                self.snapshots.recent_policy.borrow_mut().resident_count -= 1;
                if entry.branch_head_refs == 0
                    && entry.replay_refs == 0
                    && entry.active_snapshot_refs == 0
                {
                    residency.remove(&version_id);
                }
                drop(residency);
                self.snapshots
                    .visibility_states
                    .borrow_mut()
                    .remove(&version_id);
                self.instrumentation
                    .complexity_counters
                    .borrow_mut()
                    .visibility_cache_recent_evictions += 1;
                evicted = true;
                break;
            }
            if !evicted {
                break;
            }
        }
    }

    fn maybe_remove_unprotected_visibility_state(
        &self,
        version_id: crate::identity::data::VersionId,
    ) {
        let residency = self.visibility_residency_for_version(version_id);
        if residency.branch_head_refs == 0
            && residency.replay_refs == 0
            && residency.active_snapshot_refs == 0
            && !residency.recent_resident
        {
            self.snapshots
                .visibility_states
                .borrow_mut()
                .remove(&version_id);
        }
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

    pub(crate) fn pin_branch_state(&mut self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.pin_branch_entity(crate::identity::data::EntityId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.pin_branch_relation(crate::identity::data::RelationId::new(
                    *partition_id,
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
                        self.relation_record_for_id_at_version(
                            &current_state,
                            *relation_id,
                            version_id,
                        )
                        .is_some()
                    });
                    let is_visible = self
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
        for partition in self.partitions.values_mut() {
            for counter in &mut partition.entity_arena.branch_pins {
                *counter = 0;
            }
            for counter in &mut partition.relation_arena.branch_pins {
                *counter = 0;
            }
        }
        let head_versions = self
            .history
            .branch_heads
            .values()
            .filter_map(|head| head.as_ref().map(|head| head.version_id))
            .collect::<Vec<_>>();
        for version_id in head_versions {
            let state = self.ensure_visibility_state(version_id, false);
            self.pin_branch_state(&state);
        }
    }

    pub(crate) fn rebuild_branch_head_visibility_residency(&self) {
        let tracked_versions = self
            .snapshots
            .visibility_residency
            .borrow()
            .iter()
            .filter_map(|(version_id, residency)| {
                (residency.branch_head_refs > 0).then_some(*version_id)
            })
            .collect::<Vec<_>>();
        {
            let mut residency = self.snapshots.visibility_residency.borrow_mut();
            for version_id in &tracked_versions {
                if let Some(entry) = residency.get_mut(version_id) {
                    entry.branch_head_refs = 0;
                    if entry.replay_refs == 0
                        && entry.active_snapshot_refs == 0
                        && !entry.recent_resident
                    {
                        residency.remove(version_id);
                    }
                }
            }
        }
        for version_id in tracked_versions {
            self.maybe_remove_unprotected_visibility_state(version_id);
        }
        if !self.config.visibility_cache_policy.protect_branch_heads {
            self.evict_visibility_cache_if_needed();
            return;
        }
        let head_versions = self
            .history
            .branch_heads
            .values()
            .filter_map(|head| head.as_ref().map(|head| head.version_id))
            .collect::<Vec<_>>();
        for version_id in head_versions {
            self.protect_branch_head_version(version_id);
            self.instrumentation
                .complexity_counters
                .borrow_mut()
                .visibility_cache_branch_head_promotions += 1;
        }
        self.evict_visibility_cache_if_needed();
    }

    pub(crate) fn move_branch_head_visibility_residency(
        &self,
        previous_head: Option<crate::identity::data::VersionId>,
        next_head: Option<crate::identity::data::VersionId>,
    ) {
        if !self.config.visibility_cache_policy.protect_branch_heads || previous_head == next_head {
            return;
        }
        if let Some(version_id) = previous_head {
            self.bump_visibility_ref(version_id, -1, |residency| {
                residency.branch_head_refs = residency.branch_head_refs.saturating_sub(1);
            });
        }
        if let Some(version_id) = next_head {
            self.protect_branch_head_version(version_id);
            self.instrumentation
                .complexity_counters
                .borrow_mut()
                .visibility_cache_branch_head_promotions += 1;
        }
        self.evict_visibility_cache_if_needed();
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
                } else if partition.relation_arena.branch_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByBranch
                } else if partition.relation_arena.replay_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByReplayRetention
                } else {
                    RecordLifecycleState::Reclaimable
                }
            }
            crate::config::data::RetentionBackend::EpochChunkRetention => {
                if partition.relation_arena.branch_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByBranch
                } else if partition.relation_arena.replay_pins[slot] > 0 {
                    RecordLifecycleState::PinnedByReplayRetention
                } else if retired_at.is_some_and(|retired| retired <= retention_fence) {
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
    let slot = entity_id.local_slot.0 as usize;
    let Some(partition_len) = runtime
        .partitions
        .get(&entity_id.partition_id)
        .map(|partition| partition.entity_arena.snapshot_pins.len())
    else {
        return;
    };
    if slot >= partition_len {
        return;
    }
    {
        let partition = runtime
            .partitions
            .get_mut(&entity_id.partition_id)
            .expect("entity partition present while adjusting pin");
        let counter = match class {
            PinClass::Branch => &mut partition.entity_arena.branch_pins[slot],
            PinClass::Replay => &mut partition.entity_arena.replay_pins[slot],
        };
        if delta < 0 {
            if *counter == 0 {
                return;
            }
            *counter -= 1;
        } else {
            *counter += delta as u32;
        }
    }
    let retired_at = runtime
        .partitions
        .get(&entity_id.partition_id)
        .and_then(|partition| partition.entity_arena.retired_at.get(slot).copied())
        .flatten();
    let retention_fence = runtime.retention_fence_version(runtime.current_version_id());
    runtime.refresh_entity_retention_state(
        entity_id.partition_id,
        slot,
        retired_at,
        retention_fence,
    );
}

fn adjust_relation_pin(
    runtime: &mut RelationalRuntime,
    relation_id: crate::identity::data::RelationId,
    class: PinClass,
    delta: i32,
) {
    let slot = relation_id.local_slot.0 as usize;
    let Some(partition_len) = runtime
        .partitions
        .get(&relation_id.partition_id)
        .map(|partition| partition.relation_arena.snapshot_pins.len())
    else {
        return;
    };
    if slot >= partition_len {
        return;
    }
    {
        let partition = runtime
            .partitions
            .get_mut(&relation_id.partition_id)
            .expect("relation partition present while adjusting pin");
        let counter = match class {
            PinClass::Branch => &mut partition.relation_arena.branch_pins[slot],
            PinClass::Replay => &mut partition.relation_arena.replay_pins[slot],
        };
        if delta < 0 {
            if *counter == 0 {
                return;
            }
            *counter -= 1;
        } else {
            *counter += delta as u32;
        }
    }
    let retired_at = runtime
        .partitions
        .get(&relation_id.partition_id)
        .and_then(|partition| partition.relation_arena.retired_at.get(slot).copied())
        .flatten();
    let retention_fence = runtime.retention_fence_version(runtime.current_version_id());
    runtime.refresh_relation_retention_state(
        relation_id.partition_id,
        slot,
        retired_at,
        retention_fence,
    );
}
