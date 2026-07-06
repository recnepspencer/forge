use super::session::BlobStreamingChunkingSession;
use crate::{
    BlobChunkSequenceAdmission, BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial,
    BlobStreamingWrittenChunk,
};

pub(crate) fn advance_chunk_frontier(
    session: &mut BlobStreamingChunkingSession,
    admission: BlobChunkSequenceAdmission,
    written: BlobStreamingWrittenChunk,
    counters: &mut BlobStreamingIngestCounterSnapshot,
) -> Result<BlobChunkSequenceAdmission, BlobStreamingIngestDenial> {
    use super::super::verification::backend_write;

    let ordinal = session.ordinal();
    let (payload, bytes) = backend_write::verify_backend_observation(ordinal, written)?;
    let admission = admission.push_payload(session.start_offset(), payload)?;
    session.advance_after_chunk(bytes);
    *counters = (*counters).observe_chunk_read().observe_chunk_write();
    Ok(admission)
}