use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitId};
use crate::logic::runtime::{RelationalReplayRecord, RelationalRuntime, ReplaySchemaVersion};
use crate::publication::data::{PublicationBundle, PublicationStatus};
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::logic::state::PublicationArtifacts;
use serde_json::json;

use super::diagnostics::DiagnosticArtifactBuilder;

pub struct PublicationAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub fn publication_authority(&mut self) -> PublicationAuthority<'_> {
        PublicationAuthority::new(self)
    }
}

impl<'runtime> PublicationAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(crate) fn push_diagnostic_artifact(
        &mut self,
        artifact: crate::diagnostics::data::RelationalDiagnosticArtifact,
    ) {
        self.runtime.publication.diagnostics.push(artifact);
    }

    pub(crate) fn prune_published_snapshot_handles_if_needed(&mut self) {
        let limit = self
            .runtime
            .config
            .publication
            .policy
            .max_published_snapshot_handles
            .max(1);
        while self.runtime.visibility.published_snapshot_handle_count() > limit {
            let Some(oldest_snapshot_id) = self.runtime.visibility.oldest_published_snapshot_id()
            else {
                break;
            };
            self.runtime.visibility.remove_published_handle(oldest_snapshot_id);
        }
    }

    pub(crate) fn push_bounded_diagnostic(
        &mut self,
        scope: crate::diagnostics::data::DiagnosticsScope,
        kind: crate::diagnostics::data::DiagnosticsArtifactKind,
        entries: Vec<crate::diagnostics::data::RelationalDiagnosticsEntry>,
    ) -> crate::diagnostics::data::RelationalDiagnosticArtifact {
        let max_entries = self.runtime.config.diagnostics.profile.max_entries_per_artifact;
        let artifact = crate::diagnostics::data::RelationalDiagnosticArtifact {
            scope,
            kind,
            determinism: crate::diagnostics::data::DeterminismExpectation::Required,
            entries: entries.into_iter().take(max_entries).collect(),
        };
        self.runtime.publication.diagnostics.push(artifact.clone());
        artifact
    }

    pub(crate) fn diagnostic(
        self,
        scope: crate::diagnostics::data::DiagnosticsScope,
    ) -> DiagnosticArtifactBuilder<'runtime> {
        DiagnosticArtifactBuilder::new(self.runtime, scope)
    }

    pub(crate) fn assemble_publication_bundle(
        &mut self,
        commit_reference: crate::history::data::CommitReference,
        version_id: crate::identity::data::VersionId,
        patch: crate::publication::data::diff::RelationalPatchRecord,
        diagnostics_summary: crate::diagnostics::data::RelationalDiagnosticArtifact,
    ) -> PublicationArtifacts {
        let snapshot_id = self.runtime.visibility.allocate_snapshot_id();
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
            schema_registry: self.runtime.config.schema.registry.clone(),
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

    pub(crate) fn publish_artifacts(
        &mut self,
        version_id: crate::identity::data::VersionId,
        artifacts: PublicationArtifacts,
    ) -> PublicationArtifacts {
        self.runtime
            .visibility
            .insert_published_handle(artifacts.snapshot.snapshot_id, version_id);
        self.runtime.publication.latest_bundle = Some(artifacts.bundle.clone());
        self.push_diagnostic_artifact(artifacts.diagnostics_summary.clone());
        self.prune_published_snapshot_handles_if_needed();
        artifacts
    }

    pub(crate) fn emit_commit_publication_diagnostic(
        &mut self,
        commit_id: CommitId,
        snapshot_id: SnapshotId,
        branch_id: BranchId,
        parents: &[CommitId],
        merge_parent_branches: &[BranchId],
        merge_base_commits: &[CommitId],
    ) {
        let publication_code = if parents.len() > 1 {
            DiagnosticCode::MergeCommitPublished
        } else {
            DiagnosticCode::CommitPublished
        };
        let mut entries = Vec::new();
        if parents.len() > 1 {
            entries.push(RelationalDiagnosticsEntry {
                code: DiagnosticCode::MergeBaseResolved,
                message: "merge bases resolved deterministically".to_string(),
                fields: json!({
                    "commit_id": commit_id.0,
                    "merge_base_commit_ids": merge_base_commits.iter().map(|base| base.0).collect::<Vec<_>>(),
                }),
            });
        }
        entries.push(RelationalDiagnosticsEntry {
            code: publication_code,
            message: if parents.len() > 1 {
                "merge commit published coherently".to_string()
            } else {
                "commit published coherently".to_string()
            },
            fields: json!({
                "commit_id": commit_id.0,
                "snapshot_id": snapshot_id.0,
                "branch_id": branch_id.0,
                "parent_commit_ids": parents.iter().map(|parent| parent.0).collect::<Vec<_>>(),
                "merge_parent_branches": merge_parent_branches.iter().map(|branch| branch.0.clone()).collect::<Vec<_>>(),
                "merge_base_commit_ids": merge_base_commits.iter().map(|base| base.0).collect::<Vec<_>>(),
            }),
        });
        self.push_bounded_diagnostic(
            DiagnosticsScope::PatchPublication,
            DiagnosticsArtifactKind::MinimalSummary,
            entries,
        );
    }
}
