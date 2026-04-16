use crate::failure::{StoreError, StoreErrorKind};

use crate::backend::records::{
    AuthoritativeArtifactFamily, DurableCursorIdentityRecord, EmbeddedCheckpointRecord, StoreState,
    SubscriberCheckpointRecord,
};

use super::{
    durable_cursor_identity_artifact_id, stable_structural_digest,
    subscriber_checkpoint_artifact_id,
};

impl StoreState {
    pub fn verify_durable_cursor_identity_record(
        &self,
        record: &DurableCursorIdentityRecord,
    ) -> Result<(), StoreError> {
        if record.cursor_id.trim().is_empty()
            || record.subscriber_id.trim().is_empty()
            || record.feed_shape_id.trim().is_empty()
            || record.schema_interpretation_id.trim().is_empty()
        {
            return Err(StoreError::new(
                StoreErrorKind::CursorEquivalenceViolation,
                "durable cursor identity records must declare non-empty cursor, subscriber, feed-shape, and schema-interpretation identities",
            ));
        }
        if record.artifact_id != durable_cursor_identity_artifact_id(&record.cursor_id) {
            return Err(StoreError::new(
                StoreErrorKind::CursorEquivalenceViolation,
                format!(
                    "durable cursor identity `{}` did not match canonical artifact identity",
                    record.cursor_id
                ),
            ));
        }
        let latest_checkpoint_artifact_id =
            subscriber_checkpoint_artifact_id(&record.cursor_id, record.latest_checkpoint_sequence);
        let latest_checkpoint = self
            .subscriber_checkpoint_records
            .get(&latest_checkpoint_artifact_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CursorCheckpointMissing,
                    format!(
                        "durable cursor `{}` missing latest checkpoint sequence {}",
                        record.cursor_id, record.latest_checkpoint_sequence
                    ),
                )
            })?;
        if latest_checkpoint.subscriber_id != record.subscriber_id
            || latest_checkpoint.branch_id != record.branch_id
            || latest_checkpoint.feed_shape_id != record.feed_shape_id
            || latest_checkpoint.schema_interpretation_id != record.schema_interpretation_id
            || latest_checkpoint.cursor_semantics_version != record.cursor_semantics_version
            || latest_checkpoint.basis_commit_id != record.latest_basis_commit_id
            || latest_checkpoint.schema_support_artifact_id
                != record.latest_schema_support_artifact_id
        {
            return Err(StoreError::new(
                StoreErrorKind::CursorEquivalenceViolation,
                format!(
                    "durable cursor identity `{}` drifted from its latest persisted checkpoint",
                    record.cursor_id
                ),
            ));
        }
        self.require_digest_record(
            AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
            record.artifact_id.clone(),
            &stable_structural_digest(record)?,
        )?;
        Ok(())
    }

    pub fn verify_subscriber_checkpoint_record(
        &self,
        record: &SubscriberCheckpointRecord,
    ) -> Result<(), StoreError> {
        if record.cursor_id.trim().is_empty()
            || record.subscriber_id.trim().is_empty()
            || record.feed_shape_id.trim().is_empty()
            || record.schema_interpretation_id.trim().is_empty()
        {
            return Err(StoreError::new(
                StoreErrorKind::CursorEquivalenceViolation,
                "subscriber checkpoint records must declare non-empty cursor, subscriber, feed-shape, and schema-interpretation identities",
            ));
        }
        if record.artifact_id
            != subscriber_checkpoint_artifact_id(&record.cursor_id, record.checkpoint_sequence)
        {
            return Err(StoreError::new(
                StoreErrorKind::CursorCheckpointMissing,
                format!(
                    "subscriber checkpoint for cursor `{}` sequence {} did not match canonical artifact identity",
                    record.cursor_id, record.checkpoint_sequence
                ),
            ));
        }
        let identity = self
            .durable_cursor_identity_records
            .get(&durable_cursor_identity_artifact_id(&record.cursor_id))
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CursorCheckpointMissing,
                    format!(
                        "subscriber checkpoint for cursor `{}` is missing its durable identity record",
                        record.cursor_id
                    ),
                )
            })?;
        if identity.subscriber_id != record.subscriber_id
            || identity.branch_id != record.branch_id
            || identity.feed_shape_id != record.feed_shape_id
            || identity.schema_interpretation_id != record.schema_interpretation_id
            || identity.cursor_semantics_version != record.cursor_semantics_version
        {
            return Err(StoreError::new(
                StoreErrorKind::CursorEquivalenceViolation,
                format!(
                    "subscriber checkpoint for cursor `{}` drifted from its durable cursor identity basis",
                    record.cursor_id
                ),
            ));
        }
        let commit_record = self.commit_record(record.basis_commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::CursorBasisMismatch,
                format!(
                    "subscriber checkpoint for cursor `{}` references missing basis commit {}",
                    record.cursor_id, record.basis_commit_id.0
                ),
            )
        })?;
        if commit_record.envelope.branch_context != record.branch_id {
            return Err(StoreError::new(
                StoreErrorKind::CursorBasisMismatch,
                format!(
                    "subscriber checkpoint for cursor `{}` references commit {} on a different branch",
                    record.cursor_id, record.basis_commit_id.0
                ),
            ));
        }
        if let Some(schema_support_artifact_id) = &record.schema_support_artifact_id {
            let schema_support = self
                .schema_support_records
                .get(schema_support_artifact_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CursorSchemaBasisMismatch,
                        format!(
                            "subscriber checkpoint for cursor `{}` references missing schema support artifact `{schema_support_artifact_id}`",
                            record.cursor_id
                        ),
                    )
                })?;
            if schema_support.artifact_id != schema_support_artifact_id.as_str()
                || schema_support.branch_id != record.branch_id
            {
                return Err(StoreError::new(
                    StoreErrorKind::CursorSchemaBasisMismatch,
                    format!(
                        "subscriber checkpoint for cursor `{}` drifted from schema support artifact `{schema_support_artifact_id}`",
                        record.cursor_id
                    ),
                ));
            }
        }
        self.require_digest_record(
            AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
            record.artifact_id.clone(),
            &stable_structural_digest(record)?,
        )?;
        Ok(())
    }

    pub fn verify_embedded_checkpoint_record(
        &self,
        record: &EmbeddedCheckpointRecord,
    ) -> Result<(), StoreError> {
        let has_branch = record.basis_branch_id.is_some();
        let has_commit = record.basis_commit_id.is_some();
        if has_branch != has_commit {
            return Err(StoreError::new(
                StoreErrorKind::CheckpointShapeViolation,
                format!(
                    "embedded checkpoint `{}` must declare branch and commit basis together or omit both",
                    record.checkpoint_id
                ),
            ));
        }
        if let (Some(branch_id), Some(commit_id)) =
            (&record.basis_branch_id, record.basis_commit_id)
        {
            let commit_record = self.commit_record(commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CheckpointBasisMissing,
                    format!(
                        "embedded checkpoint `{}` references missing basis commit {}",
                        record.checkpoint_id, commit_id.0
                    ),
                )
            })?;
            if commit_record.envelope.branch_context != *branch_id {
                return Err(StoreError::new(
                    StoreErrorKind::CheckpointBasisMissing,
                    format!(
                        "embedded checkpoint `{}` basis commit {} belongs to a different branch than `{}`",
                        record.checkpoint_id, commit_id.0, branch_id.0
                    ),
                ));
            }
        }
        for contained_commit_id in &record.contained_commit_ids {
            if self.commit_record(*contained_commit_id).is_none() {
                return Err(StoreError::new(
                    StoreErrorKind::CheckpointContainedCommitMissing,
                    format!(
                        "embedded checkpoint `{}` references missing contained commit {}",
                        record.checkpoint_id, contained_commit_id.0
                    ),
                ));
            }
        }
        Ok(())
    }

    pub fn verify_cursor_record_family(&self) -> Result<(), StoreError> {
        for record in self.durable_cursor_identity_records.values() {
            self.verify_durable_cursor_identity_record(record)?;
        }

        for record in self.subscriber_checkpoint_records.values() {
            self.verify_subscriber_checkpoint_record(record)?;
        }

        for identity in self.durable_cursor_identity_records.values() {
            let matching_sequences = self
                .subscriber_checkpoint_records
                .values()
                .filter(|record| record.cursor_id == identity.cursor_id)
                .map(|record| record.checkpoint_sequence)
                .collect::<Vec<_>>();
            if matching_sequences.is_empty() {
                return Err(StoreError::new(
                    StoreErrorKind::CursorCheckpointMissing,
                    format!(
                        "durable cursor `{}` has no subscriber checkpoints",
                        identity.cursor_id
                    ),
                ));
            }
            let mut ordered = matching_sequences;
            ordered.sort_unstable();
            for (expected, actual) in (1_u64..).zip(ordered.iter().copied()) {
                if actual != expected {
                    return Err(StoreError::new(
                        StoreErrorKind::CursorCheckpointMissing,
                        format!(
                            "durable cursor `{}` is missing checkpoint sequence {}",
                            identity.cursor_id, expected
                        ),
                    ));
                }
                if expected == identity.latest_checkpoint_sequence {
                    break;
                }
            }
        }

        for record in self.embedded_checkpoint_records.values() {
            self.verify_embedded_checkpoint_record(record)?;
        }

        Ok(())
    }
}
