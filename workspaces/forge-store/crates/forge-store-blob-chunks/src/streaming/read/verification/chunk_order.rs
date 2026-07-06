use crate::{BlobChunkOrdinal, BlobChunkProofLeaf, BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial};

pub(crate) fn verify_chunk_order(
    expected: &BlobChunkProofLeaf,
    actual: BlobChunkOrdinal,
    counters: &mut BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    if expected.ordinal() == actual {
        Ok(())
    } else {
        *counters = counters.record_order_denial();
        Err(BlobStreamingReadDenial::ReorderedChunk {
            expected: expected.ordinal(),
            actual,
            counters: *counters,
        })
    }
}