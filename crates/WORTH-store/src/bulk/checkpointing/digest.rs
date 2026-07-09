use crate::bulk::{BulkChunkCommitWitness, ChunkOrdinal};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub(crate) fn compute_checkpoint_digest(
    program_id: &str,
    plan_id: &str,
    checkpoint_sequence: u64,
    completed_chunk_ordinal: ChunkOrdinal,
    next_chunk_ordinal: ChunkOrdinal,
    last_committed_chunk_witness_artifact_id: &str,
) -> String {
    #[derive(Serialize)]
    struct CheckpointDigestInput<'a> {
        program_id: &'a str,
        plan_id: &'a str,
        checkpoint_sequence: u64,
        completed_chunk_ordinal: ChunkOrdinal,
        next_chunk_ordinal: ChunkOrdinal,
        last_committed_chunk_witness_artifact_id: &'a str,
    }

    let payload = serde_json::to_string(&CheckpointDigestInput {
        program_id,
        plan_id,
        checkpoint_sequence,
        completed_chunk_ordinal,
        next_chunk_ordinal,
        last_committed_chunk_witness_artifact_id,
    })
    .expect("checkpoint digest input must serialize deterministically");
    let mut digest = Sha256::new();
    digest.update(payload.as_bytes());
    format!("{:x}", digest.finalize())
}

pub(crate) fn checkpoint_artifact_id(witness: &BulkChunkCommitWitness) -> String {
    format!(
        "bulk-chunk-witness:{}:{}:{}",
        witness.program_id(),
        witness.plan_id(),
        witness.chunk_ordinal().value()
    )
}
