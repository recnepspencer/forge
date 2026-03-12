use crate::authority::commit::phases::publication::{
    canonical_commit_envelope, canonicalize_changed_records, enforce_patch_budget,
};
use crate::authority::commit::publication::{assemble_patch, diagnostics_summary_artifact};
use crate::history::data::CommitReference;
use crate::publication::data::diff::RelationalPatchRecord;
use crate::transactions::data::{MergedCommitPlan, RecordRef, TransactionCommitError};

pub(crate) struct PublicationPreparation {
    pub(crate) patch: RelationalPatchRecord,
    pub(crate) artifacts: crate::storage::overlay::PublicationArtifacts,
    pub(crate) published_snapshot: crate::snapshots::data::SnapshotHandle,
    pub(crate) changed_records: Vec<RecordRef>,
    pub(crate) canonical_commit_envelope: crate::replay::data::CanonicalCommitEnvelope,
    pub(crate) adjacency_deltas: Vec<crate::authority::mutation::AdjacencyDelta>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn prepare_publication_artifacts(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    working_state: &mut crate::logic::runtime::WorkingState,
    commit_reference: &CommitReference,
    branch_id: &crate::history::data::BranchId,
    version_id: crate::identity::data::VersionId,
    merge_parent_branches: Vec<crate::history::data::BranchId>,
    merge_base_commits: Vec<crate::history::data::CommitId>,
    merged_plan: &MergedCommitPlan,
    effect: crate::authority::mutation::MutationEffect,
) -> Result<PublicationPreparation, TransactionCommitError> {
    let patch = assemble_patch(&runtime.config, commit_reference.commit_id, &effect);
    enforce_patch_budget(runtime, &patch)?;
    let diagnostics_summary = diagnostics_summary_artifact(&runtime.config, &effect);
    let artifacts = runtime.publication_authority().assemble_publication_bundle(
        commit_reference.clone(),
        version_id,
        patch.clone(),
        diagnostics_summary.clone(),
    );
    let published_snapshot = artifacts.snapshot.clone();
    let lineage_event_ids = runtime.lineage_authority().ensure_lineage_for_commit(
        working_state,
        commit_reference,
        &merged_plan.merged_intents,
        &effect.changed_records,
    );
    let canonical_commit_envelope = canonical_commit_envelope(
        runtime,
        commit_reference,
        branch_id,
        merge_parent_branches,
        merge_base_commits,
        merged_plan,
        patch.clone(),
        diagnostics_summary.clone(),
        lineage_event_ids,
    );
    let mut changed_records = effect.changed_records;
    let adjacency_deltas = effect.adjacency_deltas;
    canonicalize_changed_records(&mut changed_records);

    Ok(PublicationPreparation {
        patch,
        artifacts,
        published_snapshot,
        changed_records,
        canonical_commit_envelope,
        adjacency_deltas,
    })
}
