use crate::{
    BlobStreamingReadDenial, BlobStreamingReadObservedChunk, BlobStreamingReadWindow,
};

pub(crate) fn verify_read_resident_envelope(
    chunk: &BlobStreamingReadObservedChunk,
    window: BlobStreamingReadWindow,
) -> Result<(), BlobStreamingReadDenial> {
    if chunk.payload().bytes_checked() > window.max_resident_bytes() {
        Err(BlobStreamingReadDenial::ReadWindowExceedsResidentEnvelope {
            window_bytes: chunk.payload().bytes_checked(),
            envelope_bytes: window.max_resident_bytes(),
        })
    } else {
        Ok(())
    }
}