use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::{RecordLifecycleState, RetentionPassOutcome, RetentionPlan};
use crate::storage::logic::state::{EntityRecordKind, HistoricalMetadata, RecordKind, RelationRecordKind};

pub struct VisibilityRetentionAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> VisibilityRetentionAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn inspect_plan(&mut self) -> RetentionPlan {
        let retention_fence = self
            .runtime
            .retention_fence_version(self.runtime.current_version_id());
        let mut branch_pinned_entities = 0;
        let mut replay_pinned_entities = 0;
        let mut snapshot_pinned_entities = 0;
        let mut branch_pinned_relations = 0;
        let mut replay_pinned_relations = 0;
        let mut snapshot_pinned_relations = 0;
        let mut reclaimable_entities = 0;
        let mut reclaimable_relations = 0;
        let mut branch_replay_overlap_entities = 0;
        let mut branch_replay_overlap_relations = 0;

        let partition_ids = self.runtime.storage_access().partition_ids();
        for partition_id in partition_ids {
            inspect_partition_retention::<EntityRecordKind>(
                self.runtime,
                partition_id,
                retention_fence,
                &mut branch_pinned_entities,
                &mut replay_pinned_entities,
                &mut snapshot_pinned_entities,
                &mut reclaimable_entities,
                &mut branch_replay_overlap_entities,
                refresh_entity_retention_state,
            );
            inspect_partition_retention::<RelationRecordKind>(
                self.runtime,
                partition_id,
                retention_fence,
                &mut branch_pinned_relations,
                &mut replay_pinned_relations,
                &mut snapshot_pinned_relations,
                &mut reclaimable_relations,
                &mut branch_replay_overlap_relations,
                refresh_relation_retention_state,
            );
        }

        let plan = RetentionPlan {
            retention_fence_version: retention_fence,
            active_snapshot_count: self.runtime.active_snapshot_count(),
            branch_pinned_entities,
            replay_pinned_entities,
            snapshot_pinned_entities,
            branch_pinned_relations,
            replay_pinned_relations,
            snapshot_pinned_relations,
            reclaimable_entities,
            reclaimable_relations,
        };
        self.runtime.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::Retention,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::RetentionPlanInspected,
                message: "retention plan inspected with raw pin surface counts".to_string(),
                fields: json!({
                    "retention_fence_version": plan.retention_fence_version.0,
                    "active_snapshot_count": plan.active_snapshot_count,
                    "branch_pinned_entities": plan.branch_pinned_entities,
                    "replay_pinned_entities": plan.replay_pinned_entities,
                    "snapshot_pinned_entities": plan.snapshot_pinned_entities,
                    "branch_pinned_relations": plan.branch_pinned_relations,
                    "replay_pinned_relations": plan.replay_pinned_relations,
                    "snapshot_pinned_relations": plan.snapshot_pinned_relations,
                    "branch_replay_overlap_entities": branch_replay_overlap_entities,
                    "branch_replay_overlap_relations": branch_replay_overlap_relations,
                    "reclaimable_entities": plan.reclaimable_entities,
                    "reclaimable_relations": plan.reclaimable_relations,
                }),
            }],
        );
        plan
    }

    pub fn run_pass(&mut self) -> RetentionPassOutcome {
        let mut outcome = RetentionPassOutcome {
            entity_reclaimable: 0,
            entity_reclaimed: 0,
            entity_chunks_scanned: 0,
            relation_reclaimable: 0,
            relation_reclaimed: 0,
            relation_chunks_scanned: 0,
        };

        let entity_chunk_size = self.runtime.entity_chunk_size();
        let relation_chunk_size = self.runtime.relation_chunk_size();
        let retention_fence = self
            .runtime
            .retention_fence_version(self.runtime.current_version_id());

        let partition_ids = self.runtime.storage_access().partition_ids();
        for partition_id in partition_ids {
            run_partition_retention_pass::<EntityRecordKind>(
                self.runtime,
                partition_id,
                entity_chunk_size,
                retention_fence,
                &mut outcome.entity_chunks_scanned,
                &mut outcome.entity_reclaimable,
                &mut outcome.entity_reclaimed,
                |runtime| {
                    let mut counters = runtime
                        .services
                        .instrumentation
                        .complexity_counters
                        .lock()
                        .expect("complexity counter lock poisoned");
                    counters.retention_entity_slots_scanned += 1;
                },
                refresh_entity_retention_state,
            );
            run_partition_retention_pass::<RelationRecordKind>(
                self.runtime,
                partition_id,
                relation_chunk_size,
                retention_fence,
                &mut outcome.relation_chunks_scanned,
                &mut outcome.relation_reclaimable,
                &mut outcome.relation_reclaimed,
                |runtime| {
                    let mut counters = runtime
                        .services
                        .instrumentation
                        .complexity_counters
                        .lock()
                        .expect("complexity counter lock poisoned");
                    counters.retention_relation_slots_scanned += 1;
                },
                refresh_relation_retention_state,
            );
        }

        outcome
    }

    pub(crate) fn trim_live_history_for_records(
        &mut self,
        changed_records: &[crate::transactions::data::RecordRef],
        published_version: crate::identity::data::VersionId,
    ) {
        let oldest_pinned_version = self.runtime.retention_fence_version(published_version);

        let mut entity_slots = std::collections::BTreeMap::new();
        let mut relation_slots = std::collections::BTreeMap::new();
        for record in changed_records {
            match record {
                crate::transactions::data::RecordRef::Entity(entity_id) => {
                    entity_slots
                        .entry(entity_id.partition_id)
                        .or_insert_with(std::collections::BTreeSet::new)
                        .insert(entity_id.local_slot.0 as usize);
                }
                crate::transactions::data::RecordRef::Relation(relation_id) => {
                    relation_slots
                        .entry(relation_id.partition_id)
                        .or_insert_with(std::collections::BTreeSet::new)
                        .insert(relation_id.local_slot.0 as usize);
                }
            }
        }

        trim_live_history::<EntityRecordKind>(self.runtime, entity_slots, oldest_pinned_version, |runtime, trimmed| {
            runtime
                .services
                .instrumentation
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned")
                .live_entity_history_entries_trimmed += trimmed;
        });

        trim_live_history::<RelationRecordKind>(self.runtime, relation_slots, oldest_pinned_version, |runtime, trimmed| {
            runtime
                .services
                .instrumentation
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned")
                .live_relation_history_entries_trimmed += trimmed;
        });
    }
}

