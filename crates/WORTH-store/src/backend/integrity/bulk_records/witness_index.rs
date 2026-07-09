use crate::{
    backend::records::StoreState,
    bulk::BULK_FAMILY_VERSION,
    failure::{StoreError, StoreErrorKind},
};

use super::super::identity::{bulk_checkpoint_artifact_id, bulk_witness_index_artifact_id};

impl StoreState {
    pub(super) fn verify_bulk_witness_index_records(&self) -> Result<(), StoreError> {
        for (stored_key, record) in &self.program_chunk_witness_index_records {
            let expected = bulk_witness_index_artifact_id(&record.program_id, &record.plan_id);
            if stored_key != &expected || record.artifact_id != expected {
                return Err(StoreError::backend_integrity(format!(
                    "bulk witness index key `{stored_key}` did not match expected artifact id `{expected}`"
                )));
            }
            if record.family_version != BULK_FAMILY_VERSION {
                return Err(StoreError::new(
                    StoreErrorKind::BulkProgramVersionUnsupported,
                    format!(
                        "bulk witness index `{}` used unsupported family version",
                        record.artifact_id
                    ),
                ));
            }
            let witnesses: Vec<_> = self
                .bulk_chunk_witness_records
                .values()
                .filter(|witness| {
                    witness.program_id == record.program_id && witness.plan_id == record.plan_id
                })
                .collect();
            if witnesses.is_empty() {
                return Err(StoreError::backend_integrity(format!(
                    "bulk witness index `{}` existed without any witness records",
                    record.artifact_id
                )));
            }
            let highest = witnesses
                .iter()
                .max_by_key(|witness| witness.witness.chunk_ordinal().value())
                .expect("non-empty witnesses");
            if highest.witness.chunk_ordinal() != record.index.highest_committed_chunk_ordinal()
                || highest.witness.canonical_commit_id()
                    != record.index.highest_committed_commit_id()
                || witnesses.len() as u64 != record.index.witness_count()
            {
                return Err(StoreError::backend_integrity(format!(
                    "bulk witness index `{}` did not match persisted witness set",
                    record.artifact_id
                )));
            }
            if let Some(checkpoint_sequence) = record.index.latest_checkpoint_sequence() {
                let checkpoint_artifact_id = bulk_checkpoint_artifact_id(
                    &record.program_id,
                    &record.plan_id,
                    checkpoint_sequence,
                );
                let checkpoint = self
                    .bulk_progress_checkpoint_records
                    .get(&checkpoint_artifact_id)
                    .ok_or_else(|| {
                        StoreError::backend_integrity(format!(
                            "bulk witness index `{}` referenced missing checkpoint `{checkpoint_artifact_id}`",
                            record.artifact_id
                        ))
                    })?;
                if checkpoint.checkpoint.completed_chunk_ordinal().value()
                    >= record.index.witness_count()
                {
                    return Err(StoreError::backend_integrity(format!(
                        "bulk witness index `{}` referenced checkpoint `{}` beyond persisted witness count",
                        record.artifact_id, checkpoint.artifact_id
                    )));
                }
            }
        }
        Ok(())
    }
}
