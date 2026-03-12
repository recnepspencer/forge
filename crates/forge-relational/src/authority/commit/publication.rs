use crate::authority::mutation::MutationEffect;
use crate::diagnostics::data::{
    DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticArtifact,
};
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::VersionId;
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
        compatibility: match config.publication.policy.patch_surface_policy {
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
            .take(config.diagnostics.profile.max_entries_per_artifact)
            .collect(),
    }
}

pub(super) fn finalize_published_commit(
    runtime: &mut RelationalRuntime,
    committed_partitions: std::collections::BTreeMap<crate::identity::data::PartitionId, PartitionState>,
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
        patch_position,
        canonical_commit_envelope,
    );
    runtime
        .visibility_pins()
        .move_branch_head_visibility_residency(previous_branch_head_version, Some(version_id));
    runtime.visibility_pins().advance_branch_pins_for_changed_records(
        previous_branch_head_version,
        version_id,
        changed_records,
    );
    runtime.durability_authority().compact_log_if_needed();
    let artifacts = runtime
        .publication_authority()
        .publish_artifacts(version_id, artifacts);
    let _ = runtime.retention_access().run_pass();
    runtime.publication_authority().emit_commit_publication_diagnostic(
        commit_id,
        artifacts.snapshot.snapshot_id,
        branch_id,
        &commit_reference.parents,
        &merge_parent_branches,
        &merge_base_commits,
    );
}
