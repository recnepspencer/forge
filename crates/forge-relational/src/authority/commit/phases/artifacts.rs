use crate::authority::commit::phases::publication::{
    canonical_commit_envelope, canonicalize_changed_records,
};
use crate::authority::commit::publication::diagnostics_summary_artifact;
use crate::history::data::CommitReference;
use crate::publication::data::diff::RelationalPatchRecord;
use crate::transactions::data::{
    CommitChangeSummary, CommitPublicationSummary, MergedCommitPlan, RecordRef,
    TransactionCommitError,
};

pub(crate) struct PublicationPreparation {
    pub(crate) change_summary: CommitChangeSummary,
    pub(crate) summary: CommitPublicationSummary,
    pub(crate) finalize: PublicationFinalizeArtifacts,
}

pub(crate) struct PublicationFinalizeArtifacts {
    pub(crate) artifacts: crate::storage::overlay::PublicationArtifacts,
    pub(crate) changed_records: Vec<RecordRef>,
    pub(crate) canonical_commit_envelope: crate::replay::data::CanonicalCommitEnvelope,
    pub(crate) adjacency_deltas: Vec<crate::authority::mutation::AdjacencyDelta>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn prepare_publication_artifacts(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    working_state: &mut crate::logic::runtime::WorkingState,
    patch: RelationalPatchRecord,
    commit_reference: &CommitReference,
    branch_id: &crate::history::data::BranchId,
    version_id: crate::identity::data::VersionId,
    merge_parent_branches: &[crate::history::data::BranchId],
    merge_base_commits: &[crate::history::data::CommitId],
    merged_plan: &MergedCommitPlan,
    effect: crate::authority::mutation::MutationEffect,
) -> Result<PublicationPreparation, TransactionCommitError> {
    let diagnostics_summary =
        diagnostics_summary_artifact(&runtime.config, effect.diagnostics.entries);
    let artifacts = runtime.publication_authority().assemble_publication_bundle(
        commit_reference.clone(),
        version_id,
        patch.clone(),
        diagnostics_summary.clone(),
    );
    let lineage_event_ids = runtime.lineage_authority().ensure_lineage_for_commit(
        working_state,
        commit_reference,
        &merged_plan.merged_intents,
        &effect.publication.changed_records,
    );
    let lineage_event_count = lineage_event_ids.len();
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
    let mut changed_records = effect.publication.changed_records;
    let adjacency_deltas = effect.adjacency.deltas;
    canonicalize_changed_records(&mut changed_records);
    let change_summary = CommitChangeSummary {
        changed_record_count: changed_records.len(),
        adjacency_delta_count: adjacency_deltas.len(),
    };
    let summary = CommitPublicationSummary {
        patch_record_count: patch.records.len(),
        diagnostics_entry_count: artifacts.bundle.diagnostics_summary.entries.len(),
        lineage_event_count,
        patch_position: Some(patch.position),
        final_snapshot_id: Some(artifacts.bundle.snapshot.snapshot_id),
        merge_parent_count: commit_reference.parents.len().saturating_sub(1),
    };

    Ok(PublicationPreparation {
        change_summary,
        summary,
        finalize: PublicationFinalizeArtifacts {
            artifacts,
            changed_records,
            canonical_commit_envelope,
            adjacency_deltas,
        },
    })
}
