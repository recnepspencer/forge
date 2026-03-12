use crate::identity::data::{VersionBound, VersionId};
use crate::logic::runtime::RelationalRuntime;
use crate::query::data::{QueryWorkPacket, ReadPacketPlan};
use crate::storage::data::{
    ChunkDiagnostics, ChunkVisibilitySummary, ChunkedStorageSummary, RecordLifecycleState,
};
use crate::transactions::data::RecordRef;

pub(crate) fn chunked_storage_summary(
    runtime: &RelationalRuntime,
    version_id: VersionId,
) -> ChunkedStorageSummary {
    ChunkedStorageSummary {
        entity_chunks: summarize_entity_chunks(runtime, version_id),
        relation_chunks: summarize_relation_chunks(runtime, version_id),
    }
}

pub(crate) fn chunk_diagnostics(
    runtime: &RelationalRuntime,
    version_id: VersionId,
) -> ChunkDiagnostics {
    let summary = chunked_storage_summary(runtime, version_id);
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

pub(crate) fn plan_read_packet(
    runtime: &RelationalRuntime,
    handle: &crate::snapshots::data::SnapshotHandle,
    packet: &QueryWorkPacket,
) -> Option<ReadPacketPlan> {
    if !runtime.visibility.is_known_snapshot(handle.snapshot_id) {
        return None;
    }
    let mut entity_chunk_indexes = Vec::new();
    let mut relation_chunk_indexes = Vec::new();

    for target in &packet.targets {
        match target {
            RecordRef::Entity(entity_id) => {
                let chunk_index = slot_chunk_index(
                    entity_id.local_slot.0 as usize,
                    runtime.config.storage.layout.entity_chunk_size,
                );
                if !entity_chunk_indexes.contains(&chunk_index) {
                    entity_chunk_indexes.push(chunk_index);
                }
            }
            RecordRef::Relation(relation_id) => {
                let chunk_index = slot_chunk_index(
                    relation_id.local_slot.0 as usize,
                    runtime.config.storage.layout.relation_chunk_size,
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

fn summarize_entity_chunks(
    runtime: &RelationalRuntime,
    version_id: VersionId,
) -> Vec<ChunkVisibilitySummary> {
    let mut summaries = Vec::new();
    let current_version = runtime.current_version_id();
    for partition in runtime.partitions.values() {
        if version_id == current_version {
            summaries.extend(summarize_current_entity_chunks(
                partition,
                runtime.config.storage.layout.entity_chunk_size,
            ));
            continue;
        }
        summaries.extend(summarize_chunks(
            partition.entity_arena.slot_count(),
            runtime.config.storage.layout.entity_chunk_size,
            |slot| partition.entity_arena.created_at.get(slot).copied(),
            |slot| partition.entity_arena.retired_at_for_slot(slot),
            |slot| partition.entity_arena.get_slot(slot).map(|slot_view| slot_view.lifecycle()),
            |slot| {
                partition
                    .entity_arena
                    .created_at
                    .get(slot)
                    .is_some_and(|created| {
                        let bound = VersionBound::new(version_id);
                        bound.includes_created(*created)
                            && partition.entity_arena.retired_at_for_slot(slot)
                                .is_none_or(|retired| bound.retains_retired(retired))
                            && partition
                                .entity_arena
                                .get_slot(slot)
                                .is_some_and(|slot_view| {
                                    slot_view.lifecycle() != RecordLifecycleState::Reusable
                                })
                    })
            },
        ));
    }
    summaries
}

fn summarize_relation_chunks(
    runtime: &RelationalRuntime,
    version_id: VersionId,
) -> Vec<ChunkVisibilitySummary> {
    let mut summaries = Vec::new();
    let current_version = runtime.current_version_id();
    for partition in runtime.partitions.values() {
        if version_id == current_version {
            summaries.extend(summarize_current_relation_chunks(
                partition,
                runtime.config.storage.layout.relation_chunk_size,
            ));
            continue;
        }
        summaries.extend(summarize_chunks(
            partition.relation_arena.slot_count(),
            runtime.config.storage.layout.relation_chunk_size,
            |slot| partition.relation_arena.created_at.get(slot).copied(),
            |slot| partition.relation_arena.retired_at_for_slot(slot),
            |slot| partition.relation_arena.get_slot(slot).map(|slot_view| slot_view.lifecycle()),
            |slot| {
                partition
                    .relation_arena
                    .created_at
                    .get(slot)
                    .is_some_and(|created| {
                        let bound = VersionBound::new(version_id);
                        bound.includes_created(*created)
                            && partition.relation_arena.retired_at_for_slot(slot)
                                .is_none_or(|retired| bound.retains_retired(retired))
                            && partition
                                .relation_arena
                                .get_slot(slot)
                                .is_some_and(|slot_view| {
                                    slot_view.lifecycle() != RecordLifecycleState::Reusable
                                })
                    })
            },
        ));
    }
    summaries
}

fn summarize_current_entity_chunks(
    partition: &crate::storage::logic::state::PartitionState,
    chunk_size: usize,
) -> Vec<ChunkVisibilitySummary> {
    summarize_current_chunks(
        partition.entity_arena.slot_count(),
        chunk_size,
        |start, end| {
            partition
                .entity_arena
                .live_bitset
                .count_ones_in_range(start, end)
        },
        |slot| partition.entity_arena.created_at.get(slot).copied(),
        |slot| partition.entity_arena.retired_at_for_slot(slot),
        |slot| partition.entity_arena.get_slot(slot).map(|slot_view| slot_view.lifecycle()),
    )
}

fn summarize_current_relation_chunks(
    partition: &crate::storage::logic::state::PartitionState,
    chunk_size: usize,
) -> Vec<ChunkVisibilitySummary> {
    summarize_current_chunks(
        partition.relation_arena.slot_count(),
        chunk_size,
        |start, end| {
            partition
                .relation_arena
                .live_bitset
                .count_ones_in_range(start, end)
        },
        |slot| partition.relation_arena.created_at.get(slot).copied(),
        |slot| partition.relation_arena.retired_at_for_slot(slot),
        |slot| partition.relation_arena.get_slot(slot).map(|slot_view| slot_view.lifecycle()),
    )
}

fn summarize_current_chunks<FLiveCount, FCreated, FRetired, FLifecycle>(
    slot_count: usize,
    chunk_size: usize,
    live_count_in_range: FLiveCount,
    created_at: FCreated,
    retired_at: FRetired,
    lifecycle: FLifecycle,
) -> Vec<ChunkVisibilitySummary>
where
    FLiveCount: Fn(usize, usize) -> usize,
    FCreated: Fn(usize) -> Option<VersionId>,
    FRetired: Fn(usize) -> Option<VersionId>,
    FLifecycle: Fn(usize) -> Option<RecordLifecycleState>,
{
    if chunk_size == 0 || slot_count == 0 {
        return Vec::new();
    }

    let mut summaries = Vec::new();
    let mut chunk_index = 0;
    let mut slot_start = 0;

    while slot_start < slot_count {
        let slot_end = (slot_start + chunk_size).min(slot_count);
        let visible_records = live_count_in_range(slot_start, slot_end);
        let mut retained_records = 0;
        let mut reclaimable_records = 0;
        let mut earliest_created_at: Option<VersionId> = None;
        let mut latest_retired_at: Option<VersionId> = None;

        for slot in slot_start..slot_end {
            if let Some(created) = created_at(slot) {
                earliest_created_at = Some(match earliest_created_at {
                    Some(current) => current.min(created),
                    None => created,
                });
            }

            if let Some(retired) = retired_at(slot) {
                retained_records += 1;
                latest_retired_at = Some(match latest_retired_at {
                    Some(current) => current.max(retired),
                    None => retired,
                });
            }

            if lifecycle(slot) == Some(RecordLifecycleState::Reclaimable) {
                reclaimable_records += 1;
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
                retained_records += 1;
                latest_retired_at = Some(match latest_retired_at {
                    Some(current) => current.max(retired),
                    None => retired,
                });
            }

            if lifecycle(slot) == Some(RecordLifecycleState::Reclaimable) {
                reclaimable_records += 1;
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

fn slot_chunk_index(slot: usize, chunk_size: usize) -> usize {
    if chunk_size == 0 {
        0
    } else {
        slot / chunk_size
    }
}
