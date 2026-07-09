use crate::authority::AuthoritativeExportBundle;
use crate::backend::records::EmbeddedCheckpointRecord;
use crate::compatibility::{
    CompatibilityFamilyKind, CompatibilityManifestSummary, CompatibilityRecoveredManifestIndex,
    ManifestRecoverySummary,
};
use crate::failure::{StoreError, StoreErrorKind};
use crate::publication::{
    classify_durable_publication, classify_snapshot_publication, durable_publication_facts,
    PublicationWriteOutcome,
};
use crate::recovery::{
    build_backup_restore_compatibility_report, build_maintenance_recovery_report,
    build_support_artifact_recovery_report, classify_snapshot_maintenance_recovery,
    BackupRestoreCompatibilityReport, MaintenanceRecoveryReport, SnapshotMaintenanceRecoveryReport,
    SupportArtifactRecoveryReport,
};
use crate::snapshot::SnapshotId;
use crate::wal::DurableMutationId;

use super::{core::verify_durable_barrier, StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    fn publication_write_interference(
        &self,
        commit_id: Option<worth_relational::facade::history::CommitId>,
    ) -> Option<crate::ForegroundIsolationOutcome> {
        let commit_id = commit_id?;
        let branch_id = self
            .state
            .commit_record(commit_id)?
            .envelope
            .branch_context
            .clone();
        Some(self.assess_write_foreground_isolation(&branch_id))
    }

    pub fn classify_durable_publication(
        &self,
        durable_mutation_id: DurableMutationId,
        expected_commit_id: Option<worth_relational::facade::history::CommitId>,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        let facts =
            durable_publication_facts(&self.state, durable_mutation_id, expected_commit_id)?;
        let outcome = classify_durable_publication(self.persistence.durable_media_report(), facts);
        Ok(
            match self.publication_write_interference(expected_commit_id) {
                Some(interference) => outcome.with_foreground_write_isolation(interference),
                None => outcome,
            },
        )
    }

    pub fn classify_snapshot_publication(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        let basis = self
            .state
            .snapshot_basis_records
            .get(&snapshot_id.0)
            .cloned();
        let image = self
            .state
            .snapshot_image_records
            .get(&snapshot_id.0)
            .cloned();
        classify_snapshot_publication(self.persistence.durable_media_report(), basis, image)
    }

    pub fn classify_snapshot_maintenance_recovery(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotMaintenanceRecoveryReport, StoreError> {
        classify_snapshot_maintenance_recovery(
            &self.state,
            snapshot_id,
            self.persistence.durable_media_report(),
        )
    }

    pub fn maintenance_recovery_report(&self) -> Result<MaintenanceRecoveryReport, StoreError> {
        build_maintenance_recovery_report(&self.state, self.persistence.durable_media_report())
    }

    pub fn support_artifact_recovery_report(&self) -> SupportArtifactRecoveryReport {
        build_support_artifact_recovery_report(&self.state)
    }

    pub fn backup_restore_compatibility_report(
        &self,
    ) -> Result<BackupRestoreCompatibilityReport, StoreError> {
        build_backup_restore_compatibility_report(
            &self.state,
            self.persistence.durable_media_report().backend_family(),
        )
    }

    pub fn export_bundle(&self) -> AuthoritativeExportBundle {
        self.state.authoritative_export_bundle()
    }

    pub fn compatibility_manifest_recovery_summary(&self) -> ManifestRecoverySummary {
        self.state.compatibility_manifest_recovery_summary()
    }

    pub fn recover_compatibility_manifest_index(&self) -> CompatibilityRecoveredManifestIndex {
        self.state.recovered_compatibility_manifest_index()
    }

    pub fn compatibility_manifest_summaries(&self) -> Vec<CompatibilityManifestSummary> {
        self.state.compatibility_manifest_summaries()
    }

    pub fn persist_embedded_checkpoint(
        &mut self,
        record: EmbeddedCheckpointRecord,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        self.admit_runtime_write_compatibility(
            CompatibilityFamilyKind::EmbeddedCheckpointAuthority,
            "persist_embedded_checkpoint",
        )?;
        if self
            .state
            .embedded_checkpoint_records
            .contains_key(&record.checkpoint_id)
        {
            return Err(StoreError::new(
                StoreErrorKind::DuplicateArtifactIdentity,
                format!(
                    "embedded checkpoint `{}` already exists in worth-store",
                    record.checkpoint_id
                ),
            ));
        }
        self.state
            .embedded_checkpoint_records
            .insert(record.checkpoint_id.clone(), record.clone());
        if let Err(error) = self.state.verify_integrity() {
            self.state
                .embedded_checkpoint_records
                .remove(&record.checkpoint_id);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state
                    .embedded_checkpoint_records
                    .remove(&record.checkpoint_id);
                return Err(error);
            }
        };
        if let Err(error) = verify_durable_barrier(&mut self.counters, &report) {
            self.state
                .embedded_checkpoint_records
                .remove(&record.checkpoint_id);
            return Err(error);
        }
        self.counters.record_state_delta_apply(1, 1);
        Ok(record)
    }

    pub fn fetch_embedded_checkpoint(
        &self,
        checkpoint_id: &str,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        self.admit_runtime_read_compatibility(
            CompatibilityFamilyKind::EmbeddedCheckpointAuthority,
            "fetch_embedded_checkpoint",
        )?;
        let record = self
            .state
            .embedded_checkpoint_records
            .get(checkpoint_id)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CommitNotFound,
                    format!("embedded checkpoint `{checkpoint_id}` not found"),
                )
            })?;
        let basis_reads = u64::from(record.basis_commit_id.is_some());
        let verification = self.state.verify_embedded_checkpoint_record(&record);
        if matches!(
            verification.as_ref().err().map(StoreError::kind),
            Some(StoreErrorKind::CheckpointShapeViolation)
        ) {
            self.counters.record_checkpoint_shape_reject();
        }
        verification?;
        self.counters.record_embedded_checkpoint_fetch(basis_reads);
        Ok(record)
    }
}
