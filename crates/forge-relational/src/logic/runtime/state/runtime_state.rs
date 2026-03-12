use std::collections::BTreeMap;

use crate::snapshots::data::{SnapshotHandle, SnapshotReadPolicy};
use crate::storage::overlay::{
    BorrowedWorkingState, OverlayStateView, PartitionState, SnapshotState, WorkingState,
};

use super::{
    DurabilitySubsystem, HistorySubsystem, IndexingSubsystem, LineageSubsystem,
    PublicationSubsystem, RuntimeServices, VisibilitySubsystem,
};
use crate::logic::runtime::{RelationalRuntimeConfig, RuntimeComplexityCounters};

#[derive(Debug)]
pub struct RelationalRuntime {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) partitions: BTreeMap<crate::identity::data::PartitionId, PartitionState>,
    pub(crate) visibility: VisibilitySubsystem,
    pub(crate) publication: PublicationSubsystem,
    pub(crate) history: HistorySubsystem,
    pub(crate) indexes: IndexingSubsystem,
    pub(crate) lineage: LineageSubsystem,
    pub(crate) durability: DurabilitySubsystem,
    pub(crate) services: RuntimeServices,
}

impl RelationalRuntime {
    pub(crate) fn active_snapshot_count(&self) -> usize {
        self.visibility.active_snapshot_count()
    }

    pub(crate) fn branch_head_versions(&self) -> Vec<crate::identity::data::VersionId> {
        self.history
            .branch_heads
            .values()
            .filter_map(|head| head.as_ref().map(|head| head.version_id))
            .collect()
    }

    pub(crate) fn durable_store_layout(
        &self,
    ) -> Option<crate::durability::data::DurableStoreLayout> {
        self.config.durability.policy.store_layout.clone()
    }

    pub(crate) fn set_durable_store(
        &mut self,
        store: Option<crate::durability::data::DurableStore>,
    ) {
        self.durability.store = store;
    }

    pub(crate) fn latest_durable_checkpoint(
        &self,
    ) -> Option<&crate::durability::data::DurableCheckpoint> {
        self.durability.checkpoints.last()
    }

    pub(crate) fn durable_log_len(&self) -> usize {
        self.durability.log.len()
    }

    pub(crate) fn push_durable_log_entry(
        &mut self,
        envelope: crate::replay::data::CanonicalCommitEnvelope,
    ) {
        self.durability.log.push(envelope);
    }

    pub(crate) fn last_durable_log_commit_id(
        &self,
    ) -> Option<crate::history::data::CommitId> {
        self.durability.log.last().map(|entry| entry.commit.commit_id)
    }

    pub(crate) fn retain_durable_log_newer_than(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) {
        self.durability
            .log
            .retain(|entry| entry.commit.commit_id > commit_id);
    }

    pub(crate) fn drain_oldest_durable_log_entries(&mut self, count: usize) {
        self.durability.log.drain(0..count);
    }

    pub(crate) fn push_durable_checkpoint(
        &mut self,
        checkpoint: crate::durability::data::DurableCheckpoint,
    ) {
        self.durability.checkpoints.push(checkpoint);
    }

    pub(crate) fn commit_envelopes_snapshot(
        &self,
    ) -> Vec<crate::replay::data::CanonicalCommitEnvelope> {
        self.history.commit_envelopes.values().cloned().collect()
    }

    pub(crate) fn symbol_table_snapshot(&self) -> crate::symbols::data::SymbolTableSnapshot {
        self.services.symbols.snapshot()
    }

    pub(crate) fn resolve_symbol_name(
        &self,
        symbol: crate::symbols::data::Symbol,
    ) -> Option<&str> {
        self.services.symbols.resolve(symbol)
    }

    pub fn config(&self) -> &RelationalRuntimeConfig {
        &self.config
    }

