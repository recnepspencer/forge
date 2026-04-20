use super::digest::{checkpoint_artifact_id, compute_checkpoint_digest};
use crate::{
    bulk::{BulkChunkCommitWitness, ChunkOrdinal},
    failure::{StoreError, StoreErrorKind},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulkProgressCheckpointRecordInput {
    program_id: String,
    plan_id: String,
    checkpoint_sequence: u64,
    completed_chunk_ordinal: ChunkOrdinal,
    next_chunk_ordinal: ChunkOrdinal,
    last_committed_chunk_witness_artifact_id: String,
    checkpoint_digest: String,
}

impl BulkProgressCheckpointRecordInput {
    pub(crate) fn publish_next(
        latest_checkpoint_sequence: Option<u64>,
        witness: &BulkChunkCommitWitness,
    ) -> Result<Self, StoreError> {
        let checkpoint_sequence = latest_checkpoint_sequence.map(|sequence| sequence + 1).unwrap_or(1);
        if checkpoint_sequence == 0 {
            return Err(StoreError::new(
                StoreErrorKind::BulkCheckpointPublicationGap,
                "bulk progress checkpoints must start at sequence 1",
            ));
        }
        let completed_chunk_ordinal = witness.chunk_ordinal();
        let next_chunk_ordinal = ChunkOrdinal::new(completed_chunk_ordinal.value() + 1);
        let last_committed_chunk_witness_artifact_id = checkpoint_artifact_id(witness);
        let checkpoint_digest = compute_checkpoint_digest(
            witness.program_id(),
            witness.plan_id(),
            checkpoint_sequence,
            completed_chunk_ordinal,
            next_chunk_ordinal,
            &last_committed_chunk_witness_artifact_id,
        );
        Ok(Self {
            program_id: witness.program_id().to_string(),
            plan_id: witness.plan_id().to_string(),
            checkpoint_sequence,
            completed_chunk_ordinal,
            next_chunk_ordinal,
            last_committed_chunk_witness_artifact_id,
            checkpoint_digest,
        })
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    pub fn checkpoint_sequence(&self) -> u64 {
        self.checkpoint_sequence
    }

    pub fn completed_chunk_ordinal(&self) -> ChunkOrdinal {
        self.completed_chunk_ordinal
    }

    pub fn next_chunk_ordinal(&self) -> ChunkOrdinal {
        self.next_chunk_ordinal
    }

    pub fn last_committed_chunk_witness_artifact_id(&self) -> &str {
        &self.last_committed_chunk_witness_artifact_id
    }

    pub fn checkpoint_digest(&self) -> &str {
        &self.checkpoint_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishedBulkProgressCheckpoint {
    program_id: String,
    plan_id: String,
    checkpoint_sequence: u64,
    completed_chunk_ordinal: ChunkOrdinal,
    next_chunk_ordinal: ChunkOrdinal,
    last_committed_chunk_witness_artifact_id: String,
    checkpoint_digest: String,
}

impl PublishedBulkProgressCheckpoint {
    pub(crate) fn new(
        program_id: String,
        plan_id: String,
        checkpoint_sequence: u64,
        completed_chunk_ordinal: ChunkOrdinal,
        next_chunk_ordinal: ChunkOrdinal,
        last_committed_chunk_witness_artifact_id: String,
        checkpoint_digest: String,
    ) -> Self {
        Self {
            program_id,
            plan_id,
            checkpoint_sequence,
            completed_chunk_ordinal,
            next_chunk_ordinal,
            last_committed_chunk_witness_artifact_id,
            checkpoint_digest,
        }
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    pub fn checkpoint_sequence(&self) -> u64 {
        self.checkpoint_sequence
    }

    pub fn completed_chunk_ordinal(&self) -> ChunkOrdinal {
        self.completed_chunk_ordinal
    }

    pub fn next_chunk_ordinal(&self) -> ChunkOrdinal {
        self.next_chunk_ordinal
    }

    pub fn last_committed_chunk_witness_artifact_id(&self) -> &str {
        &self.last_committed_chunk_witness_artifact_id
    }

    pub fn checkpoint_digest(&self) -> &str {
        &self.checkpoint_digest
    }
}
