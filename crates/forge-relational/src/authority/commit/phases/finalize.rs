use crate::authority::commit::publication::finalize_published_commit;
use crate::authority::mutation::{apply_adjacency_deltas, AdjacencyDelta};
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::VersionId;
use crate::publication::data::diff::PatchStreamPosition;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::storage::logic::state::PublicationArtifacts;
use crate::storage::overlay::RelationalDraft;
use crate::transactions::data::RecordRef;

pub(crate) struct FinalizeCommitInput {
    pub(crate) changed_records: Vec<RecordRef>,
    pub(crate) version_id: VersionId,
    pub(crate) previous_branch_head_version: Option<VersionId>,
    pub(crate) commit_id: CommitId,
    pub(crate) commit_reference: CommitReference,
    pub(crate) canonical_commit_envelope: CanonicalCommitEnvelope,
    pub(crate) patch_position: PatchStreamPosition,
    pub(crate) branch_id: BranchId,
    pub(crate) merge_base_commits: Vec<CommitId>,
    pub(crate) artifacts: PublicationArtifacts,
    pub(crate) merge_parent_branches: Vec<BranchId>,
    pub(crate) adjacency_deltas: Vec<AdjacencyDelta>,
}

pub(crate) fn finalize_commit_publication(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    mut draft: RelationalDraft,
    input: FinalizeCommitInput,
) {
    apply_adjacency_deltas(&mut draft, &input.adjacency_deltas);
    finalize_published_commit(
        runtime,
        draft.commit(),
        &input.changed_records,
        input.version_id,
        input.previous_branch_head_version,
        input.commit_id,
        &input.commit_reference,
        input.canonical_commit_envelope,
        input.patch_position,
        input.branch_id,
        input.merge_base_commits,
        input.artifacts,
        input.merge_parent_branches,
    );
}
