use crate::{
    authority::{
        AuthoritativeExportBundle, AuthoritativeExportRestoreRequest,
        EmbeddedCheckpointFetchRequest, PersistedEmbeddedCheckpoint,
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
use forge_relational::facade::history::CommitId;

use super::ForgeStore;

impl ForgeStore {
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
}
