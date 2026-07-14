use super::super::chunk_movement::BlobStreamingChunkingSession;
use crate::{
    AdmittedBlobChunkSequence, BlobChunkSequenceAdmission, BlobStreamingChunkWriter,
    BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial,
};

pub(crate) fn finalize_sequence<W>(
    chunking: BlobStreamingChunkingSession,
    admission: BlobChunkSequenceAdmission,
    counters: BlobStreamingIngestCounterSnapshot,
    writer: &mut W,
) -> Result<
    (
        AdmittedBlobChunkSequence,
        BlobStreamingIngestCounterSnapshot,
    ),
    BlobStreamingIngestDenial,
>
where
    W: BlobStreamingChunkWriter,
{
    let finished = chunking.finish(admission, writer, counters)?;
    let sequence = finished.admission.finish()?;
    Ok((sequence, finished.counters))
}
