use super::*;
use crate::visibility::snapshot_states::build_visibility_state;

impl<'runtime> VisibilityReadContext<'runtime> {
    pub fn inspect_snapshot(&self, handle: &SnapshotHandle) -> Option<SnapshotInspectionSummary> {
        resolve_snapshot_inspection(self.runtime, handle)
    }

    pub fn read_snapshot(&self, handle: &SnapshotHandle) -> Option<RelationalReadView> {
        let resolved = resolve_snapshot_state(self.runtime, handle)?;
        let mut read_view = read_view_from_snapshot_state(self.runtime, &resolved.state);
        read_view.snapshot = resolved.handle;
        Some(read_view)
    }

    pub(crate) fn read_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> RelationalReadView {
        let state = reconstruct_state(self.runtime, version_id, true).unwrap_or_else(|| {
            build_visibility_state(
                self.runtime,
                version_id,
                crate::snapshots::data::SnapshotId(0),
                crate::snapshots::data::SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
            )
        });
        read_view_from_snapshot_state(self.runtime, &state)
    }

    pub fn query_plan_context(&self, handle: &SnapshotHandle) -> Option<QueryPlanContextId> {
        let snapshot = self.resolved_snapshot_handle(handle)?;
        let (schema_version, descriptor_semantics_version, evidence_basis) =
            self.query_schema_context(snapshot.version_id)?;
        Some(QueryPlanContextId {
            runtime_instance_id: self.runtime.runtime_instance_id(),
            snapshot_id: snapshot.snapshot_id,
            version_id: snapshot.version_id,
            schema_version,
            descriptor_semantics_version,
            evidence_basis,
        })
    }

    pub(super) fn resolved_snapshot_handle(
        &self,
        handle: &SnapshotHandle,
    ) -> Option<SnapshotHandle> {
        resolve_snapshot_handle(self.runtime, handle)
    }

    pub(super) fn query_schema_context(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<(
        crate::schema::data::SchemaVersionId,
        crate::schema::data::DescriptorSemanticsVersion,
        QueryPlanEvidenceBasis,
    )> {
        if let Some(envelope) = self
            .runtime
            .history()
            .commit_envelope_for_version(version_id)
        {
            return Some((
                envelope.schema_version,
                envelope.descriptor_semantics_version,
                QueryPlanEvidenceBasis::CanonicalCommitEnvelope {
                    commit_id: envelope.commit.commit_id,
                },
            ));
        }

        if version_id == self.runtime.current_version_id()
            && self.runtime.history().latest_commit().is_none()
        {
            return Some((
                self.runtime.primary_schema_version_id(),
                runtime_descriptor_semantics_policy().current_write_version(),
                QueryPlanEvidenceBasis::GenesisRuntimeBootstrap,
            ));
        }

        None
    }
}
