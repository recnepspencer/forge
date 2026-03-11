use serde_json::json;

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::logic::runtime::{RelationalRuntime, VisibilityResidency};
use crate::snapshots::data::SnapshotHandle;

impl RelationalRuntime {
    pub fn inspect_version_read_path(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<RelationalDiagnosticArtifact> {
        if version_id.0 == 0 || version_id.0 > self.current_version_id().0 {
            return None;
        }

        let residency = self.visibility_residency_for_version(version_id);
        let cached = self.visibility_state_for_version(version_id).is_some();
        let recent_window = self.config.visibility_cache_policy.recent_version_window;
        let protected = is_protected(&residency);
        let recent_candidate =
            self.config.visibility_cache_policy.enabled && recent_window > 0 && !protected;

        let mut entries = Vec::new();
        if !cached {
            entries.push(snapshot_miss_entry(false));
        }
        entries.push(snapshot_decision_entry(
            cached,
            protected,
            recent_candidate,
            false,
        ));
        Some(snapshot_read_path_artifact(
            version_id,
            cached,
            recent_candidate,
            recent_window,
            residency,
            entries,
        ))
    }

    pub fn inspect_snapshot_read_path(
        &self,
        handle: &SnapshotHandle,
    ) -> Option<RelationalDiagnosticArtifact> {
        if let Some(binding) = self.snapshots.active.get(&handle.snapshot_id) {
            let residency = self.visibility_residency_for_version(binding.version_id);
            let cached = self
                .visibility_state_for_version(binding.version_id)
                .is_some();
            let recent_window = self.config.visibility_cache_policy.recent_version_window;
            let recent_candidate = !self.config.visibility_cache_policy.protect_active_snapshots
                && self.config.visibility_cache_policy.enabled
                && recent_window > 0
                && !is_protected(&residency);
            let mut entries = Vec::new();
            if !cached {
                entries.push(snapshot_miss_entry(false));
            }
            entries.push(snapshot_decision_entry(
                cached,
                is_protected(&residency),
                recent_candidate,
                false,
            ));
            return Some(snapshot_read_path_artifact(
                binding.version_id,
                cached,
                recent_candidate,
                recent_window,
                residency,
                entries,
            ));
        }

        let version_id = *self.snapshots.published_handles.get(&handle.snapshot_id)?;
        let residency = self.visibility_residency_for_version(version_id);
        let cached = self.visibility_state_for_version(version_id).is_some();
        let recent_window = self.config.visibility_cache_policy.recent_version_window;
        let recent_candidate = self.config.visibility_cache_policy.enabled
            && recent_window > 0
            && !is_protected(&residency);
        let mut entries = vec![RelationalDiagnosticsEntry {
            code: DiagnosticCode::PublishedSnapshotHandleRead,
            message: "snapshot read will resolve through a published handle".to_string(),
            fields: json!({
                "snapshot_id": handle.snapshot_id.0,
                "version_id": version_id.0,
            }),
        }];
        if !cached {
            entries.push(snapshot_miss_entry(true));
        }
        entries.push(snapshot_decision_entry(
            cached,
            is_protected(&residency),
            recent_candidate,
            true,
        ));
        Some(snapshot_read_path_artifact(
            version_id,
            cached,
            recent_candidate,
            recent_window,
            residency,
            entries,
        ))
    }
}

fn snapshot_read_path_artifact(
    version_id: crate::identity::data::VersionId,
    cached: bool,
    recent_candidate: bool,
    recent_window: usize,
    residency: VisibilityResidency,
    mut extra_entries: Vec<RelationalDiagnosticsEntry>,
) -> RelationalDiagnosticArtifact {
    let mut entries = vec![RelationalDiagnosticsEntry {
        code: DiagnosticCode::SnapshotReadPathInspected,
        message: "snapshot/version read path inspected".to_string(),
        fields: json!({
            "version_id": version_id.0,
            "cached_visibility_state": cached,
            "recent_candidate": recent_candidate,
            "recent_window": recent_window,
            "recent_resident": residency.recent_resident,
            "branch_head_refs": residency.branch_head_refs,
            "replay_refs": residency.replay_refs,
            "active_snapshot_refs": residency.active_snapshot_refs,
        }),
    }];
    entries.append(&mut extra_entries);
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::Snapshot,
        kind: DiagnosticsArtifactKind::DetailedTrace,
        determinism: DeterminismExpectation::Required,
        entries,
    }
}

fn snapshot_decision_entry(
    cached: bool,
    protected: bool,
    recent_candidate: bool,
    published_handle: bool,
) -> RelationalDiagnosticsEntry {
    let (code, message) = if cached {
        (
            DiagnosticCode::VisibilityCacheHit,
            "read will reuse cached visibility state",
        )
    } else if protected {
        (
            DiagnosticCode::VisibilityCacheProtectedRead,
            "read will reconstruct and keep a protected visibility state",
        )
    } else if recent_candidate {
        (
            DiagnosticCode::VisibilityCacheRecentAdmissionCandidate,
            "read will reconstruct and may admit visibility state into the recent cache",
        )
    } else {
        (
            DiagnosticCode::VisibilityCacheTransientRead,
            "read will reconstruct transient visibility state without cache residency",
        )
    };
    RelationalDiagnosticsEntry {
        code,
        message: message.to_string(),
        fields: json!({
            "cached_visibility_state": cached,
            "protected_visibility_state": protected,
            "recent_admission_candidate": recent_candidate,
            "published_handle": published_handle,
        }),
    }
}

fn snapshot_miss_entry(published_handle: bool) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code: DiagnosticCode::VisibilityCacheMissReconstructed,
        message: "read will reconstruct visibility state from committed history".to_string(),
        fields: json!({
            "published_handle": published_handle,
        }),
    }
}

fn is_protected(residency: &VisibilityResidency) -> bool {
    residency.branch_head_refs > 0
        || residency.replay_refs > 0
        || residency.active_snapshot_refs > 0
}
