use super::super::chunk_movement::{BlobStreamingChunkingSession, BlobStreamingChunkingStep};
use super::super::verification::source_frame;
use crate::{
    BlobChunkSequenceAdmission, BlobStreamingChunkWriter, BlobStreamingIngestCounterSnapshot,
    BlobStreamingIngestDenial, BlobStreamingSourceFrame, BlobStreamingWindow,
};

pub(crate) fn advance_frontier<W>(
    source_frames: impl IntoIterator<Item = BlobStreamingSourceFrame>,
    window: BlobStreamingWindow,
    declared_total_bytes: u64,
    mut admission: BlobChunkSequenceAdmission,
    mut chunking: BlobStreamingChunkingSession,
    mut counters: BlobStreamingIngestCounterSnapshot,
    writer: &mut W,
) -> Result<
    (
        BlobChunkSequenceAdmission,
        BlobStreamingChunkingSession,
        BlobStreamingIngestCounterSnapshot,
    ),
    BlobStreamingIngestDenial,
>
where
    W: BlobStreamingChunkWriter,
{
    for frame in source_frames {
        let frame_bytes = frame.into_bytes();
        let frame_len = frame_bytes.len() as u64;
        source_frame::reject_whole_object_frame(frame_len, declared_total_bytes)?;
        counters = counters.observe_source_window(frame_len, 0);
        let BlobStreamingChunkingStep {
            admission: next_admission,
            counters: next_counters,
        } = chunking.push_frame_bytes(
            &frame_bytes,
            window,
            declared_total_bytes,
            admission,
            writer,
            counters,
        )?;
        admission = next_admission;
        counters = next_counters;
    }
    Ok((admission, chunking, counters))
}