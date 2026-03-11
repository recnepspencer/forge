use std::collections::BTreeMap;

use crate::logic::runtime::{RelationalRuntime, VisibilityResidency};
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::data::RelationalReadView;
use crate::storage::logic::state::{
    DenseSlotBitSet, SnapshotPartitionPins, SnapshotState,
};

impl RelationalRuntime {
    pub(crate) fn build_partition_pins_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> BTreeMap<crate::identity::data::PartitionId, SnapshotPartitionPins> {
        if version_id == self.current_version_id() {
            let mut pinned_partitions = BTreeMap::new();
            for (partition_id, partition) in &self.partitions {
                let mut entity_slots =
                    DenseSlotBitSet::with_capacity(partition.entity_arena.generations.len());
                for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                    entity_slots.set(slot, true);
                }
                let mut relation_slots =
                    DenseSlotBitSet::with_capacity(partition.relation_arena.generations.len());
                for slot in partition.relation_arena.live_bitset.iter_set_slots() {
                    relation_slots.set(slot, true);
                }
                if entity_slots.count_ones() > 0 || relation_slots.count_ones() > 0 {
                    pinned_partitions.insert(
                        *partition_id,
                        SnapshotPartitionPins {
                            entity_slots,
                            relation_slots,
                        },
                    );
                }
            }
            return pinned_partitions;
        }
        self.build_visibility_state(
            version_id,
            SnapshotId(0),
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        )
        .pinned_partitions
    }

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
        let entity_partitions = self.visible_entity_slots_from_state(&current_state, version_id);
        let relation_partitions = self.visible_relation_slots_from_state(&current_state, version_id);
        let mut pinned_partitions = BTreeMap::new();
        let mut pinned_entity_count = 0;
        for (partition_id, entity_slots) in entity_partitions {
            pinned_entity_count += entity_slots.count_ones();
            let pins = pinned_partitions
                .entry(partition_id)
                .or_insert_with(|| SnapshotPartitionPins {
                    entity_slots: DenseSlotBitSet::with_capacity(entity_slots.words().len() * 64),
                    relation_slots: DenseSlotBitSet::with_capacity(0),
                });
            pins.entity_slots = entity_slots;
        }
        let mut pinned_relation_count = 0;
        for (partition_id, relation_slots) in relation_partitions {
            pinned_relation_count += relation_slots.count_ones();
            let pins = pinned_partitions
                .entry(partition_id)
                .or_insert_with(|| SnapshotPartitionPins {
                    entity_slots: DenseSlotBitSet::with_capacity(0),
                    relation_slots: DenseSlotBitSet::with_capacity(relation_slots.words().len() * 64),
                });
            pins.relation_slots = relation_slots;
        }
        SnapshotState {
            handle,
            pinned_entity_count,
            pinned_relation_count,
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
            .read()
            .expect("visibility state lock poisoned")
            .get(&version_id)
            .cloned()
    }

    pub(crate) fn insert_visibility_state(&self, state: SnapshotState) {
        self.snapshots
            .visibility_states
            .write()
            .expect("visibility state lock poisoned")
            .insert(state.handle.version_id, state);
    }

    pub(crate) fn visibility_residency_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> VisibilityResidency {
        self.snapshots
            .visibility_residency
            .read()
            .expect("visibility residency lock poisoned")
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
                .count(|counters| counters.visibility_cache_snapshot_promotions += delta as usize);
        }
    }

    pub(crate) fn bump_replay_ref(
        &self,
        version_id: crate::identity::data::VersionId,
        delta: i32,
    ) {
        self.bump_visibility_ref(version_id, delta, |residency| {
            residency.replay_refs = residency.replay_refs.saturating_add_signed(delta);
        });
        if delta > 0 {
            self.instrumentation
                .count(|counters| counters.visibility_cache_replay_promotions += delta as usize);
        }
    }

    pub(crate) fn bump_visibility_ref(
        &self,
        version_id: crate::identity::data::VersionId,
        _delta: i32,
        update: impl FnOnce(&mut VisibilityResidency),
    ) {
        let mut residency = self
            .snapshots
            .visibility_residency
            .write()
            .expect("visibility residency lock poisoned");
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

    pub(crate) fn protect_branch_head_version(&self, version_id: crate::identity::data::VersionId) {
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
                .count(|counters| counters.visibility_cache_hits += 1);
            return state;
        }
        self.instrumentation
            .count(|counters| counters.visibility_cache_miss_reconstructions += 1);
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
                .count(|counters| counters.visibility_cache_hits += 1);
            return Some(state);
        }
        let recent_candidate = allow_recent_admission
            && self.config.visibility_cache_policy.enabled
            && self
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned")
                .recent_version_window
                > 0
            && !self.is_protected_visibility_version(version_id);
        if recent_candidate || self.is_protected_visibility_version(version_id) {
            return Some(self.ensure_visibility_state(version_id, recent_candidate));
        }
        self.instrumentation
            .count(|counters| counters.visibility_cache_miss_reconstructions += 1);
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
            || self
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned")
                .recent_version_window
                == 0
        {
            return;
        }
        {
            let mut residency = self
                .snapshots
                .visibility_residency
                .write()
                .expect("visibility residency lock poisoned");
            let entry = residency.entry(version_id).or_default();
            if entry.recent_resident {
                return;
            }
            entry.recent_resident = true;
        }
        {
            let mut recent_policy = self
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned");
            recent_policy.order.push_back(version_id);
            recent_policy.resident_count += 1;
        }
        self.evict_visibility_cache_if_needed();
    }

    pub(crate) fn evict_visibility_cache_if_needed(&self) {
        let window = self
            .snapshots
            .recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .recent_version_window;
        if !self.config.visibility_cache_policy.enabled || window == 0 {
            return;
        }
        loop {
            if self
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned")
                .resident_count
                <= window
            {
                break;
            }
            let scan_len = self
                .snapshots
                .recent_policy
                .lock()
                .expect("recent visibility policy lock poisoned")
                .order
                .len();
            if scan_len == 0 {
                break;
            }
            let mut evicted = false;
            for _ in 0..scan_len {
                let candidate = self
                    .snapshots
                    .recent_policy
                    .lock()
                    .expect("recent visibility policy lock poisoned")
                    .order
                    .pop_front();
                let Some(version_id) = candidate else {
                    break;
                };
                let mut residency = self
                    .snapshots
                    .visibility_residency
                    .write()
                    .expect("visibility residency lock poisoned");
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
                        .lock()
                        .expect("recent visibility policy lock poisoned")
                        .order
                        .push_back(version_id);
                    continue;
                }
                entry.recent_resident = false;
                self.snapshots
                    .recent_policy
                    .lock()
                    .expect("recent visibility policy lock poisoned")
                    .resident_count -= 1;
                if entry.branch_head_refs == 0
                    && entry.replay_refs == 0
                    && entry.active_snapshot_refs == 0
                {
                    residency.remove(&version_id);
                }
                drop(residency);
                self.snapshots
                    .visibility_states
                    .write()
                    .expect("visibility state lock poisoned")
                    .remove(&version_id);
                self.instrumentation
                    .count(|counters| counters.visibility_cache_recent_evictions += 1);
                evicted = true;
                break;
            }
            if !evicted {
                break;
            }
        }
    }

    pub(crate) fn maybe_remove_unprotected_visibility_state(
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
                .write()
                .expect("visibility state lock poisoned")
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
        self.instrumentation.count(|counters| {
            counters.visible_entity_records_materialized += entities.len();
            counters.visible_relation_records_materialized += relations.len();
        });
        RelationalReadView {
            snapshot: state.handle.clone(),
            entities,
            relations,
        }
    }
}
