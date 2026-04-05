use crate::authority::commit::preparation::planning::strategy::{
    coarse_preparation_packet_count, packet_width_is_profitable, MIN_PARALLEL_PACKET_WIDTH,
    TARGET_PREPARATION_ITEMS_PER_PACKET,
};
use crate::authority::commit::preparation::reduction::merge::{
    canonical_merge_streams, OrderedReductionStream,
};
use crate::diagnostics::data::{
    DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticArtifact,
    RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::VersionId;
use crate::logic::planning::RelationalExecutionModel;
use crate::logic::runtime::RelationalRuntime;
use crate::publication::data::diff::{
    PatchCompatibilityClass, PatchOrdering, PatchPublicationMode, PatchRecord, PatchStreamPosition,
    RelationalPatchRecord,
};
use crate::storage::logic::state::{PartitionState, PublicationArtifacts};
use crate::transactions::data::RecordRef;
use rayon::prelude::*;
use std::collections::BTreeSet;
use std::sync::Arc;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TestDiffPreparationFault {
    FragmentCanonicalizationFailure,
    PacketOverlapDetected,
}

#[cfg(test)]
static TEST_DIFF_PREPARATION_FAULT: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);

#[cfg(test)]
pub(crate) fn current_test_diff_preparation_fault() -> Option<TestDiffPreparationFault> {
    match TEST_DIFF_PREPARATION_FAULT.load(std::sync::atomic::Ordering::SeqCst) {
        1 => Some(TestDiffPreparationFault::FragmentCanonicalizationFailure),
        2 => Some(TestDiffPreparationFault::PacketOverlapDetected),
        _ => None,
    }
}

#[cfg(test)]
pub(crate) fn with_test_diff_preparation_fault<T>(
    fault: TestDiffPreparationFault,
    run: impl FnOnce() -> T,
) -> T {
    struct ResetGuard<'a> {
        fault: &'a std::sync::atomic::AtomicU8,
        _lock: std::sync::MutexGuard<'a, ()>,
    }

    impl Drop for ResetGuard<'_> {
        fn drop(&mut self) {
            self.fault.store(0, std::sync::atomic::Ordering::SeqCst);
        }
    }

    let guard = crate::testing::fault_injection_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let _reset = ResetGuard {
        fault: &TEST_DIFF_PREPARATION_FAULT,
        _lock: guard,
    };
    TEST_DIFF_PREPARATION_FAULT.store(
        match fault {
            TestDiffPreparationFault::FragmentCanonicalizationFailure => 1,
            TestDiffPreparationFault::PacketOverlapDetected => 2,
        },
        std::sync::atomic::Ordering::SeqCst,
    );
    run()
}

pub(super) fn assemble_patch(
    runtime: &RelationalRuntime,
    commit_id: CommitId,
    records: Vec<PatchRecord>,
) -> RelationalPatchRecord {
    let records = prepare_patch_fragments(runtime, records);
    RelationalPatchRecord {
        ordering: PatchOrdering::CanonicalCommitOrder,
        publication_mode: PatchPublicationMode::CommitNative,
        position: PatchStreamPosition(commit_id.0),
        compatibility: match runtime.config.publication.policy.patch_surface_policy {
            crate::config::data::PatchSurfacePolicy::StructuredPatchSurface => {
                PatchCompatibilityClass::StructuredCompatible
            }
            crate::config::data::PatchSurfacePolicy::DensePatchSurface => {
                PatchCompatibilityClass::DenseCompatible
            }
        },
        records,
    }
    .canonicalized()
}

