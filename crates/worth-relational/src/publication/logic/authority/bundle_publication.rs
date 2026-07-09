use crate::history::data::CommitReference;
use crate::identity::data::VersionId;
use crate::logic::runtime::{RelationalReplayRecord, ReplaySchemaVersion};
use crate::publication::bundle::{PublicationBundle, PublicationStatus};
use crate::publication::logic::PublicationAuthority;
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::logic::state::PublicationArtifacts;

impl<'runtime> PublicationAuthority<'runtime> {
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
            let _ = self
                .runtime
                .visibility
                .remove_published_handle(oldest_snapshot_id);
        }
    }

    pub(crate) fn assemble_publication_bundle(
        &mut self,
        commit_reference: CommitReference,
        version_id: VersionId,
        patch: crate::publication::patch::data::PublishedAuthoritativePatchEnvelope,
        diagnostics_summary: crate::diagnostics::data::RelationalDiagnosticArtifact,
    ) -> PublicationArtifacts {
        let snapshot_id = self.runtime.visibility.allocate_snapshot_id();
        let snapshot = SnapshotHandle {
            runtime_instance_id: self.runtime.runtime_instance_id(),
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
            schema_authority: self.runtime.config.schema.registry.authority_snapshot(),
        };
        let bundle = PublicationBundle {
            commit: commit_reference,
            snapshot,
            diagnostics_summary,
            patch,
            replay,
            status: PublicationStatus::Published,
        };
        PublicationArtifacts { bundle }
    }

    pub(crate) fn publish_artifacts(
        &mut self,
        version_id: VersionId,
        artifacts: PublicationArtifacts,
    ) -> SnapshotId {
        let PublicationArtifacts { bundle } = artifacts;
        let snapshot_id = bundle.snapshot.snapshot_id;
        self.runtime.visibility.insert_published_handle(
            snapshot_id,
            crate::logic::runtime::SnapshotHandleBinding {
                version_id,
                read_policy: bundle.snapshot.read_policy,
            },
        );
        self.push_diagnostic_artifact(bundle.diagnostics_summary.clone());
        self.runtime.publication.replace_latest_bundle(bundle);
        self.prune_published_snapshot_handles_if_needed();
        snapshot_id
    }
}
