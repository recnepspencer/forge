use crate::authority::commit::publication::finalize_published_commit;
use crate::authority::mutation::{apply_adjacency_deltas, AdjacencyDelta};
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::VersionId;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::storage::logic::state::PublicationArtifacts;
use crate::storage::overlay::WorkingState;
use crate::transactions::data::RecordRef;
use std::sync::Arc;

pub(crate) struct FinalizeCommitInput<'a> {
    pub(crate) changed_records: &'a [RecordRef],
    pub(crate) version_id: VersionId,
    pub(crate) previous_branch_head_version: Option<VersionId>,
    pub(crate) commit_id: CommitId,
    pub(crate) commit_reference: &'a CommitReference,
    pub(crate) canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
    pub(crate) branch_id: &'a BranchId,
    pub(crate) merge_base_commits: &'a [CommitId],
    pub(crate) artifacts: PublicationArtifacts,
    pub(crate) merge_parent_branches: &'a [BranchId],
    pub(crate) adjacency_deltas: Vec<AdjacencyDelta>,
}

pub(crate) fn finalize_commit_publication(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    mut working_state: WorkingState,
    input: FinalizeCommitInput<'_>,
) {
    apply_adjacency_deltas(&mut working_state, &input.adjacency_deltas);
    finalize_published_commit(
        runtime,
        working_state.into_partitions(),
        input.changed_records,
        input.version_id,
        input.previous_branch_head_version,
        input.commit_id,
        input.commit_reference,
        input.canonical_commit_envelope,
        input.branch_id,
        input.merge_base_commits,
        input.artifacts,
        input.merge_parent_branches,
    );
}
