use super::super::classification::observation_kind;
use crate::{
    BlobChunkOrdinal, BlobChunkProofLeaf, BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial,
    BlobStreamingReadObservation, BlobStreamingReadRequest,
};

pub(crate) fn expected_leaf_or_deny<'a>(
    request: &'a BlobStreamingReadRequest,
    next_index: usize,
    observation: &BlobStreamingReadObservation,
    counters: &mut BlobStreamingReadCounterSnapshot,
) -> Result<&'a BlobChunkProofLeaf, BlobStreamingReadDenial> {
    match request
        .frontier()
        .proof_frontier()
        .ordered_leaves()
        .get(next_index)
    {
        Some(expected) => Ok(expected),
        None => Err(match observation_kind::observation_ordinal(observation) {
            Some(ordinal) => BlobStreamingReadDenial::ExtraChunk {
                ordinal,
                counters: counters.record_order_denial(),
            },
            None => BlobStreamingReadDenial::MissingChunk {
                ordinal: BlobChunkOrdinal::first(),
                counters: counters.record_missing_chunk_denial(),
            },
        }),
    }
}

pub(crate) fn verify_all_leaves_consumed(
    request: &BlobStreamingReadRequest,
    next_index: usize,
    counters: BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    let frontier = request.frontier().proof_frontier();
    if next_index < frontier.ordered_leaves().len() {
        let counters = counters.record_missing_chunk_denial();
        return Err(BlobStreamingReadDenial::MissingChunk {
            ordinal: frontier.ordered_leaves()[next_index].ordinal(),
            counters,
        });
    }
    Ok(())
}