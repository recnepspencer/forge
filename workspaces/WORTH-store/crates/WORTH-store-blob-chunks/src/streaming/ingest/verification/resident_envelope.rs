use crate::{BlobStreamingIngestDenial, BlobStreamingWindow};

pub(crate) fn reject_if_exceeded(
    pending_len: usize,
    window: BlobStreamingWindow,
) -> Result<(), BlobStreamingIngestDenial> {
    let window_bytes = pending_len as u64;
    if window_bytes > window.max_resident_bytes() {
        Err(
            BlobStreamingIngestDenial::SourceWindowExceedsResidentEnvelope {
                window_bytes,
                envelope_bytes: window.max_resident_bytes(),
            },
        )
    } else {
        Ok(())
    }
}
