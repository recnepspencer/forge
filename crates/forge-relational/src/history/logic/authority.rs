use serde_json::json;

use crate::diagnostics::data::{DiagnosticCode, DiagnosticsScope, RelationalDiagnosticsEntry};
use crate::history::data::{BranchCreateError, BranchId};
use crate::logic::runtime::RelationalRuntime;
use crate::storage::logic::state::SnapshotState;

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
        self.runtime.visibility_pins().move_branch_head_visibility_residency(
            None,
            source_head.as_ref().map(|head| head.version_id),
        );
        if let Some(source_head) = source_head {
            self.runtime.visibility_pins().pin_branch_version(source_head.version_id);
        }
        Ok(())
    }

    pub fn retain_version_for_replay(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        if let Some(retained) = self.runtime.visibility.replay_retention.retained_mut(version_id) {
            retained.ref_count += 1;
            if self.runtime.config.visibility.cache_policy.protect_replay_retained {
                self.runtime.bump_replay_ref(version_id, 1);
            }
            return true;
        }
        if version_id.0 == 0 || version_id.0 > self.runtime.current_version_id().0 {
            return false;
        }
        let state = self.runtime.ensure_visibility_state(version_id, false);
        self.runtime.visibility_pins().pin_replay_state(&state);
        if self.runtime.config.visibility.cache_policy.protect_replay_retained {
            self.runtime.bump_replay_ref(version_id, 1);
        }
        self.runtime
            .diagnostic(DiagnosticsScope::Retention)
            .minimal_summary()
            .entries([replay_retention_diagnostic(
                DiagnosticCode::ReplayRetentionPinned,
                "replay retention pinned historical visibility state",
                version_id,
                &state,
            )])
            .emit();
        self.runtime.visibility.replay_retention.insert_retained(
            version_id,
            crate::logic::runtime::ReplayRetentionState { ref_count: 1 },
        );
        true
    }

    pub fn release_version_replay_retention(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        let Some(mut retained) = self.runtime.visibility.replay_retention.take_retained(version_id)
        else {
            return false;
        };
        if retained.ref_count > 1 {
            retained.ref_count -= 1;
            self.runtime
                .visibility
                .replay_retention
                .insert_retained(version_id, retained);
            if self.runtime.config.visibility.cache_policy.protect_replay_retained {
                self.runtime.bump_replay_ref(version_id, -1);
            }
            return true;
        }
        let Some(state) = self.runtime.visibility_state_for_version(version_id) else {
            return false;
        };
        self.runtime.visibility_pins().unpin_replay_state(&state);
        if self.runtime.config.visibility.cache_policy.protect_replay_retained {
            self.runtime.bump_replay_ref(version_id, -1);
            self.runtime.evict_visibility_cache_if_needed();
        }
        self.runtime
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
