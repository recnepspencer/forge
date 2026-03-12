mod access;

use serde_json::Value;

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::logic::runtime::{RelationalReplayRecord, RelationalRuntime, ReplaySchemaVersion};
use crate::publication::data::{PublicationBundle, PublicationStatus};
use crate::snapshots::data::{SnapshotHandle, SnapshotReadPolicy};
use crate::storage::logic::state::PublicationArtifacts;

pub use access::PublicationAccess;
pub(crate) use access::publication_failure_diagnostic;

impl RelationalRuntime {
    pub fn publication_access(&self) -> PublicationAccess<'_> {
        PublicationAccess::new(self)
    }

    pub(crate) fn push_diagnostic_artifact(&mut self, artifact: RelationalDiagnosticArtifact) {
        self.publication.diagnostics.push(artifact);
    }

    pub(crate) fn prune_published_snapshot_handles_if_needed(&mut self) {
        let limit = self
            .config
            .publication
            .policy
            .max_published_snapshot_handles
            .max(1);
        while self.visibility.published_snapshot_handle_count() > limit {
            let Some(oldest_snapshot_id) = self.visibility.oldest_published_snapshot_id() else {
                break;
            };
            self.visibility.remove_published_handle(oldest_snapshot_id);
        }
    }

    pub(crate) fn push_bounded_diagnostic(
        &mut self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
        entries: Vec<RelationalDiagnosticsEntry>,
    ) -> RelationalDiagnosticArtifact {
        self.diagnostic(scope).kind(kind).entries(entries).emit()
    }

    pub(crate) fn diagnostic(&mut self, scope: DiagnosticsScope) -> DiagnosticArtifactBuilder<'_> {
        DiagnosticArtifactBuilder::new(self, scope)
    }

    pub(crate) fn assemble_publication_bundle(
        &mut self,
        commit_reference: crate::history::data::CommitReference,
        version_id: crate::identity::data::VersionId,
        patch: crate::publication::data::diff::RelationalPatchRecord,
        diagnostics_summary: RelationalDiagnosticArtifact,
    ) -> PublicationArtifacts {
        let snapshot_id = self.visibility.allocate_snapshot_id();
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
            schema_registry: self.config.schema.registry.clone(),
        };
        let bundle = PublicationBundle {
            commit: commit_reference,
            snapshot: snapshot.clone(),
            diagnostics_summary: diagnostics_summary.clone(),
            patch: patch.clone(),
            replay: replay.clone(),
            status: PublicationStatus::Published,
        };
        PublicationArtifacts {
            snapshot,
            diagnostics_summary,
            bundle,
        }
    }
}

pub(crate) struct DiagnosticArtifactBuilder<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
    scope: DiagnosticsScope,
    kind: DiagnosticsArtifactKind,
    entries: Vec<RelationalDiagnosticsEntry>,
}

impl<'runtime> DiagnosticArtifactBuilder<'runtime> {
    fn new(runtime: &'runtime mut RelationalRuntime, scope: DiagnosticsScope) -> Self {
        Self {
            runtime,
            scope,
            kind: DiagnosticsArtifactKind::MinimalSummary,
            entries: Vec::new(),
        }
    }

    pub(crate) fn kind(mut self, kind: DiagnosticsArtifactKind) -> Self {
        self.kind = kind;
        self
    }

    pub(crate) fn minimal_summary(self) -> Self {
        self.kind(DiagnosticsArtifactKind::MinimalSummary)
    }

    pub(crate) fn failure(self) -> Self {
        self.kind(DiagnosticsArtifactKind::Failure)
    }

    pub(crate) fn rollback(self) -> Self {
        self.kind(DiagnosticsArtifactKind::Rollback)
    }

    pub(crate) fn comparison(self) -> Self {
        self.kind(DiagnosticsArtifactKind::Comparison)
    }

    pub(crate) fn entry(
        mut self,
        code: crate::diagnostics::data::DiagnosticCode,
        message: impl Into<String>,
        fields: Value,
    ) -> Self {
        self.entries.push(RelationalDiagnosticsEntry {
            code,
            message: message.into(),
            fields,
        });
        self
    }

    pub(crate) fn entries(
        mut self,
        entries: impl IntoIterator<Item = RelationalDiagnosticsEntry>,
    ) -> Self {
        self.entries.extend(entries);
        self
    }

    pub(crate) fn emit_entry(
        self,
        code: crate::diagnostics::data::DiagnosticCode,
        message: impl Into<String>,
        fields: Value,
    ) -> RelationalDiagnosticArtifact {
        self.entry(code, message, fields).emit()
    }

    pub(crate) fn emit(self) -> RelationalDiagnosticArtifact {
        let max_entries = self.runtime.config.diagnostics.profile.max_entries_per_artifact;
        let artifact = RelationalDiagnosticArtifact {
            scope: self.scope,
            kind: self.kind,
            determinism: DeterminismExpectation::Required,
            entries: self.entries.into_iter().take(max_entries).collect(),
        };
        self.runtime.push_diagnostic_artifact(artifact.clone());
        artifact
    }
}
