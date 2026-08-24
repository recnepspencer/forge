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
    pub(super) canonical_commit_envelope: &'a CanonicalCommitEnvelope,
    pub(super) patch_position: crate::publication::patch::data::PatchStreamPosition,
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
        canonical_commit_envelope,
        patch_position,
        append_authority,
        commit_id,
        branch_id,
    } = input;
    commit_log.begin_phase(CommitPhase::DurableAppend);
    let phase_started = std::time::Instant::now();
    commit_log.record_durable_append_prepared(commit_id, &branch_id.0, patch_position);
    append_durable_commit(runtime, append_authority, canonical_commit_envelope)
        .map_err(|error| attach_rejection(commit_log, CommitPhase::DurableAppend, error))?;
    commit_log.complete_phase(CommitPhase::DurableAppend);
    phase_timing.durable_append_micros = elapsed_micros(phase_started);
    Ok(())
}

pub(super) struct FinalizePublicationPhaseInput<'a> {
    pub(super) commit_log: &'a mut CommitLog,
    pub(super) phase_timing: &'a mut CommitPhaseTiming,
    pub(super) working_state: crate::storage::overlay::WorkingState,
    pub(super) record_allocations: crate::runtime::PendingRecordAllocations,
    pub(super) selected_branch_state: &'a crate::branch::SelectedRelationalBranchState,
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
        record_allocations,
        selected_branch_state,
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
    let (artifacts, changed_records, canonical_commit_envelope, adjacency_deltas, lineage_nodes) =
        publication.into_finalize().into_parts();
    let canonical_commit_envelope = Arc::new(canonical_commit_envelope);
    let mut working_state = working_state;
    apply_adjacency_deltas(&mut working_state, &adjacency_deltas);
    let clone_mode = working_state.clone_mode();
    let committed_partitions = working_state.into_partition_commits().1;
    runtime
        .history
        .validate_branch_root_capture(committed_partitions.len())
        .map_err(|denial| {
            TransactionCommitError::publication(PublicationError::new(
                PublicationStage::BundleAssembly,
                format!("branch-root capture preflight denied: {denial:?}"),
            ))
            .with_commit_log(commit_log.clone())
        })?;
    let published_partition_delta =
        crate::storage::RelationalPublishedPartitionDelta::from_committed_partitions(
            &committed_partitions,
        );
    let prepared_history = runtime
        .mvcc_publication_authority()
        .prepare_commit(
            commit_id,
            commit_reference.clone(),
            branch_binding,
            selected_branch_state,
            published_partition_delta,
            canonical_commit_envelope.patch.position,
            Arc::clone(&canonical_commit_envelope),
        )
        .map_err(|detail| {
            TransactionCommitError::publication(PublicationError::new(
                PublicationStage::BundleAssembly,
                detail,
            ))
            .with_commit_log(commit_log.clone())
        })?;
    let append_authority = crate::durability::authority::DurableAppendAuthority::from_commit(
        super::CommitDurableAppendAdmission::new(runtime, commit_id, branch_id),
    );
    let validated_lineage_events = runtime
        .lineage
        .validate_live_event_batch(canonical_commit_envelope.lineage_events())
        .map_err(|detail| {
            TransactionCommitError::publication(PublicationError::new(
                PublicationStage::BundleAssembly,
                detail,
            ))
            .with_commit_log(commit_log.clone())
        })?;
    append_durable_commit_phase(
        runtime,
        DurableAppendPhaseInput {
            commit_log,
            phase_timing,
            canonical_commit_envelope: canonical_commit_envelope.as_ref(),
            patch_position: canonical_commit_envelope.patch.position,
            append_authority,
            commit_id,
            branch_id,
        },
    )?;
    runtime.lineage_authority().install_published_lineage(
        validated_lineage_events,
        commit_id,
        lineage_nodes,
    );
    commit_log.begin_phase(CommitPhase::Publication);
    let phase_started = std::time::Instant::now();
    let mut publication_phase_timing =
        crate::authority::commit::phases::finalize::PublicationPhaseTiming::default();
    record_allocations.commit();
    finalize_published_commit(
        runtime,
        FinalizationInput {
            clone_mode,
            committed_partitions,
            prepared_history,
            changed_records: &changed_records,
            version_id,
            previous_branch_head_version,
            commit_id,
            commit_reference,
            branch_id,
            merge_base_commits,
            artifacts,
            merge_parent_branches,
            phase_timing: &mut publication_phase_timing,
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

    Ok(FinalizedPublicationPhase {
        canonical_commit_envelope,
        changed_records,
    })
}
