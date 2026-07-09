use crate::failure::StoreError;
use rusqlite::Transaction;

use super::super::super::records::StoreState;
use super::super::helpers::persist_bulk_json_record;

pub(super) fn persist_compatibility(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.compatibility_manifest_records.values() {
        persist_bulk_json_record(
            transaction,
            "compatibility_manifest_records",
            &record.artifact_id,
            Vec::new(),
            record,
        )?;
    }
    Ok(())
}
