use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::{RecordLifecycleState, RetentionPassOutcome, RetentionPlan};
use crate::storage::logic::state::{
    EntityRecordKind, HistoricalMetadata, RecordKind, RelationRecordKind,
};

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
            .visibility
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
            active_snapshot_count: self.runtime.visibility.active_snapshot_count(),
            branch_pinned_entities,
            replay_pinned_entities,
            snapshot_pinned_entities,
            branch_pinned_relations,
            replay_pinned_relations,
            snapshot_pinned_relations,
            reclaimable_entities,
            reclaimable_relations,
        };
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
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

        let entity_chunk_size = self.runtime.storage_access().entity_chunk_size();
        let relation_chunk_size = self.runtime.storage_access().relation_chunk_size();
        let retention_fence = self
            .runtime
            .visibility
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
        let oldest_visibility_retained_version = self
            .runtime
            .visibility
            .retention_fence_version(published_version);
        let oldest_branch_head_version = self
            .runtime
            .history_access()
            .branch_head_versions()
            .into_iter()
            .min()
            .unwrap_or(published_version);
        let oldest_pinned_version =
            oldest_visibility_retained_version.min(oldest_branch_head_version);

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

        trim_live_history::<EntityRecordKind>(
            self.runtime,
            entity_slots,
            oldest_pinned_version,
            |runtime, trimmed| {
                runtime
                    .services
                    .instrumentation
                    .complexity_counters
                    .lock()
                    .expect("complexity counter lock poisoned")
                    .live_entity_history_entries_trimmed += trimmed;
            },
        );

        trim_live_history::<RelationRecordKind>(
            self.runtime,
            relation_slots,
            oldest_pinned_version,
            |runtime, trimmed| {
                runtime
                    .services
                    .instrumentation
                    .complexity_counters
                    .lock()
                    .expect("complexity counter lock poisoned")
                    .live_relation_history_entries_trimmed += trimmed;
            },
        );
    }
}

pub(crate) fn refresh_entity_retention_state(
    runtime: &mut RelationalRuntime,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
    retired_at: Option<crate::identity::data::VersionId>,
    retention_fence: crate::identity::data::VersionId,
) {
    runtime
        .storage_authority()
        .refresh_retention_state::<EntityRecordKind>(
            partition_id,
            slot,
            retired_at,
            retention_fence,
        );
}

pub(crate) fn refresh_relation_retention_state(
    runtime: &mut RelationalRuntime,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
    retired_at: Option<crate::identity::data::VersionId>,
    retention_fence: crate::identity::data::VersionId,
) {
    runtime
        .storage_authority()
        .refresh_retention_state::<RelationRecordKind>(
            partition_id,
            slot,
            retired_at,
            retention_fence,
        );
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
        .storage_access()
        .record_slot_count::<K>(partition_id);
    for slot in 0..len {
        let surface_before = runtime
            .storage_access()
            .record_slot_surface::<K>(partition_id, slot);
        let retired_at = surface_before.and_then(|surface| surface.retired_at);
        if retired_at.is_some() {
            refresh_retention(runtime, partition_id, slot, retired_at, retention_fence);
        }
        let Some(surface) = runtime
            .storage_access()
            .record_slot_surface::<K>(partition_id, slot)
        else {
            continue;
        };
        if surface.branch_pins > 0 {
            *branch_pinned += 1;
        }
        if surface.replay_pins > 0 {
            *replay_pinned += 1;
        }
        if surface.branch_pins > 0 && surface.replay_pins > 0 {
            *branch_replay_overlap += 1;
        }
        if surface.snapshot_pins > 0 {
            *snapshot_pinned += 1;
        }
        if surface.lifecycle == RecordLifecycleState::Reclaimable {
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
        .storage_access()
        .record_slot_count::<K>(partition_id);
    for slot_start in (0..len).step_by(chunk_size.max(1)) {
        *chunks_scanned += 1;
        let slot_end = (slot_start + chunk_size.max(1)).min(len);
        for slot in slot_start..slot_end {
            count_scan(runtime);
            let retired_at = runtime
                .storage_access()
                .record_slot_surface::<K>(partition_id, slot)
                .and_then(|surface| surface.retired_at);
            if let Some(version) = retired_at {
                refresh_retention(runtime, partition_id, slot, Some(version), retention_fence);
                if runtime
                    .storage_access()
                    .record_slot_surface::<K>(partition_id, slot)
                    .is_some_and(|surface| surface.lifecycle == RecordLifecycleState::Reclaimable)
                {
                    *reclaimable += 1;
                    if runtime.config.storage.mvcc.auto_reclaim_deleted_records
                        && *reclaimed < runtime.config.storage.mvcc.reclaim_batch_size
                    {
                        if runtime
                            .storage_authority()
                            .reclaim_record_if_reclaimable::<K>(partition_id, slot)
                        {
                            *reclaimed += 1;
                        }
                    }
                }
            }
        }
    }
}

fn trim_live_history<K: RecordKind>(
    runtime: &mut RelationalRuntime,
    slots_by_partition: std::collections::BTreeMap<
        crate::identity::data::PartitionId,
        std::collections::BTreeSet<usize>,
    >,
    oldest_pinned_version: crate::identity::data::VersionId,
    count_trimmed: impl Fn(&RelationalRuntime, usize),
) where
    K::Meta: HistoricalMetadata,
{
    let total_trimmed = runtime
        .storage_authority()
        .trim_live_history::<K>(slots_by_partition, oldest_pinned_version);
    count_trimmed(runtime, total_trimmed);
}

impl RelationalRuntime {
    pub fn retention_authority(&mut self) -> VisibilityRetentionAuthority<'_> {
        VisibilityRetentionAuthority::new(self)
    }
}
