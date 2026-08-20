use super::HistoryAuthority;

impl<'runtime> HistoryAuthority<'runtime> {
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
        self.runtime.history.replace_catalog_from_legacy_for_test();
        true
    }

    pub(crate) fn remove_commit_envelope_preserving_patch_stream_position_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
    ) -> bool {
        let removed = self.runtime.history.commit_envelopes.remove(&commit_id);
        self.runtime.history.replace_catalog_from_legacy_for_test();
        removed.is_some()
    }

    pub(crate) fn tamper_commit_patch_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
        mutate: impl FnOnce(&mut crate::publication::patch::data::PublishedAuthoritativePatchEnvelope),
    ) -> bool {
        let Some(envelope) = self.runtime.history.commit_envelopes.get(&commit_id) else {
            return false;
        };
        let mut replacement = envelope.as_ref().clone();
        mutate(&mut replacement.patch);
        self.runtime
            .history
            .commit_envelopes
            .insert(commit_id, std::sync::Arc::new(replacement));
        self.runtime.history.replace_catalog_from_legacy_for_test();
        true
    }

    pub(crate) fn tamper_commit_envelope_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
        mutate: impl FnOnce(&mut crate::history::data::CanonicalCommitEnvelope),
    ) -> bool {
        let Some(envelope) = self.runtime.history.commit_envelopes.get(&commit_id) else {
            return false;
        };
        let mut replacement = envelope.as_ref().clone();
        if replacement.derived_index_artifacts().is_empty() {
            let generations = self
                .runtime
                .index_access()
                .generations_snapshot()
                .into_iter()
                .filter(|generation| generation.source_commit_id == commit_id)
                .collect::<Vec<_>>();
            replacement.derived_index_artifacts =
                crate::indexes::data::DerivedIndexArtifacts::new(generations);
        }
        mutate(&mut replacement);
        self.runtime
            .history
            .commit_envelopes
            .insert(commit_id, std::sync::Arc::new(replacement));
        self.runtime.history.replace_catalog_from_legacy_for_test();
        true
    }
}
