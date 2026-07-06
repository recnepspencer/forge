use forge_store_budgets::{AllocationEnvelopeSet, CounterEvidenceStrength};
use forge_store_buffer_pool::AllocationReceipt;

use super::bind_resume_session;
use super::super::types::BlobStreamingResumeAdmission;
use super::super::verification::request_match;
use crate::{
    BlobStreamingChunkWriter, BlobStreamingIngest, BlobStreamingIngestDenial,
    BlobStreamingIngestRequest, BlobStreamingPressureAdmission, BlobStreamingSourceFrame,
    BlobStreamingWindow,
};

pub fn run_resumable_streaming_ingest<W>(
    request: BlobStreamingIngestRequest,
    resume_admission: BlobStreamingResumeAdmission,
    window: BlobStreamingWindow,
    allocation: AllocationReceipt,
    envelopes: AllocationEnvelopeSet,
    pressure: BlobStreamingPressureAdmission,
    source_frames: impl IntoIterator<Item = BlobStreamingSourceFrame>,
    writer: &mut W,
    counter_strength: CounterEvidenceStrength,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial>
where
    W: BlobStreamingChunkWriter,
{
    request_match::verify_resume_request_matches(&resume_admission, &request)?;
    let ingest = BlobStreamingIngest::run_bounded(
        request,
        window,
        allocation,
        envelopes,
        pressure,
        source_frames,
        writer,
        counter_strength,
    )?;
    Ok(bind_resume_session::bind_resume_session(ingest, resume_admission))
}