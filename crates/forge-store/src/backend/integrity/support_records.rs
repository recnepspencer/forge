use crate::failure::{StoreError, StoreErrorKind};

use crate::backend::records::{
    AuthoritativeArtifactFamily, CommitSupportSummaryRecord, LineageSupportRecord,
    SchemaSupportRecord, StoreState,
};

use super::{
    commit_support_summary_artifact_id, lineage_support_artifact_id, schema_support_artifact_id,
};

impl StoreState {
    pub fn verify_commit_support_summary(
        &self,
        summary: &CommitSupportSummaryRecord,
    ) -> Result<(), StoreError> {
        let commit_record = self
            .commit_envelopes
            .get(&summary.commit_id.0)
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "support summary references missing commit {}",
                    summary.commit_id.0
                ))
            })?;
        if summary.branch_id != commit_record.envelope.branch_context {
            return Err(StoreError::new(
                StoreErrorKind::CommitSupportPublicationGap,
                format!(
                    "support summary for commit {} drifted from commit branch context",
                    summary.commit_id.0
                ),
            ));
        }

        let expected_schema = commit_record.envelope.schema_transition.is_some()
            || commit_record
                .envelope
                .schema_continuation_descriptor
                .is_some()
            || commit_record
                .envelope
                .schema_reconciliation_descriptor
                .is_some();
        let expected_lineage = !commit_record.envelope.lineage_event_ids().is_empty()
            || !commit_record.envelope.lineage_events().is_empty();

        if summary.emitted_schema_artifact != expected_schema {
            return Err(StoreError::new(
                StoreErrorKind::CommitSupportPublicationGap,
                format!(
                    "schema support summary for commit {} did not match canonical envelope content",
                    summary.commit_id.0
                ),
            ));
        }
        if summary.emitted_lineage_artifact != expected_lineage {
            return Err(StoreError::new(
                StoreErrorKind::CommitSupportPublicationGap,
                format!(
                    "lineage support summary for commit {} did not match canonical envelope content",
                    summary.commit_id.0
                ),
            ));
        }

        if expected_schema {
            let expected_id = schema_support_artifact_id(summary.commit_id);
            if summary.schema_support_artifact_id.as_deref() != Some(expected_id.as_str()) {
                return Err(StoreError::new(
                    StoreErrorKind::CommitSupportPublicationGap,
                    format!(
                        "schema support summary for commit {} did not point at the required schema support artifact",
                        summary.commit_id.0
                    ),
                ));
            }
            let schema = self
                .schema_support_records
                .get(&expected_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CommitSupportPublicationGap,
                        format!(
                            "schema support artifact for commit {} missing while summary claimed it exists",
                            summary.commit_id.0
                        ),
                    )
                })?;
            self.verify_schema_support_record(schema)?;
        } else if summary.schema_support_artifact_id.is_some() {
            return Err(StoreError::new(
                StoreErrorKind::SupportAuthorityTaxonomyViolation,
                format!(
                    "commit {} recorded schema support artifact identity without schema support content",
                    summary.commit_id.0
                ),
            ));
        }

        if expected_lineage {
            let expected_id = lineage_support_artifact_id(summary.commit_id);
            if summary.lineage_support_artifact_id.as_deref() != Some(expected_id.as_str()) {
                return Err(StoreError::new(
                    StoreErrorKind::CommitSupportPublicationGap,
                    format!(
                        "lineage support summary for commit {} did not point at the required lineage support artifact",
                        summary.commit_id.0
                    ),
                ));
            }
            let lineage = self
                .lineage_support_records
                .get(&expected_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CommitSupportPublicationGap,
                        format!(
                            "lineage support artifact for commit {} missing while summary claimed it exists",
                            summary.commit_id.0
                        ),
                    )
                })?;
            self.verify_lineage_support_record(lineage)?;
        } else if summary.lineage_support_artifact_id.is_some() {
            return Err(StoreError::new(
                StoreErrorKind::SupportAuthorityTaxonomyViolation,
                format!(
                    "commit {} recorded lineage support artifact identity without lineage support content",
                    summary.commit_id.0
                ),
            ));
        }

        self.require_digest_record(
            AuthoritativeArtifactFamily::CommitSupportSummary,
            commit_support_summary_artifact_id(summary.commit_id),
            &super::stable_structural_digest(summary)?,
        )?;
        Ok(())
    }

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
            &super::stable_structural_digest(record)?,
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
            &super::stable_structural_digest(record)?,
        )?;
        Ok(())
    }

    pub fn verify_support_record_family(&self) -> Result<(), StoreError> {
        for summary in self.commit_support_summaries.values() {
            self.verify_commit_support_summary(summary)?;
        }

        for record in self.schema_support_records.values() {
            self.verify_schema_support_record(record)?;
        }

        for record in self.lineage_support_records.values() {
            self.verify_lineage_support_record(record)?;
        }

        for commit_record in self.commit_envelopes.values() {
            let commit_id = commit_record.envelope.commit.commit_id;
            let expected_schema = commit_record.envelope.schema_transition.is_some()
                || commit_record
                    .envelope
                    .schema_continuation_descriptor
                    .is_some()
                || commit_record
                    .envelope
                    .schema_reconciliation_descriptor
                    .is_some();
            let expected_lineage = !commit_record.envelope.lineage_event_ids().is_empty()
                || !commit_record.envelope.lineage_events().is_empty();
            let summary = self
                .commit_support_summaries
                .get(&commit_id.0)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CommitSupportPublicationGap,
                        format!("commit {} missing support summary", commit_id.0),
                    )
                })?;
            if summary.emitted_schema_artifact != expected_schema
                || summary.emitted_lineage_artifact != expected_lineage
            {
                return Err(StoreError::new(
                    StoreErrorKind::CommitSupportPublicationGap,
                    format!(
                        "commit {} support summary did not match canonical support expectations",
                        commit_id.0
                    ),
                ));
            }
        }

        Ok(())
    }
}
