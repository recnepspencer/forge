use crate::failure::StoreError;
use crate::publication::PublicationWriteOutcome;
use crate::recovery::{
    BackupRestoreCompatibilityReport, MaintenanceRecoveryReport, SnapshotMaintenanceRecoveryReport,
};
use crate::snapshot::SnapshotId;
use crate::wal::DurableMutationId;
use crate::{
    CompatibilityManifestSummary, CompatibilityRecoveredManifestIndex, ManifestRecoverySummary,
};
use forge_relational::facade::history::CommitId;

#[cfg(test)]
use super::dispatch_mut;
use super::{dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn classify_durable_publication(
        &self,
        durable_mutation_id: DurableMutationId,
        expected_commit_id: Option<CommitId>,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        dispatch_ref!(self, |backend| backend
            .classify_durable_publication(durable_mutation_id, expected_commit_id))
    }
    pub fn classify_snapshot_publication(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        dispatch_ref!(self, |backend| backend
            .classify_snapshot_publication(snapshot_id))
    }
    pub fn classify_snapshot_maintenance_recovery(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotMaintenanceRecoveryReport, StoreError> {
        dispatch_ref!(self, |backend| backend
            .classify_snapshot_maintenance_recovery(snapshot_id))
    }
    pub fn maintenance_recovery_report(&self) -> Result<MaintenanceRecoveryReport, StoreError> {
        dispatch_ref!(self, |backend| backend.maintenance_recovery_report())
    }
    pub fn support_artifact_recovery_report(&self) -> crate::SupportArtifactRecoveryReport {
        dispatch_ref!(self, |backend| backend.support_artifact_recovery_report())
    }
    pub fn backup_restore_compatibility_report(
        &self,
    ) -> Result<BackupRestoreCompatibilityReport, StoreError> {
        dispatch_ref!(self, |backend| backend
            .backup_restore_compatibility_report())
    }

    pub fn compatibility_manifest_recovery_summary(&self) -> ManifestRecoverySummary {
        dispatch_ref!(self, |backend| backend
            .compatibility_manifest_recovery_summary())
    }

    pub fn recover_compatibility_manifest_index(&self) -> CompatibilityRecoveredManifestIndex {
        dispatch_ref!(self, |backend| backend
            .recover_compatibility_manifest_index())
    }

    pub fn compatibility_manifest_summaries(&self) -> Vec<CompatibilityManifestSummary> {
        dispatch_ref!(self, |backend| backend.compatibility_manifest_summaries())
    }

    #[cfg(test)]
    pub fn remove_compatibility_manifest_record_for_test(
        &mut self,
        family_kind: crate::CompatibilityFamilyKind,
    ) {
        dispatch_mut!(self, |backend| backend
            .remove_compatibility_manifest_record_for_test(family_kind))
    }
}
