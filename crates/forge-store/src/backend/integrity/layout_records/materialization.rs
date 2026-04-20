use crate::{
    backend::records::{Milestone6LayoutMaterializationRecord, StoreState},
    failure::StoreError,
    layout::layout_materialization_artifact_id,
};

impl StoreState {
    pub(super) fn verify_layout_materialization_record(
        &self,
        stored_key: &str,
        record: &Milestone6LayoutMaterializationRecord,
    ) -> Result<(), StoreError> {
        let expected_artifact_id =
            layout_materialization_artifact_id(record.materialization.admitted_plan());
        if stored_key != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization map key `{stored_key}` did not match expected artifact id `{expected_artifact_id}`"
            )));
        }
        if record.artifact_id != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization payload `{}` drifted from expected artifact id `{expected_artifact_id}`",
                record.artifact_id
            )));
        }
        if record.materialization.artifact_id() != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted from its internal artifact id"
            )));
        }
        if record.materialization.block_reuse().structural_block_id()
            != record.materialization.admitted_plan().structural_block_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted between admitted plan and structural block reuse witness"
            )));
        }
        if record.materialization.frozen_layout().witness().physical_chunk_id()
            != record.materialization.milestone_9_reference().physical_chunk_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted between frozen layout witness and milestone 9 physical chunk reference"
            )));
        }
        if record.materialization.frozen_layout().witness().determinism_digest()
            != record.materialization.milestone_9_reference().determinism_digest()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted between frozen layout determinism and milestone 9 physical chunk determinism"
            )));
        }
        if record.materialization.milestone_7_reference().branch_id()
            != record.materialization.admitted_plan().request().target().branch_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted between admitted plan target branch and milestone 7 reference"
            )));
        }
        if record.materialization.milestone_7_reference().frontier_commit_id()
            != record.materialization.admitted_plan().request().target().frontier_commit_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted between admitted plan target frontier and milestone 7 reference"
            )));
        }
        let control = self.read_branch_delta_control_from_milestone_7_reference(
            crate::Milestone7IndependentReference::new(
                record.materialization.milestone_7_reference().branch_id().clone(),
                record.materialization.milestone_7_reference().frontier_commit_id(),
            ),
        )?;
        let expected_semantic_truth_digest =
            crate::layout::stable_layout_truth_digest(control.authoritative_export());
        if record.materialization.semantic_truth_digest() != expected_semantic_truth_digest {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted from canonical semantic truth digest"
            )));
        }
        let expected_authoritative_commit_count = control.authoritative_export().commit_envelopes.len();
        if record.materialization.authoritative_commit_count() != expected_authoritative_commit_count {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 layout materialization `{expected_artifact_id}` drifted from canonical authoritative commit count"
            )));
        }
        Ok(())
    }
}
