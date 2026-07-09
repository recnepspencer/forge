use crate::backend::records::{
    CompactionProductRecord, RebuildDebtRecord, RetentionBasisRecord, RetentionClosureRecord,
    StoreState,
};
use crate::failure::{StoreError, StoreErrorKind};

use super::{
    compaction_product_artifact_id, rebuild_debt_artifact_id, retention_basis_artifact_id,
    retention_closure_artifact_id,
};

impl StoreState {
    fn verify_compaction_product_record(
        &self,
        record: &CompactionProductRecord,
    ) -> Result<(), StoreError> {
        if record.artifact_id
            != compaction_product_artifact_id(
                &record.retained_basis_label,
                &record.compacted_family_labels,
            )
        {
            return Err(StoreError::new(
                StoreErrorKind::CompactionProductShadowAuthorityViolation,
                format!(
                    "compaction product `{}` drifted from its canonical retained-basis identity",
                    record.artifact_id
                ),
            ));
        }
        if !record.closure_record_artifact_id.is_empty()
            && !self
                .retention_closure_records
                .contains_key(&record.closure_record_artifact_id)
        {
            return Err(StoreError::new(
                StoreErrorKind::CompactionPlanBasisAmbiguous,
                format!(
                    "compaction product `{}` referenced missing closure record `{}`",
                    record.artifact_id, record.closure_record_artifact_id
                ),
            ));
        }
        for artifact_id in &record.basis_record_artifact_ids {
            if !self.retention_basis_records.contains_key(artifact_id) {
                return Err(StoreError::new(
                    StoreErrorKind::CompactionPlanBasisAmbiguous,
                    format!(
                        "compaction product `{}` referenced missing basis record `{artifact_id}`",
                        record.artifact_id
                    ),
                ));
            }
        }
        Ok(())
    }

    fn verify_retention_basis_record(
        &self,
        record: &RetentionBasisRecord,
    ) -> Result<(), StoreError> {
        if record.artifact_id != retention_basis_artifact_id(&record.basis_label) {
            return Err(StoreError::new(
                StoreErrorKind::RetentionClosureBasisMissing,
                format!(
                    "retention basis record `{}` drifted from basis label `{}`",
                    record.artifact_id, record.basis_label
                ),
            ));
        }
        if let Some(commit_id) = record.basis_commit_id {
            self.commit_record(commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::RetentionClosureBasisMissing,
                    format!(
                        "retention basis `{}` references missing commit {}",
                        record.basis_label, commit_id.0
                    ),
                )
            })?;
        }
        Ok(())
    }

    fn verify_retention_closure_record(
        &self,
        record: &RetentionClosureRecord,
    ) -> Result<(), StoreError> {
        if record.artifact_id != retention_closure_artifact_id(&record.retained_basis_label) {
            return Err(StoreError::new(
                StoreErrorKind::RetentionClosureViolation,
                format!(
                    "retention closure record `{}` drifted from retained basis label `{}`",
                    record.artifact_id, record.retained_basis_label
                ),
            ));
        }
        for commit_id in &record.closure_commit_ids {
            self.commit_record(*commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::RetentionClosureBasisMissing,
                    format!(
                        "retention closure `{}` references missing commit {}",
                        record.artifact_id, commit_id.0
                    ),
                )
            })?;
        }
        Ok(())
    }

    fn verify_rebuild_debt_record(&self, record: &RebuildDebtRecord) -> Result<(), StoreError> {
        if record.artifact_id
            != rebuild_debt_artifact_id(
                &record.family_label,
                &record.retained_basis_label,
                &record.rebuild_target_id,
            )
        {
            return Err(StoreError::new(
                StoreErrorKind::ReclaimEligibilityViolation,
                format!(
                    "rebuild debt record `{}` drifted from family `{}`, retained basis `{}`, and target `{}`",
                    record.artifact_id, record.family_label, record.retained_basis_label, record.rebuild_target_id
                ),
            ));
        }
        Ok(())
    }

    pub fn verify_retention_record_family(&self) -> Result<(), StoreError> {
        for record in self.retention_basis_records.values() {
            self.verify_retention_basis_record(record)?;
        }
        for record in self.retention_closure_records.values() {
            self.verify_retention_closure_record(record)?;
        }
        for record in self.compaction_product_records.values() {
            self.verify_compaction_product_record(record)?;
        }
        for record in self.rebuild_debt_records.values() {
            self.verify_rebuild_debt_record(record)?;
        }
        Ok(())
    }
}