fn prepare_patch_fragments(
    runtime: &RelationalRuntime,
    records: Vec<PatchRecord>,
) -> Vec<PatchRecord> {
    use crate::authority::commit::preparation::packets::diff::{
        DiffPreparationHeader, DiffPreparationPacket,
    };

    if records.is_empty() {
        return records;
    }

    let packet_count =
        coarse_preparation_packet_count(records.len(), TARGET_PREPARATION_ITEMS_PER_PACKET);
    runtime.performance_access().count_preparation_packet_shape(
        packet_count,
        records.len(),
        records
            .chunks(TARGET_PREPARATION_ITEMS_PER_PACKET)
            .map(|chunk| chunk.len())
            .max()
            .unwrap_or(0),
        records
            .iter()
            .map(|record| match record.target {
                RecordRef::Entity(entity_id) => entity_id.partition_id,
                RecordRef::Relation(relation_id) => relation_id.partition_id,
            })
            .collect::<BTreeSet<_>>()
            .len(),
    );

    let use_parallel_packets =
        matches!(
            runtime.config.execution.execution_model,
            RelationalExecutionModel::StagedParallelPreparation
        ) && packet_width_is_profitable(packet_count, MIN_PARALLEL_PACKET_WIDTH);

    if !use_parallel_packets {
        runtime
            .performance_access()
            .count_preparation_serial_strategy();
        return direct_diff_record_order(records);
    }

    runtime
        .performance_access()
        .count_preparation_parallel_legal();
    runtime
        .performance_access()
        .count_preparation_parallel_profitable();
    runtime
        .performance_access()
        .count_preparation_staged_parallel_strategy();

    let mut packets = Vec::with_capacity(packet_count);
    for (packet_index, chunk) in records
        .chunks(TARGET_PREPARATION_ITEMS_PER_PACKET)
        .enumerate()
    {
        packets.push(DiffPreparationPacket {
            header: DiffPreparationHeader {
                packet_index_floor: packet_index * TARGET_PREPARATION_ITEMS_PER_PACKET,
            },
            records: chunk.to_vec(),
        });
    }

    canonical_merge_streams(
        packets
            .par_iter()
            .map(diff_packet_stream)
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .map(|(_, record)| record)
    .collect()
}

fn direct_diff_record_order(records: Vec<PatchRecord>) -> Vec<PatchRecord> {
    use crate::authority::commit::preparation::packets::diff::DiffFragmentKind;
    use crate::authority::commit::preparation::reduction::keys::DiffReductionKey;

    let mut keyed_records = records
        .into_iter()
        .enumerate()
        .map(|(record_index, record)| {
            let canonical = record.canonicalized();
            #[allow(unused_mut)]
            let mut key = DiffReductionKey::new(
                canonical.target.clone(),
                diff_kind_order(DiffFragmentKind::from(&canonical.kind)),
                record_index,
            );
            #[cfg(test)]
            if matches!(
                current_test_diff_preparation_fault(),
                Some(TestDiffPreparationFault::PacketOverlapDetected)
            ) {
                key = DiffReductionKey::new(canonical.target.clone(), 0, 0);
            }
            (key, canonical)
        })
        .collect::<Vec<_>>();
    keyed_records.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    keyed_records
        .into_iter()
        .map(|(_key, record)| record)
        .collect()
}

fn diff_packet_stream(
    packet: &crate::authority::commit::preparation::packets::diff::DiffPreparationPacket,
) -> OrderedReductionStream<
    crate::authority::commit::preparation::reduction::keys::DiffReductionKey,
    PatchRecord,
> {
    use crate::authority::commit::preparation::reduction::keys::DiffReductionKey;

    let mut canonical_records = Vec::with_capacity(packet.records.len());
    let mut headers = Vec::with_capacity(packet.records.len());

    for (offset, record) in packet.records.iter().enumerate() {
        let canonical = record.canonicalized();
        #[allow(unused_mut)]
        let mut key = DiffReductionKey::new(
            canonical.target.clone(),
            diff_kind_order(
                crate::authority::commit::preparation::packets::diff::DiffFragmentKind::from(
                    &canonical.kind,
                ),
            ),
            packet.header.packet_index_floor + offset,
        );
        #[cfg(test)]
        if matches!(
            current_test_diff_preparation_fault(),
            Some(TestDiffPreparationFault::PacketOverlapDetected)
        ) {
            key = DiffReductionKey::new(
                canonical.target.clone(),
                0,
                packet.header.packet_index_floor,
            );
        }
        headers.push((key, offset));
        canonical_records.push(canonical);
    }

    headers.sort_unstable_by(|left, right| left.0.cmp(&right.0));

    let mut canonical_records = canonical_records.into_iter().map(Some).collect::<Vec<_>>();
    let mut stream = Vec::with_capacity(headers.len());
    for (key, record_index) in headers {
        let canonical = canonical_records[record_index]
            .take()
            .expect("diff packet header index must resolve exactly once");
        stream.push((key, canonical));
    }
    OrderedReductionStream::new(stream)
}

fn diff_kind_order(
    kind: crate::authority::commit::preparation::packets::diff::DiffFragmentKind,
) -> u8 {
    match kind {
        crate::authority::commit::preparation::packets::diff::DiffFragmentKind::Created => 0,
        crate::authority::commit::preparation::packets::diff::DiffFragmentKind::Updated => 1,
        crate::authority::commit::preparation::packets::diff::DiffFragmentKind::Deleted => 2,
        crate::authority::commit::preparation::packets::diff::DiffFragmentKind::RetainedForAudit => 3,
    }
}

pub(super) fn diagnostics_summary_artifact(
    config: &crate::config::data::RelationalRuntimeConfig,
    reserved_entries: Vec<RelationalDiagnosticsEntry>,
    entries: Vec<RelationalDiagnosticsEntry>,
) -> RelationalDiagnosticArtifact {
    let max_entries = config.diagnostics.profile.max_entries_per_artifact;
    // Reserved entries are proof-carrying summary surfaces for the authoritative lifecycle.
    // They must survive even when the optional diagnostics budget is exhausted or configured
    // below the reserved-entry count.
    let mut kept = reserved_entries;
    let remaining_capacity = max_entries.saturating_sub(kept.len());
    kept.extend(entries.into_iter().take(remaining_capacity));
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::Transaction,
        kind: DiagnosticsArtifactKind::MinimalSummary,
        determinism: crate::diagnostics::data::DeterminismExpectation::Required,
        entries: kept,
    }
}