pub(crate) fn refresh_entity_retention_state(
    runtime: &mut RelationalRuntime,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
    retired_at: Option<crate::identity::data::VersionId>,
    retention_fence: crate::identity::data::VersionId,
) {
    refresh_retention_state::<EntityRecordKind>(runtime, partition_id, slot, retired_at, retention_fence);
}

pub(crate) fn refresh_relation_retention_state(
    runtime: &mut RelationalRuntime,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
    retired_at: Option<crate::identity::data::VersionId>,
    retention_fence: crate::identity::data::VersionId,
) {
    refresh_retention_state::<RelationRecordKind>(runtime, partition_id, slot, retired_at, retention_fence);
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
    let lifecycle = match runtime.config.storage.retention.backend {
        crate::config::data::RetentionBackend::PinTrackedRetention => {
            if arena.snapshot_pin_count(slot).unwrap_or(0) > 0 {
                RecordLifecycleState::PinnedBySnapshot
            } else if arena.branch_pin_count(slot).unwrap_or(0) > 0 {
                RecordLifecycleState::PinnedByBranch
            } else if arena.replay_pin_count(slot).unwrap_or(0) > 0 {
                RecordLifecycleState::PinnedByReplayRetention
            } else {
                RecordLifecycleState::Reclaimable
            }
        }
        crate::config::data::RetentionBackend::EpochChunkRetention => {
            if arena.branch_pin_count(slot).unwrap_or(0) > 0 {
                RecordLifecycleState::PinnedByBranch
            } else if arena.replay_pin_count(slot).unwrap_or(0) > 0 {
                RecordLifecycleState::PinnedByReplayRetention
            } else if retired_at.is_some_and(|retired| {
                !crate::identity::data::VersionBound::new(retention_fence).retains_retired(retired)
            }) {
                RecordLifecycleState::Reclaimable
            } else {
                RecordLifecycleState::PinnedBySnapshot
            }
        }
    };
    arena.set_lifecycle_for_slot(slot, lifecycle);
}

