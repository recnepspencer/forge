use crate::authority::{DurableCursorAcknowledgeRequest, PersistedSubscriberCheckpoint};
use crate::backend::records::{DurableCursorIdentityRecord, SubscriberCheckpointRecord};
use crate::compatibility::CompatibilityFamilyKind;
use crate::failure::{StoreError, StoreErrorKind};

use super::{core::verify_durable_barrier, StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn acknowledge_cursor(
        &mut self,
        request: DurableCursorAcknowledgeRequest,
    ) -> Result<PersistedSubscriberCheckpoint, StoreError> {
        self.admit_runtime_write_compatibility(
            CompatibilityFamilyKind::SchemaLineageCursorCheckpointSupport,
            "acknowledge_cursor",
        )?;
        let cursor_id = request.cursor_id().to_string();
        let cursor_artifact_id =
            super::super::integrity::durable_cursor_identity_artifact_id(&cursor_id);
        self.counters.record_cursor_identity_lookup();

        let basis_commit = self
            .state
            .commit_record(request.basis_commit_id())
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CursorBasisMismatch,
                    format!(
                        "durable cursor `{}` references missing basis commit {}",
                        request.cursor_id(),
                        request.basis_commit_id().0
                    ),
                )
            })?;
        if basis_commit.envelope.branch_context != *request.branch_id() {
            return Err(StoreError::new(
                StoreErrorKind::CursorBasisMismatch,
                format!(
                    "durable cursor `{}` basis commit {} belongs to branch `{}` not `{}`",
                    request.cursor_id(),
                    request.basis_commit_id().0,
                    basis_commit.envelope.branch_context.0,
                    request.branch_id().0
                ),
            ));
        }

        let schema_support_artifact_id = request.schema_support_artifact_id().map(str::to_string);
        if let Some(schema_support_id) = &schema_support_artifact_id {
            let schema_support = self
                .state
                .schema_support_records
                .get(schema_support_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CursorSchemaBasisMismatch,
                        format!(
                            "durable cursor `{}` references missing schema support artifact `{schema_support_id}`",
                            request.cursor_id()
                        ),
                    )
                })?;
            if schema_support.branch_id != *request.branch_id() {
                return Err(StoreError::new(
                    StoreErrorKind::CursorSchemaBasisMismatch,
                    format!(
                        "durable cursor `{}` references schema support artifact `{schema_support_id}` on a different branch",
                        request.cursor_id()
                    ),
                ));
            }
        }
        let previous_identity = self
            .state
            .durable_cursor_identity_records
            .get(&cursor_artifact_id)
            .cloned();
        let next_checkpoint_sequence = if let Some(identity) = &previous_identity {
            if identity.subscriber_id != request.subscriber_id()
                || identity.branch_id != *request.branch_id()
                || identity.feed_shape_id != request.feed_shape_id()
                || identity.schema_interpretation_id != request.schema_interpretation_id()
                || identity.cursor_semantics_version != request.cursor_semantics_version()
            {
                self.counters.record_cursor_equivalence_reject();
                return Err(StoreError::new(
                    StoreErrorKind::CursorEquivalenceViolation,
                    "durable cursor cannot be reused with a different subscriber, branch scope, feed shape, schema interpretation, or semantics version",
                ));
            }
            let latest_basis_commit = self
                .state
                .commit_record(identity.latest_basis_commit_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CursorBasisMismatch,
                        format!(
                            "durable cursor `{}` latest basis commit {} is missing",
                            request.cursor_id(),
                            identity.latest_basis_commit_id.0
                        ),
                    )
                })?;
            if basis_commit.commit_sequence < latest_basis_commit.commit_sequence {
                self.counters.record_cursor_regression_reject();
                return Err(StoreError::new(
                    StoreErrorKind::CursorRegression,
                    format!(
                        "durable cursor `{}` cannot regress from commit {} to commit {}",
                        request.cursor_id(),
                        identity.latest_basis_commit_id.0,
                        request.basis_commit_id().0
                    ),
                ));
            }
            identity.latest_checkpoint_sequence + 1
        } else {
            1
        };

        let checkpoint_artifact_id = super::super::integrity::subscriber_checkpoint_artifact_id(
            &cursor_id,
            next_checkpoint_sequence,
        );
        let checkpoint_record = SubscriberCheckpointRecord {
            artifact_id: checkpoint_artifact_id.clone(),
            cursor_id: cursor_id.clone(),
            subscriber_id: request.subscriber_id().to_string(),
            branch_id: request.branch_id().clone(),
            feed_shape_id: request.feed_shape_id().to_string(),
            schema_interpretation_id: request.schema_interpretation_id().to_string(),
            cursor_semantics_version: request.cursor_semantics_version(),
            checkpoint_sequence: next_checkpoint_sequence,
            basis_commit_id: request.basis_commit_id(),
            schema_support_artifact_id: schema_support_artifact_id.clone(),
        };
        let identity_record = DurableCursorIdentityRecord {
            artifact_id: cursor_artifact_id.clone(),
            cursor_id: cursor_id.clone(),
            subscriber_id: request.subscriber_id().to_string(),
            branch_id: request.branch_id().clone(),
            feed_shape_id: request.feed_shape_id().to_string(),
            schema_interpretation_id: request.schema_interpretation_id().to_string(),
            cursor_semantics_version: request.cursor_semantics_version(),
            latest_checkpoint_sequence: next_checkpoint_sequence,
            latest_basis_commit_id: request.basis_commit_id(),
            latest_schema_support_artifact_id: schema_support_artifact_id,
        };

        let previous_digest = previous_identity
            .as_ref()
            .map(super::super::integrity::stable_structural_digest)
            .transpose()?;
        self.state
            .subscriber_checkpoint_records
            .insert(checkpoint_artifact_id.clone(), checkpoint_record.clone());
        self.state
            .durable_cursor_identity_records
            .insert(cursor_artifact_id.clone(), identity_record.clone());
        self.state.upsert_digest_record(
            super::super::records::AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
            checkpoint_artifact_id.clone(),
            super::super::integrity::stable_structural_digest(&checkpoint_record)?,
        );
        self.state.upsert_digest_record(
            super::super::records::AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
            cursor_artifact_id.clone(),
            super::super::integrity::stable_structural_digest(&identity_record)?,
        );

        if let Err(error) = self.state.verify_cursor_record_family() {
            restore_cursor_state(
                &mut self.state,
                &checkpoint_artifact_id,
                &cursor_artifact_id,
                previous_identity,
                previous_digest,
            )?;
            if matches!(error.kind(), StoreErrorKind::CheckpointShapeViolation) {
                self.counters.record_checkpoint_shape_reject();
            }
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                restore_cursor_state(
                    &mut self.state,
                    &checkpoint_artifact_id,
                    &cursor_artifact_id,
                    previous_identity,
                    previous_digest,
                )?;
                return Err(error);
            }
        };
        verify_durable_barrier(&mut self.counters, &report)?;
        self.counters.record_state_delta_apply(2, 2);
        self.counters.record_cursor_ack();
        self.counters.record_subscriber_checkpoint_write();
        Ok(PersistedSubscriberCheckpoint::new(checkpoint_record))
    }
}

fn restore_cursor_state(
    state: &mut crate::backend::records::StoreState,
    checkpoint_artifact_id: &str,
    cursor_artifact_id: &str,
    previous_identity: Option<DurableCursorIdentityRecord>,
    previous_digest: Option<String>,
) -> Result<(), StoreError> {
    state
        .subscriber_checkpoint_records
        .remove(checkpoint_artifact_id);
    state
        .authoritative_artifact_digests
        .remove(&super::super::integrity::digest_artifact_key(
            &super::super::records::AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
            checkpoint_artifact_id,
            state.canonicalization_version,
        ));
    match (previous_identity, previous_digest) {
        (Some(previous), Some(digest)) => {
            state
                .durable_cursor_identity_records
                .insert(cursor_artifact_id.to_string(), previous);
            state.upsert_digest_record(
                super::super::records::AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                cursor_artifact_id.to_string(),
                digest,
            );
        }
        _ => {
            state
                .durable_cursor_identity_records
                .remove(cursor_artifact_id);
            state
                .authoritative_artifact_digests
                .remove(&super::super::integrity::digest_artifact_key(
                &super::super::records::AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                cursor_artifact_id,
                state.canonicalization_version,
            ));
        }
    }
    Ok(())
}
