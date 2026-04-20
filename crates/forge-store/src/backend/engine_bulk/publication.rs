use crate::{
    bulk::{
        BulkChunkCommitWitness, BulkProgressCheckpointRecordInput, ProgramChunkWitnessIndex,
        PublishedBulkProgressCheckpoint, BULK_FAMILY_VERSION,
    },
    failure::{StoreError, StoreErrorKind},
};

use super::super::{
    engine::{StateBackedStoreBackend, StatePersistence},
    integrity::{
        bulk_checkpoint_artifact_id, bulk_witness_artifact_id, bulk_witness_index_artifact_id,
    },
    records::{
        BulkChunkWitnessRecord, BulkProgressCheckpointRecord, ProgramChunkWitnessIndexRecord,
    },
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn publish_bulk_chunk_witness(
        &mut self,
        witness: BulkChunkCommitWitness,
    ) -> Result<BulkChunkCommitWitness, StoreError> {
        let mut next = self.state.clone();
        let artifact_id = bulk_witness_artifact_id(
            witness.program_id(),
            witness.plan_id(),
            witness.chunk_ordinal().value(),
        );
        if next.bulk_chunk_witness_records.contains_key(&artifact_id) {
            return Err(StoreError::new(
                StoreErrorKind::BulkChunkDuplicateCommit,
                format!("bulk witness `{artifact_id}` already exists"),
            ));
        }

        let existing_witnesses: Vec<_> = next
            .bulk_chunk_witness_records
            .values()
            .filter(|record| {
                record.program_id == witness.program_id() && record.plan_id == witness.plan_id()
            })
            .collect();
        let expected_ordinal = existing_witnesses.len() as u64;
        if witness.chunk_ordinal().value() != expected_ordinal {
            return Err(StoreError::new(
                StoreErrorKind::BulkChunkWitnessGap,
                format!(
                    "bulk witness ordinal {} was published before expected ordinal {}",
                    witness.chunk_ordinal().value(),
                    expected_ordinal
                ),
            ));
        }

        next.bulk_chunk_witness_records.insert(
            artifact_id.clone(),
            BulkChunkWitnessRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: witness.program_id().to_string(),
                plan_id: witness.plan_id().to_string(),
                witness: witness.clone(),
            },
        );
        let index_artifact_id =
            bulk_witness_index_artifact_id(witness.program_id(), witness.plan_id());
        let checkpoint_sequence = next
            .bulk_progress_checkpoint_records
            .values()
            .filter(|record| {
                record.program_id == witness.program_id() && record.plan_id == witness.plan_id()
            })
            .map(|record| record.checkpoint.checkpoint_sequence())
            .max();
        next.program_chunk_witness_index_records.insert(
            index_artifact_id.clone(),
            ProgramChunkWitnessIndexRecord {
                artifact_id: index_artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: witness.program_id().to_string(),
                plan_id: witness.plan_id().to_string(),
                index: ProgramChunkWitnessIndex::new(
                    witness.program_id().to_string(),
                    witness.plan_id().to_string(),
                    witness.chunk_ordinal(),
                    witness.canonical_commit_id(),
                    checkpoint_sequence,
                    expected_ordinal + 1,
                ),
            },
        );
        self.commit_replacement_state(next)?;
        self.counters.record_bulk_chunk_witness_write();
        Ok(witness)
    }

    pub fn publish_bulk_progress_checkpoint(
        &mut self,
        witness: BulkChunkCommitWitness,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        let mut next = self.state.clone();
        let latest_checkpoint: Option<PublishedBulkProgressCheckpoint> = next
            .bulk_progress_checkpoint_records
            .values()
            .filter(|record| {
                record.program_id == witness.program_id() && record.plan_id == witness.plan_id()
            })
            .max_by_key(|record| record.checkpoint.checkpoint_sequence())
            .map(|record| record.checkpoint.clone());
        let latest_sequence = latest_checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.checkpoint_sequence());
        let input = BulkProgressCheckpointRecordInput::publish_next(latest_sequence, &witness)?;
        let witness_artifact_id = input.last_committed_chunk_witness_artifact_id().to_string();
        let witness = next
            .bulk_chunk_witness_records
            .get(&witness_artifact_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkCheckpointPublicationGap,
                    format!(
                        "bulk progress checkpoint referenced missing witness `{witness_artifact_id}`"
                    ),
                )
            })?;
        if witness.program_id != input.program_id() || witness.plan_id != input.plan_id() {
            return Err(StoreError::new(
                StoreErrorKind::BulkCheckpointPublicationGap,
                "bulk progress checkpoint must reference a witness from the same program and plan",
            ));
        }
        if let Some(previous_checkpoint) = latest_checkpoint {
            if input.completed_chunk_ordinal().value()
                < previous_checkpoint.next_chunk_ordinal().value()
            {
                return Err(StoreError::new(
                    StoreErrorKind::BulkCheckpointPublicationGap,
                    format!(
                        "bulk checkpoint for witness ordinal {} would not advance beyond prior checkpoint boundary {}",
                        input.completed_chunk_ordinal().value(),
                        previous_checkpoint.next_chunk_ordinal().value()
                    ),
                ));
            }
        }
        let checkpoint = PublishedBulkProgressCheckpoint::new(
            input.program_id().to_string(),
            input.plan_id().to_string(),
            input.checkpoint_sequence(),
            input.completed_chunk_ordinal(),
            input.next_chunk_ordinal(),
            witness_artifact_id.clone(),
            input.checkpoint_digest().to_string(),
        );
        let artifact_id = bulk_checkpoint_artifact_id(
            input.program_id(),
            input.plan_id(),
            input.checkpoint_sequence(),
        );
        next.bulk_progress_checkpoint_records.insert(
            artifact_id.clone(),
            BulkProgressCheckpointRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: input.program_id().to_string(),
                plan_id: input.plan_id().to_string(),
                checkpoint: checkpoint.clone(),
            },
        );
        if let Some(index) =
            next.program_chunk_witness_index_records
                .get_mut(&bulk_witness_index_artifact_id(
                    input.program_id(),
                    input.plan_id(),
                ))
        {
            index.index = ProgramChunkWitnessIndex::new(
                index.program_id.clone(),
                index.plan_id.clone(),
                index.index.highest_committed_chunk_ordinal(),
                index.index.highest_committed_commit_id(),
                Some(input.checkpoint_sequence()),
                index.index.witness_count(),
            );
        }
        self.commit_replacement_state(next)?;
        self.counters.record_bulk_checkpoint_write();
        Ok(checkpoint)
    }
}
