use super::super::artifact_execution::preparation::PublicationPreparation;
use super::super::rejection::{attach_rejection, elapsed_micros};
use crate::authority::commit::phases::publication::append_durable_commit;
use crate::authority::mutation::apply_adjacency_deltas;
use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{BranchId, CommitId, RelationalCommitReceipt};
use crate::identity::data::VersionId;
use crate::publication::bundle::PublicationStage;
use crate::publication::data::PublicationError;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitLog, CommitPhase, CommitPhaseTiming, TransactionCommitError,
};
use std::sync::Arc;

mod finalization;

use finalization::{finalize_published_commit, FinalizationInput};

pub(super) struct FinalizedPublicationPhase {
    canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
    changed_records: Vec<crate::transactions::data::RecordRef>,
}

impl FinalizedPublicationPhase {
    pub(super) fn into_parts(
        self,
    ) -> (
        Arc<CanonicalCommitEnvelope>,
        Vec<crate::transactions::data::RecordRef>,
    ) {
        (self.canonical_commit_envelope, self.changed_records)
    }
}

pub(super) struct DurableAppendPhaseInput<'a> {
    pub(super) commit_log: &'a mut CommitLog,
    pub(super) phase_timing: &'a mut CommitPhaseTiming,
    pub(super) publication: &'a PublicationPreparation,
    pub(super) append_authority: crate::durability::authority::DurableAppendAuthority,
    pub(super) commit_id: CommitId,
    pub(super) branch_id: &'a BranchId,
}

pub(super) fn append_durable_commit_phase(
    runtime: &mut RelationalRuntime,
    input: DurableAppendPhaseInput<'_>,
) -> Result<(), TransactionCommitError> {
    let DurableAppendPhaseInput {
        commit_log,
        phase_timing,
        publication,
        append_authority,
        commit_id,
        branch_id,
    } = input;
    commit_log.begin_phase(CommitPhase::DurableAppend);
    let phase_started = std::time::Instant::now();
    commit_log.record_durable_append_prepared(
        commit_id,
        &branch_id.0,
        publication.patch_position(),
    );
    append_durable_commit(
        runtime,
        append_authority,
        publication.canonical_commit_envelope(),
    )
    .map_err(|error| attach_rejection(commit_log, CommitPhase::DurableAppend, error))?;
    commit_log.complete_phase(CommitPhase::DurableAppend);
    phase_timing.durable_append_micros = elapsed_micros(phase_started);
    Ok(())
}

pub(super) struct FinalizePublicationPhaseInput<'a> {
    pub(super) commit_log: &'a mut CommitLog,
    pub(super) phase_timing: &'a mut CommitPhaseTiming,
    pub(super) working_state: crate::storage::overlay::WorkingState,
    pub(super) publication: PublicationPreparation,
    pub(super) version_id: VersionId,
    pub(super) previous_branch_head_version: Option<VersionId>,
    pub(super) commit_id: CommitId,
    pub(super) commit_reference: &'a RelationalCommitReceipt,
    pub(super) branch_binding: &'a crate::branch::RelationalLegacyBranchBinding,
    pub(super) branch_id: &'a BranchId,
    pub(super) merge_base_commits: &'a [CommitId],
    pub(super) merge_parent_branches: &'a [BranchId],
}

pub(super) fn finalize_publication_phase(
    runtime: &mut RelationalRuntime,
    input: FinalizePublicationPhaseInput<'_>,
) -> Result<FinalizedPublicationPhase, TransactionCommitError> {
    let FinalizePublicationPhaseInput {
        commit_log,
        phase_timing,
        working_state,
        publication,
        version_id,
        previous_branch_head_version,
        commit_id,
        commit_reference,
        branch_binding,
        branch_id,
        merge_base_commits,
        merge_parent_branches,
    } = input;
    commit_log.begin_phase(CommitPhase::Publication);
    let phase_started = std::time::Instant::now();
    let (artifacts, changed_records, canonical_commit_envelope, adjacency_deltas) =
        publication.into_finalize().into_parts();
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
            branch_binding,
            canonical_commit_envelope: canonical_commit_envelope.clone(),
            branch_id,
            merge_base_commits,
            artifacts,
            merge_parent_branches,
            adjacency_deltas,
        },
    )
    .map_err(|detail| {
        TransactionCommitError::publication(PublicationError::new(
            PublicationStage::BundleAssembly,
            detail,
        ))
        .with_commit_log(commit_log.clone())
    })?;
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

    Ok(FinalizedPublicationPhase {
        canonical_commit_envelope,
        changed_records,
    })
}

struct FinalizeCommitInput<'a> {
    changed_records: &'a [crate::transactions::data::RecordRef],
    version_id: VersionId,
    previous_branch_head_version: Option<VersionId>,
    commit_id: CommitId,
    commit_reference: &'a RelationalCommitReceipt,
    branch_binding: &'a crate::branch::RelationalLegacyBranchBinding,
    canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
    branch_id: &'a BranchId,
    merge_base_commits: &'a [CommitId],
    artifacts: crate::storage::overlay::PublicationArtifacts,
    merge_parent_branches: &'a [BranchId],
    adjacency_deltas: Vec<crate::authority::mutation::AdjacencyDelta>,
}

fn finalize_commit_publication(
    runtime: &mut RelationalRuntime,
    mut working_state: crate::storage::overlay::WorkingState,
    input: FinalizeCommitInput<'_>,
) -> Result<crate::authority::commit::phases::finalize::PublicationPhaseTiming, String> {
    let mut phase_timing =
        crate::authority::commit::phases::finalize::PublicationPhaseTiming::default();
    let phase_started = std::time::Instant::now();
    apply_adjacency_deltas(&mut working_state, &input.adjacency_deltas);
    phase_timing.storage_commit_micros = phase_started.elapsed().as_micros() as u64;
    let clone_mode = working_state.clone_mode();
    let committed_partitions = working_state.into_partition_commits().1;
    finalize_published_commit(
        runtime,
        FinalizationInput {
            clone_mode,
            committed_partitions,
            changed_records: input.changed_records,
            version_id: input.version_id,
            previous_branch_head_version: input.previous_branch_head_version,
            commit_id: input.commit_id,
            commit_reference: input.commit_reference,
            branch_binding: input.branch_binding,
            canonical_commit_envelope: input.canonical_commit_envelope,
            branch_id: input.branch_id,
            merge_base_commits: input.merge_base_commits,
            artifacts: input.artifacts,
            merge_parent_branches: input.merge_parent_branches,
            phase_timing: &mut phase_timing,
        },
    )?;
    Ok(phase_timing)
}
