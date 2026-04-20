use crate::{
    backend::records::StoreState,
    bulk::{BULK_FAMILY_VERSION, compute_checkpoint_digest},
    failure::{StoreError, StoreErrorKind},
};

use super::super::identity::bulk_checkpoint_artifact_id;

impl StoreState {
    pub(super) fn verify_bulk_checkpoint_records(&self) -> Result<(), StoreError> {
        verify_checkpoint_records(self)?;
        verify_checkpoint_families(self)?;
        Ok(())
    }
}

fn verify_checkpoint_records(state: &StoreState) -> Result<(), StoreError> {
    for (stored_key, record) in &state.bulk_progress_checkpoint_records {
        let expected = bulk_checkpoint_artifact_id(
            &record.program_id,
            &record.plan_id,
            record.checkpoint.checkpoint_sequence(),
        );
        if stored_key != &expected || record.artifact_id != expected {
            return Err(StoreError::backend_integrity(format!(
                "bulk checkpoint key `{stored_key}` did not match expected artifact id `{expected}`"
            )));
        }
        if record.family_version != BULK_FAMILY_VERSION {
            return Err(StoreError::new(
                StoreErrorKind::BulkProgramVersionUnsupported,
                format!("bulk checkpoint `{}` used unsupported family version", record.artifact_id),
            ));
        }
        let recomputed = compute_checkpoint_digest(
            record.checkpoint.program_id(),
            record.checkpoint.plan_id(),
            record.checkpoint.checkpoint_sequence(),
            record.checkpoint.completed_chunk_ordinal(),
            record.checkpoint.next_chunk_ordinal(),
            record.checkpoint.last_committed_chunk_witness_artifact_id(),
        );
        if recomputed != record.checkpoint.checkpoint_digest() {
            return Err(StoreError::new(
                StoreErrorKind::BulkCheckpointDigestMismatch,
                format!("bulk checkpoint `{}` digest mismatch", record.artifact_id),
            ));
        }
        let witness = state
            .bulk_chunk_witness_records
            .get(record.checkpoint.last_committed_chunk_witness_artifact_id());
        let Some(witness) = witness else {
            return Err(StoreError::backend_integrity(format!(
                "bulk checkpoint `{}` referenced missing chunk witness `{}`",
                record.artifact_id,
                record.checkpoint.last_committed_chunk_witness_artifact_id()
            )));
        };
        if witness.program_id != record.program_id
            || witness.plan_id != record.plan_id
            || witness.witness.program_id() != record.checkpoint.program_id()
            || witness.witness.plan_id() != record.checkpoint.plan_id()
        {
            return Err(StoreError::backend_integrity(format!(
                "bulk checkpoint `{}` drifted from its witness program or plan linkage",
                record.artifact_id
            )));
        }
        if witness.witness.chunk_ordinal() != record.checkpoint.completed_chunk_ordinal() {
            return Err(StoreError::backend_integrity(format!(
                "bulk checkpoint `{}` completed chunk ordinal did not match its witness ordinal",
                record.artifact_id
            )));
        }
        if record.checkpoint.next_chunk_ordinal().value()
            != record.checkpoint.completed_chunk_ordinal().value() + 1
        {
            return Err(StoreError::backend_integrity(format!(
                "bulk checkpoint `{}` next chunk ordinal was not the successor of its completed chunk",
                record.artifact_id
            )));
        }
    }
    Ok(())
}

fn verify_checkpoint_families(state: &StoreState) -> Result<(), StoreError> {
    let checkpoint_families = state.bulk_progress_checkpoint_records.values().fold(
        std::collections::BTreeMap::<(&str, &str), Vec<_>>::new(),
        |mut acc, record| {
            acc.entry((&record.program_id, &record.plan_id))
                .or_default()
                .push(record);
            acc
        },
    );
    for ((program_id, plan_id), mut checkpoints) in checkpoint_families {
        checkpoints.sort_by_key(|record| record.checkpoint.checkpoint_sequence());
        let mut expected_sequence = 1;
        let mut minimum_completed_chunk_ordinal = 0;
        for checkpoint in checkpoints {
            if checkpoint.checkpoint.checkpoint_sequence() != expected_sequence {
                return Err(StoreError::backend_integrity(format!(
                    "bulk checkpoint family `{program_id}:{plan_id}` contained non-contiguous sequence {}; expected {}",
                    checkpoint.checkpoint.checkpoint_sequence(),
                    expected_sequence
                )));
            }
            if checkpoint.checkpoint.completed_chunk_ordinal().value() < minimum_completed_chunk_ordinal {
                return Err(StoreError::backend_integrity(format!(
                    "bulk checkpoint family `{program_id}:{plan_id}` regressed completed chunk ordinal {} below required boundary {}",
                    checkpoint.checkpoint.completed_chunk_ordinal().value(),
                    minimum_completed_chunk_ordinal
                )));
            }
            minimum_completed_chunk_ordinal = checkpoint.checkpoint.next_chunk_ordinal().value();
            expected_sequence += 1;
        }
    }
    Ok(())
}
