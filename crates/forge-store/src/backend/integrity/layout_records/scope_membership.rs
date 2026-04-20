use crate::{
    backend::records::{Milestone6ScopeSliceMembershipRecord, StoreState},
    failure::StoreError,
    layout::layout_scope_membership_artifact_id,
};

impl StoreState {
    pub(super) fn verify_scope_slice_membership_record(
        &self,
        stored_key: &str,
        record: &Milestone6ScopeSliceMembershipRecord,
    ) -> Result<(), StoreError> {
        let materialization = self
            .milestone_6_layout_materialization_records
            .get(&record.layout_materialization_artifact_id)
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "milestone 6 scope membership `{}` referenced missing layout materialization `{}`",
                    record.artifact_id, record.layout_materialization_artifact_id
                ))
            })?;
        let expected_artifact_id =
            layout_scope_membership_artifact_id(materialization.materialization.admitted_plan().request())?;
        if stored_key != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership map key `{stored_key}` did not match expected artifact id `{expected_artifact_id}`"
            )));
        }
        if record.artifact_id != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership payload `{}` drifted from expected artifact id `{expected_artifact_id}`",
                record.artifact_id
            )));
        }
        if record.branch_id != *materialization.materialization.admitted_plan().request().target().branch_id() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership `{expected_artifact_id}` drifted from admitted plan branch"
            )));
        }
        if record.frontier_commit_id
            != materialization.materialization.admitted_plan().request().target().frontier_commit_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership `{expected_artifact_id}` drifted from admitted plan frontier"
            )));
        }
        if record.scope_class
            != materialization.materialization.admitted_plan().request().scope_class().label()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership `{expected_artifact_id}` drifted from admitted plan scope class"
            )));
        }
        if record.projection_digest != materialization.materialization.milestone_7_reference().projection_digest() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership `{expected_artifact_id}` drifted from milestone 7 projection digest"
            )));
        }
        if record.slice_ids != materialization.materialization.admitted_plan().slice_ids() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 scope membership `{expected_artifact_id}` drifted from admitted plan slice ids"
            )));
        }
        Ok(())
    }
}
