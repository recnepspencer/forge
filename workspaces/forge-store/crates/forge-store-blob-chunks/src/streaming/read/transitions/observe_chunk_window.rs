use super::super::classification::{observation_kind, ObservationKind};
use super::super::verification::{
    chunk_order, chunk_range, corruption_observation, frontier_coverage, logical_content_digest,
    resident_envelope, StreamingReadVerifier,
};
use crate::{
    BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial, BlobStreamingReadObservation,
};

pub(crate) fn observe_chunk_window(
    verifier: &mut StreamingReadVerifier,
    observation: BlobStreamingReadObservation,
    counters: &mut BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    let expected = frontier_coverage::expected_leaf_or_deny(
        &verifier.request,
        verifier.next_index,
        &observation,
        counters,
    )?
    .clone();
    match observation_kind::classify(&observation) {
        ObservationKind::Chunk(chunk) => {
            observe_verified_chunk(verifier, &expected, chunk, counters)
        }
        ObservationKind::ColdUnavailable { ordinal } => {
            chunk_order::verify_chunk_order(&expected, ordinal, counters)?;
            *counters = counters.record_cold_unavailable_denial();
            Err(BlobStreamingReadDenial::ColdChunkUnavailable {
                ordinal,
                counters: *counters,
            })
        }
    }
}

fn observe_verified_chunk(
    verifier: &mut StreamingReadVerifier,
    expected: &crate::BlobChunkProofLeaf,
    chunk: &crate::BlobStreamingReadObservedChunk,
    counters: &mut BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    chunk_order::verify_chunk_order(expected, chunk.ordinal(), counters)?;
    chunk_range::verify_chunk_range(expected, chunk.byte_range(), counters)?;
    resident_envelope::verify_read_resident_envelope(chunk, verifier.window)?;
    *counters = counters.observe_read_window(chunk.payload().bytes_checked());
    corruption_observation::seal_and_deny_corruption(
        &verifier.request,
        &mut verifier.quarantine_authority,
        expected,
        chunk,
        counters,
    )?;
    verifier.logical_content_basis = logical_content_digest::accumulate_chunk_bytes(
        verifier.logical_content_basis,
        chunk.payload().payload_bytes(),
    );
    *counters = counters.record_verified_chunk();
    verifier.advance_index();
    Ok(())
}