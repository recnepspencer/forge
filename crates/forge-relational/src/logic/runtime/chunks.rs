use crate::data::identity::VersionId;
use crate::data::query::{QueryWorkPacket, ReadPacketPlan, ReadTarget};
use crate::logic::runtime::{
    ChunkDiagnostics, ChunkVisibilitySummary, ChunkedStorageSummary, RecordLifecycleState,
    RelationalRuntime,
};

impl RelationalRuntime {
    pub fn chunked_storage_summary(&self, version_id: VersionId) -> ChunkedStorageSummary {
        ChunkedStorageSummary {
            entity_chunks: summarize_entity_chunks(self, version_id),
            relation_chunks: summarize_relation_chunks(self, version_id),
        }
    }

    pub fn chunk_diagnostics(&self, version_id: VersionId) -> ChunkDiagnostics {
        let summary = self.chunked_storage_summary(version_id);
        ChunkDiagnostics {
            version_id,
            entity_chunks_total: summary.entity_chunks.len(),
            entity_chunks_with_visible_records: summary
                .entity_chunks
                .iter()
                .filter(|chunk| chunk.visible_records > 0)
                .count(),
            entity_chunks_with_retained_records: summary
                .entity_chunks
                .iter()
                .filter(|chunk| chunk.retained_records > 0)
                .count(),
            relation_chunks_total: summary.relation_chunks.len(),
            relation_chunks_with_visible_records: summary
                .relation_chunks
                .iter()
                .filter(|chunk| chunk.visible_records > 0)
                .count(),
            relation_chunks_with_retained_records: summary
                .relation_chunks
                .iter()
                .filter(|chunk| chunk.retained_records > 0)
                .count(),
        }
    }

    pub fn plan_read_packet(
        &self,
        handle: &crate::data::snapshot::SnapshotHandle,
        packet: &QueryWorkPacket,
    ) -> Option<ReadPacketPlan> {
        self.snapshots.get(&handle.snapshot_id)?;
        let mut entity_chunk_indexes = Vec::new();
        let mut relation_chunk_indexes = Vec::new();

        for target in &packet.targets {
            match target {
                ReadTarget::Entity(entity_id) => {
                    let chunk_index = slot_chunk_index(
                        entity_id.local_slot.0 as usize,
                        self.config.storage_layout.entity_chunk_size,
                    );
                    if !entity_chunk_indexes.contains(&chunk_index) {
                        entity_chunk_indexes.push(chunk_index);
                    }
                }
                ReadTarget::Relation(relation_id) => {
                    let chunk_index = slot_chunk_index(
                        relation_id.local_slot.0 as usize,
                        self.config.storage_layout.relation_chunk_size,
                    );
                    if !relation_chunk_indexes.contains(&chunk_index) {
                        relation_chunk_indexes.push(chunk_index);
                    }
                }
            }
        }

        Some(ReadPacketPlan {
            label: packet.label.clone(),
            entity_chunk_indexes,
            relation_chunk_indexes,
            target_count: packet.targets.len(),
        })
    }
}

fn summarize_entity_chunks(
    runtime: &RelationalRuntime,
    version_id: VersionId,
) -> Vec<ChunkVisibilitySummary> {
    summarize_chunks(
        runtime.entity_arena.generations.len(),
        runtime.config.storage_layout.entity_chunk_size,
        |slot| runtime.entity_arena.created_at.get(slot).copied(),
        |slot| runtime.entity_arena.retired_at.get(slot).copied().flatten(),
        |slot| runtime.entity_arena.lifecycle.get(slot).copied(),
        |slot| {
            runtime
                .entity_arena
                .created_at
                .get(slot)
                .is_some_and(|created| {
                    *created <= version_id
                        && runtime.entity_arena.retired_at[slot]
                            .is_none_or(|retired| version_id < retired)
                        && runtime.entity_arena.lifecycle[slot] != RecordLifecycleState::Reusable
                })
        },
    )
}

fn summarize_relation_chunks(
    runtime: &RelationalRuntime,
    version_id: VersionId,
) -> Vec<ChunkVisibilitySummary> {
    summarize_chunks(
        runtime.relation_arena.generations.len(),
        runtime.config.storage_layout.relation_chunk_size,
        |slot| runtime.relation_arena.created_at.get(slot).copied(),
        |slot| {
            runtime
                .relation_arena
                .retired_at
                .get(slot)
                .copied()
                .flatten()
        },
        |slot| runtime.relation_arena.lifecycle.get(slot).copied(),
        |slot| {
            runtime
                .relation_arena
                .created_at
                .get(slot)
                .is_some_and(|created| {
                    *created <= version_id
                        && runtime.relation_arena.retired_at[slot]
                            .is_none_or(|retired| version_id < retired)
                        && runtime.relation_arena.lifecycle[slot] != RecordLifecycleState::Reusable
                })
        },
    )
}

fn summarize_chunks<FCreated, FRetired, FLifecycle, FVisible>(
    slot_count: usize,
    chunk_size: usize,
    created_at: FCreated,
    retired_at: FRetired,
    lifecycle: FLifecycle,
    visible_at_version: FVisible,
) -> Vec<ChunkVisibilitySummary>
where
    FCreated: Fn(usize) -> Option<VersionId>,
    FRetired: Fn(usize) -> Option<VersionId>,
    FLifecycle: Fn(usize) -> Option<RecordLifecycleState>,
    FVisible: Fn(usize) -> bool,
{
    if chunk_size == 0 || slot_count == 0 {
        return Vec::new();
    }

    let mut summaries = Vec::new();
    let mut chunk_index = 0;
    let mut slot_start = 0;

    while slot_start < slot_count {
        let slot_end = (slot_start + chunk_size).min(slot_count);
        let mut visible_records = 0;
        let mut retained_records = 0;
        let mut reclaimable_records = 0;
        let mut earliest_created_at: Option<VersionId> = None;
        let mut latest_retired_at: Option<VersionId> = None;

        for slot in slot_start..slot_end {
            if visible_at_version(slot) {
                visible_records += 1;
            }

            if let Some(created) = created_at(slot) {
                earliest_created_at = Some(match earliest_created_at {
                    Some(current) => current.min(created),
                    None => created,
                });
            }

            if let Some(retired) = retired_at(slot) {
                latest_retired_at = Some(match latest_retired_at {
                    Some(current) => current.max(retired),
                    None => retired,
                });
            }

            match lifecycle(slot) {
                Some(
                    RecordLifecycleState::DeletedRetained
                    | RecordLifecycleState::PinnedBySnapshot
                    | RecordLifecycleState::PinnedByBranch
                    | RecordLifecycleState::PinnedByReplayRetention,
                ) => retained_records += 1,
                Some(RecordLifecycleState::Reclaimable) => {
                    retained_records += 1;
                    reclaimable_records += 1;
                }
                _ => {}
            }
        }

        summaries.push(ChunkVisibilitySummary {
            chunk_index,
            slot_start,
            slot_len: slot_end - slot_start,
            visible_records,
            retained_records,
            reclaimable_records,
            earliest_created_at,
            latest_retired_at,
        });

        chunk_index += 1;
        slot_start = slot_end;
    }

    summaries
}

pub(super) fn slot_chunk_index(slot: usize, chunk_size: usize) -> usize {
    if chunk_size == 0 {
        0
    } else {
        slot / chunk_size
    }
}