#[cfg(test)]
mod tests {
    use super::{diff_packet_stream, direct_diff_record_order};
    use crate::authority::commit::preparation::packets::diff::{
        DiffPreparationHeader, DiffPreparationPacket,
    };
    use crate::authority::commit::preparation::reduction::merge::canonical_merge_streams;
    use crate::identity::data::{EntityId, PartitionId};
    use crate::publication::data::diff::{
        AspectKey, CanonicalAspectSet, PatchDetail, PatchRecord, PatchRecordKind,
        RecordStructuralChange,
    };
    use crate::symbols::data::InternedString;
    use crate::transactions::data::RecordRef;

    #[test]
    fn direct_serial_patch_preparation_matches_packet_merge_order() {
        let records = vec![
            patch_record(7, PatchRecordKind::Updated),
            patch_record(3, PatchRecordKind::Deleted),
            patch_record(3, PatchRecordKind::Created),
            patch_record(8, PatchRecordKind::RetainedForAudit),
            patch_record(7, PatchRecordKind::Created),
            patch_record(3, PatchRecordKind::Updated),
        ];

        let mut packets = Vec::new();
        for (packet_index, chunk) in records.chunks(2).enumerate() {
            packets.push(DiffPreparationPacket {
                header: DiffPreparationHeader {
                    packet_index_floor: packet_index * 2,
                },
                records: chunk.to_vec(),
            });
        }

        let merged =
            canonical_merge_streams(packets.iter().map(diff_packet_stream).collect::<Vec<_>>())
                .into_iter()
                .map(|(_key, record)| record)
                .collect::<Vec<_>>();
        let direct = direct_diff_record_order(records);

        assert_eq!(direct, merged);
    }

