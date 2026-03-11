use serde_json::json;

use crate::authority::mutation::MutationEffect;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticArtifact,
    RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitId, CommitReference, VersionNode};
use crate::identity::data::{PartitionId, VersionId};
use crate::logic::runtime::RelationalRuntime;
use crate::publication::data::diff::{
    PatchCompatibilityClass, PatchOrdering, PatchPublicationMode, PatchStreamPosition,
    RelationalPatchRecord,
};
use crate::storage::logic::state::{PartitionState, PublicationArtifacts};
use crate::transactions::data::RecordRef;

pub(super) fn assemble_patch(
    config: &crate::config::data::RelationalRuntimeConfig,
    commit_id: CommitId,
    effect: &MutationEffect,
) -> RelationalPatchRecord {
    RelationalPatchRecord {
        ordering: PatchOrdering::CanonicalCommitOrder,
        publication_mode: PatchPublicationMode::CommitNative,
        position: PatchStreamPosition(commit_id.0),
        compatibility: match config.publication.patch_surface_policy {
            crate::config::data::PatchSurfacePolicy::StructuredPatchSurface => {
                PatchCompatibilityClass::StructuredCompatible
            }
            crate::config::data::PatchSurfacePolicy::DensePatchSurface => {
                PatchCompatibilityClass::DenseCompatible
            }
        },
        records: effect.patch_records.clone(),
    }
    .canonicalized()
}

pub(super) fn diagnostics_summary_artifact(
    config: &crate::config::data::RelationalRuntimeConfig,
    effect: &MutationEffect,
) -> RelationalDiagnosticArtifact {
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::Transaction,
        kind: DiagnosticsArtifactKind::MinimalSummary,
        determinism: crate::diagnostics::data::DeterminismExpectation::Required,
        entries: effect
            .diagnostics
            .clone()
            .into_iter()
            .take(config.diagnostics.max_entries_per_artifact)
            .collect(),
    }
}

pub(super) fn finalize_published_commit(
    runtime: &mut RelationalRuntime,
    committed_partitions: std::collections::BTreeMap<PartitionId, PartitionState>,
    changed_records: &[RecordRef],
    version_id: VersionId,
    previous_branch_head_version: Option<VersionId>,
    commit_id: CommitId,
    commit_reference: &CommitReference,
    canonical_commit_envelope: crate::replay::data::CanonicalCommitEnvelope,
    patch_position: PatchStreamPosition,
    branch_id: BranchId,
    merge_base_commits: Vec<CommitId>,
    artifacts: PublicationArtifacts,
    merge_parent_branches: Vec<BranchId>,
) {
    for (partition_id, partition_state) in committed_partitions {
        runtime.partitions.insert(partition_id, partition_state);
    }
    runtime.refresh_unique_field_index_for_records(changed_records, version_id);
    runtime
        .snapshots
        .published_handles
        .insert(artifacts.snapshot.snapshot_id, version_id);
    runtime.trim_live_history_for_records(changed_records, version_id);
    runtime.history.next_commit_id += 1;
    runtime.history.next_version_id += 1;
    runtime
        .history
        .branch_heads
        .insert(branch_id.clone(), Some(commit_reference.clone()));
    runtime.move_branch_head_visibility_residency(previous_branch_head_version, Some(version_id));
    runtime.advance_branch_pins_for_changed_records(
        previous_branch_head_version,
        version_id,
        changed_records,
    );
    runtime.history.commit_graph.insert(
        commit_id,
        VersionNode {
            commit: commit_reference.clone(),
        },
    );
    runtime
        .history
        .commit_envelopes
        .insert(commit_id, canonical_commit_envelope);
    runtime
        .history
        .patch_stream_index
        .insert(patch_position, commit_id);
    runtime.prune_published_snapshot_handles_if_needed();
    runtime.compact_durable_log_if_needed();
    runtime.publication.latest_bundle = Some(artifacts.bundle.clone());
    runtime.push_diagnostic_artifact(artifacts.diagnostics_summary);
    let _ = runtime.run_retention_pass();
    runtime.push_bounded_diagnostic(
        DiagnosticsScope::PatchPublication,
        DiagnosticsArtifactKind::MinimalSummary,
        publication_entries(
            commit_id,
            artifacts.snapshot.snapshot_id,
            branch_id,
            &commit_reference.parents,
            &merge_parent_branches,
            &merge_base_commits,
        ),
    );
}

fn publication_entries(
    commit_id: CommitId,
    snapshot_id: crate::snapshots::data::SnapshotId,
    branch_id: BranchId,
    parents: &[CommitId],
    merge_parent_branches: &[BranchId],
    merge_base_commits: &[CommitId],
) -> Vec<RelationalDiagnosticsEntry> {
    let publication_code = if parents.len() > 1 {
        DiagnosticCode::MergeCommitPublished
    } else {
        DiagnosticCode::CommitPublished
    };
    let mut entries = Vec::new();
    if parents.len() > 1 {
        entries.push(RelationalDiagnosticsEntry {
            code: DiagnosticCode::MergeBaseResolved,
            message: "merge bases resolved deterministically".to_string(),
            fields: json!({
                "commit_id": commit_id.0,
                "merge_base_commit_ids": merge_base_commits.iter().map(|base| base.0).collect::<Vec<_>>(),
            }),
        });
    }
    entries.push(RelationalDiagnosticsEntry {
        code: publication_code,
        message: if parents.len() > 1 {
            "merge commit published coherently".to_string()
        } else {
            "commit published coherently".to_string()
        },
        fields: json!({
            "commit_id": commit_id.0,
            "snapshot_id": snapshot_id.0,
            "branch_id": branch_id.0,
            "parent_commit_ids": parents.iter().map(|parent| parent.0).collect::<Vec<_>>(),
            "merge_parent_branches": merge_parent_branches.iter().map(|branch| branch.0.clone()).collect::<Vec<_>>(),
            "merge_base_commit_ids": merge_base_commits.iter().map(|base| base.0).collect::<Vec<_>>(),
        }),
    });
    entries
}
