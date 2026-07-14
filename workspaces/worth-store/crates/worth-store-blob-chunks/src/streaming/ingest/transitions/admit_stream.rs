use super::super::admission::pressure::BlobStreamingPressureAdmission;
use super::super::chunk_movement::BlobStreamingChunkingSession;
use super::super::classification::ingest_pressure;
use crate::{
    BlobChunkSequenceAdmission, BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial,
    BlobStreamingIngestRequest,
};

pub(crate) fn admit_stream(
    request: BlobStreamingIngestRequest,
    pressure: BlobStreamingPressureAdmission,
) -> Result<
    (
        BlobChunkSequenceAdmission,
        BlobStreamingChunkingSession,
        BlobStreamingIngestCounterSnapshot,
    ),
    BlobStreamingIngestDenial,
> {
    let counters = ingest_pressure::classify_pressure_outcome(pressure)?.record_allocation();
    let (security_scope, rule, declared_total_bytes) = request.into_parts();
    let chunk_size = rule.chunk_size().bytes() as usize;
    let admission = BlobChunkSequenceAdmission::start(security_scope, rule, declared_total_bytes)?;
    let chunking = BlobStreamingChunkingSession::new(chunk_size);
    Ok((admission, chunking, counters))
}
