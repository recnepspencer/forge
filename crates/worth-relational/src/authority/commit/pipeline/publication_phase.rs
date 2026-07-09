use super::rejection::{attach_rejection, elapsed_micros};
use crate::authority::commit::phases::artifacts::{
    PublicationFinalizeArtifacts, PublicationPreparation,
};
use crate::authority::commit::phases::finalize::{
    finalize_commit_publication, FinalizeCommitInput,
};
use crate::authority::commit::phases::publication::append_durable_commit;
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::transactions::data::{
    CommitLog, CommitPhase, CommitPhaseTiming, TransactionCommitError,
};
use std::sync::Arc;

pub(super) struct FinalizedPublicationPhase {
    pub(super) canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
    pub(super) changed_records: Vec<crate::transactions::data::RecordRef>,
}

pub(super) fn append_durable_commit_phase(
    runtime: &mut RelationalRuntime,
    commit_log: &mut CommitLog,
    phase_timing: &mut CommitPhaseTiming,
    publication: &PublicationPreparation,
    commit_id: CommitId,
    branch_id: &BranchId,
) -> Result<(), TransactionCommitError> {
    commit_log.begin_phase(CommitPhase::DurableAppend);
    let phase_started = std::time::Instant::now();
    commit_log.record_durable_append_prepared(
        commit_id,
        &branch_id.0,
        publication
            .finalize
            .canonical_commit_envelope
            .patch
            .position,
    );
    append_durable_commit(
        runtime,
        &publication.finalize.canonical_commit_envelope,
        commit_id,
        branch_id,
    )
    .map_err(|error| attach_rejection(commit_log, CommitPhase::DurableAppend, error))?;
    commit_log.complete_phase(CommitPhase::DurableAppend);
    phase_timing.durable_append_micros = elapsed_micros(phase_started);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn finalize_publication_phase(
    runtime: &mut RelationalRuntime,
    commit_log: &mut CommitLog,
    phase_timing: &mut CommitPhaseTiming,
    working_state: crate::storage::overlay::WorkingState,
    publication: PublicationPreparation,
    version_id: VersionId,
    previous_branch_head_version: Option<VersionId>,
    commit_id: CommitId,
    commit_reference: &CommitReference,
    branch_id: &BranchId,
    merge_base_commits: &[CommitId],
    merge_parent_branches: &[BranchId],
) -> FinalizedPublicationPhase {
    commit_log.begin_phase(CommitPhase::Publication);
    let phase_started = std::time::Instant::now();
    let PublicationPreparation {
        change_summary: _,
        aspect_summary: _,
        aspect_evaluation_traces: _,
        aspect_emission_traces: _,
        summary: _,
        finalize:
            PublicationFinalizeArtifacts {
                artifacts,
                changed_records,
                canonical_commit_envelope,
                adjacency_deltas,
            },
    } = publication;
    let canonical_commit_envelope = Arc::new(canonical_commit_envelope);
    let publication_phase_timing = finalize_commit_publication(
        runtime,
        working_state,
        FinalizeCommitInput {
            changed_records: &changed_records,
            version_id,
            previous_branch_head_version,
            commit_id,
            commit_reference,
            canonical_commit_envelope: canonical_commit_envelope.clone(),
            branch_id,
            merge_base_commits,
            artifacts,
            merge_parent_branches,
            adjacency_deltas,
        },
    );
    commit_log.record_commit_published(commit_id, &commit_reference.branch_id.0);
    commit_log.complete_phase(CommitPhase::Publication);
    phase_timing.publication_micros = elapsed_micros(phase_started);
    phase_timing.publication_storage_commit_micros = publication_phase_timing.storage_commit_micros;
    phase_timing.publication_index_refresh_micros = publication_phase_timing.index_refresh_micros;
    phase_timing.publication_history_publish_micros =
        publication_phase_timing.history_publish_micros;
    phase_timing.publication_visibility_pin_micros = publication_phase_timing.visibility_pin_micros;
    phase_timing.publication_retention_trim_micros = publication_phase_timing.retention_trim_micros;
    phase_timing.publication_compaction_micros = publication_phase_timing.compaction_micros;
    phase_timing.publication_bundle_publish_micros = publication_phase_timing.bundle_publish_micros;
    phase_timing.publication_retention_pass_micros = publication_phase_timing.retention_pass_micros;
    phase_timing.publication_post_commit_consumer_micros =
        publication_phase_timing.post_commit_consumer_micros;

    FinalizedPublicationPhase {
        canonical_commit_envelope,
        changed_records,
    }
}
