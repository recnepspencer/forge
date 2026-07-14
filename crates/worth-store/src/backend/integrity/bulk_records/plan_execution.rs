use crate::{
    backend::records::StoreState,
    bulk::BULK_FAMILY_VERSION,
    failure::{StoreError, StoreErrorKind},
};

use super::super::identity::{
    bulk_plan_artifact_id, bulk_program_artifact_id, bulk_witness_artifact_id,
};

impl StoreState {
    pub(super) fn verify_bulk_plan_and_witness_records(&self) -> Result<(), StoreError> {
        verify_deterministic_plans(self)?;
        verify_chunk_witnesses(self)?;
        Ok(())
    }
}

fn verify_deterministic_plans(state: &StoreState) -> Result<(), StoreError> {
    for (stored_key, record) in &state.bulk_deterministic_plan_records {
        let expected = bulk_plan_artifact_id(&record.program_id, record.plan.plan_id());
        if stored_key != &expected || record.artifact_id != expected {
            return Err(StoreError::backend_integrity(format!(
                "bulk plan key `{stored_key}` did not match expected artifact id `{expected}`"
            )));
        }
        if record.family_version != BULK_FAMILY_VERSION
            || record.plan.family_version() != BULK_FAMILY_VERSION
        {
            return Err(StoreError::new(
                StoreErrorKind::BulkProgramVersionUnsupported,
                format!(
                    "bulk plan `{}` used unsupported family version",
                    record.artifact_id
                ),
            ));
        }
        if record.program_id != record.plan.program_id() {
            return Err(StoreError::backend_integrity(format!(
                "bulk plan `{}` did not preserve program linkage",
                record.artifact_id
            )));
        }
        if !record.plan.has_valid_plan_id()? {
            return Err(StoreError::backend_integrity(format!(
                "bulk plan `{}` id no longer matched its payload",
                record.artifact_id
            )));
        }
        state
            .bulk_program_identity_records
            .get(&bulk_program_artifact_id(&record.program_id))
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "bulk plan `{}` referenced missing program identity",
                    record.artifact_id
                ))
            })?;
    }
    Ok(())
}

fn verify_chunk_witnesses(state: &StoreState) -> Result<(), StoreError> {
    for (stored_key, record) in &state.bulk_chunk_witness_records {
        let expected = bulk_witness_artifact_id(
            &record.program_id,
            &record.plan_id,
            record.witness.chunk_ordinal().value(),
        );
        if stored_key != &expected || record.artifact_id != expected {
            return Err(StoreError::backend_integrity(format!(
                "bulk witness key `{stored_key}` did not match expected artifact id `{expected}`"
            )));
        }
        if record.family_version != BULK_FAMILY_VERSION {
            return Err(StoreError::new(
                StoreErrorKind::BulkProgramVersionUnsupported,
                format!(
                    "bulk witness `{}` used unsupported family version",
                    record.artifact_id
                ),
            ));
        }
        let plan = state
            .bulk_deterministic_plan_records
            .get(&bulk_plan_artifact_id(&record.program_id, &record.plan_id))
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "bulk witness `{}` referenced missing plan",
                    record.artifact_id
                ))
            })?;
        if plan.plan.plan_id() != record.witness.plan_id()
            || plan.plan.program_id() != record.witness.program_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "bulk witness `{}` drifted from its plan linkage",
                record.artifact_id
            )));
        }
        if plan.plan.target_branch_scope() != record.witness.target_branch_scope() {
            return Err(StoreError::backend_integrity(format!(
                "bulk witness `{}` drifted from its plan branch scope",
                record.artifact_id
            )));
        }
        if !state.has_commit(record.witness.canonical_commit_id()) {
            return Err(StoreError::backend_integrity(format!(
                "bulk witness `{}` referenced missing canonical commit {}",
                record.artifact_id,
                record.witness.canonical_commit_id().0
            )));
        }
    }
    Ok(())
}