    fn patch_record(raw_entity_id: u64, kind: PatchRecordKind) -> PatchRecord {
        let kind_label = format!("{kind:?}");
        let structural_change = match kind {
            PatchRecordKind::Created => RecordStructuralChange::Created,
            PatchRecordKind::Updated => RecordStructuralChange::Updated,
            PatchRecordKind::Deleted => RecordStructuralChange::Deleted,
            PatchRecordKind::RetainedForAudit => RecordStructuralChange::RetainedForAudit,
        };
        PatchRecord {
            kind,
            target: RecordRef::Entity(EntityId::new(PartitionId(0), raw_entity_id, 0)),
            structural_change,
            aspects: CanonicalAspectSet::new([AspectKey(InternedString::from("geometry"))]),
            contains_degraded_precision: false,
            detail: PatchDetail::StructuredJson(serde_json::json!({
                "entity": raw_entity_id,
                "kind": kind_label,
            })),
        }
    }
}

pub(super) fn finalize_published_commit(
    runtime: &mut RelationalRuntime,
    clone_mode: crate::storage::overlay::PartitionCloneMode,
    committed_partitions: std::collections::BTreeMap<
        crate::identity::data::PartitionId,
        (
            PartitionState,
            crate::storage::overlay::PartitionMutationJournal,
        ),
    >,
    changed_records: &[RecordRef],
    version_id: VersionId,
    previous_branch_head_version: Option<VersionId>,
    commit_id: CommitId,
    commit_reference: &CommitReference,
    canonical_commit_envelope: Arc<crate::replay::data::CanonicalCommitEnvelope>,
    branch_id: &BranchId,
    merge_base_commits: &[CommitId],
    artifacts: PublicationArtifacts,
    merge_parent_branches: &[BranchId],
    phase_timing: &mut crate::authority::commit::phases::finalize::PublicationPhaseTiming,
) {
    let phase_started = std::time::Instant::now();
    runtime
        .storage_authority()
        .publish_partition_commits(clone_mode, committed_partitions);
    phase_timing.storage_commit_micros = phase_started.elapsed().as_micros() as u64;

    let phase_started = std::time::Instant::now();
    runtime
        .index_authority()
        .refresh_unique_field_index_for_records(changed_records, version_id);
    phase_timing.index_refresh_micros = phase_started.elapsed().as_micros() as u64;

    let phase_started = std::time::Instant::now();
    runtime.history_authority().publish_commit(
        commit_id,
        commit_reference.clone(),
        branch_id.clone(),
        canonical_commit_envelope.patch.position,
        canonical_commit_envelope,
    );
    phase_timing.history_publish_micros = phase_started.elapsed().as_micros() as u64;

    let phase_started = std::time::Instant::now();
    runtime
        .visibility_pins()
        .move_branch_head_visibility_residency(previous_branch_head_version, Some(version_id));
    runtime
        .visibility_pins()
        .advance_branch_pins_for_changed_records(
            previous_branch_head_version,
            version_id,
            changed_records,
        );
    phase_timing.visibility_pin_micros = phase_started.elapsed().as_micros() as u64;

    let phase_started = std::time::Instant::now();
    runtime
        .retention()
        .trim_live_history_for_records(changed_records, version_id);
    phase_timing.retention_trim_micros = phase_started.elapsed().as_micros() as u64;

    let phase_started = std::time::Instant::now();
    runtime.durability_authority().compact_log_if_needed();
    phase_timing.compaction_micros = phase_started.elapsed().as_micros() as u64;

    let phase_started = std::time::Instant::now();
    let snapshot_id = runtime
        .publication_authority()
        .publish_artifacts(version_id, artifacts);
    phase_timing.bundle_publish_micros = phase_started.elapsed().as_micros() as u64;

    if runtime.config.storage.mvcc.auto_reclaim_deleted_records
        || runtime.config.storage.mvcc.snapshot_release_policy
            == crate::config::data::SnapshotReleasePolicy::ReleaseOnRetentionPass
    {
        let phase_started = std::time::Instant::now();
        let _ = runtime.retention().run_pass();
        phase_timing.retention_pass_micros = phase_started.elapsed().as_micros() as u64;
    }

    let phase_started = std::time::Instant::now();
    runtime
        .publication_authority()
        .consume_post_commit_artifacts(
            commit_id,
            snapshot_id,
            branch_id.clone(),
            &commit_reference.parents,
            merge_parent_branches,
            merge_base_commits,
        );
    phase_timing.post_commit_consumer_micros = phase_started.elapsed().as_micros() as u64;
}
