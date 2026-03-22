use serde_json::json;
use std::sync::Arc;

use crate::diagnostics::data::{DiagnosticCode, DiagnosticsScope, RelationalDiagnosticsEntry};
use crate::history::data::{BranchCreateError, BranchId, CommitId, CommitReference, VersionNode};
use crate::indexes::data::DerivedIndexGeneration;
use crate::lineage::data::LineageEventRecord;
use crate::logic::runtime::RelationalRuntime;
use crate::publication::data::diff::PatchStreamPosition;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::storage::logic::state::SnapshotState;
use crate::visibility::cache_state::{
    bump_replay_ref, cached_state_for_version, ensure_state, evict_cache_if_needed,
};

pub struct HistoryAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub fn history_authority(&mut self) -> HistoryAuthority<'_> {
        HistoryAuthority::new(self)
    }
}

impl<'runtime> HistoryAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: &BranchId,
    ) -> Result<(), BranchCreateError> {
        if self.runtime.history.branch_heads.contains_key(&new_branch) {
            return Err(BranchCreateError::branch_already_exists());
        }
        let Some(source_head) = self.runtime.history.branch_heads.get(from_branch).cloned() else {
            return Err(BranchCreateError::source_branch_missing());
        };
        self.runtime
            .history
            .branch_heads
            .insert(new_branch, source_head.clone());
        self.runtime
            .visibility_pins()
            .move_branch_head_visibility_residency(
                None,
                source_head.as_ref().map(|head| head.version_id),
            );
        if let Some(source_head) = source_head {
            self.runtime
                .visibility_pins()
                .pin_branch_version(source_head.version_id);
        }
        Ok(())
    }

    pub fn retain_version_for_replay(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        if self
            .runtime
            .visibility
            .increment_replay_retention(version_id)
            .is_some()
        {
            if self
                .runtime
                .config
                .visibility
                .cache_policy
                .protect_replay_retained
            {
                bump_replay_ref(self.runtime, version_id, 1);
            }
            return true;
        }
        if version_id.0 == 0 || version_id.0 > self.runtime.current_version_id().0 {
            return false;
        }
        let state = ensure_state(self.runtime, version_id, false);
        self.runtime.visibility_pins().pin_replay_state(&state);
        if self
            .runtime
            .config
            .visibility
            .cache_policy
            .protect_replay_retained
        {
            bump_replay_ref(self.runtime, version_id, 1);
        }
        self.runtime
            .publication_authority()
            .diagnostic(DiagnosticsScope::Retention)
            .minimal_summary()
            .entries([replay_retention_diagnostic(
                DiagnosticCode::ReplayRetentionPinned,
                "replay retention pinned historical visibility state",
                version_id,
                &state,
            )])
            .emit();
        self.runtime.visibility.insert_replay_retention(
            version_id,
            crate::logic::runtime::ReplayRetentionState { ref_count: 1 },
        );
        true
    }

    pub fn release_version_replay_retention(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        let Some(mut retained) = self.runtime.visibility.take_replay_retention(version_id) else {
            return false;
        };
        if retained.ref_count > 1 {
            retained.ref_count -= 1;
            self.runtime
                .visibility
                .restore_replay_retention(version_id, retained);
            if self
                .runtime
                .config
                .visibility
                .cache_policy
                .protect_replay_retained
            {
                bump_replay_ref(self.runtime, version_id, -1);
            }
            return true;
        }
        let Some(state) = cached_state_for_version(self.runtime, version_id) else {
            return false;
        };
        self.runtime.visibility_pins().unpin_replay_state(&state);
        if self
            .runtime
            .config
            .visibility
            .cache_policy
            .protect_replay_retained
        {
            bump_replay_ref(self.runtime, version_id, -1);
            evict_cache_if_needed(self.runtime);
        }
        self.runtime
            .publication_authority()
            .diagnostic(DiagnosticsScope::Retention)
            .minimal_summary()
            .entries([replay_retention_diagnostic(
                DiagnosticCode::ReplayRetentionReleased,
                "replay retention released historical visibility state",
                version_id,
                &state,
            )])
            .emit();
        true
    }

    pub(crate) fn publish_commit(
        &mut self,
        commit_id: CommitId,
        commit_reference: CommitReference,
        branch_id: BranchId,
        patch_position: PatchStreamPosition,
        canonical_commit_envelope: Arc<CanonicalCommitEnvelope>,
    ) {
        self.runtime.history.advance_commit_sequence();
        self.runtime
            .history
            .branch_heads
            .insert(branch_id, Some(commit_reference.clone()));
        self.runtime.history.commit_graph.insert(
            commit_id,
            VersionNode {
                commit: commit_reference,
            },
        );
        self.runtime
            .history
            .commit_envelopes
            .insert(commit_id, canonical_commit_envelope);
        self.runtime
            .history
            .patch_stream_index
            .insert(patch_position, commit_id);
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

    pub(crate) fn append_lineage_events(
        &mut self,
        commit_id: CommitId,
        events: &[LineageEventRecord],
    ) {
        if let Some(envelope) = self.runtime.history.commit_envelopes.get_mut(&commit_id) {
            Arc::make_mut(envelope).append_lineage_events_canonical(events);
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
    pub(crate) fn tamper_commit_patch_for_test(
        &mut self,
        commit_id: crate::history::data::CommitId,
        mutate: impl FnOnce(&mut crate::publication::data::diff::RelationalPatchRecord),
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

fn replay_retention_diagnostic(
    code: DiagnosticCode,
    message: &str,
    version_id: crate::identity::data::VersionId,
    state: &SnapshotState,
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code,
        message: message.to_string(),
        fields: json!({
            "version_id": version_id.0,
            "pinned_entity_count": state.pinned_entity_count,
            "pinned_relation_count": state.pinned_relation_count,
            "pinned_partition_count": state.pinned_partitions.len(),
        }),
    }
}
