use serde_json::json;

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
use crate::storage::logic::state::{PartitionAccess, PublicationArtifacts};

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

        let mut envelopes = self
            .history
            .commit_envelopes
            .values()
            .map(|envelope| &envelope.patch)
            .collect::<Vec<_>>();
        envelopes.sort_by_key(|patch| patch.position);

        let latest_position = envelopes.last().map(|patch| patch.position);
        let latest_commit_id = self.latest_commit().map(|commit| commit.commit_id);

        if let Some(after_position) = request.after_position {
            if !envelopes
                .iter()
                .any(|patch| patch.position == after_position)
            {
                return Err(PatchStreamReadError {
                    class: PatchStreamReadErrorClass::UnknownResumePosition,
                    detail: format!("unknown patch stream resume position {}", after_position.0),
                });
            }
        }

        let patches = envelopes
            .into_iter()
            .filter(|patch| {
                request
                    .after_position
                    .is_none_or(|position| patch.position > position)
            })
            .take(request.max_commits)
            .cloned()
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

    pub(crate) fn push_bounded_diagnostic(
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

    pub(crate) fn assemble_publication_bundle(
        &mut self,
        _staged: &impl PartitionAccess,
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

pub(crate) fn publication_failure_diagnostic(detail: String) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code: crate::diagnostics::data::DiagnosticCode::InvariantViolation,
        message: detail,
        fields: json!({ "execution_point": "snapshot_publication" }),
    }
}