    pub(crate) fn partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
    ) -> Option<&PartitionState> {
        self.partitions.get(&partition_id)
    }

    pub(crate) fn entity_slot_count(&self) -> usize {
        self.partitions
            .values()
            .map(|partition| partition.entity_arena.slot_count())
            .sum()
    }

    pub(crate) fn relation_slot_count(&self) -> usize {
        self.partitions
            .values()
            .map(|partition| partition.relation_arena.slot_count())
            .sum()
    }

    pub(crate) fn entity_chunk_size(&self) -> usize {
        self.config.storage.layout.entity_chunk_size.max(1)
    }

    pub(crate) fn relation_chunk_size(&self) -> usize {
        self.config.storage.layout.relation_chunk_size.max(1)
    }

    pub fn complexity_counters(&self) -> RuntimeComplexityCounters {
        self.services
            .instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned")
            .clone()
    }

    pub fn reset_complexity_counters(&self) {
        *self
            .services
            .instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned") = RuntimeComplexityCounters::default();
    }

    pub(crate) fn primary_schema_version(&self) -> crate::schema::data::SchemaVersionId {
        self.config
            .schema
            .registry
            .entity_kinds
            .values()
            .next()
            .map(|registration| registration.schema_version_id)
            .or_else(|| {
                self.config
                    .schema
                    .registry
                    .relation_kinds
                    .values()
                    .next()
                    .map(|registration| registration.schema_version_id)
            })
            .unwrap_or(crate::schema::data::SchemaVersionId(0))
    }

    pub(crate) fn current_state(&self) -> BorrowedWorkingState<'_> {
        BorrowedWorkingState::new(&self.partitions)
    }

    pub(crate) fn working_state_for_touched_partitions(
        &self,
        touched_partitions: impl IntoIterator<Item = crate::identity::data::PartitionId>,
    ) -> WorkingState {
        WorkingState::from_touched_partitions(
            &self.partitions,
            touched_partitions,
            self.config.storage.adjacency_policy.clone(),
        )
    }

    pub(crate) fn overlay_state_view<'a>(
        &'a self,
        staged: &'a WorkingState,
    ) -> OverlayStateView<'a, WorkingState> {
        OverlayStateView::new(&self.partitions, staged)
    }

    pub(crate) fn mutation_config(&self) -> crate::config::data::MutationConfig {
        crate::config::data::MutationConfig {
            patch_surface_policy: self.config.publication.policy.patch_surface_policy,
            cascade_delete_policy: self.config.storage.cascade_delete_policy,
            adjacency_policy: self.config.storage.adjacency_policy.clone(),
            cross_context_policy: self.config.storage.cross_context_policy,
        }
    }

    pub(crate) fn retention_fence_version(
        &self,
        published_version: crate::identity::data::VersionId,
    ) -> crate::identity::data::VersionId {
        self.visibility
            .active_versions()
            .chain(self.visibility.replay_retention.versions())
            .min()
            .unwrap_or(published_version)
    }

    pub(crate) fn snapshot_state_for_current(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> (SnapshotHandle, SnapshotState) {
        let snapshot_id = self.visibility.allocate_snapshot_id();
        let state = self.build_visibility_state(
            version_id,
            snapshot_id,
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        );
        self.visibility_pins().pin_snapshot_state(&state);
        (state.handle.clone(), state)
    }

    #[cfg(test)]
    pub(crate) fn entity_history_len_for_test(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> usize {
        self.partition(entity_id.partition_id)
            .and_then(|partition| {
                partition
                    .entity_arena
                    .payload_history_at(entity_id.local_slot.0 as usize)
            })
            .map(|history| history.len())
            .unwrap_or(0)
    }

    #[cfg(test)]
    pub(crate) fn relation_history_len_for_test(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> usize {
        self.partition(relation_id.partition_id)
            .and_then(|partition| {
                partition
                    .relation_arena
                    .payload_history_at(relation_id.local_slot.0 as usize)
            })
            .map(|history| history.len())
            .unwrap_or(0)
    }
}
