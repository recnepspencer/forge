use super::flush_chunk;
use super::frame_slice;
use crate::{
    BlobChunkOrdinal, BlobChunkSequenceAdmission, BlobStreamingChunkWriter,
    BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial, BlobStreamingWindow,
};

pub(crate) struct BlobStreamingChunkingSession {
    pending: Vec<u8>,
    start_offset: u64,
    ordinal: BlobChunkOrdinal,
    chunk_size: usize,
}

pub(crate) struct BlobStreamingChunkingStep {
    pub(crate) admission: BlobChunkSequenceAdmission,
    pub(crate) counters: BlobStreamingIngestCounterSnapshot,
}

impl BlobStreamingChunkingSession {
    pub(crate) fn new(chunk_size: usize) -> Self {
        Self {
            pending: Vec::with_capacity(chunk_size),
            start_offset: 0,
            ordinal: BlobChunkOrdinal::first(),
            chunk_size,
        }
    }

    pub(crate) const fn ordinal(&self) -> BlobChunkOrdinal {
        self.ordinal
    }

    pub(crate) const fn start_offset(&self) -> u64 {
        self.start_offset
    }

    pub(crate) fn pending_is_full(&self) -> bool {
        self.pending.len() == self.chunk_size
    }

    pub(crate) fn pending_as_slice(&self) -> &[u8] {
        self.pending.as_slice()
    }

    pub(crate) fn extend_pending(&mut self, slice: &[u8]) {
        self.pending.extend_from_slice(slice);
    }

    pub(crate) fn advance_after_chunk(&mut self, bytes: u64) {
        self.start_offset += bytes;
        self.ordinal = self.ordinal.next();
        self.pending.clear();
    }

    pub(crate) fn push_frame_bytes<W>(
        &mut self,
        frame: &[u8],
        window: BlobStreamingWindow,
        declared_total_bytes: u64,
        mut admission: BlobChunkSequenceAdmission,
        writer: &mut W,
        mut counters: BlobStreamingIngestCounterSnapshot,
    ) -> Result<BlobStreamingChunkingStep, BlobStreamingIngestDenial>
    where
        W: BlobStreamingChunkWriter,
    {
        use super::super::verification::{backend_write, resident_envelope, source_frame};

        let mut remaining = frame;
        while !remaining.is_empty() {
            let (slice, rest) =
                frame_slice::take_next_slice(self.chunk_size, self.pending.len(), remaining);
            self.extend_pending(slice);
            remaining = rest;
            counters = frame_slice::observe_pending_residency(counters, self.pending.len());
            resident_envelope::reject_if_exceeded(self.pending.len(), window)?;
            if self.pending_is_full() {
                let written =
                    writer.write_streaming_chunk(self.ordinal, self.pending_as_slice())?;
                backend_write::verify_payload_matches_pending(
                    self.ordinal,
                    &written,
                    self.pending_as_slice(),
                )?;
                admission =
                    flush_chunk::advance_chunk_frontier(self, admission, written, &mut counters)?;
            }
            source_frame::reject_if_offset_exceeds_declared(self.start_offset, declared_total_bytes)?;
        }
        Ok(BlobStreamingChunkingStep {
            admission,
            counters,
        })
    }

    pub(crate) fn finish<W>(
        mut self,
        mut admission: BlobChunkSequenceAdmission,
        writer: &mut W,
        mut counters: BlobStreamingIngestCounterSnapshot,
    ) -> Result<BlobStreamingChunkingStep, BlobStreamingIngestDenial>
    where
        W: BlobStreamingChunkWriter,
    {
        use super::super::verification::backend_write;

        if !self.pending.is_empty() {
            let written =
                writer.write_streaming_chunk(self.ordinal, self.pending_as_slice())?;
            backend_write::verify_payload_matches_pending(
                self.ordinal,
                &written,
                self.pending_as_slice(),
            )?;
            admission = flush_chunk::advance_chunk_frontier(
                &mut self,
                admission,
                written,
                &mut counters,
            )?;
        }
        Ok(BlobStreamingChunkingStep {
            admission,
            counters,
        })
    }
}