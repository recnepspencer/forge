use crate::{
    backend::records::StoreState,
    bulk::BULK_FAMILY_VERSION,
    failure::{StoreError, StoreErrorKind},
};

use super::super::identity::{
    bulk_program_artifact_id, frozen_bulk_manifest_artifact_id, frozen_transform_basis_artifact_id,
    frozen_transform_partition_artifact_id,
};

impl StoreState {
    pub(super) fn verify_bulk_frozen_input_records(&self) -> Result<(), StoreError> {
        verify_manifests(self)?;
        verify_transform_bases(self)?;
        verify_transform_partitions(self)?;
        Ok(())
    }
}

fn verify_manifests(state: &StoreState) -> Result<(), StoreError> {
    for (stored_key, record) in &state.frozen_bulk_manifest_records {
        let expected =
            frozen_bulk_manifest_artifact_id(&record.program_id, record.manifest.manifest_digest());
        if stored_key != &expected || record.artifact_id != expected {
            return Err(StoreError::backend_integrity(format!(
                "bulk manifest key `{stored_key}` did not match expected artifact id `{expected}`"
            )));
        }
        if record.family_version != BULK_FAMILY_VERSION
            || record.manifest.family_version() != BULK_FAMILY_VERSION
        {
            return Err(StoreError::new(
                StoreErrorKind::BulkProgramVersionUnsupported,
                format!(
                    "bulk manifest `{}` used unsupported family version",
                    record.program_id
                ),
            ));
        }
        if record.program_id != record.manifest.program_id() {
            return Err(StoreError::backend_integrity(format!(
                "bulk manifest `{}` did not preserve program linkage",
                record.artifact_id
            )));
        }
        if !record.manifest.has_valid_digest()? {
            return Err(StoreError::backend_integrity(format!(
                "bulk manifest `{}` digest no longer matched its payload",
                record.artifact_id
            )));
        }
        let program_id = state
            .bulk_program_identity_records
            .get(&bulk_program_artifact_id(&record.program_id))
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "bulk manifest `{}` referenced missing program identity",
                    record.artifact_id
                ))
            })?;
        if program_id.source_identity != record.manifest.source_identity() {
            return Err(StoreError::backend_integrity(format!(
                "bulk manifest `{}` source identity drifted from program identity",
                record.artifact_id
            )));
        }
    }
    Ok(())
}

fn verify_transform_bases(state: &StoreState) -> Result<(), StoreError> {
    for (stored_key, record) in &state.frozen_transform_basis_records {
        let expected =
            frozen_transform_basis_artifact_id(&record.program_id, record.basis.basis_digest());
        if stored_key != &expected || record.artifact_id != expected {
            return Err(StoreError::backend_integrity(format!(
                "bulk transform basis key `{stored_key}` did not match expected artifact id `{expected}`"
            )));
        }
        if record.family_version != BULK_FAMILY_VERSION
            || record.basis.family_version() != BULK_FAMILY_VERSION
        {
            return Err(StoreError::new(
                StoreErrorKind::BulkProgramVersionUnsupported,
                format!(
                    "bulk transform basis `{}` used unsupported family version",
                    record.program_id
                ),
            ));
        }
        if record.program_id != record.basis.program_id() {
            return Err(StoreError::backend_integrity(format!(
                "bulk transform basis `{}` did not preserve program linkage",
                record.artifact_id
            )));
        }
        if !record.basis.has_valid_digest()? {
            return Err(StoreError::backend_integrity(format!(
                "bulk transform basis `{}` digest no longer matched its payload",
                record.artifact_id
            )));
        }
    }
    Ok(())
}

fn verify_transform_partitions(state: &StoreState) -> Result<(), StoreError> {
    for (stored_key, record) in &state.frozen_transform_partition_records {
        let expected = frozen_transform_partition_artifact_id(
            &record.program_id,
            record.partition.partition_digest(),
        );
        if stored_key != &expected || record.artifact_id != expected {
            return Err(StoreError::backend_integrity(format!(
                "bulk transform partition key `{stored_key}` did not match expected artifact id `{expected}`"
            )));
        }
        if record.family_version != BULK_FAMILY_VERSION
            || record.partition.family_version() != BULK_FAMILY_VERSION
        {
            return Err(StoreError::new(
                StoreErrorKind::BulkProgramVersionUnsupported,
                format!(
                    "bulk transform partition `{}` used unsupported family version",
                    record.program_id
                ),
            ));
        }
        if !record.partition.has_valid_digest()? {
            return Err(StoreError::backend_integrity(format!(
                "bulk transform partition `{}` digest no longer matched its payload",
                record.artifact_id
            )));
        }
        let basis = state
            .frozen_transform_basis_records
            .values()
            .find(|basis| {
                basis.program_id == record.program_id
                    && basis.basis.transform_identity() == record.partition.transform_identity()
                    && basis.basis.target_branch_scope() == record.partition.target_branch_scope()
                    && basis.basis.basis_commit_id() == record.partition.basis_commit_id()
            })
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "bulk transform partition `{}` referenced missing frozen basis",
                    record.artifact_id
                ))
            })?;
        if basis.program_id != record.program_id {
            return Err(StoreError::backend_integrity(format!(
                "bulk transform partition `{}` drifted from basis program linkage",
                record.artifact_id
            )));
        }
    }
    Ok(())
}
