use crate::{
    backend::records::{Milestone6StructuralBlockRecord, StoreState},
    failure::StoreError,
};

impl StoreState {
    pub(super) fn verify_structural_block_record(
        &self,
        stored_key: &str,
        record: &Milestone6StructuralBlockRecord,
    ) -> Result<(), StoreError> {
        let expected_artifact_id = format!(
            "layout-structural-block:{}",
            record.structural_block_id.as_str()
        );
        if stored_key != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 structural block map key `{stored_key}` did not match expected artifact id `{expected_artifact_id}`"
            )));
        }
        if record.artifact_id != expected_artifact_id {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 structural block payload `{}` drifted from expected artifact id `{expected_artifact_id}`",
                record.artifact_id
            )));
        }
        if record.supporting_layout_materialization_artifact_ids.is_empty() {
            return Err(StoreError::backend_integrity(format!(
                "milestone 6 structural block `{expected_artifact_id}` has no supporting layout materializations"
            )));
        }
        for layout_materialization_artifact_id in &record.supporting_layout_materialization_artifact_ids {
            let materialization = self
                .milestone_6_layout_materialization_records
                .get(layout_materialization_artifact_id)
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "milestone 6 structural block `{}` referenced missing layout materialization `{}`",
                        record.artifact_id, layout_materialization_artifact_id
                    ))
                })?;
            if record.scope_class != materialization.materialization.block_reuse().scope_class() {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 structural block `{expected_artifact_id}` drifted from structural block reuse scope class"
                )));
            }
            if record.equivalence_contract_version
                != materialization.materialization.block_reuse().equivalence_contract_version()
            {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 structural block `{expected_artifact_id}` drifted from structural block reuse equivalence contract version"
                )));
            }
            if record.slice_ids != materialization.materialization.block_reuse().slice_ids() {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 structural block `{expected_artifact_id}` drifted from structural block reuse slice ids"
                )));
            }
            if record.structural_block_id != *materialization.materialization.block_reuse().structural_block_id() {
                return Err(StoreError::backend_integrity(format!(
                    "milestone 6 structural block `{expected_artifact_id}` drifted from structural block reuse id"
                )));
            }
        }
        Ok(())
    }
}
