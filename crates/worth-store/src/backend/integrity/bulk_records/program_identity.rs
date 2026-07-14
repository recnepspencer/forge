use crate::{
    backend::records::StoreState,
    bulk::BULK_FAMILY_VERSION,
    failure::{StoreError, StoreErrorKind},
};

use super::super::identity::bulk_program_artifact_id;

impl StoreState {
    pub(super) fn verify_bulk_program_identity_records(&self) -> Result<(), StoreError> {
        for (stored_key, record) in &self.bulk_program_identity_records {
            let expected = bulk_program_artifact_id(&record.program_id);
            if stored_key != &expected || record.artifact_id != expected {
                return Err(StoreError::backend_integrity(format!(
                    "bulk program identity key `{stored_key}` did not match expected artifact id `{expected}`"
                )));
            }
            if record.family_version != BULK_FAMILY_VERSION {
                return Err(StoreError::new(
                    StoreErrorKind::BulkProgramVersionUnsupported,
                    format!(
                        "bulk program `{}` used unsupported family version {}",
                        record.program_id, record.family_version
                    ),
                ));
            }
        }
        Ok(())
    }
}
