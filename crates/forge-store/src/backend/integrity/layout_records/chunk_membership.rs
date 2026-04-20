use crate::{
    backend::records::{Milestone6ChunkMembershipRecord, StoreState},
    failure::StoreError,
    layout::chunk_membership_artifact_id,
};

impl StoreState {
    pub(super) fn verify_chunk_membership_record(
        &self,
        stored_key: &str,
        record: &Milestone6ChunkMembershipRecord,
    ) -> Result<(), StoreError> {
        let materialization = self
            .milestone_6_layout_materialization_records
            .get(&record.layout_materialization_artifact_id)
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "milestone 6 chunk membership `{}` referenced missing layout materialization `{}`",
                    record.artifact_id, record.layout_materialization_artifact_id
                ))
            })?;
        let expected_artifact_id =
            chunk_membership_artifact_id(materialization.materialization.frozen_layout());
        if stored_key != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership map key `{stored_key}` did not match expected artifact id `{expected_artifact_id}`"
            )));
        }
        if record.artifact_id != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership payload `{}` drifted from expected artifact id `{expected_artifact_id}`",
                record.artifact_id
            )));
        }
        if record.physical_chunk_id != *materialization.materialization.frozen_layout().witness().physical_chunk_id() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership `{expected_artifact_id}` drifted from frozen witness physical chunk id"
            )));
        }
        if record.chunk_shape_version != materialization.materialization.frozen_layout().witness().chunk_shape_version() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership `{expected_artifact_id}` drifted from frozen witness chunk shape version"
            )));
        }
        if record.determinism_digest != materialization.materialization.frozen_layout().witness().determinism_digest() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership `{expected_artifact_id}` drifted from frozen witness determinism digest"
            )));
        }
        if record.slice_ids != materialization.materialization.frozen_layout().witness().ordered_slice_ids() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 chunk membership `{expected_artifact_id}` drifted from frozen witness slice ids"
            )));
        }
        Ok(())
    }
}
