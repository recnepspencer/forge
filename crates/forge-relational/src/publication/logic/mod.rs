use serde_json::{json, Value};

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry, RelationalDiagnosticsFacade,
};
use crate::logic::runtime::{RelationalReplayRecord, RelationalRuntime, ReplaySchemaVersion};
use crate::publication::data::diff::{
    PatchStreamBatch, PatchStreamReadError, PatchStreamReadErrorClass, PatchStreamRequest,
};
use crate::publication::data::{PublicationBundle, PublicationStatus};
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::logic::state::PublicationArtifacts;

impl RelationalRuntime {
    pub fn diagnostics(&self) -> RelationalDiagnosticsFacade {
        RelationalDiagnosticsFacade {
            artifacts: self.publication.diagnostics.clone(),
        }
    }

    pub fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>> {
        self.publication.latest_bundle.as_ref()
    }

    pub fn latest_patch(&self) -> Option<&crate::publication::data::diff::RelationalPatchRecord> {
        self.publication
            .latest_bundle
            .as_ref()
            .map(|bundle| &bundle.patch)
    }

    pub fn latest_replay(&self) -> Option<&RelationalReplayRecord> {
        self.publication
            .latest_bundle
            .as_ref()
            .map(|bundle| &bundle.replay)
    }

    pub fn read_patch_stream(
        &self,
        request: PatchStreamRequest,
    ) -> Result<PatchStreamBatch, PatchStreamReadError> {
        if request.max_commits == 0 {
            return Err(PatchStreamReadError {
                class: PatchStreamReadErrorClass::InvalidBatchSize,
                detail: "patch stream request must ask for at least one commit".to_string(),
            });
        }

        let latest_position = self
            .history
            .patch_stream_index
            .last_key_value()
            .map(|(position, _)| *position);
        let latest_commit_id = self.latest_commit().map(|commit| commit.commit_id);

        if let Some(after_position) = request.after_position {
            if !self
                .history
                .patch_stream_index
                .contains_key(&after_position)
            {
                return Err(PatchStreamReadError {
                    class: PatchStreamReadErrorClass::UnknownResumePosition,
                    detail: format!("unknown patch stream resume position {}", after_position.0),
                });
            }
        }

        let start = request
            .after_position
            .map(|position| std::ops::Bound::Excluded(position))
            .unwrap_or(std::ops::Bound::Unbounded);
        let patches = self
            .history
            .patch_stream_index
            .range((start, std::ops::Bound::Unbounded))
            .filter_map(|(_, commit_id)| self.history.commit_envelopes.get(commit_id))
            .map(|envelope| envelope.patch.clone())
            .take(request.max_commits)
            .collect::<Vec<_>>();

        Ok(PatchStreamBatch {
            resumed_after: request.after_position,
            next_position: patches.last().map(|patch| patch.position),
            latest_position,
            latest_commit_id,
            patches,
        })
    }

    pub(crate) fn push_diagnostic_artifact(&mut self, artifact: RelationalDiagnosticArtifact) {
        self.publication.diagnostics.push(artifact);
    }

    pub(crate) fn prune_published_snapshot_handles_if_needed(&mut self) {
        let limit = self
            .config
            .publication
            .max_published_snapshot_handles
            .max(1);
        while self.snapshots.published_handles.len() > limit {
            let Some(oldest_snapshot_id) = self.snapshots.published_handles.keys().next().copied()
            else {
                break;
            };
            self.snapshots.published_handles.remove(&oldest_snapshot_id);
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
        let snapshot_id = SnapshotId(self.snapshots.next_snapshot_id);
        self.snapshots.next_snapshot_id += 1;
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
        let max_entries = self.runtime.config.diagnostics.max_entries_per_artifact;
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

pub(crate) fn publication_failure_diagnostic(detail: String) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code: crate::diagnostics::data::DiagnosticCode::InvariantViolation,
        message: detail,
        fields: json!({ "execution_point": "snapshot_publication" }),
    }
}
