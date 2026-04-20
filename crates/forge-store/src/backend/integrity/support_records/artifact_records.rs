use crate::failure::{StoreError, StoreErrorKind};

use crate::backend::records::{
    AuthoritativeArtifactFamily, LineageSupportRecord, SchemaSupportRecord, StoreState,
};
use crate::backend::integrity::{
    lineage_support_artifact_id, schema_support_artifact_id, stable_structural_digest,
};

impl StoreState {
    pub fn verify_schema_support_record(
        &self,
        record: &SchemaSupportRecord,
    ) -> Result<(), StoreError> {
        let commit_record = self
            .commit_envelopes
            .get(&record.commit_id.0)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SchemaBoundaryArtifactMissing,
                    format!(
                        "schema support artifact references missing commit {}",
                        record.commit_id.0
                    ),
                )
            })?;
        if record.artifact_id != schema_support_artifact_id(record.commit_id) {
            return Err(StoreError::new(
                StoreErrorKind::SchemaBoundaryBasisMismatch,
                format!(
                    "schema support artifact id for commit {} did not match canonical support identity",
                    record.commit_id.0
                ),
            ));
        }
        if record.branch_id != commit_record.envelope.branch_context {
            return Err(StoreError::new(
                StoreErrorKind::SchemaBoundaryBasisMismatch,
                format!(
                    "schema support artifact for commit {} drifted from canonical branch context",
                    record.commit_id.0
                ),
            ));
        }
        if record.schema_version_id != commit_record.envelope.schema_version {
            return Err(StoreError::new(
                StoreErrorKind::SchemaBoundaryBasisMismatch,
                format!(
                    "schema support artifact for commit {} drifted from canonical schema version",
                    record.commit_id.0
                ),
            ));
        }
        if record.descriptor_semantics_version
            != commit_record.envelope.descriptor_semantics_version
        {
            return Err(StoreError::new(
                StoreErrorKind::SchemaBoundaryVersionUnsupported,
                format!(
                    "schema support artifact for commit {} drifted from canonical descriptor semantics version",
                    record.commit_id.0
                ),
            ));
        }
        if record.schema_transition != commit_record.envelope.schema_transition
            || record.schema_continuation_descriptor
                != commit_record.envelope.schema_continuation_descriptor
            || record.schema_reconciliation_descriptor
                != commit_record.envelope.schema_reconciliation_descriptor
        {
            return Err(StoreError::new(
                StoreErrorKind::SchemaBoundaryBasisMismatch,
                format!(
                    "schema support artifact for commit {} drifted from canonical schema support content",
                    record.commit_id.0
                ),
            ));
        }
        self.require_digest_record(
            AuthoritativeArtifactFamily::SchemaSupportRecord,
            record.artifact_id.clone(),
            &stable_structural_digest(record)?,
        )?;
        Ok(())
    }

    pub fn verify_lineage_support_record(
        &self,
        record: &LineageSupportRecord,
    ) -> Result<(), StoreError> {
        let commit_record = self
            .commit_envelopes
            .get(&record.commit_id.0)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::LineageArtifactMissing,
                    format!(
                        "lineage support artifact references missing commit {}",
                        record.commit_id.0
                    ),
                )
            })?;
        if record.artifact_id != lineage_support_artifact_id(record.commit_id) {
            return Err(StoreError::new(
                StoreErrorKind::LineageArtifactDrift,
                format!(
                    "lineage support artifact id for commit {} did not match canonical support identity",
                    record.commit_id.0
                ),
            ));
        }
        if record.branch_id != commit_record.envelope.branch_context {
            return Err(StoreError::new(
                StoreErrorKind::LineageArtifactDrift,
                format!(
                    "lineage support artifact for commit {} drifted from canonical branch context",
                    record.commit_id.0
                ),
            ));
        }
        if record.lineage_event_ids != commit_record.envelope.lineage_event_ids()
            || record.lineage_events != commit_record.envelope.lineage_events()
            || record.lineage_digest_basis != *commit_record.envelope.lineage_digest_basis()
            || record.event_batch_digest_basis != *commit_record.envelope.event_batch_digest_basis()
            || record.decision_log_digest_basis
                != *commit_record.envelope.decision_log_digest_basis()
            || record.lineage_artifact_counters
                != commit_record.envelope.lineage_artifact_counters()
        {
            return Err(StoreError::new(
                StoreErrorKind::LineageArtifactDrift,
                format!(
                    "lineage support artifact for commit {} drifted from canonical lineage support content",
                    record.commit_id.0
                ),
            ));
        }
        self.require_digest_record(
            AuthoritativeArtifactFamily::LineageSupportRecord,
            record.artifact_id.clone(),
            &stable_structural_digest(record)?,
        )?;
        Ok(())
    }
}
