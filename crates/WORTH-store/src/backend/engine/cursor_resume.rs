use crate::authority::{
    DurableCursorResumePlan, DurableCursorResumeRequest, FetchedDurableCursorIdentity,
};
use crate::compatibility::CompatibilityFamilyKind;
use crate::failure::{StoreError, StoreErrorKind};

use super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn fetch_durable_cursor_identity(
        &self,
        cursor_id: &str,
    ) -> Result<FetchedDurableCursorIdentity, StoreError> {
        self.admit_runtime_read_compatibility(
            CompatibilityFamilyKind::SchemaLineageCursorCheckpointSupport,
            "fetch_durable_cursor_identity",
        )?;
        self.counters.record_cursor_identity_lookup();
        let artifact_id = super::super::integrity::durable_cursor_identity_artifact_id(cursor_id);
        let record = self
            .state
            .durable_cursor_identity_records
            .get(&artifact_id)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CursorCheckpointMissing,
                    format!("durable cursor `{cursor_id}` not found"),
                )
            })?;
        self.state.verify_durable_cursor_identity_record(&record)?;
        Ok(FetchedDurableCursorIdentity::new(record))
    }

    pub fn plan_cursor_resume(
        &self,
        request: DurableCursorResumeRequest,
    ) -> Result<DurableCursorResumePlan, StoreError> {
        self.admit_runtime_read_compatibility(
            CompatibilityFamilyKind::SchemaLineageCursorCheckpointSupport,
            "plan_cursor_resume",
        )?;
        self.counters.record_cursor_identity_lookup();
        let identity = self
            .state
            .durable_cursor_identity_records
            .get(&super::super::integrity::durable_cursor_identity_artifact_id(request.cursor_id()))
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CursorCheckpointMissing,
                    format!("durable cursor `{}` not found", request.cursor_id()),
                )
            })?;
        if identity.subscriber_id != request.subscriber_id()
            || identity.branch_id != *request.branch_id()
            || identity.feed_shape_id != request.feed_shape_id()
            || identity.schema_interpretation_id != request.schema_interpretation_id()
            || identity.cursor_semantics_version != request.cursor_semantics_version()
        {
            self.counters.record_cursor_equivalence_reject();
            return Err(StoreError::new(
                StoreErrorKind::CursorEquivalenceViolation,
                format!(
                    "durable cursor `{}` does not match the requested resume identity basis",
                    request.cursor_id()
                ),
            ));
        }
        let latest_checkpoint = self
            .state
            .subscriber_checkpoint_records
            .get(&super::super::integrity::subscriber_checkpoint_artifact_id(
                request.cursor_id(),
                identity.latest_checkpoint_sequence,
            ))
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CursorCheckpointMissing,
                    format!(
                        "durable cursor `{}` is missing checkpoint sequence {}",
                        request.cursor_id(),
                        identity.latest_checkpoint_sequence
                    ),
                )
            })?;
        self.state
            .verify_durable_cursor_identity_record(&identity)?;
        self.state
            .verify_subscriber_checkpoint_record(&latest_checkpoint)?;
        self.counters.record_cursor_resume(2, 1);
        Ok(DurableCursorResumePlan::new(identity, latest_checkpoint))
    }
}
