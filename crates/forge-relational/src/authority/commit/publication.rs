use crate::diagnostics::data::{
    DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticArtifact,
    RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::logic::planning::RelationalExecutionModel;
use crate::publication::data::diff::{
    PatchCompatibilityClass, PatchOrdering, PatchPublicationMode, PatchRecord, PatchStreamPosition,
    RelationalPatchRecord,
};
use crate::storage::logic::state::{PartitionState, PublicationArtifacts};
use crate::transactions::data::RecordRef;
use rayon::prelude::*;
use std::sync::Arc;

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

fn prepare_patch_fragments(runtime: &RelationalRuntime, records: Vec<PatchRecord>) -> Vec<PatchRecord> {
    use crate::authority::commit::preparation::packets::diff::{
        DiffFragmentIdentity, DiffFragmentKind, DiffPreparationPacket,
    };
    use crate::authority::commit::preparation::reduction::keys::DiffReductionKey;

    if records.is_empty() {
        return records;
    }

    runtime
        .performance_access()
        .count_preparation_packets(records.len());

    let packets = records
        .into_iter()
        .enumerate()
        .map(|(packet_index, record)| DiffPreparationPacket {
            packet_index,
            identity: DiffFragmentIdentity {
                target: record.target.clone(),
                kind: DiffFragmentKind::from(&record.kind),
                packet_index,
            },
            record,
        })
        .collect::<Vec<_>>();

    let mut fragments = match runtime.config.execution.execution_model {
        RelationalExecutionModel::StagedParallelPreparation if packets.len() > 1 => {
            runtime
                .performance_access()
                .count_preparation_parallel_legal();
            runtime
                .performance_access()
                .count_preparation_parallel_profitable();
            runtime
                .performance_access()
                .count_preparation_staged_parallel_strategy();
            packets
                .par_iter()
                .map(|packet| {
                    (
                        DiffReductionKey::new(
                            packet.identity.target.clone(),
                            diff_kind_order(packet.identity.kind),
                            packet.packet_index,
                        ),
                        packet.record.canonicalized(),
                    )
                })
                .collect::<Vec<_>>()
        }
        _ => {
            runtime
                .performance_access()
                .count_preparation_serial_strategy();
            packets
                .into_iter()
                .map(|packet| {
                    (
                        DiffReductionKey::new(
                            packet.identity.target.clone(),
                            diff_kind_order(packet.identity.kind),
                            packet.packet_index,
                        ),
                        packet.record.canonicalized(),
                    )
                })
                .collect::<Vec<_>>()
        }
    };

    fragments.sort_by(|left, right| left.0.cmp(&right.0));
    fragments.into_iter().map(|(_, record)| record).collect()
}

fn diff_kind_order(kind: crate::authority::commit::preparation::packets::diff::DiffFragmentKind) -> u8 {
    match kind {
        crate::authority::commit::preparation::packets::diff::DiffFragmentKind::Created => 0,
        crate::authority::commit::preparation::packets::diff::DiffFragmentKind::Updated => 1,
        crate::authority::commit::preparation::packets::diff::DiffFragmentKind::Deleted => 2,
        crate::authority::commit::preparation::packets::diff::DiffFragmentKind::RetainedForAudit => 3,
    }
}

pub(super) fn diagnostics_summary_artifact(
    config: &crate::config::data::RelationalRuntimeConfig,
    entries: Vec<RelationalDiagnosticsEntry>,
) -> RelationalDiagnosticArtifact {
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::Transaction,
        kind: DiagnosticsArtifactKind::MinimalSummary,
        determinism: crate::diagnostics::data::DeterminismExpectation::Required,
        entries: entries
            .into_iter()
            .take(config.diagnostics.profile.max_entries_per_artifact)
            .collect(),
    }
}

pub(super) fn finalize_published_commit(
    runtime: &mut RelationalRuntime,
    committed_partitions: std::collections::BTreeMap<
        crate::identity::data::PartitionId,
        PartitionState,
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
) {
    runtime
        .storage_authority()
        .publish_partitions(committed_partitions);
    runtime
        .index_authority()
        .refresh_unique_field_index_for_records(changed_records, version_id);
    runtime
        .retention_access()
        .trim_live_history_for_records(changed_records, version_id);
    runtime.history_authority().publish_commit(
        commit_id,
        commit_reference.clone(),
        branch_id.clone(),
        canonical_commit_envelope.patch.position,
        canonical_commit_envelope,
    );
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
    runtime.durability_authority().compact_log_if_needed();
    let snapshot_id = runtime
        .publication_authority()
        .publish_artifacts(version_id, artifacts);
    let _ = runtime.retention_access().run_pass();
    runtime
        .publication_authority()
        .emit_commit_publication_diagnostic(
            commit_id,
            snapshot_id,
            branch_id.clone(),
            &commit_reference.parents,
            merge_parent_branches,
            merge_base_commits,
        );
}
