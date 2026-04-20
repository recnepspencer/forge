use crate::backend::records::{
    AuthoritativeArtifactFamily, CommitSupportSummaryRecord, StoreState,
};
use crate::backend::integrity::{
    commit_support_summary_artifact_id, lineage_support_artifact_id, schema_support_artifact_id,
    stable_structural_digest,
};
use crate::failure::{StoreError, StoreErrorKind};

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
            || commit_record.envelope.schema_continuation_descriptor.is_some()
            || commit_record.envelope.schema_reconciliation_descriptor.is_some();
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

        let mut expected_layout_request_ids = self
            .milestone_6_commit_coupled_layout_seed_records
            .values()
            .filter(|record| record.authority_basis_commit_id == summary.commit_id)
            .map(|record| record.artifact_id.clone())
            .collect::<Vec<_>>();
        expected_layout_request_ids.sort();
        let has_persisted_milestone_6_layout_state =
            !self.milestone_6_layout_materialization_records.is_empty()
                || !self.milestone_6_scope_slice_membership_records.is_empty()
                || !self.milestone_6_chunk_membership_records.is_empty()
                || !self.milestone_6_structural_block_records.is_empty();
        if summary.milestone_6_published_layout_request_artifact_ids != expected_layout_request_ids
            && (has_persisted_milestone_6_layout_state || !expected_layout_request_ids.is_empty())
        {
            return Err(StoreError::new(
                StoreErrorKind::CommitSupportPublicationGap,
                format!(
                    "milestone 6 support summary for commit {} did not match the commit-coupled layout seed set",
                    summary.commit_id.0
                ),
            ));
        }
        for artifact_id in &summary.milestone_6_published_layout_request_artifact_ids {
            if !has_persisted_milestone_6_layout_state && expected_layout_request_ids.is_empty() {
                break;
            }
            let record = self
                .milestone_6_commit_coupled_layout_seed_records
                .get(artifact_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CommitSupportPublicationGap,
                        format!(
                            "commit {} summary referenced missing milestone 6 commit-coupled layout seed `{artifact_id}`",
                            summary.commit_id.0
                        ),
                    )
                })?;
            let expected_artifact_id =
                crate::layout::published_layout_request_artifact_id(&record.request)?;
            if artifact_id != &expected_artifact_id {
                return Err(StoreError::new(
                    StoreErrorKind::CommitSupportPublicationGap,
                    format!(
                        "commit {} summary referenced non-canonical milestone 6 commit-coupled layout seed `{artifact_id}`",
                        summary.commit_id.0
                    ),
                ));
            }
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
            let schema = self.schema_support_records.get(&expected_id).ok_or_else(|| {
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
            let lineage = self.lineage_support_records.get(&expected_id).ok_or_else(|| {
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
            &stable_structural_digest(summary)?,
        )?;
        Ok(())
    }
}
