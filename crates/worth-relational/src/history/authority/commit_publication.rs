use std::sync::Arc;

use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{BranchId, CommitId, CommitReference, VersionNode};
use crate::indexes::data::DerivedIndexGeneration;
use crate::publication::patch::data::PatchStreamPosition;

use super::HistoryAuthority;

impl<'runtime> HistoryAuthority<'runtime> {
    pub(crate) fn publish_commit(
        &mut self,
        commit_id: CommitId,
        commit_reference: CommitReference,
        branch_id: BranchId,
        patch_position: PatchStreamPosition,
        canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
    ) {
        self.runtime.history.advance_commit_sequence();
        insert_published_commit(
            &mut self.runtime.history,
            commit_id,
            commit_reference,
            branch_id,
            patch_position,
            canonical_commit_envelope,
        );
    }

    pub(crate) fn publish_metadata_only_commit(
        &mut self,
        commit_id: CommitId,
        commit_reference: CommitReference,
        branch_id: BranchId,
        patch_position: PatchStreamPosition,
        canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
    ) {
        self.runtime.history.advance_metadata_commit_sequence();
        insert_published_commit(
            &mut self.runtime.history,
            commit_id,
            commit_reference,
            branch_id,
            patch_position,
            canonical_commit_envelope,
        );
    }

    pub(crate) fn append_index_generations(
        &mut self,
        commit_id: CommitId,
        generations: &[DerivedIndexGeneration],
    ) {
        if let Some(envelope) = self.runtime.history.commit_envelopes.get_mut(&commit_id) {
            Arc::make_mut(envelope).append_index_generations_canonical(generations);
        }
    }

    #[cfg(test)]
    pub(crate) fn remove_commit_envelope_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> bool {
        let Some(envelope) = self.runtime.history.commit_envelopes.remove(&commit_id) else {
            return false;
        };
        self.runtime
            .history
            .patch_stream_index
            .remove(&envelope.patch.position);
        true
    }

    #[cfg(test)]
    pub(crate) fn remove_commit_envelope_preserving_patch_stream_position_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> bool {
        self.runtime
            .history
            .commit_envelopes
            .remove(&commit_id)
            .is_some()
    }

    #[cfg(test)]
    pub(crate) fn tamper_commit_patch_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
        mutate: impl FnOnce(&mut crate::publication::patch::data::PublishedAuthoritativePatchEnvelope),
    ) -> bool {
        let Some(envelope) = self.runtime.history.commit_envelopes.get_mut(&commit_id) else {
            return false;
        };
        mutate(&mut Arc::make_mut(envelope).patch);
        true
    }

    #[cfg(test)]
    pub(crate) fn tamper_commit_envelope_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
        mutate: impl FnOnce(&mut CanonicalCommitEnvelope),
    ) -> bool {
        let Some(envelope) = self.runtime.history.commit_envelopes.get_mut(&commit_id) else {
            return false;
        };
        mutate(Arc::make_mut(envelope));
        true
    }
}

fn insert_published_commit(
    history: &mut crate::runtime::HistorySubsystem,
    commit_id: CommitId,
    commit_reference: CommitReference,
    branch_id: BranchId,
    patch_position: PatchStreamPosition,
    canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
) {
    history
        .branch_heads
        .insert(branch_id, Some(commit_reference.clone()));
    history.commit_graph.insert(
        commit_id,
        VersionNode {
            commit: commit_reference,
        },
    );
    history
        .commit_envelopes
        .insert(commit_id, canonical_commit_envelope);
    history.patch_stream_index.insert(patch_position, commit_id);
}
