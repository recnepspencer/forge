use crate::{
    authority::{
        AuthoritativeExportBundle, AuthoritativeExportRestoreRequest,
        EmbeddedCheckpointFetchRequest, PersistedEmbeddedCheckpoint,
    },
    compatibility::{
        CompatibilityManifestSummary, CompatibilityRecoveredManifestIndex, ManifestRecoverySummary,
    },
    evidence::{OperatingModeLane, PersistedModeLaneEvidence, StoreCounterSnapshot},
    failure::StoreError,
    media::DurableMediaReport,
    publication::PublicationWriteOutcome,
    recovery::{
        BackupRestoreCompatibilityReport, MaintenanceRecoveryReport,
        SnapshotMaintenanceRecoveryReport,
    },
    snapshot::SnapshotId,
    wal::DurableMutationId,
};
use worth_relational::facade::history::CommitId;

use super::WORTHStore;

impl WORTHStore {
    pub fn execute_compatibility_authoritative_adapter(
        &self,
        request: crate::CompatibilityAuthoritativeAdapterRequest,
    ) -> Result<crate::CompatibilityAuthoritativeAdapterOutcome, StoreError> {
        self.backend
            .execute_compatibility_authoritative_adapter(request)
    }

    pub fn fetch_embedded_checkpoint(
        &self,
        request: EmbeddedCheckpointFetchRequest,
    ) -> Result<PersistedEmbeddedCheckpoint, StoreError> {
        Ok(PersistedEmbeddedCheckpoint::new(
            self.backend
                .fetch_embedded_checkpoint(request.checkpoint_id())?,
        ))
    }

    pub fn counters(&self) -> StoreCounterSnapshot {
        self.backend.counter_snapshot()
    }

    pub fn export_authoritative_records(&self) -> AuthoritativeExportBundle {
        self.backend.export_bundle()
    }

    pub fn durable_media_report(&self) -> DurableMediaReport {
        self.backend.durable_media_report()
    }

    pub fn milestone_7_access_structure_verification(
        &self,
    ) -> crate::Milestone7AccessStructureVerification {
        self.backend.milestone_7_access_structure_verification()
    }

    pub fn milestone_6_access_structure_verification(
        &self,
    ) -> crate::Milestone6AccessStructureVerification {
        self.backend.milestone_6_access_structure_verification()
    }

    pub fn durable_publication_report(
        &self,
        durable_mutation_id: DurableMutationId,
        expected_commit_id: Option<CommitId>,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        self.backend
            .classify_durable_publication(durable_mutation_id, expected_commit_id)
    }

    pub fn snapshot_publication_report(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        self.backend.classify_snapshot_publication(snapshot_id)
    }

    pub fn snapshot_maintenance_recovery_report(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotMaintenanceRecoveryReport, StoreError> {
        self.backend
            .classify_snapshot_maintenance_recovery(snapshot_id)
    }

    pub fn maintenance_recovery_report(&self) -> Result<MaintenanceRecoveryReport, StoreError> {
        self.backend.maintenance_recovery_report()
    }

    pub fn support_artifact_recovery_report(&self) -> crate::SupportArtifactRecoveryReport {
        self.backend.support_artifact_recovery_report()
    }

    pub(crate) fn record_support_artifact_recovery_gap(&self, count: u64) {
        self.backend.record_support_artifact_recovery_gap(count);
    }

    pub fn backup_restore_compatibility_report(
        &self,
    ) -> Result<BackupRestoreCompatibilityReport, StoreError> {
        self.backend.backup_restore_compatibility_report()
    }

    pub fn compatibility_manifest_recovery_summary(&self) -> ManifestRecoverySummary {
        self.backend.compatibility_manifest_recovery_summary()
    }

    pub fn recover_compatibility_manifest_index(&self) -> CompatibilityRecoveredManifestIndex {
        self.backend.recover_compatibility_manifest_index()
    }

    pub fn compatibility_manifest_summaries(&self) -> Vec<CompatibilityManifestSummary> {
        self.backend.compatibility_manifest_summaries()
    }

    pub(crate) fn milestone_2_lane_evidence(
        &self,
        lane: OperatingModeLane,
    ) -> PersistedModeLaneEvidence {
        let export = self.export_authoritative_records();
        PersistedModeLaneEvidence::from_export(lane, &export, self.counters())
    }

    pub fn restore_from_authoritative_export(
        request: AuthoritativeExportRestoreRequest,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            backend: crate::backend::StoreBackend::from_export_bundle(request.into_bundle())?,
        })
    }

    pub fn restore_from_authoritative_export_with_compatibility(
        request: AuthoritativeExportRestoreRequest,
    ) -> Result<(Self, crate::CompatibilityRestoreExecutionOutcome), StoreError> {
        let (backend, outcome) =
            crate::backend::StoreBackend::from_export_bundle_with_compatibility(
                request.into_bundle(),
            )?;
        Ok((Self { backend }, outcome))
    }

    #[cfg(test)]
    pub(crate) fn remove_compatibility_manifest_record_for_test(
        &mut self,
        family_kind: crate::CompatibilityFamilyKind,
    ) {
        self.backend
            .remove_compatibility_manifest_record_for_test(family_kind);
    }

    #[cfg(test)]
    pub(crate) fn execute_restore_from_authoritative_export_with_conflicts_for_test(
        request: AuthoritativeExportRestoreRequest,
        conflicts: std::collections::BTreeMap<
            crate::CompatibilityFamilyKind,
            crate::RestorePublicationConflictSet,
        >,
    ) -> Result<(usize, u64, u64), StoreError> {
        let execution = crate::backend::engine::compatibility_runtime::execute_authoritative_export_restore_with_conflicts(
            &request.into_bundle(),
            &conflicts,
        )?;
        Ok((
            execution.receipts().len(),
            execution.counters().restore_accept_count(),
            execution.counters().restore_rejection_count(),
        ))
    }
}
