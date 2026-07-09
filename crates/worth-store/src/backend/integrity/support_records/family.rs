use crate::backend::records::StoreState;
use crate::failure::{StoreError, StoreErrorKind};

impl StoreState {
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
