use crate::BlobStreamingIngestCounterSnapshot;

pub(crate) fn take_next_slice(
    chunk_size: usize,
    pending_len: usize,
    remaining: &[u8],
) -> (&[u8], &[u8]) {
    let take = chunk_size.saturating_sub(pending_len).min(remaining.len());
    remaining.split_at(take)
}

pub(crate) fn observe_pending_residency(
    counters: BlobStreamingIngestCounterSnapshot,
    pending_len: usize,
) -> BlobStreamingIngestCounterSnapshot {
    counters.observe_residency(pending_len as u64)
}
