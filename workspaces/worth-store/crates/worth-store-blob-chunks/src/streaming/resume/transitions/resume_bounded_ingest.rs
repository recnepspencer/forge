use super::super::types::BlobStreamingResumeAdmission;
use super::super::verification::request_match;
use super::bind_resume_session;
use crate::{
    BlobStreamingChunkWriter, BlobStreamingIngest, BlobStreamingIngestDenial,
    BlobStreamingIngestExecution, BlobStreamingIngestRequest, BlobStreamingSourceFrame,
};

pub fn run_resumable_streaming_ingest<'runtime, W>(
    request: BlobStreamingIngestRequest,
    resume_admission: BlobStreamingResumeAdmission,
    execution: BlobStreamingIngestExecution<'runtime>,
    source_frames: impl IntoIterator<Item = BlobStreamingSourceFrame>,
    writer: &mut W,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial>
where
    W: BlobStreamingChunkWriter,
{
    request_match::verify_resume_request_matches(&resume_admission, &request)?;
    let ingest = BlobStreamingIngest::run_bounded(request, execution, source_frames, writer)?;
    Ok(bind_resume_session::bind_resume_session(
        ingest,
        resume_admission,
    ))
}
