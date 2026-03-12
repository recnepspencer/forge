use crate::snapshots::data::{SnapshotId, SnapshotReadPolicy};
use crate::storage::data::RelationalReadView;
use crate::storage::overlay::SnapshotState;

use crate::logic::runtime::{RelationalRuntime, VisibilityResidency};

impl RelationalRuntime {
    pub(crate) fn visibility_state_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<SnapshotState> {
        self.visibility.cache.state_for_version(version_id)
    }

    pub(crate) fn insert_visibility_state(&self, state: SnapshotState) {
        self.visibility.cache.insert_state(state);
    }

    pub(crate) fn visibility_residency_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> VisibilityResidency {
        self.visibility.cache.residency_for_version(version_id)
    }

    pub(crate) fn bump_active_snapshot_ref(
        &self,
        version_id: crate::identity::data::VersionId,
        delta: i32,
    ) {
        self.bump_visibility_ref(version_id, |residency| {
            residency.active_snapshot_refs =
                residency.active_snapshot_refs.saturating_add_signed(delta);
        });
        if delta > 0 {
            self.services.instrumentation
                .count(|counters| counters.visibility_cache_snapshot_promotions += delta as usize);
        }
    }

    pub(crate) fn bump_replay_ref(
        &self,
        version_id: crate::identity::data::VersionId,
        delta: i32,
    ) {
        self.bump_visibility_ref(version_id, |residency| {
            residency.replay_refs = residency.replay_refs.saturating_add_signed(delta);
        });
        if delta > 0 {
            self.services.instrumentation
                .count(|counters| counters.visibility_cache_replay_promotions += delta as usize);
        }
    }

    pub(crate) fn bump_visibility_ref(
        &self,
        version_id: crate::identity::data::VersionId,
        update: impl FnOnce(&mut VisibilityResidency),
    ) {
        self.visibility.cache.update_residency(version_id, update);
        self.maybe_remove_unprotected_visibility_state(version_id);
    }

    pub(crate) fn protect_branch_head_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) {
        self.bump_visibility_ref(version_id, |residency| {
            residency.branch_head_refs += 1;
        });
    }

    pub(crate) fn ensure_visibility_state(
        &self,
        version_id: crate::identity::data::VersionId,
        recent_candidate: bool,
    ) -> SnapshotState {
        if let Some(state) = self.visibility_state_for_version(version_id) {
            self.services.instrumentation
                .count(|counters| counters.visibility_cache_hits += 1);
            return state;
        }
        self.services.instrumentation
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
            self.services.instrumentation
                .count(|counters| counters.visibility_cache_hits += 1);
            return Some(state);
        }
        let recent_candidate = allow_recent_admission
            && self.config.visibility.cache_policy.enabled
            && self.visibility.cache.recent_window() > 0
            && !self.is_protected_visibility_version(version_id);
        if recent_candidate || self.is_protected_visibility_version(version_id) {
            return Some(self.ensure_visibility_state(version_id, recent_candidate));
        }
        self.services.instrumentation
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
        if !self.config.visibility.cache_policy.enabled
            || self.visibility.cache.recent_window() == 0
        {
            return;
        }
        if !self.visibility.cache.mark_recent_resident(version_id) {
            return;
        }
        self.evict_visibility_cache_if_needed();
    }

    pub(crate) fn evict_visibility_cache_if_needed(&self) {
        let window = self.visibility.cache.recent_window();
        if !self.config.visibility.cache_policy.enabled || window == 0 {
            return;
        }
        loop {
            if self.visibility.cache.resident_recent_count() <= window {
                break;
            }
            let scan_len = self.visibility.cache.recent_candidate_count();
            if scan_len == 0 {
                break;
            }
            let mut evicted = false;
            for _ in 0..scan_len {
                let candidate = self.visibility.cache.pop_oldest_recent_candidate();
                let Some(version_id) = candidate else {
                    break;
                };
                if self.is_protected_visibility_version(version_id) {
                    self.visibility.cache.enqueue_recent_candidate(version_id);
                    continue;
                }
                if !self.visibility.cache.evict_recent_resident_if_unprotected(version_id) {
                    continue;
                }
                self.visibility.cache.remove_state(version_id);
                self.services.instrumentation
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
            self.visibility.cache.remove_state(version_id);
        }
    }

    pub(crate) fn read_from_snapshot_state(&self, state: &SnapshotState) -> RelationalReadView {
        let current_state = self.current_state();
        let reader = self.visibility_reads();
        let mut entities = Vec::with_capacity(state.pinned_entity_count);
        let mut relations = Vec::with_capacity(state.pinned_relation_count);
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                let entity_id = crate::identity::data::EntityId::new(*partition_id, slot as u64, 0);
                if let Some(record) = reader.entity_record_for_id_at_version(
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
                if let Some(record) = reader.relation_record_for_id_at_version(
                    &current_state,
                    relation_id,
                    state.handle.version_id,
                ) {
                    relations.push(record);
                }
            }
        }
        self.services.instrumentation.count(|counters| {
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
