use serde_json::json;

use crate::data::diagnostics::{
    DeterminismExpectation, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::data::publication::{PublicationBundle, PublicationStatus};
use crate::data::snapshot::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::logic::runtime::{RelationalReplayRecord, RelationalRuntime, ReplaySchemaVersion};

use super::state::{PublicationArtifacts, SnapshotState, WorkingState};

impl RelationalRuntime {
    pub(super) fn push_diagnostic_artifact(&mut self, artifact: RelationalDiagnosticArtifact) {
        self.diagnostics.push(artifact);
    }

    pub(super) fn push_bounded_diagnostic(
        &mut self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
        entries: Vec<RelationalDiagnosticsEntry>,
    ) -> RelationalDiagnosticArtifact {
        let max_entries = self.config.diagnostics.max_entries_per_artifact;
        let artifact = RelationalDiagnosticArtifact {
            scope,
            kind,
            determinism: DeterminismExpectation::Required,
            entries: entries.into_iter().take(max_entries).collect(),
        };
        self.push_diagnostic_artifact(artifact.clone());
        artifact
    }

    pub(super) fn assemble_publication_bundle(
        &mut self,
        staged: &WorkingState,
        commit_id: crate::data::history::CommitId,
        version_id: crate::data::identity::VersionId,
        patch: crate::data::diff::RelationalPatchRecord,
        diagnostics_summary: RelationalDiagnosticArtifact,
    ) -> PublicationArtifacts {
        let snapshot_id = SnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        let snapshot = SnapshotHandle {
            snapshot_id,
            version_id,
            read_policy: SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        };
        let replay = RelationalReplayRecord {
            schema_version: ReplaySchemaVersion(1),
            commit_id,
            version_id,
            snapshot_id,
            patch: patch.clone(),
            schema_registry: self.config.schema_registry.clone(),
        };
        let commit_reference = crate::data::history::CommitReference {
            commit_id,
            version_id,
            branch_id: self.config.main_branch.clone(),
        };
        let bundle = PublicationBundle {
            commit: commit_reference,
            snapshot: snapshot.clone(),
            diagnostics_summary: diagnostics_summary.clone(),
            patch: patch.clone(),
            replay: replay.clone(),
            status: PublicationStatus::Published,
        };
        let snapshot_state = SnapshotState {
            handle: snapshot.clone(),
            entities: self.live_entities_from_state(staged),
            relations: self.live_relations_from_state(staged),
        };
        self.snapshots.insert(snapshot_id, snapshot_state);
        PublicationArtifacts {
            snapshot,
            diagnostics_summary,
            bundle,
        }
    }
}

pub(super) fn publication_failure_diagnostic(detail: String) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code: crate::data::diagnostics::DiagnosticCode::InvariantViolation,
        message: detail,
        fields: json!({ "execution_point": "snapshot_publication" }),
    }
}
