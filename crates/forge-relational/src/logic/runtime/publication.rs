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
        commit_reference: crate::data::history::CommitReference,
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
            commit_id: commit_reference.commit_id,
            version_id,
            snapshot_id,
            patch: patch.clone(),
            schema_registry: self.config.schema_registry.clone(),
        };
        let bundle = PublicationBundle {
            commit: commit_reference,
            snapshot: snapshot.clone(),
            diagnostics_summary: diagnostics_summary.clone(),
            patch: patch.clone(),
            replay: replay.clone(),
            status: PublicationStatus::Published,
        };
        let entities = self.visible_entities_from_state(staged, version_id);
        let relations = self.visible_relations_from_state(staged, version_id);
        let pinned_entities = entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>();
        let pinned_relations = relations
            .iter()
            .map(|record| record.relation_id)
            .collect::<Vec<_>>();
        let snapshot_state = SnapshotState {
            handle: snapshot.clone(),
            pinned_entities,
            pinned_relations,
        };
        PublicationArtifacts {
            snapshot,
            snapshot_state,
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
