use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::logic::runtime::RelationalRuntime;
use crate::storage::logic::state::{
    EntityRecordKind, RecordKind, RelationRecordKind,
};
use crate::storage::data::{RecordLifecycleState, RetentionPassOutcome, RetentionPlan};

impl RelationalRuntime {
    pub fn inspect_retention_plan(&mut self) -> RetentionPlan {
        let retention_fence = self.retention_fence_version(self.current_version_id());
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

        let partition_ids = self.partitions.keys().copied().collect::<Vec<_>>();
        for partition_id in partition_ids {
            inspect_partition_retention::<EntityRecordKind>(
                self,
                partition_id,
                retention_fence,
                &mut branch_pinned_entities,
                &mut replay_pinned_entities,
                &mut snapshot_pinned_entities,
                &mut reclaimable_entities,
                &mut branch_replay_overlap_entities,
                RelationalRuntime::refresh_entity_retention_state,
            );
            inspect_partition_retention::<RelationRecordKind>(
                self,
                partition_id,
                retention_fence,
                &mut branch_pinned_relations,
                &mut replay_pinned_relations,
                &mut snapshot_pinned_relations,
                &mut reclaimable_relations,
                &mut branch_replay_overlap_relations,
                RelationalRuntime::refresh_relation_retention_state,
            );
        }

        let plan = RetentionPlan {
            retention_fence_version: retention_fence,
            active_snapshot_count: self.snapshots.active.len(),
            branch_pinned_entities,
            replay_pinned_entities,
            snapshot_pinned_entities,
            branch_pinned_relations,
            replay_pinned_relations,
            snapshot_pinned_relations,
            reclaimable_entities,
            reclaimable_relations,
        };
        self.push_bounded_diagnostic(
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

    pub fn run_retention_pass(&mut self) -> RetentionPassOutcome {
        let mut outcome = RetentionPassOutcome {
            entity_reclaimable: 0,
            entity_reclaimed: 0,
            entity_chunks_scanned: 0,
            relation_reclaimable: 0,
            relation_reclaimed: 0,
            relation_chunks_scanned: 0,
        };

        let entity_chunk_size = self.config.storage_layout.entity_chunk_size.max(1);
        let relation_chunk_size = self.config.storage_layout.relation_chunk_size.max(1);
        let retention_fence = self.retention_fence_version(self.current_version_id());

        let partition_ids = self.partitions.keys().copied().collect::<Vec<_>>();
        for partition_id in partition_ids {
            run_partition_retention_pass::<EntityRecordKind>(
                self,
                partition_id,
                entity_chunk_size,
                retention_fence,
                &mut outcome.entity_chunks_scanned,
                &mut outcome.entity_reclaimable,
                &mut outcome.entity_reclaimed,
                |runtime| {
                    let mut counters = runtime
                        .instrumentation
                        .complexity_counters
                        .lock()
                        .expect("complexity counter lock poisoned");
                    counters.retention_entity_slots_scanned += 1;
                },
                RelationalRuntime::refresh_entity_retention_state,
            );
            run_partition_retention_pass::<RelationRecordKind>(
                self,
                partition_id,
                relation_chunk_size,
                retention_fence,
                &mut outcome.relation_chunks_scanned,
                &mut outcome.relation_reclaimable,
                &mut outcome.relation_reclaimed,
                |runtime| {
                    let mut counters = runtime
                        .instrumentation
                        .complexity_counters
                        .lock()
                        .expect("complexity counter lock poisoned");
                    counters.retention_relation_slots_scanned += 1;
                },
                RelationalRuntime::refresh_relation_retention_state,
            );
        }

        outcome
    }
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
        .map(|partition| K::arena(partition).lifecycle.len())
        .unwrap_or(0);
    for slot in 0..len {
        let retired_at = runtime
            .partitions
            .get(&partition_id)
            .and_then(|partition| K::arena(partition).retired_at.get(slot).copied())
            .flatten();
        if retired_at.is_some() {
            refresh_retention(runtime, partition_id, slot, retired_at, retention_fence);
        }
        let partition = &runtime.partitions[&partition_id];
        let arena = K::arena(partition);
        if arena.branch_pins[slot] > 0 {
            *branch_pinned += 1;
        }
        if arena.replay_pins[slot] > 0 {
            *replay_pinned += 1;
        }
        if arena.branch_pins[slot] > 0 && arena.replay_pins[slot] > 0 {
            *branch_replay_overlap += 1;
        }
        if arena.snapshot_pins[slot] > 0 {
            *snapshot_pinned += 1;
        }
        if arena.lifecycle[slot] == RecordLifecycleState::Reclaimable {
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
        .map(|partition| K::arena(partition).lifecycle.len())
        .unwrap_or(0);
    for slot_start in (0..len).step_by(chunk_size.max(1)) {
        *chunks_scanned += 1;
        let slot_end = (slot_start + chunk_size.max(1)).min(len);
        for slot in slot_start..slot_end {
            count_scan(runtime);
            let retired_at = runtime
                .partitions
                .get(&partition_id)
                .and_then(|partition| K::arena(partition).retired_at.get(slot).copied())
                .flatten();
            if let Some(version) = retired_at {
                refresh_retention(runtime, partition_id, slot, Some(version), retention_fence);
                if runtime.partitions.get(&partition_id).is_some_and(|partition| {
                    K::arena(partition).lifecycle[slot] == RecordLifecycleState::Reclaimable
                }) {
                    *reclaimable += 1;
                    if runtime.config.mvcc.auto_reclaim_deleted_records
                        && *reclaimed < runtime.config.mvcc.reclaim_batch_size
                    {
                        let partition = runtime
                            .partitions
                            .get_mut(&partition_id)
                            .expect("partition for reclaim");
                        let arena = K::arena_mut(partition);
                        arena.lifecycle[slot] = RecordLifecycleState::Reusable;
                        K::reset_reclaimed_slot(arena, slot);
                        *reclaimed += 1;
                    }
                }
            }
        }
    }
}
