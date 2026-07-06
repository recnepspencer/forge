use crate::{
    BlobChunkByteRange, BlobChunkProofLeaf, BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial,
};

pub(crate) fn verify_chunk_range(
    expected: &BlobChunkProofLeaf,
    actual: BlobChunkByteRange,
    counters: &mut BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    if expected.byte_range() == actual {
        Ok(())
    } else {
        *counters = counters.record_order_denial();
        Err(BlobStreamingReadDenial::ChunkRangeMismatch {
            ordinal: expected.ordinal(),
            expected: expected.byte_range(),
            actual,
            counters: *counters,
        })
    }
}