fn inspect_partition_retention<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    partition_id: crate::identity::data::PartitionId,
    retention_fence: crate::identity::data::VersionId,
    branch_pinned: &mut usize,
    replay_pinned: &mut usize,
    snapshot_pinned: &mut usize,
    reclaimable: &mut usize,
    branch_replay_overlap: &mut usize,
    refresh_retention: fn(
        &mut RelationalRuntime,
        crate::identity::data::PartitionId,
        usize,
        Option<crate::identity::data::VersionId>,
        crate::identity::data::VersionId,
    ),
) {
    let len = runtime
        .partitions
        .get(&partition_id)
        .map(|partition| K::arena(partition).slot_count())
        .unwrap_or(0);
    for slot in 0..len {
        let retired_at = runtime
            .partitions
            .get(&partition_id)
            .and_then(|partition| K::arena(partition).retired_at_for_slot(slot));
        if retired_at.is_some() {
            refresh_retention(runtime, partition_id, slot, retired_at, retention_fence);
        }
        let partition = &runtime.partitions[&partition_id];
        let arena = K::arena(partition);
        if arena.branch_pin_count(slot).unwrap_or(0) > 0 {
            *branch_pinned += 1;
        }
        if arena.replay_pin_count(slot).unwrap_or(0) > 0 {
            *replay_pinned += 1;
        }
        if arena.branch_pin_count(slot).unwrap_or(0) > 0
            && arena.replay_pin_count(slot).unwrap_or(0) > 0
        {
            *branch_replay_overlap += 1;
        }
        if arena.snapshot_pin_count(slot).unwrap_or(0) > 0 {
            *snapshot_pinned += 1;
        }
        if arena
            .get_slot(slot)
            .is_some_and(|slot_view| slot_view.lifecycle() == RecordLifecycleState::Reclaimable)
        {
            *reclaimable += 1;
        }
    }
}

fn run_partition_retention_pass<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    partition_id: crate::identity::data::PartitionId,
    chunk_size: usize,
    retention_fence: crate::identity::data::VersionId,
    chunks_scanned: &mut usize,
    reclaimable: &mut usize,
    reclaimed: &mut usize,
    count_scan: impl Fn(&RelationalRuntime),
    refresh_retention: fn(
        &mut RelationalRuntime,
        crate::identity::data::PartitionId,
        usize,
        Option<crate::identity::data::VersionId>,
        crate::identity::data::VersionId,
    ),
) {
    let len = runtime
        .partitions
        .get(&partition_id)
        .map(|partition| K::arena(partition).slot_count())
        .unwrap_or(0);
    for slot_start in (0..len).step_by(chunk_size.max(1)) {
        *chunks_scanned += 1;
        let slot_end = (slot_start + chunk_size.max(1)).min(len);
        for slot in slot_start..slot_end {
            count_scan(runtime);
            let retired_at = runtime
                .partitions
                .get(&partition_id)
                .and_then(|partition| K::arena(partition).retired_at_for_slot(slot));
            if let Some(version) = retired_at {
                refresh_retention(runtime, partition_id, slot, Some(version), retention_fence);
                if runtime.partitions.get(&partition_id).is_some_and(|partition| {
                    K::arena(partition)
                        .get_slot(slot)
                        .is_some_and(|slot_view| {
                            slot_view.lifecycle() == RecordLifecycleState::Reclaimable
                        })
                }) {
                    *reclaimable += 1;
                    if runtime.config.storage.mvcc.auto_reclaim_deleted_records
                        && *reclaimed < runtime.config.storage.mvcc.reclaim_batch_size
                    {
                        let partition = runtime
                            .partitions
                            .get_mut(&partition_id)
                            .expect("partition for reclaim");
                        let arena = K::arena_mut(partition);
                        arena.set_lifecycle_for_slot(slot, RecordLifecycleState::Reusable);
                        arena.reset_slot(slot);
                        *reclaimed += 1;
                    }
                }
            }
        }
    }
}

fn trim_live_history<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    slots_by_partition: std::collections::BTreeMap<crate::identity::data::PartitionId, std::collections::BTreeSet<usize>>,
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

impl RelationalRuntime {
    pub fn retention_access(&mut self) -> VisibilityRetentionAuthority<'_> {
        VisibilityRetentionAuthority::new(self)
    }
}
