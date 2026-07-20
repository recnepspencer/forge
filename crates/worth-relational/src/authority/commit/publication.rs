use crate::authority::commit::preparation::planning::strategy::{
    coarse_preparation_packet_count, packet_width_is_profitable, MIN_PARALLEL_PACKET_WIDTH,
    TARGET_PREPARATION_ITEMS_PER_PACKET,
};
use crate::authority::commit::preparation::reduction::merge::{
    canonical_merge_streams, OrderedReductionStream,
};
use crate::authority::mutation::FoundationalPatchFragment;
use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::VersionId;
use crate::logic::planning::RelationalExecutionModel;
use crate::logic::runtime::RelationalRuntime;
use crate::publication::patch::data::{
    PatchOrdering, PatchPublicationMode, PatchStreamPosition, PublishedAuthoritativePatchEnvelope,
    PublishedAuthoritativeRecordPatch,
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
std::thread_local! {
    static TEST_DIFF_PREPARATION_FAULT: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn current_test_diff_preparation_fault() -> Option<TestDiffPreparationFault> {
    match TEST_DIFF_PREPARATION_FAULT.with(std::cell::Cell::get) {
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
    struct ResetGuard {
        previous: u8,
    }

    impl Drop for ResetGuard {
        fn drop(&mut self) {
            TEST_DIFF_PREPARATION_FAULT.with(|fault| fault.set(self.previous));
        }
    }

    let next = match fault {
        TestDiffPreparationFault::FragmentCanonicalizationFailure => 1,
        TestDiffPreparationFault::PacketOverlapDetected => 2,
    };
    let previous = TEST_DIFF_PREPARATION_FAULT.with(|active| {
        let previous = active.get();
        active.set(next);
        previous
    });
    let _reset = ResetGuard { previous };
    run()
}

pub(super) fn assemble_patch(
    runtime: &RelationalRuntime,
    commit_id: CommitId,
    fragments: Vec<FoundationalPatchFragment>,
) -> PublishedAuthoritativePatchEnvelope {
    let records = prepare_patch_fragments(
        runtime,
        fragments
            .into_iter()
            .map(|fragment| fragment.published_record())
            .collect(),
    );
    PublishedAuthoritativePatchEnvelope {
        ordering: PatchOrdering::CanonicalCommitOrder,
        publication_mode: PatchPublicationMode::CommitNative,
        position: PatchStreamPosition(commit_id.0),
        authoritative_record_patches: records,
    }
    .canonicalized()
}

fn prepare_patch_fragments(
    runtime: &RelationalRuntime,
    authoritative_record_patches: Vec<PublishedAuthoritativeRecordPatch>,
) -> Vec<PublishedAuthoritativeRecordPatch> {
    use crate::authority::commit::preparation::packets::diff::{
        DiffPreparationHeader, DiffPreparationPacket,
    };

    if authoritative_record_patches.is_empty() {
        return authoritative_record_patches;
    }

    let packet_count = coarse_preparation_packet_count(
        authoritative_record_patches.len(),
        TARGET_PREPARATION_ITEMS_PER_PACKET,
    );
    runtime.performance_access().count_preparation_packet_shape(
        packet_count,
        authoritative_record_patches.len(),
        authoritative_record_patches
            .chunks(TARGET_PREPARATION_ITEMS_PER_PACKET)
            .map(|chunk| chunk.len())
            .max()
            .unwrap_or(0),
        authoritative_record_patches
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
        return direct_diff_record_order(authoritative_record_patches);
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
    for (packet_index, chunk) in authoritative_record_patches
        .chunks(TARGET_PREPARATION_ITEMS_PER_PACKET)
        .enumerate()
    {
        packets.push(DiffPreparationPacket {
            header: DiffPreparationHeader {
                packet_index_floor: packet_index * TARGET_PREPARATION_ITEMS_PER_PACKET,
            },
            authoritative_record_patches: chunk.to_vec(),
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

fn direct_diff_record_order(
    authoritative_record_patches: Vec<PublishedAuthoritativeRecordPatch>,
) -> Vec<PublishedAuthoritativeRecordPatch> {
    use crate::authority::commit::preparation::packets::diff::DiffFragmentKind;
    use crate::authority::commit::preparation::reduction::keys::DiffReductionKey;

    let mut keyed_records = authoritative_record_patches
        .into_iter()
        .enumerate()
        .map(|(record_index, record)| {
            let canonical = record.canonicalized();
            #[allow(unused_mut)]
            let mut key = DiffReductionKey::new(
                canonical.target.clone(),
                diff_kind_order(DiffFragmentKind::from(canonical.structural_change)),
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
    PublishedAuthoritativeRecordPatch,
> {
    use crate::authority::commit::preparation::reduction::keys::DiffReductionKey;

    let mut canonical_records = Vec::with_capacity(packet.authoritative_record_patches.len());
    let mut headers = Vec::with_capacity(packet.authoritative_record_patches.len());

    for (offset, record) in packet.authoritative_record_patches.iter().enumerate() {
        let canonical = record.canonicalized();
        #[allow(unused_mut)]
        let mut key = DiffReductionKey::new(
            canonical.target.clone(),
            diff_kind_order(
                crate::authority::commit::preparation::packets::diff::DiffFragmentKind::from(
                    canonical.structural_change,
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
    RelationalDiagnosticArtifact::new(
        DiagnosticsScope::Transaction,
        DiagnosticsArtifactKind::MinimalSummary,
        DeterminismExpectation::Required,
        kept,
    )
}

#[cfg(test)]
mod tests;

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
        .refresh_unique_entity_aspect_field_index_for_records(changed_records, version_id);
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
